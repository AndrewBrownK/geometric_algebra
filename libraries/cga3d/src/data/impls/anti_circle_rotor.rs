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
//  Average:        19      23       0
//  Maximum:       229     262       0
//
//  No SIMD:   add/sub     mul     div
//  Minimum:         0       0       0
//   Median:         4       8       0
//  Average:        22      29       0
//  Maximum:       320     353       0
impl std::ops::Add<AntiCircleRotor> for AntiCircleRotor {
    type Output = AntiCircleRotor;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       11        0        0
    fn add(self, other: AntiCircleRotor) -> Self::Output {
        use crate::elements::*;
        return AntiCircleRotor::from_groups(
            // e41, e42, e43
            Simd32x3::from([other[e41] + self[e41], other[e42] + self[e42], other[e43] + self[e43]]),
            // e23, e31, e12, e45
            Simd32x4::from([other[e23] + self[e23], other[e31] + self[e31], other[e12] + self[e12], other[e45] + self[e45]]),
            // e15, e25, e35, scalar
            Simd32x4::from([other[e15] + self[e15], other[e25] + self[e25], other[e35] + self[e35], other[scalar] + self[scalar]]),
        );
    }
}
impl std::ops::AddAssign<AntiCircleRotor> for AntiCircleRotor {
    fn add_assign(&mut self, other: AntiCircleRotor) {
        use crate::elements::*;
        *self = AntiCircleRotor::from_groups(
            // e41, e42, e43
            Simd32x3::from([other[e41] + self[e41], other[e42] + self[e42], other[e43] + self[e43]]),
            // e23, e31, e12, e45
            Simd32x4::from([other[e23] + self[e23], other[e31] + self[e31], other[e12] + self[e12], other[e45] + self[e45]]),
            // e15, e25, e35, scalar
            Simd32x4::from([other[e15] + self[e15], other[e25] + self[e25], other[e35] + self[e35], other[scalar] + self[scalar]]),
        );
    }
}
impl std::ops::Add<AntiDipoleInversion> for AntiCircleRotor {
    type Output = MultiVector;
    fn add(self, other: AntiDipoleInversion) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self[scalar], 0.0]),
            // e1, e2, e3, e4
            Simd32x4::from([other[e1], other[e2], other[e3], other[e4]]),
            // e5
            other[e5],
            // e15, e25, e35, e45
            Simd32x4::from([self[e15], self[e25], self[e35], self[e45]]),
            // e41, e42, e43
            self.group0(),
            // e23, e31, e12
            self.group1().truncate_to_3(),
            // e415, e425, e435, e321
            other.group1(),
            // e423, e431, e412
            other.group0(),
            // e235, e315, e125
            other.group2().truncate_to_3(),
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
        use crate::elements::*;
        return VersorOdd::from_groups(
            // e41, e42, e43, scalar
            crate::swizzle!(self.group0(), 0, 1, 2).extend_to_4(self[scalar] + other[scalar]),
            // e23, e31, e12, e45
            self.group1(),
            // e15, e25, e35, e1234
            Simd32x4::from([self[e15], self[e25], self[e35], 0.0]),
            // e4235, e4315, e4125, e3215
            Simd32x3::from(0.0).extend_to_4(other[e3215]),
        );
    }
}
impl std::ops::Add<AntiFlatPoint> for AntiCircleRotor {
    type Output = MultiVector;
    fn add(self, other: AntiFlatPoint) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self[scalar], 0.0]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e15, e25, e35, e45
            Simd32x4::from([self[e15], self[e25], self[e35], self[e45]]),
            // e41, e42, e43
            self.group0(),
            // e23, e31, e12
            self.group1().truncate_to_3(),
            // e415, e425, e435, e321
            Simd32x3::from(0.0).extend_to_4(other[e321]),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            other.group0().truncate_to_3(),
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
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self[scalar], 0.0]),
            // e1, e2, e3, e4
            Simd32x4::from([other[e1], other[e2], other[e3], 0.0]),
            // e5
            other[e5],
            // e15, e25, e35, e45
            Simd32x4::from([self[e15], self[e25], self[e35], self[e45]]),
            // e41, e42, e43
            self.group0(),
            // e23, e31, e12
            self.group1().truncate_to_3(),
            // e415, e425, e435, e321
            Simd32x3::from(0.0).extend_to_4(other[e321]),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            other.group0().truncate_to_3(),
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
        use crate::elements::*;
        return AntiCircleRotor::from_groups(
            // e41, e42, e43
            self.group0(),
            // e23, e31, e12, e45
            Simd32x4::from([self[e23] + other[e23], self[e31] + other[e31], self[e12] + other[e12], self[e45]]),
            // e15, e25, e35, scalar
            Simd32x4::from([self[e15] + other[e15], self[e25] + other[e25], self[e35] + other[e35], self[scalar]]),
        );
    }
}
impl std::ops::AddAssign<AntiLine> for AntiCircleRotor {
    fn add_assign(&mut self, other: AntiLine) {
        use crate::elements::*;
        *self = AntiCircleRotor::from_groups(
            // e41, e42, e43
            self.group0(),
            // e23, e31, e12, e45
            Simd32x4::from([self[e23] + other[e23], self[e31] + other[e31], self[e12] + other[e12], self[e45]]),
            // e15, e25, e35, scalar
            Simd32x4::from([self[e15] + other[e15], self[e25] + other[e25], self[e35] + other[e35], self[scalar]]),
        );
    }
}
impl std::ops::Add<AntiMotor> for AntiCircleRotor {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        7        0        0
    fn add(self, other: AntiMotor) -> Self::Output {
        use crate::elements::*;
        return VersorOdd::from_groups(
            // e41, e42, e43, scalar
            crate::swizzle!(self.group0(), 0, 1, 2).extend_to_4(self[scalar] + other[scalar]),
            // e23, e31, e12, e45
            Simd32x4::from([self[e23] + other[e23], self[e31] + other[e31], self[e12] + other[e12], self[e45]]),
            // e15, e25, e35, e1234
            Simd32x4::from([self[e15] + other[e15], self[e25] + other[e25], self[e35] + other[e35], 0.0]),
            // e4235, e4315, e4125, e3215
            Simd32x3::from(0.0).extend_to_4(other[e3215]),
        );
    }
}
impl std::ops::Add<AntiPlane> for AntiCircleRotor {
    type Output = MultiVector;
    fn add(self, other: AntiPlane) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self[scalar], 0.0]),
            // e1, e2, e3, e4
            Simd32x4::from([other[e1], other[e2], other[e3], 0.0]),
            // e5
            other[e5],
            // e15, e25, e35, e45
            Simd32x4::from([self[e15], self[e25], self[e35], self[e45]]),
            // e41, e42, e43
            self.group0(),
            // e23, e31, e12
            self.group1().truncate_to_3(),
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
            Simd32x2::from([self[scalar], other[e12345]]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e15, e25, e35, e45
            Simd32x4::from([self[e15], self[e25], self[e35], self[e45]]),
            // e41, e42, e43
            self.group0(),
            // e23, e31, e12
            self.group1().truncate_to_3(),
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
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self[scalar], 0.0]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e15, e25, e35, e45
            Simd32x4::from([self[e15], self[e25], self[e35], self[e45]]),
            // e41, e42, e43
            self.group0(),
            // e23, e31, e12
            self.group1().truncate_to_3(),
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
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self[scalar], other[e12345]]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e15, e25, e35, e45
            Simd32x4::from([self[e15], self[e25], self[e35], self[e45]]),
            // e41, e42, e43
            self.group0(),
            // e23, e31, e12
            self.group1().truncate_to_3(),
            // e415, e425, e435, e321
            other.group1(),
            // e423, e431, e412
            other.group0(),
            // e235, e315, e125
            other.group2().truncate_to_3(),
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
    //      add/sub      mul      div
    // f32       10        0        0
    fn add(self, other: Dipole) -> Self::Output {
        use crate::elements::*;
        return AntiCircleRotor::from_groups(
            // e41, e42, e43
            Simd32x3::from([self[e41] + other[e41], self[e42] + other[e42], self[e43] + other[e43]]),
            // e23, e31, e12, e45
            Simd32x4::from([self[e23] + other[e23], self[e31] + other[e31], self[e12] + other[e12], self[e45] + other[e45]]),
            // e15, e25, e35, scalar
            Simd32x4::from([self[e15] + other[e15], self[e25] + other[e25], self[e35] + other[e35], self[scalar]]),
        );
    }
}
impl std::ops::AddAssign<Dipole> for AntiCircleRotor {
    fn add_assign(&mut self, other: Dipole) {
        use crate::elements::*;
        *self = AntiCircleRotor::from_groups(
            // e41, e42, e43
            Simd32x3::from([self[e41] + other[e41], self[e42] + other[e42], self[e43] + other[e43]]),
            // e23, e31, e12, e45
            Simd32x4::from([self[e23] + other[e23], self[e31] + other[e31], self[e12] + other[e12], self[e45] + other[e45]]),
            // e15, e25, e35, scalar
            Simd32x4::from([self[e15] + other[e15], self[e25] + other[e25], self[e35] + other[e35], self[scalar]]),
        );
    }
}
impl std::ops::Add<DipoleInversion> for AntiCircleRotor {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       10        0        0
    fn add(self, other: DipoleInversion) -> Self::Output {
        use crate::elements::*;
        return VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([self[e41] + other[e41], self[e42] + other[e42], self[e43] + other[e43], self[scalar]]),
            // e23, e31, e12, e45
            Simd32x4::from([self[e23] + other[e23], self[e31] + other[e31], self[e12] + other[e12], self[e45] + other[e45]]),
            // e15, e25, e35, e1234
            Simd32x4::from([self[e15] + other[e15], self[e25] + other[e25], self[e35] + other[e35], other[e1234]]),
            // e4235, e4315, e4125, e3215
            other.group3(),
        );
    }
}
impl std::ops::Add<DualNum> for AntiCircleRotor {
    type Output = MultiVector;
    fn add(self, other: DualNum) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self[scalar], other[e12345]]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            other[e5],
            // e15, e25, e35, e45
            Simd32x4::from([self[e15], self[e25], self[e35], self[e45]]),
            // e41, e42, e43
            self.group0(),
            // e23, e31, e12
            self.group1().truncate_to_3(),
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
        use crate::elements::*;
        return AntiCircleRotor::from_groups(
            // e41, e42, e43
            self.group0(),
            // e23, e31, e12, e45
            Simd32x4::from([self[e23], self[e31], self[e12], self[e45] + other[e45]]),
            // e15, e25, e35, scalar
            Simd32x4::from([self[e15] + other[e15], self[e25] + other[e25], self[e35] + other[e35], self[scalar]]),
        );
    }
}
impl std::ops::AddAssign<FlatPoint> for AntiCircleRotor {
    fn add_assign(&mut self, other: FlatPoint) {
        use crate::elements::*;
        *self = AntiCircleRotor::from_groups(
            // e41, e42, e43
            self.group0(),
            // e23, e31, e12, e45
            Simd32x4::from([self[e23], self[e31], self[e12], self[e45] + other[e45]]),
            // e15, e25, e35, scalar
            Simd32x4::from([self[e15] + other[e15], self[e25] + other[e25], self[e35] + other[e35], self[scalar]]),
        );
    }
}
impl std::ops::Add<Flector> for AntiCircleRotor {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        4        0        0
    fn add(self, other: Flector) -> Self::Output {
        use crate::elements::*;
        return VersorOdd::from_groups(
            // e41, e42, e43, scalar
            crate::swizzle!(self.group0(), 0, 1, 2).extend_to_4(self[scalar]),
            // e23, e31, e12, e45
            Simd32x4::from([self[e23], self[e31], self[e12], self[e45] + other[e45]]),
            // e15, e25, e35, e1234
            Simd32x4::from([self[e15] + other[e15], self[e25] + other[e25], self[e35] + other[e35], 0.0]),
            // e4235, e4315, e4125, e3215
            other.group1(),
        );
    }
}
impl std::ops::Add<Line> for AntiCircleRotor {
    type Output = MultiVector;
    fn add(self, other: Line) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self[scalar], 0.0]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e15, e25, e35, e45
            Simd32x4::from([self[e15], self[e25], self[e35], self[e45]]),
            // e41, e42, e43
            self.group0(),
            // e23, e31, e12
            self.group1().truncate_to_3(),
            // e415, e425, e435, e321
            crate::swizzle!(other.group0(), 0, 1, 2).extend_to_4(0.0),
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
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self[scalar], other[e12345]]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            other[e5],
            // e15, e25, e35, e45
            Simd32x4::from([self[e15], self[e25], self[e35], self[e45]]),
            // e41, e42, e43
            self.group0(),
            // e23, e31, e12
            self.group1().truncate_to_3(),
            // e415, e425, e435, e321
            Simd32x4::from([other[e415], other[e425], other[e435], 0.0]),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            other.group1().truncate_to_3(),
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
    //      add/sub      mul      div
    // f32       11        0        0
    fn add(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self[scalar] + other[scalar], other[e12345]]),
            // e1, e2, e3, e4
            other.group1(),
            // e5
            other[e5],
            // e15, e25, e35, e45
            Simd32x4::from([self[e15] + other[e15], self[e25] + other[e25], self[e35] + other[e35], self[e45] + other[e45]]),
            // e41, e42, e43
            Simd32x3::from([self[e41] + other[e41], self[e42] + other[e42], self[e43] + other[e43]]),
            // e23, e31, e12
            Simd32x3::from([self[e23] + other[e23], self[e31] + other[e31], self[e12] + other[e12]]),
            // e415, e425, e435, e321
            other.group6(),
            // e423, e431, e412
            other.group7(),
            // e235, e315, e125
            other.group8(),
            // e4235, e4315, e4125, e3215
            other.group9(),
            // e1234
            other[e1234],
        );
    }
}
impl std::ops::Add<Plane> for AntiCircleRotor {
    type Output = VersorOdd;
    fn add(self, other: Plane) -> Self::Output {
        use crate::elements::*;
        return VersorOdd::from_groups(
            // e41, e42, e43, scalar
            crate::swizzle!(self.group0(), 0, 1, 2).extend_to_4(self[scalar]),
            // e23, e31, e12, e45
            self.group1(),
            // e15, e25, e35, e1234
            Simd32x4::from([self[e15], self[e25], self[e35], 0.0]),
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
            Simd32x2::from([self[scalar], 0.0]),
            // e1, e2, e3, e4
            other.group0(),
            // e5
            other[e5],
            // e15, e25, e35, e45
            Simd32x4::from([self[e15], self[e25], self[e35], self[e45]]),
            // e41, e42, e43
            self.group0(),
            // e23, e31, e12
            self.group1().truncate_to_3(),
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
            Simd32x4::from([self[e15], self[e25], self[e35], self[scalar] + other[scalar]]),
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
            Simd32x4::from([self[e15], self[e25], self[e35], self[scalar] + other[scalar]]),
        );
    }
}
impl std::ops::Add<Sphere> for AntiCircleRotor {
    type Output = VersorOdd;
    fn add(self, other: Sphere) -> Self::Output {
        use crate::elements::*;
        return VersorOdd::from_groups(
            // e41, e42, e43, scalar
            crate::swizzle!(self.group0(), 0, 1, 2).extend_to_4(self[scalar]),
            // e23, e31, e12, e45
            self.group1(),
            // e15, e25, e35, e1234
            Simd32x4::from([self[e15], self[e25], self[e35], other[e1234]]),
            // e4235, e4315, e4125, e3215
            other.group0(),
        );
    }
}
impl std::ops::Add<VersorEven> for AntiCircleRotor {
    type Output = MultiVector;
    fn add(self, other: VersorEven) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self[scalar], other[e12345]]),
            // e1, e2, e3, e4
            other.group3(),
            // e5
            other[e5],
            // e15, e25, e35, e45
            Simd32x4::from([self[e15], self[e25], self[e35], self[e45]]),
            // e41, e42, e43
            self.group0(),
            // e23, e31, e12
            self.group1().truncate_to_3(),
            // e415, e425, e435, e321
            other.group1(),
            // e423, e431, e412
            other.group0().truncate_to_3(),
            // e235, e315, e125
            other.group2().truncate_to_3(),
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
    //      add/sub      mul      div
    // f32       11        0        0
    fn add(self, other: VersorOdd) -> Self::Output {
        use crate::elements::*;
        return VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([self[e41] + other[e41], self[e42] + other[e42], self[e43] + other[e43], self[scalar] + other[scalar]]),
            // e23, e31, e12, e45
            Simd32x4::from([self[e23] + other[e23], self[e31] + other[e31], self[e12] + other[e12], self[e45] + other[e45]]),
            // e15, e25, e35, e1234
            Simd32x4::from([self[e15] + other[e15], self[e25] + other[e25], self[e35] + other[e35], other[e1234]]),
            // e4235, e4315, e4125, e3215
            other.group3(),
        );
    }
}
impl std::ops::BitXor<AntiCircleRotor> for AntiCircleRotor {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       28       37        0
    //    simd3        1        2        0
    //    simd4        1        2        0
    // Totals...
    // yes simd       30       41        0
    //  no simd       35       51        0
    fn bitxor(self, other: AntiCircleRotor) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<AntiDipoleInversion> for AntiCircleRotor {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       29       41        0
    //    simd3        2        2        0
    //    simd4        1        2        0
    // Totals...
    // yes simd       32       45        0
    //  no simd       39       55        0
    fn bitxor(self, other: AntiDipoleInversion) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<AntiDualNum> for AntiCircleRotor {
    type Output = VersorOdd;
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
    //      f32       15       30        0
    //    simd3        2        2        0
    //    simd4        0        1        0
    // Totals...
    // yes simd       17       33        0
    //  no simd       21       40        0
    fn bitxor(self, other: AntiFlector) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<AntiLine> for AntiCircleRotor {
    type Output = DipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       13       24        0
    //    simd4        0        1        0
    // Totals...
    // yes simd       13       25        0
    //  no simd       13       28        0
    fn bitxor(self, other: AntiLine) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<AntiMotor> for AntiCircleRotor {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       17       26        0
    //    simd3        1        2        0
    //    simd4        0        1        0
    // Totals...
    // yes simd       18       29        0
    //  no simd       20       36        0
    fn bitxor(self, other: AntiMotor) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<AntiPlane> for AntiCircleRotor {
    type Output = AntiDipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       11       21        0
    //    simd3        0        1        0
    //    simd4        1        1        0
    // Totals...
    // yes simd       12       23        0
    //  no simd       15       28        0
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
    //      f32       25       33        0
    //    simd3        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd       25       35        0
    //  no simd       25       40        0
    fn bitxor(self, other: Dipole) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<DipoleInversion> for AntiCircleRotor {
    type Output = DipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       26       34        0
    //    simd3        0        1        0
    //    simd4        1        2        0
    // Totals...
    // yes simd       27       37        0
    //  no simd       30       45        0
    fn bitxor(self, other: DipoleInversion) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<DualNum> for AntiCircleRotor {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        4        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        5        0
    //  no simd        0        8        0
    fn bitxor(self, other: DualNum) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<FlatPoint> for AntiCircleRotor {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        8       12        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        8       13        0
    //  no simd        8       16        0
    fn bitxor(self, other: FlatPoint) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<Flector> for AntiCircleRotor {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        8       12        0
    //    simd4        1        2        0
    // Totals...
    // yes simd        9       14        0
    //  no simd       12       20        0
    fn bitxor(self, other: Flector) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<Line> for AntiCircleRotor {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        5       12        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        5       13        0
    //  no simd        5       16        0
    fn bitxor(self, other: Line) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<Motor> for AntiCircleRotor {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        5       10        0
    //    simd3        1        2        0
    //    simd4        1        1        0
    // Totals...
    // yes simd        7       13        0
    //  no simd       12       20        0
    fn bitxor(self, other: Motor) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<MultiVector> for AntiCircleRotor {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       50       68        0
    //    simd3        6        8        0
    //    simd4        3        5        0
    // Totals...
    // yes simd       59       81        0
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
    //      f32       14       25        0
    //    simd3        2        2        0
    //    simd4        0        1        0
    // Totals...
    // yes simd       16       28        0
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
    //      f32       26       38        0
    //    simd3        2        2        0
    //    simd4        2        3        0
    // Totals...
    // yes simd       30       43        0
    //  no simd       40       56        0
    fn bitxor(self, other: VersorEven) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<VersorOdd> for AntiCircleRotor {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       25       34        0
    //    simd3        1        2        0
    //    simd4        3        4        0
    // Totals...
    // yes simd       29       40        0
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
            from_dipole.group0(),
            // e23, e31, e12, e45
            from_dipole.group1(),
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
            Simd32x3::from(0.0).extend_to_4(from_flat_point[e45]),
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
            Simd32x3::from(0.0).extend_to_4(from_scalar[scalar]),
        );
    }
}
impl std::ops::Mul<AntiCircleRotor> for AntiCircleRotor {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       89      105        0
    //    simd4        4        4        0
    // Totals...
    // yes simd       93      109        0
    //  no simd      105      121        0
    fn mul(self, other: AntiCircleRotor) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiDipoleInversion> for AntiCircleRotor {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32      113      129        0
    //    simd4        9        9        0
    // Totals...
    // yes simd      122      138        0
    //  no simd      149      165        0
    fn mul(self, other: AntiDipoleInversion) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiDualNum> for AntiCircleRotor {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        9        0
    //    simd3        1        3        0
    //    simd4        1        2        0
    // Totals...
    // yes simd        3       14        0
    //  no simd        8       26        0
    fn mul(self, other: AntiDualNum) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiFlatPoint> for AntiCircleRotor {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       26       41        0
    //    simd3        1        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd       27       43        0
    //  no simd       29       48        0
    fn mul(self, other: AntiFlatPoint) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiFlector> for AntiCircleRotor {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       60       76        0
    //    simd4        3        3        0
    // Totals...
    // yes simd       63       79        0
    //  no simd       72       88        0
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
    //      f32       60       76        0
    //    simd4        3        3        0
    // Totals...
    // yes simd       63       79        0
    //  no simd       72       88        0
    fn mul(self, other: AntiMotor) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiPlane> for AntiCircleRotor {
    type Output = AntiDipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       21       36        0
    //    simd4        2        2        0
    // Totals...
    // yes simd       23       38        0
    //  no simd       29       44        0
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
    //      f32       86      102        0
    //    simd4        2        2        0
    // Totals...
    // yes simd       88      104        0
    //  no simd       94      110        0
    fn mul(self, other: Circle) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<CircleRotor> for AntiCircleRotor {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       93      109        0
    //    simd4        3        3        0
    // Totals...
    // yes simd       96      112        0
    //  no simd      105      121        0
    fn mul(self, other: CircleRotor) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<Dipole> for AntiCircleRotor {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       86      102        0
    //    simd4        2        2        0
    // Totals...
    // yes simd       88      104        0
    //  no simd       94      110        0
    fn mul(self, other: Dipole) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<DipoleInversion> for AntiCircleRotor {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32      129      145        0
    //    simd4        5        5        0
    // Totals...
    // yes simd      134      150        0
    //  no simd      149      165        0
    fn mul(self, other: DipoleInversion) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<DualNum> for AntiCircleRotor {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        9        0
    //    simd3        1        2        0
    //    simd4        1        3        0
    // Totals...
    // yes simd        2       14        0
    //  no simd        7       27        0
    fn mul(self, other: DualNum) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<FlatPoint> for AntiCircleRotor {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       20       35        0
    //    simd3        3        3        0
    // Totals...
    // yes simd       23       38        0
    //  no simd       29       44        0
    fn mul(self, other: FlatPoint) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<Flector> for AntiCircleRotor {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       64       80        0
    //    simd4        2        2        0
    // Totals...
    // yes simd       66       82        0
    //  no simd       72       88        0
    fn mul(self, other: Flector) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<Line> for AntiCircleRotor {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       50       66        0
    fn mul(self, other: Line) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<Motor> for AntiCircleRotor {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       60       76        0
    //    simd4        3        3        0
    // Totals...
    // yes simd       63       79        0
    //  no simd       72       88        0
    fn mul(self, other: Motor) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<MultiVector> for AntiCircleRotor {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32      184      217        0
    //    simd2       10       10        0
    //    simd3       24       24        0
    //    simd4       11       11        0
    // Totals...
    // yes simd      229      262        0
    //  no simd      320      353        0
    fn mul(self, other: MultiVector) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<Plane> for AntiCircleRotor {
    type Output = DipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       25       40        0
    //    simd4        1        1        0
    // Totals...
    // yes simd       26       41        0
    //  no simd       29       44        0
    fn mul(self, other: Plane) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<RoundPoint> for AntiCircleRotor {
    type Output = AntiDipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       29       44        0
    //    simd3        1        1        0
    //    simd4        2        2        0
    // Totals...
    // yes simd       32       47        0
    //  no simd       40       55        0
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
    //      f32       33       48        0
    //    simd3        1        1        0
    //    simd4        1        1        0
    // Totals...
    // yes simd       35       50        0
    //  no simd       40       55        0
    fn mul(self, other: Sphere) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<VersorEven> for AntiCircleRotor {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32      116      132        0
    //    simd4       11       11        0
    // Totals...
    // yes simd      127      143        0
    //  no simd      160      176        0
    fn mul(self, other: VersorEven) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<VersorOdd> for AntiCircleRotor {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32      132      148        0
    //    simd4        7        7        0
    // Totals...
    // yes simd      139      155        0
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
    //    simd3        0        1        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        0        3        0
    //  no simd        0       11        0
    fn not(self) -> Self::Output {
        return self.right_dual();
    }
}
impl std::ops::Sub<AntiCircleRotor> for AntiCircleRotor {
    type Output = AntiCircleRotor;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       11        0        0
    fn sub(self, other: AntiCircleRotor) -> Self::Output {
        use crate::elements::*;
        return AntiCircleRotor::from_groups(
            // e41, e42, e43
            Simd32x3::from([self[e41] - other[e41], self[e42] - other[e42], self[e43] - other[e43]]),
            // e23, e31, e12, e45
            Simd32x4::from([self[e23] - other[e23], self[e31] - other[e31], self[e12] - other[e12], self[e45] - other[e45]]),
            // e15, e25, e35, scalar
            Simd32x4::from([self[e15] - other[e15], self[e25] - other[e25], self[e35] - other[e35], self[scalar] - other[scalar]]),
        );
    }
}
impl std::ops::SubAssign<AntiCircleRotor> for AntiCircleRotor {
    fn sub_assign(&mut self, other: AntiCircleRotor) {
        use crate::elements::*;
        *self = AntiCircleRotor::from_groups(
            // e41, e42, e43
            Simd32x3::from([self[e41] - other[e41], self[e42] - other[e42], self[e43] - other[e43]]),
            // e23, e31, e12, e45
            Simd32x4::from([self[e23] - other[e23], self[e31] - other[e31], self[e12] - other[e12], self[e45] - other[e45]]),
            // e15, e25, e35, scalar
            Simd32x4::from([self[e15] - other[e15], self[e25] - other[e25], self[e35] - other[e35], self[scalar] - other[scalar]]),
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
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self[scalar], 0.0]),
            // e1, e2, e3, e4
            Simd32x4::from([other[e1], other[e2], other[e3], other[e4]]) * Simd32x4::from(-1.0),
            // e5
            other[e5] * -1.0,
            // e15, e25, e35, e45
            Simd32x4::from([self[e15], self[e25], self[e35], self[e45]]),
            // e41, e42, e43
            self.group0(),
            // e23, e31, e12
            self.group1().truncate_to_3(),
            // e415, e425, e435, e321
            other.group1() * Simd32x4::from(-1.0),
            // e423, e431, e412
            other.group0() * Simd32x3::from(-1.0),
            // e235, e315, e125
            other.group2().truncate_to_3() * Simd32x3::from(-1.0),
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
        use crate::elements::*;
        return VersorOdd::from_groups(
            // e41, e42, e43, scalar
            crate::swizzle!(self.group0(), 0, 1, 2).extend_to_4(self[scalar] - other[scalar]),
            // e23, e31, e12, e45
            self.group1(),
            // e15, e25, e35, e1234
            Simd32x4::from([self[e15], self[e25], self[e35], 0.0]),
            // e4235, e4315, e4125, e3215
            Simd32x3::from(0.0).extend_to_4(other[e3215] * -1.0),
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
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self[scalar], 0.0]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e15, e25, e35, e45
            Simd32x4::from([self[e15], self[e25], self[e35], self[e45]]),
            // e41, e42, e43
            self.group0(),
            // e23, e31, e12
            self.group1().truncate_to_3(),
            // e415, e425, e435, e321
            Simd32x3::from(0.0).extend_to_4(other[e321] * -1.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            other.group0().truncate_to_3() * Simd32x3::from(-1.0),
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
    //      f32        0        2        0
    //    simd3        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        4        0
    //  no simd        0        9        0
    fn sub(self, other: AntiFlector) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self[scalar], 0.0]),
            // e1, e2, e3, e4
            Simd32x4::from([other[e1], other[e2], other[e3], 1.0]) * Simd32x4::from([-1.0, -1.0, -1.0, 0.0]),
            // e5
            other[e5] * -1.0,
            // e15, e25, e35, e45
            Simd32x4::from([self[e15], self[e25], self[e35], self[e45]]),
            // e41, e42, e43
            self.group0(),
            // e23, e31, e12
            self.group1().truncate_to_3(),
            // e415, e425, e435, e321
            Simd32x3::from(0.0).extend_to_4(other[e321] * -1.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            other.group0().truncate_to_3() * Simd32x3::from(-1.0),
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
        use crate::elements::*;
        return AntiCircleRotor::from_groups(
            // e41, e42, e43
            self.group0(),
            // e23, e31, e12, e45
            Simd32x4::from([self[e23] - other[e23], self[e31] - other[e31], self[e12] - other[e12], self[e45]]),
            // e15, e25, e35, scalar
            Simd32x4::from([self[e15] - other[e15], self[e25] - other[e25], self[e35] - other[e35], self[scalar]]),
        );
    }
}
impl std::ops::SubAssign<AntiLine> for AntiCircleRotor {
    fn sub_assign(&mut self, other: AntiLine) {
        use crate::elements::*;
        *self = AntiCircleRotor::from_groups(
            // e41, e42, e43
            self.group0(),
            // e23, e31, e12, e45
            Simd32x4::from([self[e23] - other[e23], self[e31] - other[e31], self[e12] - other[e12], self[e45]]),
            // e15, e25, e35, scalar
            Simd32x4::from([self[e15] - other[e15], self[e25] - other[e25], self[e35] - other[e35], self[scalar]]),
        );
    }
}
impl std::ops::Sub<AntiMotor> for AntiCircleRotor {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        7        1        0
    fn sub(self, other: AntiMotor) -> Self::Output {
        use crate::elements::*;
        return VersorOdd::from_groups(
            // e41, e42, e43, scalar
            crate::swizzle!(self.group0(), 0, 1, 2).extend_to_4(self[scalar] - other[scalar]),
            // e23, e31, e12, e45
            Simd32x4::from([self[e23] - other[e23], self[e31] - other[e31], self[e12] - other[e12], self[e45]]),
            // e15, e25, e35, e1234
            Simd32x4::from([self[e15] - other[e15], self[e25] - other[e25], self[e35] - other[e35], 0.0]),
            // e4235, e4315, e4125, e3215
            Simd32x3::from(0.0).extend_to_4(other[e3215] * -1.0),
        );
    }
}
impl std::ops::Sub<AntiPlane> for AntiCircleRotor {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        2        0
    //  no simd        0        5        0
    fn sub(self, other: AntiPlane) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self[scalar], 0.0]),
            // e1, e2, e3, e4
            Simd32x4::from([other[e1], other[e2], other[e3], 1.0]) * Simd32x4::from([-1.0, -1.0, -1.0, 0.0]),
            // e5
            other[e5] * -1.0,
            // e15, e25, e35, e45
            Simd32x4::from([self[e15], self[e25], self[e35], self[e45]]),
            // e41, e42, e43
            self.group0(),
            // e23, e31, e12
            self.group1().truncate_to_3(),
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
    //          add/sub      mul      div
    //   simd2        0        1        0
    // no simd        0        2        0
    fn sub(self, other: AntiScalar) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self[scalar], other[e12345]]) * Simd32x2::from([1.0, -1.0]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e15, e25, e35, e45
            Simd32x4::from([self[e15], self[e25], self[e35], self[e45]]),
            // e41, e42, e43
            self.group0(),
            // e23, e31, e12
            self.group1().truncate_to_3(),
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
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self[scalar], 0.0]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e15, e25, e35, e45
            Simd32x4::from([self[e15], self[e25], self[e35], self[e45]]),
            // e41, e42, e43
            self.group0(),
            // e23, e31, e12
            self.group1().truncate_to_3(),
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
    //    simd2        0        1        0
    //    simd3        0        2        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        4        0
    //  no simd        0       12        0
    fn sub(self, other: CircleRotor) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self[scalar], other[e12345]]) * Simd32x2::from([1.0, -1.0]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e15, e25, e35, e45
            Simd32x4::from([self[e15], self[e25], self[e35], self[e45]]),
            // e41, e42, e43
            self.group0(),
            // e23, e31, e12
            self.group1().truncate_to_3(),
            // e415, e425, e435, e321
            other.group1() * Simd32x4::from(-1.0),
            // e423, e431, e412
            other.group0() * Simd32x3::from(-1.0),
            // e235, e315, e125
            other.group2().truncate_to_3() * Simd32x3::from(-1.0),
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
    //      add/sub      mul      div
    // f32       10        0        0
    fn sub(self, other: Dipole) -> Self::Output {
        use crate::elements::*;
        return AntiCircleRotor::from_groups(
            // e41, e42, e43
            Simd32x3::from([self[e41] - other[e41], self[e42] - other[e42], self[e43] - other[e43]]),
            // e23, e31, e12, e45
            Simd32x4::from([self[e23] - other[e23], self[e31] - other[e31], self[e12] - other[e12], self[e45] - other[e45]]),
            // e15, e25, e35, scalar
            Simd32x4::from([self[e15] - other[e15], self[e25] - other[e25], self[e35] - other[e35], self[scalar]]),
        );
    }
}
impl std::ops::SubAssign<Dipole> for AntiCircleRotor {
    fn sub_assign(&mut self, other: Dipole) {
        use crate::elements::*;
        *self = AntiCircleRotor::from_groups(
            // e41, e42, e43
            Simd32x3::from([self[e41] - other[e41], self[e42] - other[e42], self[e43] - other[e43]]),
            // e23, e31, e12, e45
            Simd32x4::from([self[e23] - other[e23], self[e31] - other[e31], self[e12] - other[e12], self[e45] - other[e45]]),
            // e15, e25, e35, scalar
            Simd32x4::from([self[e15] - other[e15], self[e25] - other[e25], self[e35] - other[e35], self[scalar]]),
        );
    }
}
impl std::ops::Sub<DipoleInversion> for AntiCircleRotor {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       10        0        0
    //    simd4        0        2        0
    // Totals...
    // yes simd       10        2        0
    //  no simd       10        8        0
    fn sub(self, other: DipoleInversion) -> Self::Output {
        use crate::elements::*;
        return VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([self[e41] - other[e41], self[e42] - other[e42], self[e43] - other[e43], self[scalar]]),
            // e23, e31, e12, e45
            Simd32x4::from([self[e23] - other[e23], self[e31] - other[e31], self[e12] - other[e12], self[e45] - other[e45]]),
            // e15, e25, e35, e1234
            Simd32x4::from([self[e15] - other[e15], self[e25] - other[e25], self[e35] - other[e35], other[e1234]]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e4235, e4315, e4125, e3215
            other.group3() * Simd32x4::from(-1.0),
        );
    }
}
impl std::ops::Sub<DualNum> for AntiCircleRotor {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd2        0        1        0
    // Totals...
    // yes simd        0        2        0
    //  no simd        0        3        0
    fn sub(self, other: DualNum) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self[scalar], other[e12345]]) * Simd32x2::from([1.0, -1.0]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            other[e5] * -1.0,
            // e15, e25, e35, e45
            Simd32x4::from([self[e15], self[e25], self[e35], self[e45]]),
            // e41, e42, e43
            self.group0(),
            // e23, e31, e12
            self.group1().truncate_to_3(),
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
        use crate::elements::*;
        return AntiCircleRotor::from_groups(
            // e41, e42, e43
            self.group0(),
            // e23, e31, e12, e45
            Simd32x4::from([self[e23], self[e31], self[e12], self[e45] - other[e45]]),
            // e15, e25, e35, scalar
            Simd32x4::from([self[e15] - other[e15], self[e25] - other[e25], self[e35] - other[e35], self[scalar]]),
        );
    }
}
impl std::ops::SubAssign<FlatPoint> for AntiCircleRotor {
    fn sub_assign(&mut self, other: FlatPoint) {
        use crate::elements::*;
        *self = AntiCircleRotor::from_groups(
            // e41, e42, e43
            self.group0(),
            // e23, e31, e12, e45
            Simd32x4::from([self[e23], self[e31], self[e12], self[e45] - other[e45]]),
            // e15, e25, e35, scalar
            Simd32x4::from([self[e15] - other[e15], self[e25] - other[e25], self[e35] - other[e35], self[scalar]]),
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
        use crate::elements::*;
        return VersorOdd::from_groups(
            // e41, e42, e43, scalar
            crate::swizzle!(self.group0(), 0, 1, 2).extend_to_4(self[scalar]),
            // e23, e31, e12, e45
            Simd32x4::from([self[e23], self[e31], self[e12], self[e45] - other[e45]]),
            // e15, e25, e35, e1234
            Simd32x4::from([self[e15] - other[e15], self[e25] - other[e25], self[e35] - other[e35], 0.0]),
            // e4235, e4315, e4125, e3215
            other.group1() * Simd32x4::from(-1.0),
        );
    }
}
impl std::ops::Sub<Line> for AntiCircleRotor {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        2        0
    //  no simd        0        7        0
    fn sub(self, other: Line) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self[scalar], 0.0]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e15, e25, e35, e45
            Simd32x4::from([self[e15], self[e25], self[e35], self[e45]]),
            // e41, e42, e43
            self.group0(),
            // e23, e31, e12
            self.group1().truncate_to_3(),
            // e415, e425, e435, e321
            Simd32x4::from([other[e415], other[e425], other[e435], 1.0]) * Simd32x4::from([-1.0, -1.0, -1.0, 0.0]),
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
    //      f32        0        1        0
    //    simd2        0        1        0
    //    simd3        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        4        0
    //  no simd        0       10        0
    fn sub(self, other: Motor) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self[scalar], other[e12345]]) * Simd32x2::from([1.0, -1.0]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            other[e5] * -1.0,
            // e15, e25, e35, e45
            Simd32x4::from([self[e15], self[e25], self[e35], self[e45]]),
            // e41, e42, e43
            self.group0(),
            // e23, e31, e12
            self.group1().truncate_to_3(),
            // e415, e425, e435, e321
            Simd32x4::from([other[e415], other[e425], other[e435], 1.0]) * Simd32x4::from([-1.0, -1.0, -1.0, 0.0]),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            other.group1().truncate_to_3() * Simd32x3::from(-1.0),
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
    //      f32       11        2        0
    //    simd2        0        1        0
    //    simd3        0        2        0
    //    simd4        0        3        0
    // Totals...
    // yes simd       11        8        0
    //  no simd       11       22        0
    fn sub(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self[scalar] - other[scalar], other[e12345]]) * Simd32x2::from([1.0, -1.0]),
            // e1, e2, e3, e4
            other.group1() * Simd32x4::from(-1.0),
            // e5
            other[e5] * -1.0,
            // e15, e25, e35, e45
            Simd32x4::from([self[e15] - other[e15], self[e25] - other[e25], self[e35] - other[e35], self[e45] - other[e45]]),
            // e41, e42, e43
            Simd32x3::from([self[e41] - other[e41], self[e42] - other[e42], self[e43] - other[e43]]),
            // e23, e31, e12
            Simd32x3::from([self[e23] - other[e23], self[e31] - other[e31], self[e12] - other[e12]]),
            // e415, e425, e435, e321
            other.group6() * Simd32x4::from(-1.0),
            // e423, e431, e412
            other.group7() * Simd32x3::from(-1.0),
            // e235, e315, e125
            other.group8() * Simd32x3::from(-1.0),
            // e4235, e4315, e4125, e3215
            other.group9() * Simd32x4::from(-1.0),
            // e1234
            other[e1234] * -1.0,
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
        use crate::elements::*;
        return VersorOdd::from_groups(
            // e41, e42, e43, scalar
            crate::swizzle!(self.group0(), 0, 1, 2).extend_to_4(self[scalar]),
            // e23, e31, e12, e45
            self.group1(),
            // e15, e25, e35, e1234
            Simd32x4::from([self[e15], self[e25], self[e35], 0.0]),
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
            Simd32x2::from([self[scalar], 0.0]),
            // e1, e2, e3, e4
            other.group0() * Simd32x4::from(-1.0),
            // e5
            other[e5] * -1.0,
            // e15, e25, e35, e45
            Simd32x4::from([self[e15], self[e25], self[e35], self[e45]]),
            // e41, e42, e43
            self.group0(),
            // e23, e31, e12
            self.group1().truncate_to_3(),
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
            Simd32x4::from([self[e15], self[e25], self[e35], self[scalar] - other[scalar]]),
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
            Simd32x4::from([self[e15], self[e25], self[e35], self[scalar] - other[scalar]]),
        );
    }
}
impl std::ops::Sub<Sphere> for AntiCircleRotor {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        2        0
    // no simd        0        8        0
    fn sub(self, other: Sphere) -> Self::Output {
        use crate::elements::*;
        return VersorOdd::from_groups(
            // e41, e42, e43, scalar
            crate::swizzle!(self.group0(), 0, 1, 2).extend_to_4(self[scalar]),
            // e23, e31, e12, e45
            self.group1(),
            // e15, e25, e35, e1234
            Simd32x4::from([self[e15], self[e25], self[e35], other[e1234]]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e4235, e4315, e4125, e3215
            other.group0() * Simd32x4::from(-1.0),
        );
    }
}
impl std::ops::Sub<VersorEven> for AntiCircleRotor {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd2        0        1        0
    //    simd3        0        2        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        0        6        0
    //  no simd        0       17        0
    fn sub(self, other: VersorEven) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self[scalar], other[e12345]]) * Simd32x2::from([1.0, -1.0]),
            // e1, e2, e3, e4
            other.group3() * Simd32x4::from(-1.0),
            // e5
            other[e5] * -1.0,
            // e15, e25, e35, e45
            Simd32x4::from([self[e15], self[e25], self[e35], self[e45]]),
            // e41, e42, e43
            self.group0(),
            // e23, e31, e12
            self.group1().truncate_to_3(),
            // e415, e425, e435, e321
            other.group1() * Simd32x4::from(-1.0),
            // e423, e431, e412
            other.group0().truncate_to_3() * Simd32x3::from(-1.0),
            // e235, e315, e125
            other.group2().truncate_to_3() * Simd32x3::from(-1.0),
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
    //      f32       11        0        0
    //    simd4        0        2        0
    // Totals...
    // yes simd       11        2        0
    //  no simd       11        8        0
    fn sub(self, other: VersorOdd) -> Self::Output {
        use crate::elements::*;
        return VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([self[e41] - other[e41], self[e42] - other[e42], self[e43] - other[e43], self[scalar] - other[scalar]]),
            // e23, e31, e12, e45
            Simd32x4::from([self[e23] - other[e23], self[e31] - other[e31], self[e12] - other[e12], self[e45] - other[e45]]),
            // e15, e25, e35, e1234
            Simd32x4::from([self[e15] - other[e15], self[e25] - other[e25], self[e35] - other[e35], other[e1234]]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
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
            error_string.push_str("e3215: ");
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
            Simd32x3::from(0.0).extend_to_4(anti_dual_num[scalar]),
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
            dipole_inversion.group0(),
            // e23, e31, e12, e45
            dipole_inversion.group1(),
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
            Simd32x3::from(0.0).extend_to_4(flector[e45]),
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
            multi_vector.group4(),
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
            versor_odd.group0().truncate_to_3(),
            // e23, e31, e12, e45
            versor_odd.group1(),
            // e15, e25, e35, scalar
            Simd32x4::from([versor_odd[e15], versor_odd[e25], versor_odd[e35], versor_odd[scalar]]),
        ));
    }
}
