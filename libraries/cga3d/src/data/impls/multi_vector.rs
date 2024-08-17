use crate::traits::GeometricProduct;
use crate::traits::Wedge;
// Note on Operative Statistics:
// Operative Statistics are not a precise predictor of performance or performance comparisons.
// This is due to varying hardware capabilities and compiler optimizations.
// As always, where performance is a concern, there is no substitute for
// real measurements on real work-loads on real hardware.
// Disclaimer aside, enjoy the fun information =)
impl std::ops::Add<AntiScalar> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        1        0        0
    fn add(self, other: AntiScalar) -> Self::Output {
        use crate::elements::*;
        let addition = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self.group0()[0], (self.group0()[1] + other[e12345])]),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e1],
            // e15, e25, e35, e45
            self.group3(),
            // e41, e42, e43
            self.group4(),
            // e415, e425, e435, e321
            self.group5(),
            // e423, e431, e412
            self.group6(),
            // e235, e315, e125
            self.group7(),
            // e4235, e4315, e4125, e3215
            self.group8(),
            // e1234
            self[e35],
            // e12, e31, e23
            self.group10(),
        );
        return addition;
    }
}
impl std::ops::AddAssign<AntiScalar> for MultiVector {
    fn add_assign(&mut self, other: AntiScalar) {
        use crate::elements::*;
        let addition = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self.group0()[0], (self.group0()[1] + other[e12345])]),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e1],
            // e15, e25, e35, e45
            self.group3(),
            // e41, e42, e43
            self.group4(),
            // e415, e425, e435, e321
            self.group5(),
            // e423, e431, e412
            self.group6(),
            // e235, e315, e125
            self.group7(),
            // e4235, e4315, e4125, e3215
            self.group8(),
            // e1234
            self[e35],
            // e12, e31, e23
            self.group10(),
        );
        *self = addition;
    }
}
impl std::ops::Add<Circle> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        2        0        0
    //    simd4        1        0        0
    // Totals...
    // yes simd        3        0        0
    //  no simd       10        0        0
    fn add(self, other: Circle) -> Self::Output {
        use crate::elements::*;
        let addition = MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e1],
            // e15, e25, e35, e45
            self.group3(),
            // e41, e42, e43
            self.group4(),
            // e415, e425, e435, e321
            (self.group5() + other.group1()),
            // e423, e431, e412
            (self.group6() + other.group0()),
            // e235, e315, e125
            (self.group7() + other.group2()),
            // e4235, e4315, e4125, e3215
            self.group8(),
            // e1234
            self[e35],
            // e12, e31, e23
            self.group10(),
        );
        return addition;
    }
}
impl std::ops::AddAssign<Circle> for MultiVector {
    fn add_assign(&mut self, other: Circle) {
        use crate::elements::*;
        let addition = MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e1],
            // e15, e25, e35, e45
            self.group3(),
            // e41, e42, e43
            self.group4(),
            // e415, e425, e435, e321
            (self.group5() + other.group1()),
            // e423, e431, e412
            (self.group6() + other.group0()),
            // e235, e315, e125
            (self.group7() + other.group2()),
            // e4235, e4315, e4125, e3215
            self.group8(),
            // e1234
            self[e35],
            // e12, e31, e23
            self.group10(),
        );
        *self = addition;
    }
}
impl std::ops::Add<CircleRotor> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        0        0
    //    simd3        2        0        0
    //    simd4        1        0        0
    // Totals...
    // yes simd        4        0        0
    //  no simd       11        0        0
    fn add(self, other: CircleRotor) -> Self::Output {
        use crate::elements::*;
        let addition = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self.group0()[0], (self.group0()[1] + other.group2()[3])]),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e1],
            // e15, e25, e35, e45
            self.group3(),
            // e41, e42, e43
            self.group4(),
            // e415, e425, e435, e321
            (self.group5() + other.group1()),
            // e423, e431, e412
            (self.group6() + other.group0()),
            // e235, e315, e125
            (self.group7() + Simd32x3::from([other.group2()[0], other.group2()[1], other.group2()[2]])),
            // e4235, e4315, e4125, e3215
            self.group8(),
            // e1234
            self[e35],
            // e12, e31, e23
            self.group10(),
        );
        return addition;
    }
}
impl std::ops::AddAssign<CircleRotor> for MultiVector {
    fn add_assign(&mut self, other: CircleRotor) {
        use crate::elements::*;
        let addition = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self.group0()[0], (self.group0()[1] + other.group2()[3])]),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e1],
            // e15, e25, e35, e45
            self.group3(),
            // e41, e42, e43
            self.group4(),
            // e415, e425, e435, e321
            (self.group5() + other.group1()),
            // e423, e431, e412
            (self.group6() + other.group0()),
            // e235, e315, e125
            (self.group7() + Simd32x3::from([other.group2()[0], other.group2()[1], other.group2()[2]])),
            // e4235, e4315, e4125, e3215
            self.group8(),
            // e1234
            self[e35],
            // e12, e31, e23
            self.group10(),
        );
        *self = addition;
    }
}
impl std::ops::Add<Dipole> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        2        0        0
    //    simd4        1        0        0
    // Totals...
    // yes simd        3        0        0
    //  no simd       10        0        0
    fn add(self, other: Dipole) -> Self::Output {
        use crate::elements::*;
        let addition = MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e1],
            // e15, e25, e35, e45
            (self.group3() + Simd32x4::from([other.group2()[0], other.group2()[1], other.group2()[2], other.group1()[3]])),
            // e41, e42, e43
            (self.group4() + other.group0()),
            // e415, e425, e435, e321
            self.group5(),
            // e423, e431, e412
            self.group6(),
            // e235, e315, e125
            self.group7(),
            // e4235, e4315, e4125, e3215
            self.group8(),
            // e1234
            self[e35],
            // e12, e31, e23
            (self.group10() + Simd32x3::from([other.group1()[2], other.group1()[1], other.group1()[0]])),
        );
        return addition;
    }
}
impl std::ops::AddAssign<Dipole> for MultiVector {
    fn add_assign(&mut self, other: Dipole) {
        use crate::elements::*;
        let addition = MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e1],
            // e15, e25, e35, e45
            (self.group3() + Simd32x4::from([other.group2()[0], other.group2()[1], other.group2()[2], other.group1()[3]])),
            // e41, e42, e43
            (self.group4() + other.group0()),
            // e415, e425, e435, e321
            self.group5(),
            // e423, e431, e412
            self.group6(),
            // e235, e315, e125
            self.group7(),
            // e4235, e4315, e4125, e3215
            self.group8(),
            // e1234
            self[e35],
            // e12, e31, e23
            (self.group10() + Simd32x3::from([other.group1()[2], other.group1()[1], other.group1()[0]])),
        );
        *self = addition;
    }
}
impl std::ops::Add<DipoleInversion> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        0        0
    //    simd3        2        0        0
    //    simd4        2        0        0
    // Totals...
    // yes simd        5        0        0
    //  no simd       15        0        0
    fn add(self, other: DipoleInversion) -> Self::Output {
        use crate::elements::*;
        let addition = MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e1],
            // e15, e25, e35, e45
            (self.group3() + Simd32x4::from([other.group2()[0], other.group2()[1], other.group2()[2], other.group1()[3]])),
            // e41, e42, e43
            (self.group4() + other.group0()),
            // e415, e425, e435, e321
            self.group5(),
            // e423, e431, e412
            self.group6(),
            // e235, e315, e125
            self.group7(),
            // e4235, e4315, e4125, e3215
            (self.group8() + other.group3()),
            // e1234
            (self[e35] + other.group2()[3]),
            // e12, e31, e23
            (self.group10() + Simd32x3::from([other.group1()[2], other.group1()[1], other.group1()[0]])),
        );
        return addition;
    }
}
impl std::ops::AddAssign<DipoleInversion> for MultiVector {
    fn add_assign(&mut self, other: DipoleInversion) {
        use crate::elements::*;
        let addition = MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e1],
            // e15, e25, e35, e45
            (self.group3() + Simd32x4::from([other.group2()[0], other.group2()[1], other.group2()[2], other.group1()[3]])),
            // e41, e42, e43
            (self.group4() + other.group0()),
            // e415, e425, e435, e321
            self.group5(),
            // e423, e431, e412
            self.group6(),
            // e235, e315, e125
            self.group7(),
            // e4235, e4315, e4125, e3215
            (self.group8() + other.group3()),
            // e1234
            (self[e35] + other.group2()[3]),
            // e12, e31, e23
            (self.group10() + Simd32x3::from([other.group1()[2], other.group1()[1], other.group1()[0]])),
        );
        *self = addition;
    }
}
impl std::ops::Add<DualNum> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        0        0
    fn add(self, other: DualNum) -> Self::Output {
        use crate::elements::*;
        let addition = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self.group0()[0], (self.group0()[1] + other.group0()[1])]),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            (self[e1] + other.group0()[0]),
            // e15, e25, e35, e45
            self.group3(),
            // e41, e42, e43
            self.group4(),
            // e415, e425, e435, e321
            self.group5(),
            // e423, e431, e412
            self.group6(),
            // e235, e315, e125
            self.group7(),
            // e4235, e4315, e4125, e3215
            self.group8(),
            // e1234
            self[e35],
            // e12, e31, e23
            self.group10(),
        );
        return addition;
    }
}
impl std::ops::AddAssign<DualNum> for MultiVector {
    fn add_assign(&mut self, other: DualNum) {
        use crate::elements::*;
        let addition = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self.group0()[0], (self.group0()[1] + other.group0()[1])]),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            (self[e1] + other.group0()[0]),
            // e15, e25, e35, e45
            self.group3(),
            // e41, e42, e43
            self.group4(),
            // e415, e425, e435, e321
            self.group5(),
            // e423, e431, e412
            self.group6(),
            // e235, e315, e125
            self.group7(),
            // e4235, e4315, e4125, e3215
            self.group8(),
            // e1234
            self[e35],
            // e12, e31, e23
            self.group10(),
        );
        *self = addition;
    }
}
impl std::ops::Add<FlatPoint> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        1        0        0
    // no simd        4        0        0
    fn add(self, other: FlatPoint) -> Self::Output {
        use crate::elements::*;
        let addition = MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e1],
            // e15, e25, e35, e45
            (self.group3() + other.group0()),
            // e41, e42, e43
            self.group4(),
            // e415, e425, e435, e321
            self.group5(),
            // e423, e431, e412
            self.group6(),
            // e235, e315, e125
            self.group7(),
            // e4235, e4315, e4125, e3215
            self.group8(),
            // e1234
            self[e35],
            // e12, e31, e23
            self.group10(),
        );
        return addition;
    }
}
impl std::ops::AddAssign<FlatPoint> for MultiVector {
    fn add_assign(&mut self, other: FlatPoint) {
        use crate::elements::*;
        let addition = MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e1],
            // e15, e25, e35, e45
            (self.group3() + other.group0()),
            // e41, e42, e43
            self.group4(),
            // e415, e425, e435, e321
            self.group5(),
            // e423, e431, e412
            self.group6(),
            // e235, e315, e125
            self.group7(),
            // e4235, e4315, e4125, e3215
            self.group8(),
            // e1234
            self[e35],
            // e12, e31, e23
            self.group10(),
        );
        *self = addition;
    }
}
impl std::ops::Add<Flector> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        2        0        0
    // no simd        8        0        0
    fn add(self, other: Flector) -> Self::Output {
        use crate::elements::*;
        let addition = MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e1],
            // e15, e25, e35, e45
            (self.group3() + other.group0()),
            // e41, e42, e43
            self.group4(),
            // e415, e425, e435, e321
            self.group5(),
            // e423, e431, e412
            self.group6(),
            // e235, e315, e125
            self.group7(),
            // e4235, e4315, e4125, e3215
            (self.group8() + other.group1()),
            // e1234
            self[e35],
            // e12, e31, e23
            self.group10(),
        );
        return addition;
    }
}
impl std::ops::AddAssign<Flector> for MultiVector {
    fn add_assign(&mut self, other: Flector) {
        use crate::elements::*;
        let addition = MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e1],
            // e15, e25, e35, e45
            (self.group3() + other.group0()),
            // e41, e42, e43
            self.group4(),
            // e415, e425, e435, e321
            self.group5(),
            // e423, e431, e412
            self.group6(),
            // e235, e315, e125
            self.group7(),
            // e4235, e4315, e4125, e3215
            (self.group8() + other.group1()),
            // e1234
            self[e35],
            // e12, e31, e23
            self.group10(),
        );
        *self = addition;
    }
}
impl std::ops::Add<Line> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        0        0
    //    simd3        1        0        0
    // Totals...
    // yes simd        4        0        0
    //  no simd        6        0        0
    fn add(self, other: Line) -> Self::Output {
        use crate::elements::*;
        let addition = MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e1],
            // e15, e25, e35, e45
            self.group3(),
            // e41, e42, e43
            self.group4(),
            // e415, e425, e435, e321
            Simd32x4::from([
                (self.group5()[0] + other.group0()[0]),
                (self.group5()[1] + other.group0()[1]),
                (self.group5()[2] + other.group0()[2]),
                self.group5()[3],
            ]),
            // e423, e431, e412
            self.group6(),
            // e235, e315, e125
            (self.group7() + other.group1()),
            // e4235, e4315, e4125, e3215
            self.group8(),
            // e1234
            self[e35],
            // e12, e31, e23
            self.group10(),
        );
        return addition;
    }
}
impl std::ops::AddAssign<Line> for MultiVector {
    fn add_assign(&mut self, other: Line) {
        use crate::elements::*;
        let addition = MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e1],
            // e15, e25, e35, e45
            self.group3(),
            // e41, e42, e43
            self.group4(),
            // e415, e425, e435, e321
            Simd32x4::from([
                (self.group5()[0] + other.group0()[0]),
                (self.group5()[1] + other.group0()[1]),
                (self.group5()[2] + other.group0()[2]),
                self.group5()[3],
            ]),
            // e423, e431, e412
            self.group6(),
            // e235, e315, e125
            (self.group7() + other.group1()),
            // e4235, e4315, e4125, e3215
            self.group8(),
            // e1234
            self[e35],
            // e12, e31, e23
            self.group10(),
        );
        *self = addition;
    }
}
impl std::ops::Add<Motor> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        5        0        0
    //    simd3        1        0        0
    // Totals...
    // yes simd        6        0        0
    //  no simd        8        0        0
    fn add(self, other: Motor) -> Self::Output {
        use crate::elements::*;
        let addition = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self.group0()[0], (self.group0()[1] + other.group0()[3])]),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            (self[e1] + other.group1()[3]),
            // e15, e25, e35, e45
            self.group3(),
            // e41, e42, e43
            self.group4(),
            // e415, e425, e435, e321
            Simd32x4::from([
                (self.group5()[0] + other.group0()[0]),
                (self.group5()[1] + other.group0()[1]),
                (self.group5()[2] + other.group0()[2]),
                self.group5()[3],
            ]),
            // e423, e431, e412
            self.group6(),
            // e235, e315, e125
            (self.group7() + Simd32x3::from([other.group1()[0], other.group1()[1], other.group1()[2]])),
            // e4235, e4315, e4125, e3215
            self.group8(),
            // e1234
            self[e35],
            // e12, e31, e23
            self.group10(),
        );
        return addition;
    }
}
impl std::ops::AddAssign<Motor> for MultiVector {
    fn add_assign(&mut self, other: Motor) {
        use crate::elements::*;
        let addition = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self.group0()[0], (self.group0()[1] + other.group0()[3])]),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            (self[e1] + other.group1()[3]),
            // e15, e25, e35, e45
            self.group3(),
            // e41, e42, e43
            self.group4(),
            // e415, e425, e435, e321
            Simd32x4::from([
                (self.group5()[0] + other.group0()[0]),
                (self.group5()[1] + other.group0()[1]),
                (self.group5()[2] + other.group0()[2]),
                self.group5()[3],
            ]),
            // e423, e431, e412
            self.group6(),
            // e235, e315, e125
            (self.group7() + Simd32x3::from([other.group1()[0], other.group1()[1], other.group1()[2]])),
            // e4235, e4315, e4125, e3215
            self.group8(),
            // e1234
            self[e35],
            // e12, e31, e23
            self.group10(),
        );
        *self = addition;
    }
}
impl std::ops::Add<MultiVector> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        0        0
    //    simd2        1        0        0
    //    simd3        4        0        0
    //    simd4        4        0        0
    // Totals...
    // yes simd       11        0        0
    //  no simd       32        0        0
    fn add(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        let addition = MultiVector::from_groups(
            // scalar, e12345
            (self.group0() + other.group0()),
            // e1, e2, e3, e4
            (self.group1() + other.group1()),
            // e5
            (self[e1] + other[e1]),
            // e15, e25, e35, e45
            (self.group3() + other.group3()),
            // e41, e42, e43
            (self.group4() + other.group4()),
            // e415, e425, e435, e321
            (self.group5() + other.group5()),
            // e423, e431, e412
            (self.group6() + other.group6()),
            // e235, e315, e125
            (self.group7() + other.group7()),
            // e4235, e4315, e4125, e3215
            (self.group8() + other.group8()),
            // e1234
            (self[e35] + other[e35]),
            // e12, e31, e23
            (self.group10() + other.group10()),
        );
        return addition;
    }
}
impl std::ops::AddAssign<MultiVector> for MultiVector {
    fn add_assign(&mut self, other: MultiVector) {
        use crate::elements::*;
        let addition = MultiVector::from_groups(
            // scalar, e12345
            (self.group0() + other.group0()),
            // e1, e2, e3, e4
            (self.group1() + other.group1()),
            // e5
            (self[e1] + other[e1]),
            // e15, e25, e35, e45
            (self.group3() + other.group3()),
            // e41, e42, e43
            (self.group4() + other.group4()),
            // e415, e425, e435, e321
            (self.group5() + other.group5()),
            // e423, e431, e412
            (self.group6() + other.group6()),
            // e235, e315, e125
            (self.group7() + other.group7()),
            // e4235, e4315, e4125, e3215
            (self.group8() + other.group8()),
            // e1234
            (self[e35] + other[e35]),
            // e12, e31, e23
            (self.group10() + other.group10()),
        );
        *self = addition;
    }
}
impl std::ops::Add<Plane> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        1        0        0
    // no simd        4        0        0
    fn add(self, other: Plane) -> Self::Output {
        use crate::elements::*;
        let addition = MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e1],
            // e15, e25, e35, e45
            self.group3(),
            // e41, e42, e43
            self.group4(),
            // e415, e425, e435, e321
            self.group5(),
            // e423, e431, e412
            self.group6(),
            // e235, e315, e125
            self.group7(),
            // e4235, e4315, e4125, e3215
            (self.group8() + other.group0()),
            // e1234
            self[e35],
            // e12, e31, e23
            self.group10(),
        );
        return addition;
    }
}
impl std::ops::AddAssign<Plane> for MultiVector {
    fn add_assign(&mut self, other: Plane) {
        use crate::elements::*;
        let addition = MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e1],
            // e15, e25, e35, e45
            self.group3(),
            // e41, e42, e43
            self.group4(),
            // e415, e425, e435, e321
            self.group5(),
            // e423, e431, e412
            self.group6(),
            // e235, e315, e125
            self.group7(),
            // e4235, e4315, e4125, e3215
            (self.group8() + other.group0()),
            // e1234
            self[e35],
            // e12, e31, e23
            self.group10(),
        );
        *self = addition;
    }
}
impl std::ops::Add<RoundPoint> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        0        0
    //    simd4        1        0        0
    // Totals...
    // yes simd        2        0        0
    //  no simd        5        0        0
    fn add(self, other: RoundPoint) -> Self::Output {
        use crate::elements::*;
        let addition = MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            (self.group1() + other.group0()),
            // e5
            (self[e1] + other[e2]),
            // e15, e25, e35, e45
            self.group3(),
            // e41, e42, e43
            self.group4(),
            // e415, e425, e435, e321
            self.group5(),
            // e423, e431, e412
            self.group6(),
            // e235, e315, e125
            self.group7(),
            // e4235, e4315, e4125, e3215
            self.group8(),
            // e1234
            self[e35],
            // e12, e31, e23
            self.group10(),
        );
        return addition;
    }
}
impl std::ops::AddAssign<RoundPoint> for MultiVector {
    fn add_assign(&mut self, other: RoundPoint) {
        use crate::elements::*;
        let addition = MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            (self.group1() + other.group0()),
            // e5
            (self[e1] + other[e2]),
            // e15, e25, e35, e45
            self.group3(),
            // e41, e42, e43
            self.group4(),
            // e415, e425, e435, e321
            self.group5(),
            // e423, e431, e412
            self.group6(),
            // e235, e315, e125
            self.group7(),
            // e4235, e4315, e4125, e3215
            self.group8(),
            // e1234
            self[e35],
            // e12, e31, e23
            self.group10(),
        );
        *self = addition;
    }
}
impl std::ops::Add<Scalar> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        1        0        0
    fn add(self, other: Scalar) -> Self::Output {
        use crate::elements::*;
        let addition = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([(self.group0()[0] + other[scalar]), self.group0()[1]]),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e1],
            // e15, e25, e35, e45
            self.group3(),
            // e41, e42, e43
            self.group4(),
            // e415, e425, e435, e321
            self.group5(),
            // e423, e431, e412
            self.group6(),
            // e235, e315, e125
            self.group7(),
            // e4235, e4315, e4125, e3215
            self.group8(),
            // e1234
            self[e35],
            // e12, e31, e23
            self.group10(),
        );
        return addition;
    }
}
impl std::ops::AddAssign<Scalar> for MultiVector {
    fn add_assign(&mut self, other: Scalar) {
        use crate::elements::*;
        let addition = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([(self.group0()[0] + other[scalar]), self.group0()[1]]),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e1],
            // e15, e25, e35, e45
            self.group3(),
            // e41, e42, e43
            self.group4(),
            // e415, e425, e435, e321
            self.group5(),
            // e423, e431, e412
            self.group6(),
            // e235, e315, e125
            self.group7(),
            // e4235, e4315, e4125, e3215
            self.group8(),
            // e1234
            self[e35],
            // e12, e31, e23
            self.group10(),
        );
        *self = addition;
    }
}
impl std::ops::Add<Sphere> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        0        0
    //    simd4        1        0        0
    // Totals...
    // yes simd        2        0        0
    //  no simd        5        0        0
    fn add(self, other: Sphere) -> Self::Output {
        use crate::elements::*;
        let addition = MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e1],
            // e15, e25, e35, e45
            self.group3(),
            // e41, e42, e43
            self.group4(),
            // e415, e425, e435, e321
            self.group5(),
            // e423, e431, e412
            self.group6(),
            // e235, e315, e125
            self.group7(),
            // e4235, e4315, e4125, e3215
            (self.group8() + other.group0()),
            // e1234
            (self[e35] + other[e4315]),
            // e12, e31, e23
            self.group10(),
        );
        return addition;
    }
}
impl std::ops::AddAssign<Sphere> for MultiVector {
    fn add_assign(&mut self, other: Sphere) {
        use crate::elements::*;
        let addition = MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e1],
            // e15, e25, e35, e45
            self.group3(),
            // e41, e42, e43
            self.group4(),
            // e415, e425, e435, e321
            self.group5(),
            // e423, e431, e412
            self.group6(),
            // e235, e315, e125
            self.group7(),
            // e4235, e4315, e4125, e3215
            (self.group8() + other.group0()),
            // e1234
            (self[e35] + other[e4315]),
            // e12, e31, e23
            self.group10(),
        );
        *self = addition;
    }
}
impl std::ops::Add<VersorEven> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        0        0
    //    simd3        2        0        0
    //    simd4        2        0        0
    // Totals...
    // yes simd        6        0        0
    //  no simd       16        0        0
    fn add(self, other: VersorEven) -> Self::Output {
        use crate::elements::*;
        let addition = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self.group0()[0], (self.group0()[1] + other.group0()[3])]),
            // e1, e2, e3, e4
            (self.group1() + other.group3()),
            // e5
            (self[e1] + other.group2()[3]),
            // e15, e25, e35, e45
            self.group3(),
            // e41, e42, e43
            self.group4(),
            // e415, e425, e435, e321
            (self.group5() + other.group1()),
            // e423, e431, e412
            (self.group6() + Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]])),
            // e235, e315, e125
            (self.group7() + Simd32x3::from([other.group2()[0], other.group2()[1], other.group2()[2]])),
            // e4235, e4315, e4125, e3215
            self.group8(),
            // e1234
            self[e35],
            // e12, e31, e23
            self.group10(),
        );
        return addition;
    }
}
impl std::ops::AddAssign<VersorEven> for MultiVector {
    fn add_assign(&mut self, other: VersorEven) {
        use crate::elements::*;
        let addition = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self.group0()[0], (self.group0()[1] + other.group0()[3])]),
            // e1, e2, e3, e4
            (self.group1() + other.group3()),
            // e5
            (self[e1] + other.group2()[3]),
            // e15, e25, e35, e45
            self.group3(),
            // e41, e42, e43
            self.group4(),
            // e415, e425, e435, e321
            (self.group5() + other.group1()),
            // e423, e431, e412
            (self.group6() + Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]])),
            // e235, e315, e125
            (self.group7() + Simd32x3::from([other.group2()[0], other.group2()[1], other.group2()[2]])),
            // e4235, e4315, e4125, e3215
            self.group8(),
            // e1234
            self[e35],
            // e12, e31, e23
            self.group10(),
        );
        *self = addition;
    }
}
impl std::ops::Add<VersorOdd> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        0        0
    //    simd3        2        0        0
    //    simd4        2        0        0
    // Totals...
    // yes simd        6        0        0
    //  no simd       16        0        0
    fn add(self, other: VersorOdd) -> Self::Output {
        use crate::elements::*;
        let addition = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([(self.group0()[0] + other.group0()[3]), self.group0()[1]]),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e1],
            // e15, e25, e35, e45
            (self.group3() + Simd32x4::from([other.group2()[0], other.group2()[1], other.group2()[2], other.group1()[3]])),
            // e41, e42, e43
            (self.group4() + Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]])),
            // e415, e425, e435, e321
            self.group5(),
            // e423, e431, e412
            self.group6(),
            // e235, e315, e125
            self.group7(),
            // e4235, e4315, e4125, e3215
            (self.group8() + other.group3()),
            // e1234
            (self[e35] + other.group2()[3]),
            // e12, e31, e23
            (self.group10() + Simd32x3::from([other.group1()[2], other.group1()[1], other.group1()[0]])),
        );
        return addition;
    }
}
impl std::ops::AddAssign<VersorOdd> for MultiVector {
    fn add_assign(&mut self, other: VersorOdd) {
        use crate::elements::*;
        let addition = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([(self.group0()[0] + other.group0()[3]), self.group0()[1]]),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e1],
            // e15, e25, e35, e45
            (self.group3() + Simd32x4::from([other.group2()[0], other.group2()[1], other.group2()[2], other.group1()[3]])),
            // e41, e42, e43
            (self.group4() + Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]])),
            // e415, e425, e435, e321
            self.group5(),
            // e423, e431, e412
            self.group6(),
            // e235, e315, e125
            self.group7(),
            // e4235, e4315, e4125, e3215
            (self.group8() + other.group3()),
            // e1234
            (self[e35] + other.group2()[3]),
            // e12, e31, e23
            (self.group10() + Simd32x3::from([other.group1()[2], other.group1()[1], other.group1()[0]])),
        );
        *self = addition;
    }
}
impl std::ops::BitXor<AntiScalar> for MultiVector {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn bitxor(self, other: AntiScalar) -> Self::Output {
        use crate::elements::*;
        return self.wedge(other);
    }
}
impl std::ops::BitXor<Circle> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       16       22        0
    //    simd3        0        2        0
    //    simd4        2        3        0
    // Totals...
    // yes simd       18       27        0
    //  no simd       24       40        0
    fn bitxor(self, other: Circle) -> Self::Output {
        use crate::elements::*;
        return self.wedge(other);
    }
}
impl std::ops::BitXorAssign<Circle> for MultiVector {
    fn bitxor_assign(&mut self, other: Circle) {
        use crate::elements::*;
        *self = self.wedge(other);
    }
}
impl std::ops::BitXor<CircleRotor> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       17       23        0
    //    simd3        0        2        0
    //    simd4        2        3        0
    // Totals...
    // yes simd       19       28        0
    //  no simd       25       41        0
    fn bitxor(self, other: CircleRotor) -> Self::Output {
        use crate::elements::*;
        return self.wedge(other);
    }
}
impl std::ops::BitXorAssign<CircleRotor> for MultiVector {
    fn bitxor_assign(&mut self, other: CircleRotor) {
        use crate::elements::*;
        *self = self.wedge(other);
    }
}
impl std::ops::BitXor<Dipole> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       34       44        0
    //    simd3        4        8        0
    //    simd4        2        3        0
    // Totals...
    // yes simd       40       55        0
    //  no simd       54       80        0
    fn bitxor(self, other: Dipole) -> Self::Output {
        use crate::elements::*;
        return self.wedge(other);
    }
}
impl std::ops::BitXorAssign<Dipole> for MultiVector {
    fn bitxor_assign(&mut self, other: Dipole) {
        use crate::elements::*;
        *self = self.wedge(other);
    }
}
impl std::ops::BitXor<DipoleInversion> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       39       52        0
    //    simd3        3        6        0
    //    simd4        4        5        0
    // Totals...
    // yes simd       46       63        0
    //  no simd       64       90        0
    fn bitxor(self, other: DipoleInversion) -> Self::Output {
        use crate::elements::*;
        return self.wedge(other);
    }
}
impl std::ops::BitXorAssign<DipoleInversion> for MultiVector {
    fn bitxor_assign(&mut self, other: DipoleInversion) {
        use crate::elements::*;
        *self = self.wedge(other);
    }
}
impl std::ops::BitXor<DualNum> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        6        0
    //    simd3        0        1        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        1        9        0
    //  no simd        1       17        0
    fn bitxor(self, other: DualNum) -> Self::Output {
        use crate::elements::*;
        return self.wedge(other);
    }
}
impl std::ops::BitXorAssign<DualNum> for MultiVector {
    fn bitxor_assign(&mut self, other: DualNum) {
        use crate::elements::*;
        *self = self.wedge(other);
    }
}
impl std::ops::BitXor<FlatPoint> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       13       24        0
    //    simd4        1        2        0
    // Totals...
    // yes simd       14       26        0
    //  no simd       17       32        0
    fn bitxor(self, other: FlatPoint) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXorAssign<FlatPoint> for MultiVector {
    fn bitxor_assign(&mut self, other: FlatPoint) {
        *self = self.wedge(other);
    }
}
impl std::ops::BitXor<Flector> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       17       28        0
    //    simd4        2        3        0
    // Totals...
    // yes simd       19       31        0
    //  no simd       25       40        0
    fn bitxor(self, other: Flector) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXorAssign<Flector> for MultiVector {
    fn bitxor_assign(&mut self, other: Flector) {
        *self = self.wedge(other);
    }
}
impl std::ops::BitXor<Line> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        9       17        0
    //    simd3        0        1        0
    //    simd4        1        1        0
    // Totals...
    // yes simd       10       19        0
    //  no simd       13       24        0
    fn bitxor(self, other: Line) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXorAssign<Line> for MultiVector {
    fn bitxor_assign(&mut self, other: Line) {
        *self = self.wedge(other);
    }
}
impl std::ops::BitXor<Motor> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       14       23        0
    //    simd3        1        2        0
    //    simd4        2        3        0
    // Totals...
    // yes simd       17       28        0
    //  no simd       25       41        0
    fn bitxor(self, other: Motor) -> Self::Output {
        use crate::elements::*;
        return self.wedge(other);
    }
}
impl std::ops::BitXorAssign<Motor> for MultiVector {
    fn bitxor_assign(&mut self, other: Motor) {
        use crate::elements::*;
        *self = self.wedge(other);
    }
}
impl std::ops::BitXor<MultiVector> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       99      117        0
    //    simd3       16       18        0
    //    simd4       16       18        0
    // Totals...
    // yes simd      131      153        0
    //  no simd      211      243        0
    fn bitxor(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        return self.wedge(other);
    }
}
impl std::ops::BitXorAssign<MultiVector> for MultiVector {
    fn bitxor_assign(&mut self, other: MultiVector) {
        use crate::elements::*;
        *self = self.wedge(other);
    }
}
impl std::ops::BitXor<Plane> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        4        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        3        5        0
    //  no simd        3        8        0
    fn bitxor(self, other: Plane) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXorAssign<Plane> for MultiVector {
    fn bitxor_assign(&mut self, other: Plane) {
        *self = self.wedge(other);
    }
}
impl std::ops::BitXor<RoundPoint> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       21       38        0
    //    simd3        4        6        0
    //    simd4        4        6        0
    // Totals...
    // yes simd       29       50        0
    //  no simd       49       80        0
    fn bitxor(self, other: RoundPoint) -> Self::Output {
        use crate::elements::*;
        return self.wedge(other);
    }
}
impl std::ops::BitXorAssign<RoundPoint> for MultiVector {
    fn bitxor_assign(&mut self, other: RoundPoint) {
        use crate::elements::*;
        *self = self.wedge(other);
    }
}
impl std::ops::BitXor<Scalar> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        2        0
    //    simd2        0        1        0
    //    simd3        0        4        0
    //    simd4        0        4        0
    // Totals...
    // yes simd        0       11        0
    //  no simd        0       32        0
    fn bitxor(self, other: Scalar) -> Self::Output {
        use crate::elements::*;
        return self.wedge(other);
    }
}
impl std::ops::BitXorAssign<Scalar> for MultiVector {
    fn bitxor_assign(&mut self, other: Scalar) {
        use crate::elements::*;
        *self = self.wedge(other);
    }
}
impl std::ops::BitXor<Sphere> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        6        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        4        7        0
    //  no simd        4       10        0
    fn bitxor(self, other: Sphere) -> Self::Output {
        use crate::elements::*;
        return self.wedge(other);
    }
}
impl std::ops::BitXorAssign<Sphere> for MultiVector {
    fn bitxor_assign(&mut self, other: Sphere) {
        use crate::elements::*;
        *self = self.wedge(other);
    }
}
impl std::ops::BitXor<VersorEven> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       44       61        0
    //    simd3        6        8        0
    //    simd4        7        9        0
    // Totals...
    // yes simd       57       78        0
    //  no simd       90      121        0
    fn bitxor(self, other: VersorEven) -> Self::Output {
        use crate::elements::*;
        return self.wedge(other);
    }
}
impl std::ops::BitXorAssign<VersorEven> for MultiVector {
    fn bitxor_assign(&mut self, other: VersorEven) {
        use crate::elements::*;
        *self = self.wedge(other);
    }
}
impl std::ops::BitXor<VersorOdd> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       44       62        0
    //    simd3        6        8        0
    //    simd4        7        9        0
    // Totals...
    // yes simd       57       79        0
    //  no simd       90      122        0
    fn bitxor(self, other: VersorOdd) -> Self::Output {
        use crate::elements::*;
        return self.wedge(other);
    }
}
impl std::ops::BitXorAssign<VersorOdd> for MultiVector {
    fn bitxor_assign(&mut self, other: VersorOdd) {
        use crate::elements::*;
        *self = self.wedge(other);
    }
}

impl From<AntiScalar> for MultiVector {
    fn from(anti_scalar: AntiScalar) -> Self {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, anti_scalar[e12345]]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e15, e25, e35, e45
            Simd32x4::from(0.0),
            // e41, e42, e43
            Simd32x3::from(0.0),
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
            // e12, e31, e23
            Simd32x3::from(0.0),
        );
    }
}

impl From<Circle> for MultiVector {
    fn from(circle: Circle) -> Self {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e15, e25, e35, e45
            Simd32x4::from(0.0),
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from([circle[e415], circle[e425], circle[e435], circle[e321]]),
            // e423, e431, e412
            Simd32x3::from([circle[e423], circle[e431], circle[e412]]),
            // e235, e315, e125
            Simd32x3::from([circle[e235], circle[e315], circle[e125]]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(0.0),
            // e1234
            0.0,
            // e12, e31, e23
            Simd32x3::from(0.0),
        );
    }
}

impl From<CircleRotor> for MultiVector {
    fn from(circle_rotor: CircleRotor) -> Self {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, circle_rotor[e12345]]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e15, e25, e35, e45
            Simd32x4::from(0.0),
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from([circle_rotor[e415], circle_rotor[e425], circle_rotor[e435], circle_rotor[e321]]),
            // e423, e431, e412
            Simd32x3::from([circle_rotor[e423], circle_rotor[e431], circle_rotor[e412]]),
            // e235, e315, e125
            Simd32x3::from([circle_rotor[e235], circle_rotor[e315], circle_rotor[e125]]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(0.0),
            // e1234
            0.0,
            // e12, e31, e23
            Simd32x3::from(0.0),
        );
    }
}

impl From<Dipole> for MultiVector {
    fn from(dipole: Dipole) -> Self {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e15, e25, e35, e45
            Simd32x4::from([dipole[e15], dipole[e25], dipole[e35], dipole[e45]]),
            // e41, e42, e43
            Simd32x3::from([dipole[e41], dipole[e42], dipole[e43]]),
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
            // e12, e31, e23
            Simd32x3::from([dipole[e12], dipole[e31], dipole[e23]]),
        );
    }
}

impl From<DipoleInversion> for MultiVector {
    fn from(dipole_inversion: DipoleInversion) -> Self {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e15, e25, e35, e45
            Simd32x4::from([dipole_inversion[e15], dipole_inversion[e25], dipole_inversion[e35], dipole_inversion[e45]]),
            // e41, e42, e43
            Simd32x3::from([dipole_inversion[e41], dipole_inversion[e42], dipole_inversion[e43]]),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([dipole_inversion[e4235], dipole_inversion[e4315], dipole_inversion[e4125], dipole_inversion[e3215]]),
            // e1234
            dipole_inversion[e1234],
            // e12, e31, e23
            Simd32x3::from([dipole_inversion[e12], dipole_inversion[e31], dipole_inversion[e23]]),
        );
    }
}

impl From<DualNum> for MultiVector {
    fn from(dual_num: DualNum) -> Self {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, dual_num[e12345]]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            dual_num[e5],
            // e15, e25, e35, e45
            Simd32x4::from(0.0),
            // e41, e42, e43
            Simd32x3::from(0.0),
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
            // e12, e31, e23
            Simd32x3::from(0.0),
        );
    }
}

impl From<FlatPoint> for MultiVector {
    fn from(flat_point: FlatPoint) -> Self {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e15, e25, e35, e45
            Simd32x4::from([flat_point[e15], flat_point[e25], flat_point[e35], flat_point[e45]]),
            // e41, e42, e43
            Simd32x3::from(0.0),
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
            // e12, e31, e23
            Simd32x3::from(0.0),
        );
    }
}

impl From<Flector> for MultiVector {
    fn from(flector: Flector) -> Self {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e15, e25, e35, e45
            Simd32x4::from([flector[e15], flector[e25], flector[e35], flector[e45]]),
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([flector[e4235], flector[e4315], flector[e4125], flector[e3215]]),
            // e1234
            0.0,
            // e12, e31, e23
            Simd32x3::from(0.0),
        );
    }
}

impl From<Line> for MultiVector {
    fn from(line: Line) -> Self {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e15, e25, e35, e45
            Simd32x4::from(0.0),
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from([line[e415], line[e425], line[e435], 0.0]),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from([line[e235], line[e315], line[e125]]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(0.0),
            // e1234
            0.0,
            // e12, e31, e23
            Simd32x3::from(0.0),
        );
    }
}

impl From<Motor> for MultiVector {
    fn from(motor: Motor) -> Self {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, motor[e12345]]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            motor[e5],
            // e15, e25, e35, e45
            Simd32x4::from(0.0),
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from([motor[e415], motor[e425], motor[e435], 0.0]),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from([motor[e235], motor[e315], motor[e125]]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(0.0),
            // e1234
            0.0,
            // e12, e31, e23
            Simd32x3::from(0.0),
        );
    }
}

impl From<Plane> for MultiVector {
    fn from(plane: Plane) -> Self {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e15, e25, e35, e45
            Simd32x4::from(0.0),
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([plane[e4235], plane[e4315], plane[e4125], plane[e3215]]),
            // e1234
            0.0,
            // e12, e31, e23
            Simd32x3::from(0.0),
        );
    }
}

impl From<RoundPoint> for MultiVector {
    fn from(round_point: RoundPoint) -> Self {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from([round_point[e1], round_point[e2], round_point[e3], round_point[e4]]),
            // e5
            round_point[e5],
            // e15, e25, e35, e45
            Simd32x4::from(0.0),
            // e41, e42, e43
            Simd32x3::from(0.0),
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
            // e12, e31, e23
            Simd32x3::from(0.0),
        );
    }
}

impl From<Scalar> for MultiVector {
    fn from(scalar: Scalar) -> Self {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([scalar[scalar], 0.0]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e15, e25, e35, e45
            Simd32x4::from(0.0),
            // e41, e42, e43
            Simd32x3::from(0.0),
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
            // e12, e31, e23
            Simd32x3::from(0.0),
        );
    }
}

impl From<Sphere> for MultiVector {
    fn from(sphere: Sphere) -> Self {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e15, e25, e35, e45
            Simd32x4::from(0.0),
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([sphere[e4235], sphere[e4315], sphere[e4125], sphere[e3215]]),
            // e1234
            sphere[e1234],
            // e12, e31, e23
            Simd32x3::from(0.0),
        );
    }
}

impl From<VersorEven> for MultiVector {
    fn from(versor_even: VersorEven) -> Self {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, versor_even[e12345]]),
            // e1, e2, e3, e4
            Simd32x4::from([versor_even[e1], versor_even[e2], versor_even[e3], versor_even[e4]]),
            // e5
            versor_even[e5],
            // e15, e25, e35, e45
            Simd32x4::from(0.0),
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from([versor_even[e415], versor_even[e425], versor_even[e435], versor_even[e321]]),
            // e423, e431, e412
            Simd32x3::from([versor_even[e423], versor_even[e431], versor_even[e412]]),
            // e235, e315, e125
            Simd32x3::from([versor_even[e235], versor_even[e315], versor_even[e125]]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(0.0),
            // e1234
            0.0,
            // e12, e31, e23
            Simd32x3::from(0.0),
        );
    }
}

impl From<VersorOdd> for MultiVector {
    fn from(versor_odd: VersorOdd) -> Self {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([versor_odd[scalar], 0.0]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e15, e25, e35, e45
            Simd32x4::from([versor_odd[e15], versor_odd[e25], versor_odd[e35], versor_odd[e45]]),
            // e41, e42, e43
            Simd32x3::from([versor_odd[e41], versor_odd[e42], versor_odd[e43]]),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([versor_odd[e4235], versor_odd[e4315], versor_odd[e4125], versor_odd[e3215]]),
            // e1234
            versor_odd[e1234],
            // e12, e31, e23
            Simd32x3::from([versor_odd[e12], versor_odd[e31], versor_odd[e23]]),
        );
    }
}
impl std::ops::Mul<AntiScalar> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        3        0
    //    simd2        0        2        0
    //    simd3        0        6        0
    //    simd4        0        8        0
    // Totals...
    // yes simd        0       19        0
    //  no simd        0       57        0
    fn mul(self, other: AntiScalar) -> Self::Output {
        use crate::elements::*;
        return self.geometric_product(other);
    }
}
impl std::ops::MulAssign<AntiScalar> for MultiVector {
    fn mul_assign(&mut self, other: AntiScalar) {
        use crate::elements::*;
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<Circle> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32      108      134        0
    //    simd2        1        1        0
    //    simd3       34       36        0
    //    simd4       19       19        0
    // Totals...
    // yes simd      162      190        0
    //  no simd      288      320        0
    fn mul(self, other: Circle) -> Self::Output {
        use crate::elements::*;
        return self.geometric_product(other);
    }
}
impl std::ops::MulAssign<Circle> for MultiVector {
    fn mul_assign(&mut self, other: Circle) {
        use crate::elements::*;
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<CircleRotor> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32      102      128        0
    //    simd2        1        1        0
    //    simd3       36       38        0
    //    simd4       27       27        0
    // Totals...
    // yes simd      166      194        0
    //  no simd      320      352        0
    fn mul(self, other: CircleRotor) -> Self::Output {
        use crate::elements::*;
        return self.geometric_product(other);
    }
}
impl std::ops::MulAssign<CircleRotor> for MultiVector {
    fn mul_assign(&mut self, other: CircleRotor) {
        use crate::elements::*;
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<Dipole> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32      118      136        0
    //    simd2        9       11        0
    //    simd3       36       40        0
    //    simd4       11       11        0
    // Totals...
    // yes simd      174      198        0
    //  no simd      288      322        0
    fn mul(self, other: Dipole) -> Self::Output {
        use crate::elements::*;
        return self.geometric_product(other);
    }
}
impl std::ops::MulAssign<Dipole> for MultiVector {
    fn mul_assign(&mut self, other: Dipole) {
        use crate::elements::*;
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<DipoleInversion> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32      132      158        0
    //    simd2       11       11        0
    //    simd3       54       56        0
    //    simd4       33       33        0
    // Totals...
    // yes simd      230      258        0
    //  no simd      448      480        0
    fn mul(self, other: DipoleInversion) -> Self::Output {
        use crate::elements::*;
        return self.geometric_product(other);
    }
}
impl std::ops::MulAssign<DipoleInversion> for MultiVector {
    fn mul_assign(&mut self, other: DipoleInversion) {
        use crate::elements::*;
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<DualNum> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       11       28        0
    //    simd3        3        8        0
    //    simd4        3        5        0
    // Totals...
    // yes simd       17       41        0
    //  no simd       32       72        0
    fn mul(self, other: DualNum) -> Self::Output {
        use crate::elements::*;
        return self.geometric_product(other);
    }
}
impl std::ops::MulAssign<DualNum> for MultiVector {
    fn mul_assign(&mut self, other: DualNum) {
        use crate::elements::*;
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<FlatPoint> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       35       55        0
    //    simd2        3        5        0
    //    simd3        8       11        0
    //    simd4        8        8        0
    // Totals...
    // yes simd       54       79        0
    //  no simd       97      130        0
    fn mul(self, other: FlatPoint) -> Self::Output {
        use crate::elements::*;
        return self.geometric_product(other);
    }
}
impl std::ops::MulAssign<FlatPoint> for MultiVector {
    fn mul_assign(&mut self, other: FlatPoint) {
        use crate::elements::*;
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<Flector> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       78      105        0
    //    simd2        4        4        0
    //    simd3       22       24        0
    //    simd4       18       18        0
    // Totals...
    // yes simd      122      151        0
    //  no simd      224      257        0
    fn mul(self, other: Flector) -> Self::Output {
        use crate::elements::*;
        return self.geometric_product(other);
    }
}
impl std::ops::MulAssign<Flector> for MultiVector {
    fn mul_assign(&mut self, other: Flector) {
        use crate::elements::*;
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<Line> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       89      109        0
    //    simd3       17       21        0
    //    simd4        5        5        0
    // Totals...
    // yes simd      111      135        0
    //  no simd      160      192        0
    fn mul(self, other: Line) -> Self::Output {
        use crate::elements::*;
        return self.geometric_product(other);
    }
}
impl std::ops::MulAssign<Line> for MultiVector {
    fn mul_assign(&mut self, other: Line) {
        use crate::elements::*;
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<Motor> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       65       86        0
    //    simd3       20       22        0
    //    simd4       26       26        0
    // Totals...
    // yes simd      111      134        0
    //  no simd      229      256        0
    fn mul(self, other: Motor) -> Self::Output {
        use crate::elements::*;
        return self.geometric_product(other);
    }
}
impl std::ops::MulAssign<Motor> for MultiVector {
    fn mul_assign(&mut self, other: Motor) {
        use crate::elements::*;
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<MultiVector> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32      334      360        0
    //    simd2       16       16        0
    //    simd3      114      116        0
    //    simd4       71       71        0
    // Totals...
    // yes simd      535      563        0
    //  no simd      992     1024        0
    fn mul(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        return self.geometric_product(other);
    }
}
impl std::ops::MulAssign<MultiVector> for MultiVector {
    fn mul_assign(&mut self, other: MultiVector) {
        use crate::elements::*;
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<Plane> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       32       56        0
    //    simd2        1        1        0
    //    simd3       10       13        0
    //    simd4        8        8        0
    // Totals...
    // yes simd       51       78        0
    //  no simd       96      129        0
    fn mul(self, other: Plane) -> Self::Output {
        use crate::elements::*;
        return self.geometric_product(other);
    }
}
impl std::ops::MulAssign<Plane> for MultiVector {
    fn mul_assign(&mut self, other: Plane) {
        use crate::elements::*;
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<RoundPoint> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       28       50        0
    //    simd2        3        3        0
    //    simd3       14       16        0
    //    simd4       13       15        0
    // Totals...
    // yes simd       58       84        0
    //  no simd      128      164        0
    fn mul(self, other: RoundPoint) -> Self::Output {
        use crate::elements::*;
        return self.geometric_product(other);
    }
}
impl std::ops::MulAssign<RoundPoint> for MultiVector {
    fn mul_assign(&mut self, other: RoundPoint) {
        use crate::elements::*;
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<Scalar> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        2        0
    //    simd2        0        1        0
    //    simd3        0        4        0
    //    simd4        0        4        0
    // Totals...
    // yes simd        0       11        0
    //  no simd        0       32        0
    fn mul(self, other: Scalar) -> Self::Output {
        use crate::elements::*;
        return self.geometric_product(other);
    }
}
impl std::ops::MulAssign<Scalar> for MultiVector {
    fn mul_assign(&mut self, other: Scalar) {
        use crate::elements::*;
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<Sphere> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       35       58        0
    //    simd2        2        2        0
    //    simd3       15       18        0
    //    simd4       11       11        0
    // Totals...
    // yes simd       63       89        0
    //  no simd      128      160        0
    fn mul(self, other: Sphere) -> Self::Output {
        use crate::elements::*;
        return self.geometric_product(other);
    }
}
impl std::ops::MulAssign<Sphere> for MultiVector {
    fn mul_assign(&mut self, other: Sphere) {
        use crate::elements::*;
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<VersorEven> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32      148      180        0
    //    simd2        4        4        0
    //    simd3       48       48        0
    //    simd4       45       45        0
    // Totals...
    // yes simd      245      277        0
    //  no simd      480      512        0
    fn mul(self, other: VersorEven) -> Self::Output {
        use crate::elements::*;
        return self.geometric_product(other);
    }
}
impl std::ops::MulAssign<VersorEven> for MultiVector {
    fn mul_assign(&mut self, other: VersorEven) {
        use crate::elements::*;
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<VersorOdd> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32      136      168        0
    //    simd2       12       12        0
    //    simd3       52       52        0
    //    simd4       41       41        0
    // Totals...
    // yes simd      241      273        0
    //  no simd      480      512        0
    fn mul(self, other: VersorOdd) -> Self::Output {
        use crate::elements::*;
        return self.geometric_product(other);
    }
}
impl std::ops::MulAssign<VersorOdd> for MultiVector {
    fn mul_assign(&mut self, other: VersorOdd) {
        use crate::elements::*;
        *self = self.geometric_product(other);
    }
}
impl std::ops::Neg for MultiVector {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        2        0
    //    simd2        0        1        0
    //    simd3        0        4        0
    //    simd4        0        4        0
    // Totals...
    // yes simd        0       11        0
    //  no simd        0       32        0
    fn neg(self) -> Self {
        use crate::elements::*;
        let negation = MultiVector::from_groups(
            // scalar, e12345
            (self.group0() * Simd32x2::from(-1.0)),
            // e1, e2, e3, e4
            (self.group1() * Simd32x4::from(-1.0)),
            // e5
            (self[e1] * -1.0),
            // e15, e25, e35, e45
            (self.group3() * Simd32x4::from(-1.0)),
            // e41, e42, e43
            (self.group4() * Simd32x3::from(-1.0)),
            // e415, e425, e435, e321
            (self.group5() * Simd32x4::from(-1.0)),
            // e423, e431, e412
            (self.group6() * Simd32x3::from(-1.0)),
            // e235, e315, e125
            (self.group7() * Simd32x3::from(-1.0)),
            // e4235, e4315, e4125, e3215
            (self.group8() * Simd32x4::from(-1.0)),
            // e1234
            (self[e35] * -1.0),
            // e12, e31, e23
            (self.group10() * Simd32x3::from(-1.0)),
        );
        return negation;
    }
}
impl std::ops::Not for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0       10        0
    //    simd3        0        2        0
    // Totals...
    // yes simd        0       12        0
    //  no simd        0       16        0
    fn not(self) -> Self::Output {
        use crate::elements::*;
        let dual = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([(self.group0()[1] * -1.0), self.group0()[0]]),
            // e1, e2, e3, e4
            Simd32x4::from([(self.group8()[0] * -1.0), (self.group8()[1] * -1.0), (self.group8()[2] * -1.0), self[e35]]),
            // e5
            self.group8()[3],
            // e15, e25, e35, e45
            Simd32x4::from([self.group7()[0], self.group7()[1], self.group7()[2], (self.group5()[3] * -1.0)]),
            // e41, e42, e43
            self.group6(),
            // e415, e425, e435, e321
            Simd32x4::from([(self.group10()[2] * -1.0), (self.group10()[1] * -1.0), (self.group10()[0] * -1.0), self.group3()[3]]),
            // e423, e431, e412
            (self.group4() * Simd32x3::from(-1.0)),
            // e235, e315, e125
            (Simd32x3::from([self.group3()[0], self.group3()[1], self.group3()[2]]) * Simd32x3::from(-1.0)),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([self.group1()[0], self.group1()[1], self.group1()[2], (self[e1] * -1.0)]),
            // e1234
            (self.group1()[3] * -1.0),
            // e12, e31, e23
            Simd32x3::from([self.group5()[2], self.group5()[1], self.group5()[0]]),
        );
        return dual;
    }
}
impl std::ops::Sub<AntiScalar> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        1        0        0
    fn sub(self, other: AntiScalar) -> Self::Output {
        use crate::elements::*;
        let subtraction = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self.group0()[0], (self.group0()[1] - other[e12345])]),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e1],
            // e15, e25, e35, e45
            self.group3(),
            // e41, e42, e43
            self.group4(),
            // e415, e425, e435, e321
            self.group5(),
            // e423, e431, e412
            self.group6(),
            // e235, e315, e125
            self.group7(),
            // e4235, e4315, e4125, e3215
            self.group8(),
            // e1234
            self[e35],
            // e12, e31, e23
            self.group10(),
        );
        return subtraction;
    }
}
impl std::ops::SubAssign<AntiScalar> for MultiVector {
    fn sub_assign(&mut self, other: AntiScalar) {
        use crate::elements::*;
        let subtraction = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self.group0()[0], (self.group0()[1] - other[e12345])]),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e1],
            // e15, e25, e35, e45
            self.group3(),
            // e41, e42, e43
            self.group4(),
            // e415, e425, e435, e321
            self.group5(),
            // e423, e431, e412
            self.group6(),
            // e235, e315, e125
            self.group7(),
            // e4235, e4315, e4125, e3215
            self.group8(),
            // e1234
            self[e35],
            // e12, e31, e23
            self.group10(),
        );
        *self = subtraction;
    }
}
impl std::ops::Sub<Circle> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        2        0        0
    //    simd4        1        0        0
    // Totals...
    // yes simd        3        0        0
    //  no simd       10        0        0
    fn sub(self, other: Circle) -> Self::Output {
        use crate::elements::*;
        let subtraction = MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e1],
            // e15, e25, e35, e45
            self.group3(),
            // e41, e42, e43
            self.group4(),
            // e415, e425, e435, e321
            (self.group5() - other.group1()),
            // e423, e431, e412
            (self.group6() - other.group0()),
            // e235, e315, e125
            (self.group7() - other.group2()),
            // e4235, e4315, e4125, e3215
            self.group8(),
            // e1234
            self[e35],
            // e12, e31, e23
            self.group10(),
        );
        return subtraction;
    }
}
impl std::ops::SubAssign<Circle> for MultiVector {
    fn sub_assign(&mut self, other: Circle) {
        use crate::elements::*;
        let subtraction = MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e1],
            // e15, e25, e35, e45
            self.group3(),
            // e41, e42, e43
            self.group4(),
            // e415, e425, e435, e321
            (self.group5() - other.group1()),
            // e423, e431, e412
            (self.group6() - other.group0()),
            // e235, e315, e125
            (self.group7() - other.group2()),
            // e4235, e4315, e4125, e3215
            self.group8(),
            // e1234
            self[e35],
            // e12, e31, e23
            self.group10(),
        );
        *self = subtraction;
    }
}
impl std::ops::Sub<CircleRotor> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        0        0
    //    simd3        2        0        0
    //    simd4        1        0        0
    // Totals...
    // yes simd        4        0        0
    //  no simd       11        0        0
    fn sub(self, other: CircleRotor) -> Self::Output {
        use crate::elements::*;
        let subtraction = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self.group0()[0], (self.group0()[1] - other.group2()[3])]),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e1],
            // e15, e25, e35, e45
            self.group3(),
            // e41, e42, e43
            self.group4(),
            // e415, e425, e435, e321
            (self.group5() - other.group1()),
            // e423, e431, e412
            (self.group6() - other.group0()),
            // e235, e315, e125
            (self.group7() - Simd32x3::from([other.group2()[0], other.group2()[1], other.group2()[2]])),
            // e4235, e4315, e4125, e3215
            self.group8(),
            // e1234
            self[e35],
            // e12, e31, e23
            self.group10(),
        );
        return subtraction;
    }
}
impl std::ops::SubAssign<CircleRotor> for MultiVector {
    fn sub_assign(&mut self, other: CircleRotor) {
        use crate::elements::*;
        let subtraction = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self.group0()[0], (self.group0()[1] - other.group2()[3])]),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e1],
            // e15, e25, e35, e45
            self.group3(),
            // e41, e42, e43
            self.group4(),
            // e415, e425, e435, e321
            (self.group5() - other.group1()),
            // e423, e431, e412
            (self.group6() - other.group0()),
            // e235, e315, e125
            (self.group7() - Simd32x3::from([other.group2()[0], other.group2()[1], other.group2()[2]])),
            // e4235, e4315, e4125, e3215
            self.group8(),
            // e1234
            self[e35],
            // e12, e31, e23
            self.group10(),
        );
        *self = subtraction;
    }
}
impl std::ops::Sub<Dipole> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        2        0        0
    //    simd4        1        0        0
    // Totals...
    // yes simd        3        0        0
    //  no simd       10        0        0
    fn sub(self, other: Dipole) -> Self::Output {
        use crate::elements::*;
        let subtraction = MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e1],
            // e15, e25, e35, e45
            (self.group3() - Simd32x4::from([other.group2()[0], other.group2()[1], other.group2()[2], other.group1()[3]])),
            // e41, e42, e43
            (self.group4() - other.group0()),
            // e415, e425, e435, e321
            self.group5(),
            // e423, e431, e412
            self.group6(),
            // e235, e315, e125
            self.group7(),
            // e4235, e4315, e4125, e3215
            self.group8(),
            // e1234
            self[e35],
            // e12, e31, e23
            (self.group10() - Simd32x3::from([other.group1()[2], other.group1()[1], other.group1()[0]])),
        );
        return subtraction;
    }
}
impl std::ops::SubAssign<Dipole> for MultiVector {
    fn sub_assign(&mut self, other: Dipole) {
        use crate::elements::*;
        let subtraction = MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e1],
            // e15, e25, e35, e45
            (self.group3() - Simd32x4::from([other.group2()[0], other.group2()[1], other.group2()[2], other.group1()[3]])),
            // e41, e42, e43
            (self.group4() - other.group0()),
            // e415, e425, e435, e321
            self.group5(),
            // e423, e431, e412
            self.group6(),
            // e235, e315, e125
            self.group7(),
            // e4235, e4315, e4125, e3215
            self.group8(),
            // e1234
            self[e35],
            // e12, e31, e23
            (self.group10() - Simd32x3::from([other.group1()[2], other.group1()[1], other.group1()[0]])),
        );
        *self = subtraction;
    }
}
impl std::ops::Sub<DipoleInversion> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        0        0
    //    simd3        2        0        0
    //    simd4        2        0        0
    // Totals...
    // yes simd        5        0        0
    //  no simd       15        0        0
    fn sub(self, other: DipoleInversion) -> Self::Output {
        use crate::elements::*;
        let subtraction = MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e1],
            // e15, e25, e35, e45
            (self.group3() - Simd32x4::from([other.group2()[0], other.group2()[1], other.group2()[2], other.group1()[3]])),
            // e41, e42, e43
            (self.group4() - other.group0()),
            // e415, e425, e435, e321
            self.group5(),
            // e423, e431, e412
            self.group6(),
            // e235, e315, e125
            self.group7(),
            // e4235, e4315, e4125, e3215
            (self.group8() - other.group3()),
            // e1234
            (self[e35] - other.group2()[3]),
            // e12, e31, e23
            (self.group10() - Simd32x3::from([other.group1()[2], other.group1()[1], other.group1()[0]])),
        );
        return subtraction;
    }
}
impl std::ops::SubAssign<DipoleInversion> for MultiVector {
    fn sub_assign(&mut self, other: DipoleInversion) {
        use crate::elements::*;
        let subtraction = MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e1],
            // e15, e25, e35, e45
            (self.group3() - Simd32x4::from([other.group2()[0], other.group2()[1], other.group2()[2], other.group1()[3]])),
            // e41, e42, e43
            (self.group4() - other.group0()),
            // e415, e425, e435, e321
            self.group5(),
            // e423, e431, e412
            self.group6(),
            // e235, e315, e125
            self.group7(),
            // e4235, e4315, e4125, e3215
            (self.group8() - other.group3()),
            // e1234
            (self[e35] - other.group2()[3]),
            // e12, e31, e23
            (self.group10() - Simd32x3::from([other.group1()[2], other.group1()[1], other.group1()[0]])),
        );
        *self = subtraction;
    }
}
impl std::ops::Sub<DualNum> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        0        0
    fn sub(self, other: DualNum) -> Self::Output {
        use crate::elements::*;
        let subtraction = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self.group0()[0], (self.group0()[1] - other.group0()[1])]),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            (self[e1] - other.group0()[0]),
            // e15, e25, e35, e45
            self.group3(),
            // e41, e42, e43
            self.group4(),
            // e415, e425, e435, e321
            self.group5(),
            // e423, e431, e412
            self.group6(),
            // e235, e315, e125
            self.group7(),
            // e4235, e4315, e4125, e3215
            self.group8(),
            // e1234
            self[e35],
            // e12, e31, e23
            self.group10(),
        );
        return subtraction;
    }
}
impl std::ops::SubAssign<DualNum> for MultiVector {
    fn sub_assign(&mut self, other: DualNum) {
        use crate::elements::*;
        let subtraction = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self.group0()[0], (self.group0()[1] - other.group0()[1])]),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            (self[e1] - other.group0()[0]),
            // e15, e25, e35, e45
            self.group3(),
            // e41, e42, e43
            self.group4(),
            // e415, e425, e435, e321
            self.group5(),
            // e423, e431, e412
            self.group6(),
            // e235, e315, e125
            self.group7(),
            // e4235, e4315, e4125, e3215
            self.group8(),
            // e1234
            self[e35],
            // e12, e31, e23
            self.group10(),
        );
        *self = subtraction;
    }
}
impl std::ops::Sub<FlatPoint> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        1        0        0
    // no simd        4        0        0
    fn sub(self, other: FlatPoint) -> Self::Output {
        use crate::elements::*;
        let subtraction = MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e1],
            // e15, e25, e35, e45
            (self.group3() - other.group0()),
            // e41, e42, e43
            self.group4(),
            // e415, e425, e435, e321
            self.group5(),
            // e423, e431, e412
            self.group6(),
            // e235, e315, e125
            self.group7(),
            // e4235, e4315, e4125, e3215
            self.group8(),
            // e1234
            self[e35],
            // e12, e31, e23
            self.group10(),
        );
        return subtraction;
    }
}
impl std::ops::SubAssign<FlatPoint> for MultiVector {
    fn sub_assign(&mut self, other: FlatPoint) {
        use crate::elements::*;
        let subtraction = MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e1],
            // e15, e25, e35, e45
            (self.group3() - other.group0()),
            // e41, e42, e43
            self.group4(),
            // e415, e425, e435, e321
            self.group5(),
            // e423, e431, e412
            self.group6(),
            // e235, e315, e125
            self.group7(),
            // e4235, e4315, e4125, e3215
            self.group8(),
            // e1234
            self[e35],
            // e12, e31, e23
            self.group10(),
        );
        *self = subtraction;
    }
}
impl std::ops::Sub<Flector> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        2        0        0
    // no simd        8        0        0
    fn sub(self, other: Flector) -> Self::Output {
        use crate::elements::*;
        let subtraction = MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e1],
            // e15, e25, e35, e45
            (self.group3() - other.group0()),
            // e41, e42, e43
            self.group4(),
            // e415, e425, e435, e321
            self.group5(),
            // e423, e431, e412
            self.group6(),
            // e235, e315, e125
            self.group7(),
            // e4235, e4315, e4125, e3215
            (self.group8() - other.group1()),
            // e1234
            self[e35],
            // e12, e31, e23
            self.group10(),
        );
        return subtraction;
    }
}
impl std::ops::SubAssign<Flector> for MultiVector {
    fn sub_assign(&mut self, other: Flector) {
        use crate::elements::*;
        let subtraction = MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e1],
            // e15, e25, e35, e45
            (self.group3() - other.group0()),
            // e41, e42, e43
            self.group4(),
            // e415, e425, e435, e321
            self.group5(),
            // e423, e431, e412
            self.group6(),
            // e235, e315, e125
            self.group7(),
            // e4235, e4315, e4125, e3215
            (self.group8() - other.group1()),
            // e1234
            self[e35],
            // e12, e31, e23
            self.group10(),
        );
        *self = subtraction;
    }
}
impl std::ops::Sub<Line> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        0        0
    //    simd3        1        0        0
    // Totals...
    // yes simd        4        0        0
    //  no simd        6        0        0
    fn sub(self, other: Line) -> Self::Output {
        use crate::elements::*;
        let subtraction = MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e1],
            // e15, e25, e35, e45
            self.group3(),
            // e41, e42, e43
            self.group4(),
            // e415, e425, e435, e321
            Simd32x4::from([
                (self.group5()[0] - other.group0()[0]),
                (self.group5()[1] - other.group0()[1]),
                (self.group5()[2] - other.group0()[2]),
                self.group5()[3],
            ]),
            // e423, e431, e412
            self.group6(),
            // e235, e315, e125
            (self.group7() - other.group1()),
            // e4235, e4315, e4125, e3215
            self.group8(),
            // e1234
            self[e35],
            // e12, e31, e23
            self.group10(),
        );
        return subtraction;
    }
}
impl std::ops::SubAssign<Line> for MultiVector {
    fn sub_assign(&mut self, other: Line) {
        use crate::elements::*;
        let subtraction = MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e1],
            // e15, e25, e35, e45
            self.group3(),
            // e41, e42, e43
            self.group4(),
            // e415, e425, e435, e321
            Simd32x4::from([
                (self.group5()[0] - other.group0()[0]),
                (self.group5()[1] - other.group0()[1]),
                (self.group5()[2] - other.group0()[2]),
                self.group5()[3],
            ]),
            // e423, e431, e412
            self.group6(),
            // e235, e315, e125
            (self.group7() - other.group1()),
            // e4235, e4315, e4125, e3215
            self.group8(),
            // e1234
            self[e35],
            // e12, e31, e23
            self.group10(),
        );
        *self = subtraction;
    }
}
impl std::ops::Sub<Motor> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        5        0        0
    //    simd3        1        0        0
    // Totals...
    // yes simd        6        0        0
    //  no simd        8        0        0
    fn sub(self, other: Motor) -> Self::Output {
        use crate::elements::*;
        let subtraction = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self.group0()[0], (self.group0()[1] - other.group0()[3])]),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            (self[e1] - other.group1()[3]),
            // e15, e25, e35, e45
            self.group3(),
            // e41, e42, e43
            self.group4(),
            // e415, e425, e435, e321
            Simd32x4::from([
                (self.group5()[0] - other.group0()[0]),
                (self.group5()[1] - other.group0()[1]),
                (self.group5()[2] - other.group0()[2]),
                self.group5()[3],
            ]),
            // e423, e431, e412
            self.group6(),
            // e235, e315, e125
            (self.group7() - Simd32x3::from([other.group1()[0], other.group1()[1], other.group1()[2]])),
            // e4235, e4315, e4125, e3215
            self.group8(),
            // e1234
            self[e35],
            // e12, e31, e23
            self.group10(),
        );
        return subtraction;
    }
}
impl std::ops::SubAssign<Motor> for MultiVector {
    fn sub_assign(&mut self, other: Motor) {
        use crate::elements::*;
        let subtraction = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self.group0()[0], (self.group0()[1] - other.group0()[3])]),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            (self[e1] - other.group1()[3]),
            // e15, e25, e35, e45
            self.group3(),
            // e41, e42, e43
            self.group4(),
            // e415, e425, e435, e321
            Simd32x4::from([
                (self.group5()[0] - other.group0()[0]),
                (self.group5()[1] - other.group0()[1]),
                (self.group5()[2] - other.group0()[2]),
                self.group5()[3],
            ]),
            // e423, e431, e412
            self.group6(),
            // e235, e315, e125
            (self.group7() - Simd32x3::from([other.group1()[0], other.group1()[1], other.group1()[2]])),
            // e4235, e4315, e4125, e3215
            self.group8(),
            // e1234
            self[e35],
            // e12, e31, e23
            self.group10(),
        );
        *self = subtraction;
    }
}
impl std::ops::Sub<MultiVector> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        0        0
    //    simd2        1        0        0
    //    simd3        4        0        0
    //    simd4        4        0        0
    // Totals...
    // yes simd       11        0        0
    //  no simd       32        0        0
    fn sub(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        let subtraction = MultiVector::from_groups(
            // scalar, e12345
            (self.group0() - other.group0()),
            // e1, e2, e3, e4
            (self.group1() - other.group1()),
            // e5
            (self[e1] - other[e1]),
            // e15, e25, e35, e45
            (self.group3() - other.group3()),
            // e41, e42, e43
            (self.group4() - other.group4()),
            // e415, e425, e435, e321
            (self.group5() - other.group5()),
            // e423, e431, e412
            (self.group6() - other.group6()),
            // e235, e315, e125
            (self.group7() - other.group7()),
            // e4235, e4315, e4125, e3215
            (self.group8() - other.group8()),
            // e1234
            (self[e35] - other[e35]),
            // e12, e31, e23
            (self.group10() - other.group10()),
        );
        return subtraction;
    }
}
impl std::ops::SubAssign<MultiVector> for MultiVector {
    fn sub_assign(&mut self, other: MultiVector) {
        use crate::elements::*;
        let subtraction = MultiVector::from_groups(
            // scalar, e12345
            (self.group0() - other.group0()),
            // e1, e2, e3, e4
            (self.group1() - other.group1()),
            // e5
            (self[e1] - other[e1]),
            // e15, e25, e35, e45
            (self.group3() - other.group3()),
            // e41, e42, e43
            (self.group4() - other.group4()),
            // e415, e425, e435, e321
            (self.group5() - other.group5()),
            // e423, e431, e412
            (self.group6() - other.group6()),
            // e235, e315, e125
            (self.group7() - other.group7()),
            // e4235, e4315, e4125, e3215
            (self.group8() - other.group8()),
            // e1234
            (self[e35] - other[e35]),
            // e12, e31, e23
            (self.group10() - other.group10()),
        );
        *self = subtraction;
    }
}
impl std::ops::Sub<Plane> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        1        0        0
    // no simd        4        0        0
    fn sub(self, other: Plane) -> Self::Output {
        use crate::elements::*;
        let subtraction = MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e1],
            // e15, e25, e35, e45
            self.group3(),
            // e41, e42, e43
            self.group4(),
            // e415, e425, e435, e321
            self.group5(),
            // e423, e431, e412
            self.group6(),
            // e235, e315, e125
            self.group7(),
            // e4235, e4315, e4125, e3215
            (self.group8() - other.group0()),
            // e1234
            self[e35],
            // e12, e31, e23
            self.group10(),
        );
        return subtraction;
    }
}
impl std::ops::SubAssign<Plane> for MultiVector {
    fn sub_assign(&mut self, other: Plane) {
        use crate::elements::*;
        let subtraction = MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e1],
            // e15, e25, e35, e45
            self.group3(),
            // e41, e42, e43
            self.group4(),
            // e415, e425, e435, e321
            self.group5(),
            // e423, e431, e412
            self.group6(),
            // e235, e315, e125
            self.group7(),
            // e4235, e4315, e4125, e3215
            (self.group8() - other.group0()),
            // e1234
            self[e35],
            // e12, e31, e23
            self.group10(),
        );
        *self = subtraction;
    }
}
impl std::ops::Sub<RoundPoint> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        0        0
    //    simd4        1        0        0
    // Totals...
    // yes simd        2        0        0
    //  no simd        5        0        0
    fn sub(self, other: RoundPoint) -> Self::Output {
        use crate::elements::*;
        let subtraction = MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            (self.group1() - other.group0()),
            // e5
            (self[e1] - other[e2]),
            // e15, e25, e35, e45
            self.group3(),
            // e41, e42, e43
            self.group4(),
            // e415, e425, e435, e321
            self.group5(),
            // e423, e431, e412
            self.group6(),
            // e235, e315, e125
            self.group7(),
            // e4235, e4315, e4125, e3215
            self.group8(),
            // e1234
            self[e35],
            // e12, e31, e23
            self.group10(),
        );
        return subtraction;
    }
}
impl std::ops::SubAssign<RoundPoint> for MultiVector {
    fn sub_assign(&mut self, other: RoundPoint) {
        use crate::elements::*;
        let subtraction = MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            (self.group1() - other.group0()),
            // e5
            (self[e1] - other[e2]),
            // e15, e25, e35, e45
            self.group3(),
            // e41, e42, e43
            self.group4(),
            // e415, e425, e435, e321
            self.group5(),
            // e423, e431, e412
            self.group6(),
            // e235, e315, e125
            self.group7(),
            // e4235, e4315, e4125, e3215
            self.group8(),
            // e1234
            self[e35],
            // e12, e31, e23
            self.group10(),
        );
        *self = subtraction;
    }
}
impl std::ops::Sub<Scalar> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        1        0        0
    fn sub(self, other: Scalar) -> Self::Output {
        use crate::elements::*;
        let subtraction = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([(self.group0()[0] - other[scalar]), self.group0()[1]]),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e1],
            // e15, e25, e35, e45
            self.group3(),
            // e41, e42, e43
            self.group4(),
            // e415, e425, e435, e321
            self.group5(),
            // e423, e431, e412
            self.group6(),
            // e235, e315, e125
            self.group7(),
            // e4235, e4315, e4125, e3215
            self.group8(),
            // e1234
            self[e35],
            // e12, e31, e23
            self.group10(),
        );
        return subtraction;
    }
}
impl std::ops::SubAssign<Scalar> for MultiVector {
    fn sub_assign(&mut self, other: Scalar) {
        use crate::elements::*;
        let subtraction = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([(self.group0()[0] - other[scalar]), self.group0()[1]]),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e1],
            // e15, e25, e35, e45
            self.group3(),
            // e41, e42, e43
            self.group4(),
            // e415, e425, e435, e321
            self.group5(),
            // e423, e431, e412
            self.group6(),
            // e235, e315, e125
            self.group7(),
            // e4235, e4315, e4125, e3215
            self.group8(),
            // e1234
            self[e35],
            // e12, e31, e23
            self.group10(),
        );
        *self = subtraction;
    }
}
impl std::ops::Sub<Sphere> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        0        0
    //    simd4        1        0        0
    // Totals...
    // yes simd        2        0        0
    //  no simd        5        0        0
    fn sub(self, other: Sphere) -> Self::Output {
        use crate::elements::*;
        let subtraction = MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e1],
            // e15, e25, e35, e45
            self.group3(),
            // e41, e42, e43
            self.group4(),
            // e415, e425, e435, e321
            self.group5(),
            // e423, e431, e412
            self.group6(),
            // e235, e315, e125
            self.group7(),
            // e4235, e4315, e4125, e3215
            (self.group8() - other.group0()),
            // e1234
            (self[e35] - other[e4315]),
            // e12, e31, e23
            self.group10(),
        );
        return subtraction;
    }
}
impl std::ops::SubAssign<Sphere> for MultiVector {
    fn sub_assign(&mut self, other: Sphere) {
        use crate::elements::*;
        let subtraction = MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e1],
            // e15, e25, e35, e45
            self.group3(),
            // e41, e42, e43
            self.group4(),
            // e415, e425, e435, e321
            self.group5(),
            // e423, e431, e412
            self.group6(),
            // e235, e315, e125
            self.group7(),
            // e4235, e4315, e4125, e3215
            (self.group8() - other.group0()),
            // e1234
            (self[e35] - other[e4315]),
            // e12, e31, e23
            self.group10(),
        );
        *self = subtraction;
    }
}
impl std::ops::Sub<VersorEven> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        0        0
    //    simd3        2        0        0
    //    simd4        2        0        0
    // Totals...
    // yes simd        6        0        0
    //  no simd       16        0        0
    fn sub(self, other: VersorEven) -> Self::Output {
        use crate::elements::*;
        let subtraction = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self.group0()[0], (self.group0()[1] - other.group0()[3])]),
            // e1, e2, e3, e4
            (self.group1() - other.group3()),
            // e5
            (self[e1] - other.group2()[3]),
            // e15, e25, e35, e45
            self.group3(),
            // e41, e42, e43
            self.group4(),
            // e415, e425, e435, e321
            (self.group5() - other.group1()),
            // e423, e431, e412
            (self.group6() - Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]])),
            // e235, e315, e125
            (self.group7() - Simd32x3::from([other.group2()[0], other.group2()[1], other.group2()[2]])),
            // e4235, e4315, e4125, e3215
            self.group8(),
            // e1234
            self[e35],
            // e12, e31, e23
            self.group10(),
        );
        return subtraction;
    }
}
impl std::ops::SubAssign<VersorEven> for MultiVector {
    fn sub_assign(&mut self, other: VersorEven) {
        use crate::elements::*;
        let subtraction = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self.group0()[0], (self.group0()[1] - other.group0()[3])]),
            // e1, e2, e3, e4
            (self.group1() - other.group3()),
            // e5
            (self[e1] - other.group2()[3]),
            // e15, e25, e35, e45
            self.group3(),
            // e41, e42, e43
            self.group4(),
            // e415, e425, e435, e321
            (self.group5() - other.group1()),
            // e423, e431, e412
            (self.group6() - Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]])),
            // e235, e315, e125
            (self.group7() - Simd32x3::from([other.group2()[0], other.group2()[1], other.group2()[2]])),
            // e4235, e4315, e4125, e3215
            self.group8(),
            // e1234
            self[e35],
            // e12, e31, e23
            self.group10(),
        );
        *self = subtraction;
    }
}
impl std::ops::Sub<VersorOdd> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        0        0
    //    simd3        2        0        0
    //    simd4        2        0        0
    // Totals...
    // yes simd        6        0        0
    //  no simd       16        0        0
    fn sub(self, other: VersorOdd) -> Self::Output {
        use crate::elements::*;
        let subtraction = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([(self.group0()[0] - other.group0()[3]), self.group0()[1]]),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e1],
            // e15, e25, e35, e45
            (self.group3() - Simd32x4::from([other.group2()[0], other.group2()[1], other.group2()[2], other.group1()[3]])),
            // e41, e42, e43
            (self.group4() - Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]])),
            // e415, e425, e435, e321
            self.group5(),
            // e423, e431, e412
            self.group6(),
            // e235, e315, e125
            self.group7(),
            // e4235, e4315, e4125, e3215
            (self.group8() - other.group3()),
            // e1234
            (self[e35] - other.group2()[3]),
            // e12, e31, e23
            (self.group10() - Simd32x3::from([other.group1()[2], other.group1()[1], other.group1()[0]])),
        );
        return subtraction;
    }
}
impl std::ops::SubAssign<VersorOdd> for MultiVector {
    fn sub_assign(&mut self, other: VersorOdd) {
        use crate::elements::*;
        let subtraction = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([(self.group0()[0] - other.group0()[3]), self.group0()[1]]),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e1],
            // e15, e25, e35, e45
            (self.group3() - Simd32x4::from([other.group2()[0], other.group2()[1], other.group2()[2], other.group1()[3]])),
            // e41, e42, e43
            (self.group4() - Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]])),
            // e415, e425, e435, e321
            self.group5(),
            // e423, e431, e412
            self.group6(),
            // e235, e315, e125
            self.group7(),
            // e4235, e4315, e4125, e3215
            (self.group8() - other.group3()),
            // e1234
            (self[e35] - other.group2()[3]),
            // e12, e31, e23
            (self.group10() - Simd32x3::from([other.group1()[2], other.group1()[1], other.group1()[0]])),
        );
        *self = subtraction;
    }
}
