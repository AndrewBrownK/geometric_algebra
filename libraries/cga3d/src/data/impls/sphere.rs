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
// Total Implementations: 97
//
// Yes SIMD:   add/sub     mul     div
//  Minimum:         0       0       0
//   Median:         0       2       0
//  Average:         7      11       0
//  Maximum:        90     122       0
//
//  No SIMD:   add/sub     mul     div
//  Minimum:         0       0       0
//   Median:         0       5       0
//  Average:         8      13       0
//  Maximum:       128     160       0
impl std::ops::Add<AntiCircleRotor> for Sphere {
    type Output = VersorOdd;
    fn add(self, other: AntiCircleRotor) -> Self::Output {
        use crate::elements::*;
        return VersorOdd::from_groups(
            // e41, e42, e43, scalar
            crate::swizzle!(other.group0(), 0, 1, 2).extend_to_4(other[scalar]),
            // e23, e31, e12, e45
            other.group1(),
            // e15, e25, e35, e1234
            Simd32x4::from([other[e15], other[e25], other[e35], self[e1234]]),
            // e4235, e4315, e4125, e3215
            self.group0(),
        );
    }
}
impl std::ops::Add<AntiDipoleInversion> for Sphere {
    type Output = MultiVector;
    fn add(self, other: AntiDipoleInversion) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from([other[e1], other[e2], other[e3], other[e4]]),
            // e5
            other[e5],
            // e15, e25, e35, e45
            Simd32x4::from(0.0),
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            other.group1(),
            // e423, e431, e412
            other.group0(),
            // e235, e315, e125
            Simd32x3::from([other[e235], other[e315], other[e125]]),
            // e4235, e4315, e4125, e3215
            self.group0(),
            // e1234
            self[e1234],
        );
    }
}
impl std::ops::Add<AntiDualNum> for Sphere {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        1        0        0
    fn add(self, other: AntiDualNum) -> Self::Output {
        use crate::elements::*;
        return VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([0.0, 0.0, 0.0, other[scalar]]),
            // e23, e31, e12, e45
            Simd32x4::from(0.0),
            // e15, e25, e35, e1234
            Simd32x4::from([0.0, 0.0, 0.0, other[e1234] + self[e1234]]),
            // e4235, e4315, e4125, e3215
            self.group0(),
        );
    }
}
impl std::ops::Add<AntiFlatPoint> for Sphere {
    type Output = MultiVector;
    fn add(self, other: AntiFlatPoint) -> Self::Output {
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
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from([0.0, 0.0, 0.0, other[e321]]),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from([other[e235], other[e315], other[e125]]),
            // e4235, e4315, e4125, e3215
            self.group0(),
            // e1234
            self[e1234],
        );
    }
}
impl std::ops::Add<AntiFlector> for Sphere {
    type Output = MultiVector;
    fn add(self, other: AntiFlector) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from([other[e1], other[e2], other[e3], 0.0]),
            // e5
            other[e5],
            // e15, e25, e35, e45
            Simd32x4::from(0.0),
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from([0.0, 0.0, 0.0, other[e321]]),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from([other[e235], other[e315], other[e125]]),
            // e4235, e4315, e4125, e3215
            self.group0(),
            // e1234
            self[e1234],
        );
    }
}
impl std::ops::Add<AntiLine> for Sphere {
    type Output = DipoleInversion;
    fn add(self, other: AntiLine) -> Self::Output {
        use crate::elements::*;
        return DipoleInversion::from_groups(
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12, e45
            crate::swizzle!(other.group0(), 0, 1, 2).extend_to_4(0.0),
            // e15, e25, e35, e1234
            crate::swizzle!(other.group1(), 0, 1, 2).extend_to_4(self[e1234]),
            // e4235, e4315, e4125, e3215
            self.group0(),
        );
    }
}
impl std::ops::Add<AntiMotor> for Sphere {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        1        0        0
    fn add(self, other: AntiMotor) -> Self::Output {
        use crate::elements::*;
        return VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([0.0, 0.0, 0.0, other[scalar]]),
            // e23, e31, e12, e45
            Simd32x4::from([other[e23], other[e31], other[e12], 0.0]),
            // e15, e25, e35, e1234
            Simd32x4::from([other[e15], other[e25], other[e35], self[e1234]]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([self[e4235], self[e4315], self[e4125], other[e3215] + self[e3215]]),
        );
    }
}
impl std::ops::Add<AntiPlane> for Sphere {
    type Output = MultiVector;
    fn add(self, other: AntiPlane) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from([other[e1], other[e2], other[e3], 0.0]),
            // e5
            other[e5],
            // e15, e25, e35, e45
            Simd32x4::from(0.0),
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
            self.group0(),
            // e1234
            self[e1234],
        );
    }
}
impl std::ops::Add<AntiScalar> for Sphere {
    type Output = MultiVector;
    fn add(self, other: AntiScalar) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, other[e12345]]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e15, e25, e35, e45
            Simd32x4::from(0.0),
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
            self.group0(),
            // e1234
            self[e1234],
        );
    }
}
impl std::ops::Add<Circle> for Sphere {
    type Output = MultiVector;
    fn add(self, other: Circle) -> Self::Output {
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
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            other.group1(),
            // e423, e431, e412
            other.group0(),
            // e235, e315, e125
            other.group2(),
            // e4235, e4315, e4125, e3215
            self.group0(),
            // e1234
            self[e1234],
        );
    }
}
impl std::ops::Add<CircleRotor> for Sphere {
    type Output = MultiVector;
    fn add(self, other: CircleRotor) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, other[e12345]]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e15, e25, e35, e45
            Simd32x4::from(0.0),
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            other.group1(),
            // e423, e431, e412
            other.group0(),
            // e235, e315, e125
            Simd32x3::from([other[e235], other[e315], other[e125]]),
            // e4235, e4315, e4125, e3215
            self.group0(),
            // e1234
            self[e1234],
        );
    }
}
impl std::ops::Add<Dipole> for Sphere {
    type Output = DipoleInversion;
    fn add(self, other: Dipole) -> Self::Output {
        use crate::elements::*;
        return DipoleInversion::from_groups(
            // e41, e42, e43
            other.group0(),
            // e23, e31, e12, e45
            other.group1(),
            // e15, e25, e35, e1234
            crate::swizzle!(other.group2(), 0, 1, 2).extend_to_4(self[e1234]),
            // e4235, e4315, e4125, e3215
            self.group0(),
        );
    }
}
impl std::ops::Add<DipoleInversion> for Sphere {
    type Output = DipoleInversion;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        5        0        0
    fn add(self, other: DipoleInversion) -> Self::Output {
        use crate::elements::*;
        return DipoleInversion::from_groups(
            // e41, e42, e43
            other.group0(),
            // e23, e31, e12, e45
            other.group1(),
            // e15, e25, e35, e1234
            Simd32x4::from([other[e15], other[e25], other[e35], other[e1234] + self[e1234]]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([other[e4235] + self[e4235], other[e4315] + self[e4315], other[e4125] + self[e4125], other[e3215] + self[e3215]]),
        );
    }
}
impl std::ops::Add<DualNum> for Sphere {
    type Output = MultiVector;
    fn add(self, other: DualNum) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, other[e12345]]),
            // e1, e2, e3, e4
            Simd32x4::from([0.0, 0.0, 0.0, other[e4]]),
            // e5
            0.0,
            // e15, e25, e35, e45
            Simd32x4::from(0.0),
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
            self.group0(),
            // e1234
            self[e1234],
        );
    }
}
impl std::ops::Add<FlatPoint> for Sphere {
    type Output = DipoleInversion;
    fn add(self, other: FlatPoint) -> Self::Output {
        use crate::elements::*;
        return DipoleInversion::from_groups(
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12, e45
            Simd32x4::from([0.0, 0.0, 0.0, other[e45]]),
            // e15, e25, e35, e1234
            Simd32x4::from([other[e15], other[e25], other[e35], self[e1234]]),
            // e4235, e4315, e4125, e3215
            self.group0(),
        );
    }
}
impl std::ops::Add<Flector> for Sphere {
    type Output = DipoleInversion;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        4        0        0
    fn add(self, other: Flector) -> Self::Output {
        use crate::elements::*;
        return DipoleInversion::from_groups(
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12, e45
            Simd32x4::from([0.0, 0.0, 0.0, other[e45]]),
            // e15, e25, e35, e1234
            Simd32x4::from([other[e15], other[e25], other[e35], self[e1234]]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([other[e4235] + self[e4235], other[e4315] + self[e4315], other[e4125] + self[e4125], other[e3215] + self[e3215]]),
        );
    }
}
impl std::ops::Add<Line> for Sphere {
    type Output = MultiVector;
    fn add(self, other: Line) -> Self::Output {
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
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            crate::swizzle!(other.group0(), 0, 1, 2).extend_to_4(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            other.group1(),
            // e4235, e4315, e4125, e3215
            self.group0(),
            // e1234
            self[e1234],
        );
    }
}
impl std::ops::Add<Motor> for Sphere {
    type Output = MultiVector;
    fn add(self, other: Motor) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, other[e12345]]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            other[e5],
            // e15, e25, e35, e45
            Simd32x4::from(0.0),
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from([other[e415], other[e425], other[e435], 0.0]),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from([other[e235], other[e315], other[e125]]),
            // e4235, e4315, e4125, e3215
            self.group0(),
            // e1234
            self[e1234],
        );
    }
}
impl std::ops::Add<MultiVector> for Sphere {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        5        0        0
    fn add(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            other.group0(),
            // e1, e2, e3, e4
            other.group1(),
            // e5
            other[e5],
            // e15, e25, e35, e45
            other.group3(),
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
            Simd32x4::from([other[e4235] + self[e4235], other[e4315] + self[e4315], other[e4125] + self[e4125], other[e3215] + self[e3215]]),
            // e1234
            other[e1234] + self[e1234],
        );
    }
}
impl std::ops::Add<Plane> for Sphere {
    type Output = Sphere;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        4        0        0
    fn add(self, other: Plane) -> Self::Output {
        use crate::elements::*;
        return Sphere::from_groups(
            // e4235, e4315, e4125, e3215
            Simd32x4::from([other[e4235] + self[e4235], other[e4315] + self[e4315], other[e4125] + self[e4125], other[e3215] + self[e3215]]),
            // e1234
            self[e1234],
        );
    }
}
impl std::ops::AddAssign<Plane> for Sphere {
    fn add_assign(&mut self, other: Plane) {
        use crate::elements::*;
        *self = Sphere::from_groups(
            // e4235, e4315, e4125, e3215
            Simd32x4::from([other[e4235] + self[e4235], other[e4315] + self[e4315], other[e4125] + self[e4125], other[e3215] + self[e3215]]),
            // e1234
            self[e1234],
        );
    }
}
impl std::ops::Add<RoundPoint> for Sphere {
    type Output = MultiVector;
    fn add(self, other: RoundPoint) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            other.group0(),
            // e5
            other[e5],
            // e15, e25, e35, e45
            Simd32x4::from(0.0),
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
            self.group0(),
            // e1234
            self[e1234],
        );
    }
}
impl std::ops::Add<Scalar> for Sphere {
    type Output = VersorOdd;
    fn add(self, other: Scalar) -> Self::Output {
        use crate::elements::*;
        return VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([0.0, 0.0, 0.0, other[scalar]]),
            // e23, e31, e12, e45
            Simd32x4::from(0.0),
            // e15, e25, e35, e1234
            Simd32x4::from([0.0, 0.0, 0.0, self[e1234]]),
            // e4235, e4315, e4125, e3215
            self.group0(),
        );
    }
}
impl std::ops::Add<Sphere> for Sphere {
    type Output = Sphere;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        5        0        0
    fn add(self, other: Sphere) -> Self::Output {
        use crate::elements::*;
        return Sphere::from_groups(
            // e4235, e4315, e4125, e3215
            Simd32x4::from([other[e4235] + self[e4235], other[e4315] + self[e4315], other[e4125] + self[e4125], other[e3215] + self[e3215]]),
            // e1234
            other[e1234] + self[e1234],
        );
    }
}
impl std::ops::AddAssign<Sphere> for Sphere {
    fn add_assign(&mut self, other: Sphere) {
        use crate::elements::*;
        *self = Sphere::from_groups(
            // e4235, e4315, e4125, e3215
            Simd32x4::from([other[e4235] + self[e4235], other[e4315] + self[e4315], other[e4125] + self[e4125], other[e3215] + self[e3215]]),
            // e1234
            other[e1234] + self[e1234],
        );
    }
}
impl std::ops::Add<VersorEven> for Sphere {
    type Output = MultiVector;
    fn add(self, other: VersorEven) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, other[e12345]]),
            // e1, e2, e3, e4
            other.group3(),
            // e5
            other[e5],
            // e15, e25, e35, e45
            Simd32x4::from(0.0),
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            other.group1(),
            // e423, e431, e412
            Simd32x3::from([other[e423], other[e431], other[e412]]),
            // e235, e315, e125
            Simd32x3::from([other[e235], other[e315], other[e125]]),
            // e4235, e4315, e4125, e3215
            self.group0(),
            // e1234
            self[e1234],
        );
    }
}
impl std::ops::Add<VersorOdd> for Sphere {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        5        0        0
    fn add(self, other: VersorOdd) -> Self::Output {
        use crate::elements::*;
        return VersorOdd::from_groups(
            // e41, e42, e43, scalar
            other.group0(),
            // e23, e31, e12, e45
            other.group1(),
            // e15, e25, e35, e1234
            Simd32x4::from([other[e15], other[e25], other[e35], self[e1234] + other[e1234]]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([self[e4235] + other[e4235], self[e4315] + other[e4315], self[e4125] + other[e4125], self[e3215] + other[e3215]]),
        );
    }
}
impl std::ops::BitXor<AntiCircleRotor> for Sphere {
    type Output = Sphere;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        2        0
    //  no simd        0        5        0
    fn bitxor(self, other: AntiCircleRotor) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXorAssign<AntiCircleRotor> for Sphere {
    fn bitxor_assign(&mut self, other: AntiCircleRotor) {
        *self = self.wedge(other);
    }
}
impl std::ops::BitXor<AntiDipoleInversion> for Sphere {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        4        5        0
    fn bitxor(self, other: AntiDipoleInversion) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<AntiDualNum> for Sphere {
    type Output = Sphere;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        2        0
    //  no simd        0        5        0
    fn bitxor(self, other: AntiDualNum) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXorAssign<AntiDualNum> for Sphere {
    fn bitxor_assign(&mut self, other: AntiDualNum) {
        *self = self.wedge(other);
    }
}
impl std::ops::BitXor<AntiFlector> for Sphere {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        4        0
    fn bitxor(self, other: AntiFlector) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<AntiMotor> for Sphere {
    type Output = Sphere;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        2        0
    //  no simd        0        5        0
    fn bitxor(self, other: AntiMotor) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXorAssign<AntiMotor> for Sphere {
    fn bitxor_assign(&mut self, other: AntiMotor) {
        *self = self.wedge(other);
    }
}
impl std::ops::BitXor<AntiPlane> for Sphere {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        4        0
    fn bitxor(self, other: AntiPlane) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<DualNum> for Sphere {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn bitxor(self, other: DualNum) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<Motor> for Sphere {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn bitxor(self, other: Motor) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<MultiVector> for Sphere {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        6        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        4        7        0
    //  no simd        4       10        0
    fn bitxor(self, other: MultiVector) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<RoundPoint> for Sphere {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        4        5        0
    fn bitxor(self, other: RoundPoint) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<Scalar> for Sphere {
    type Output = Sphere;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        2        0
    //  no simd        0        5        0
    fn bitxor(self, other: Scalar) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXorAssign<Scalar> for Sphere {
    fn bitxor_assign(&mut self, other: Scalar) {
        *self = self.wedge(other);
    }
}
impl std::ops::BitXor<VersorEven> for Sphere {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        4        5        0
    fn bitxor(self, other: VersorEven) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<VersorOdd> for Sphere {
    type Output = Sphere;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        2        0
    //  no simd        0        5        0
    fn bitxor(self, other: VersorOdd) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXorAssign<VersorOdd> for Sphere {
    fn bitxor_assign(&mut self, other: VersorOdd) {
        *self = self.wedge(other);
    }
}

impl From<Plane> for Sphere {
    fn from(from_plane: Plane) -> Self {
        use crate::elements::*;
        return Sphere::from_groups(
            // e4235, e4315, e4125, e3215
            Simd32x4::from([from_plane[e4235], from_plane[e4315], from_plane[e4125], from_plane[e3215]]),
            // e1234
            0.0,
        );
    }
}
impl std::ops::Mul<AntiCircleRotor> for Sphere {
    type Output = DipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       33       48        0
    //    simd3        1        1        0
    //    simd4        1        1        0
    // Totals...
    // yes simd       35       50        0
    //  no simd       40       55        0
    fn mul(self, other: AntiCircleRotor) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiDipoleInversion> for Sphere {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       47       63        0
    //    simd4        3        3        0
    // Totals...
    // yes simd       50       66        0
    //  no simd       59       75        0
    fn mul(self, other: AntiDipoleInversion) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiDualNum> for Sphere {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        2        0
    //    simd4        0        3        0
    // Totals...
    // yes simd        0        5        0
    //  no simd        0       14        0
    fn mul(self, other: AntiDualNum) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiFlatPoint> for Sphere {
    type Output = AntiDipoleInversion;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        9       21        0
    fn mul(self, other: AntiFlatPoint) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiFlector> for Sphere {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       20       35        0
    //    simd4        1        3        0
    // Totals...
    // yes simd       21       38        0
    //  no simd       24       47        0
    fn mul(self, other: AntiFlector) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiLine> for Sphere {
    type Output = DipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       16       27        0
    //    simd3        0        1        0
    // Totals...
    // yes simd       16       28        0
    //  no simd       16       30        0
    fn mul(self, other: AntiLine) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiMotor> for Sphere {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       20       32        0
    //    simd4        1        2        0
    // Totals...
    // yes simd       21       34        0
    //  no simd       24       40        0
    fn mul(self, other: AntiMotor) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiPlane> for Sphere {
    type Output = CircleRotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        5       13        0
    //    simd3        0        2        0
    //    simd4        1        1        0
    // Totals...
    // yes simd        6       16        0
    //  no simd        9       23        0
    fn mul(self, other: AntiPlane) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiScalar> for Sphere {
    type Output = RoundPoint;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        0        3        0
    //  no simd        0        9        0
    fn mul(self, other: AntiScalar) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<Circle> for Sphere {
    type Output = AntiDipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       32       47        0
    //    simd3        1        1        0
    // Totals...
    // yes simd       33       48        0
    //  no simd       35       50        0
    fn mul(self, other: Circle) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<CircleRotor> for Sphere {
    type Output = AntiDipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       37       52        0
    //    simd3        1        1        0
    // Totals...
    // yes simd       38       53        0
    //  no simd       40       55        0
    fn mul(self, other: CircleRotor) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<Dipole> for Sphere {
    type Output = DipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       32       47        0
    //    simd3        1        1        0
    // Totals...
    // yes simd       33       48        0
    //  no simd       35       50        0
    fn mul(self, other: Dipole) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<DipoleInversion> for Sphere {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       51       67        0
    //    simd4        2        2        0
    // Totals...
    // yes simd       53       69        0
    //  no simd       59       75        0
    fn mul(self, other: DipoleInversion) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<DualNum> for Sphere {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        3        0
    //    simd4        0        4        0
    // Totals...
    // yes simd        0        7        0
    //  no simd        0       19        0
    fn mul(self, other: DualNum) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<FlatPoint> for Sphere {
    type Output = DipoleInversion;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        9       23        0
    fn mul(self, other: FlatPoint) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<Flector> for Sphere {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       24       44        0
    fn mul(self, other: Flector) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<Line> for Sphere {
    type Output = AntiDipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       16       27        0
    //    simd3        0        1        0
    // Totals...
    // yes simd       16       28        0
    //  no simd       16       30        0
    fn mul(self, other: Line) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<Motor> for Sphere {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       20       32        0
    //    simd4        1        2        0
    // Totals...
    // yes simd       21       34        0
    //  no simd       24       40        0
    fn mul(self, other: Motor) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<MultiVector> for Sphere {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       72      104        0
    //    simd2        2        2        0
    //    simd3       12       12        0
    //    simd4        4        4        0
    // Totals...
    // yes simd       90      122        0
    //  no simd      128      160        0
    fn mul(self, other: MultiVector) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<Plane> for Sphere {
    type Output = AntiCircleRotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        5       17        0
    //    simd3        0        1        0
    //    simd4        1        1        0
    // Totals...
    // yes simd        6       19        0
    //  no simd        9       24        0
    fn mul(self, other: Plane) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<RoundPoint> for Sphere {
    type Output = CircleRotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        6       11        0
    //    simd3        1        2        0
    //    simd4        2        2        0
    // Totals...
    // yes simd        9       15        0
    //  no simd       17       25        0
    fn mul(self, other: RoundPoint) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<Scalar> for Sphere {
    type Output = Sphere;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        2        0
    //  no simd        0        5        0
    fn mul(self, other: Scalar) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::MulAssign<Scalar> for Sphere {
    fn mul_assign(&mut self, other: Scalar) {
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<Sphere> for Sphere {
    type Output = AntiCircleRotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        7       18        0
    //    simd3        1        2        0
    //    simd4        1        1        0
    // Totals...
    // yes simd        9       21        0
    //  no simd       14       28        0
    fn mul(self, other: Sphere) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<VersorEven> for Sphere {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       44       60        0
    //    simd4        5        5        0
    // Totals...
    // yes simd       49       65        0
    //  no simd       64       80        0
    fn mul(self, other: VersorEven) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<VersorOdd> for Sphere {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       52       68        0
    //    simd4        3        3        0
    // Totals...
    // yes simd       55       71        0
    //  no simd       64       80        0
    fn mul(self, other: VersorOdd) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Neg for Sphere {
    type Output = Sphere;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        2        0
    //  no simd        0        5        0
    fn neg(self) -> Self::Output {
        use crate::elements::*;
        return Sphere::from_groups(
            // e4235, e4315, e4125, e3215
            Simd32x4::from([self[e4235], self[e4315], self[e4125], self[e3215]]) * Simd32x4::from(-1.0),
            // e1234
            self[e1234] * -1.0,
        );
    }
}
impl std::ops::Not for Sphere {
    type Output = RoundPoint;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        3        0
    fn not(self) -> Self::Output {
        return self.right_dual();
    }
}
impl std::ops::Sub<AntiCircleRotor> for Sphere {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        3        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        0        5        0
    //  no simd        0       11        0
    fn sub(self, other: AntiCircleRotor) -> Self::Output {
        use crate::elements::*;
        return VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([other[e41], other[e42], other[e43], other[scalar]]) * Simd32x4::from(-1.0),
            // e23, e31, e12, e45
            Simd32x4::from([other[e23], other[e31], other[e12], other[e45]]) * Simd32x4::from(-1.0),
            // e15, e25, e35, e1234
            Simd32x4::from([other[e15] * -1.0, other[e25] * -1.0, other[e35] * -1.0, self[e1234]]),
            // e4235, e4315, e4125, e3215
            self.group0(),
        );
    }
}
impl std::ops::Sub<AntiDipoleInversion> for Sphere {
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
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from([other[e1], other[e2], other[e3], other[e4]]) * Simd32x4::from(-1.0),
            // e5
            other[e5] * -1.0,
            // e15, e25, e35, e45
            Simd32x4::from(0.0),
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from([other[e415], other[e425], other[e435], other[e321]]) * Simd32x4::from(-1.0),
            // e423, e431, e412
            Simd32x3::from([other[e423], other[e431], other[e412]]) * Simd32x3::from(-1.0),
            // e235, e315, e125
            Simd32x3::from([other[e235], other[e315], other[e125]]) * Simd32x3::from(-1.0),
            // e4235, e4315, e4125, e3215
            self.group0(),
            // e1234
            self[e1234],
        );
    }
}
impl std::ops::Sub<AntiDualNum> for Sphere {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        1        1        0
    fn sub(self, other: AntiDualNum) -> Self::Output {
        use crate::elements::*;
        return VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([0.0, 0.0, 0.0, other[scalar] * -1.0]),
            // e23, e31, e12, e45
            Simd32x4::from(0.0),
            // e15, e25, e35, e1234
            Simd32x4::from([0.0, 0.0, 0.0, self[e1234] - other[e1234]]),
            // e4235, e4315, e4125, e3215
            self.group0(),
        );
    }
}
impl std::ops::Sub<AntiFlatPoint> for Sphere {
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
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e15, e25, e35, e45
            Simd32x4::from(0.0),
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from([0.0, 0.0, 0.0, other[e321] * -1.0]),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from([other[e235], other[e315], other[e125]]) * Simd32x3::from(-1.0),
            // e4235, e4315, e4125, e3215
            self.group0(),
            // e1234
            self[e1234],
        );
    }
}
impl std::ops::Sub<AntiFlector> for Sphere {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        5        0
    //    simd3        0        1        0
    // Totals...
    // yes simd        0        6        0
    //  no simd        0        8        0
    fn sub(self, other: AntiFlector) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from([other[e1] * -1.0, other[e2] * -1.0, other[e3] * -1.0, 0.0]),
            // e5
            other[e5] * -1.0,
            // e15, e25, e35, e45
            Simd32x4::from(0.0),
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from([0.0, 0.0, 0.0, other[e321] * -1.0]),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from([other[e235], other[e315], other[e125]]) * Simd32x3::from(-1.0),
            // e4235, e4315, e4125, e3215
            self.group0(),
            // e1234
            self[e1234],
        );
    }
}
impl std::ops::Sub<AntiLine> for Sphere {
    type Output = DipoleInversion;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        6        0
    fn sub(self, other: AntiLine) -> Self::Output {
        use crate::elements::*;
        return DipoleInversion::from_groups(
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12, e45
            Simd32x4::from([other[e23] * -1.0, other[e31] * -1.0, other[e12] * -1.0, 0.0]),
            // e15, e25, e35, e1234
            Simd32x4::from([other[e15] * -1.0, other[e25] * -1.0, other[e35] * -1.0, self[e1234]]),
            // e4235, e4315, e4125, e3215
            self.group0(),
        );
    }
}
impl std::ops::Sub<AntiMotor> for Sphere {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        1        7        0
    fn sub(self, other: AntiMotor) -> Self::Output {
        use crate::elements::*;
        return VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([0.0, 0.0, 0.0, other[scalar] * -1.0]),
            // e23, e31, e12, e45
            Simd32x4::from([other[e23] * -1.0, other[e31] * -1.0, other[e12] * -1.0, 0.0]),
            // e15, e25, e35, e1234
            Simd32x4::from([other[e15] * -1.0, other[e25] * -1.0, other[e35] * -1.0, self[e1234]]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([self[e4235], self[e4315], self[e4125], self[e3215] - other[e3215]]),
        );
    }
}
impl std::ops::Sub<AntiPlane> for Sphere {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        4        0
    fn sub(self, other: AntiPlane) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from([other[e1] * -1.0, other[e2] * -1.0, other[e3] * -1.0, 0.0]),
            // e5
            other[e5] * -1.0,
            // e15, e25, e35, e45
            Simd32x4::from(0.0),
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
            self.group0(),
            // e1234
            self[e1234],
        );
    }
}
impl std::ops::Sub<AntiScalar> for Sphere {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn sub(self, other: AntiScalar) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, other[e12345] * -1.0]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e15, e25, e35, e45
            Simd32x4::from(0.0),
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
            self.group0(),
            // e1234
            self[e1234],
        );
    }
}
impl std::ops::Sub<Circle> for Sphere {
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
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e15, e25, e35, e45
            Simd32x4::from(0.0),
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from([other[e415], other[e425], other[e435], other[e321]]) * Simd32x4::from(-1.0),
            // e423, e431, e412
            Simd32x3::from([other[e423], other[e431], other[e412]]) * Simd32x3::from(-1.0),
            // e235, e315, e125
            Simd32x3::from([other[e235], other[e315], other[e125]]) * Simd32x3::from(-1.0),
            // e4235, e4315, e4125, e3215
            self.group0(),
            // e1234
            self[e1234],
        );
    }
}
impl std::ops::Sub<CircleRotor> for Sphere {
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
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, other[e12345] * -1.0]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e15, e25, e35, e45
            Simd32x4::from(0.0),
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from([other[e415], other[e425], other[e435], other[e321]]) * Simd32x4::from(-1.0),
            // e423, e431, e412
            Simd32x3::from([other[e423], other[e431], other[e412]]) * Simd32x3::from(-1.0),
            // e235, e315, e125
            Simd32x3::from([other[e235], other[e315], other[e125]]) * Simd32x3::from(-1.0),
            // e4235, e4315, e4125, e3215
            self.group0(),
            // e1234
            self[e1234],
        );
    }
}
impl std::ops::Sub<Dipole> for Sphere {
    type Output = DipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        3        0
    //    simd3        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        5        0
    //  no simd        0       10        0
    fn sub(self, other: Dipole) -> Self::Output {
        use crate::elements::*;
        return DipoleInversion::from_groups(
            // e41, e42, e43
            Simd32x3::from([other[e41], other[e42], other[e43]]) * Simd32x3::from(-1.0),
            // e23, e31, e12, e45
            Simd32x4::from([other[e23], other[e31], other[e12], other[e45]]) * Simd32x4::from(-1.0),
            // e15, e25, e35, e1234
            Simd32x4::from([other[e15] * -1.0, other[e25] * -1.0, other[e35] * -1.0, self[e1234]]),
            // e4235, e4315, e4125, e3215
            self.group0(),
        );
    }
}
impl std::ops::Sub<DipoleInversion> for Sphere {
    type Output = DipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        5        3        0
    //    simd3        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        5        5        0
    //  no simd        5       10        0
    fn sub(self, other: DipoleInversion) -> Self::Output {
        use crate::elements::*;
        return DipoleInversion::from_groups(
            // e41, e42, e43
            Simd32x3::from([other[e41], other[e42], other[e43]]) * Simd32x3::from(-1.0),
            // e23, e31, e12, e45
            Simd32x4::from([other[e23], other[e31], other[e12], other[e45]]) * Simd32x4::from(-1.0),
            // e15, e25, e35, e1234
            Simd32x4::from([other[e15] * -1.0, other[e25] * -1.0, other[e35] * -1.0, self[e1234] - other[e1234]]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([self[e4235] - other[e4235], self[e4315] - other[e4315], self[e4125] - other[e4125], self[e3215] - other[e3215]]),
        );
    }
}
impl std::ops::Sub<DualNum> for Sphere {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        2        0
    fn sub(self, other: DualNum) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, other[e12345] * -1.0]),
            // e1, e2, e3, e4
            Simd32x4::from([0.0, 0.0, 0.0, other[e4] * -1.0]),
            // e5
            0.0,
            // e15, e25, e35, e45
            Simd32x4::from(0.0),
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
            self.group0(),
            // e1234
            self[e1234],
        );
    }
}
impl std::ops::Sub<FlatPoint> for Sphere {
    type Output = DipoleInversion;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        4        0
    fn sub(self, other: FlatPoint) -> Self::Output {
        use crate::elements::*;
        return DipoleInversion::from_groups(
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12, e45
            Simd32x4::from([0.0, 0.0, 0.0, other[e45] * -1.0]),
            // e15, e25, e35, e1234
            Simd32x4::from([other[e15] * -1.0, other[e25] * -1.0, other[e35] * -1.0, self[e1234]]),
            // e4235, e4315, e4125, e3215
            self.group0(),
        );
    }
}
impl std::ops::Sub<Flector> for Sphere {
    type Output = DipoleInversion;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        4        4        0
    fn sub(self, other: Flector) -> Self::Output {
        use crate::elements::*;
        return DipoleInversion::from_groups(
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12, e45
            Simd32x4::from([0.0, 0.0, 0.0, other[e45] * -1.0]),
            // e15, e25, e35, e1234
            Simd32x4::from([other[e15] * -1.0, other[e25] * -1.0, other[e35] * -1.0, self[e1234]]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([self[e4235] - other[e4235], self[e4315] - other[e4315], self[e4125] - other[e4125], self[e3215] - other[e3215]]),
        );
    }
}
impl std::ops::Sub<Line> for Sphere {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        3        0
    //    simd3        0        1        0
    // Totals...
    // yes simd        0        4        0
    //  no simd        0        6        0
    fn sub(self, other: Line) -> Self::Output {
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
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from([other[e415] * -1.0, other[e425] * -1.0, other[e435] * -1.0, 0.0]),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from([other[e235], other[e315], other[e125]]) * Simd32x3::from(-1.0),
            // e4235, e4315, e4125, e3215
            self.group0(),
            // e1234
            self[e1234],
        );
    }
}
impl std::ops::Sub<Motor> for Sphere {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        5        0
    //    simd3        0        1        0
    // Totals...
    // yes simd        0        6        0
    //  no simd        0        8        0
    fn sub(self, other: Motor) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, other[e12345] * -1.0]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            other[e5] * -1.0,
            // e15, e25, e35, e45
            Simd32x4::from(0.0),
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from([other[e415] * -1.0, other[e425] * -1.0, other[e435] * -1.0, 0.0]),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from([other[e235], other[e315], other[e125]]) * Simd32x3::from(-1.0),
            // e4235, e4315, e4125, e3215
            self.group0(),
            // e1234
            self[e1234],
        );
    }
}
impl std::ops::Sub<MultiVector> for Sphere {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        5        1        0
    //    simd2        0        1        0
    //    simd3        0        4        0
    //    simd4        0        3        0
    // Totals...
    // yes simd        5        9        0
    //  no simd        5       27        0
    fn sub(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([other[scalar], other[e12345]]) * Simd32x2::from(-1.0),
            // e1, e2, e3, e4
            Simd32x4::from([other[e1], other[e2], other[e3], other[e4]]) * Simd32x4::from(-1.0),
            // e5
            other[e5] * -1.0,
            // e15, e25, e35, e45
            Simd32x4::from([other[e15], other[e25], other[e35], other[e45]]) * Simd32x4::from(-1.0),
            // e41, e42, e43
            Simd32x3::from([other[e41], other[e42], other[e43]]) * Simd32x3::from(-1.0),
            // e23, e31, e12
            Simd32x3::from([other[e23], other[e31], other[e12]]) * Simd32x3::from(-1.0),
            // e415, e425, e435, e321
            Simd32x4::from([other[e415], other[e425], other[e435], other[e321]]) * Simd32x4::from(-1.0),
            // e423, e431, e412
            Simd32x3::from([other[e423], other[e431], other[e412]]) * Simd32x3::from(-1.0),
            // e235, e315, e125
            Simd32x3::from([other[e235], other[e315], other[e125]]) * Simd32x3::from(-1.0),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([self[e4235] - other[e4235], self[e4315] - other[e4315], self[e4125] - other[e4125], self[e3215] - other[e3215]]),
            // e1234
            self[e1234] - other[e1234],
        );
    }
}
impl std::ops::Sub<Plane> for Sphere {
    type Output = Sphere;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        4        0        0
    fn sub(self, other: Plane) -> Self::Output {
        use crate::elements::*;
        return Sphere::from_groups(
            // e4235, e4315, e4125, e3215
            Simd32x4::from([self[e4235] - other[e4235], self[e4315] - other[e4315], self[e4125] - other[e4125], self[e3215] - other[e3215]]),
            // e1234
            self[e1234],
        );
    }
}
impl std::ops::SubAssign<Plane> for Sphere {
    fn sub_assign(&mut self, other: Plane) {
        use crate::elements::*;
        *self = Sphere::from_groups(
            // e4235, e4315, e4125, e3215
            Simd32x4::from([self[e4235] - other[e4235], self[e4315] - other[e4315], self[e4125] - other[e4125], self[e3215] - other[e3215]]),
            // e1234
            self[e1234],
        );
    }
}
impl std::ops::Sub<RoundPoint> for Sphere {
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
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from([other[e1], other[e2], other[e3], other[e4]]) * Simd32x4::from(-1.0),
            // e5
            other[e5] * -1.0,
            // e15, e25, e35, e45
            Simd32x4::from(0.0),
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
            self.group0(),
            // e1234
            self[e1234],
        );
    }
}
impl std::ops::Sub<Scalar> for Sphere {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn sub(self, other: Scalar) -> Self::Output {
        use crate::elements::*;
        return VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([0.0, 0.0, 0.0, other[scalar] * -1.0]),
            // e23, e31, e12, e45
            Simd32x4::from(0.0),
            // e15, e25, e35, e1234
            Simd32x4::from([0.0, 0.0, 0.0, self[e1234]]),
            // e4235, e4315, e4125, e3215
            self.group0(),
        );
    }
}
impl std::ops::Sub<Sphere> for Sphere {
    type Output = Sphere;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        5        0        0
    fn sub(self, other: Sphere) -> Self::Output {
        use crate::elements::*;
        return Sphere::from_groups(
            // e4235, e4315, e4125, e3215
            Simd32x4::from([self[e4235] - other[e4235], self[e4315] - other[e4315], self[e4125] - other[e4125], self[e3215] - other[e3215]]),
            // e1234
            self[e1234] - other[e1234],
        );
    }
}
impl std::ops::SubAssign<Sphere> for Sphere {
    fn sub_assign(&mut self, other: Sphere) {
        use crate::elements::*;
        *self = Sphere::from_groups(
            // e4235, e4315, e4125, e3215
            Simd32x4::from([self[e4235] - other[e4235], self[e4315] - other[e4315], self[e4125] - other[e4125], self[e3215] - other[e3215]]),
            // e1234
            self[e1234] - other[e1234],
        );
    }
}
impl std::ops::Sub<VersorEven> for Sphere {
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
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, other[e12345] * -1.0]),
            // e1, e2, e3, e4
            Simd32x4::from([other[e1], other[e2], other[e3], other[e4]]) * Simd32x4::from(-1.0),
            // e5
            other[e5] * -1.0,
            // e15, e25, e35, e45
            Simd32x4::from(0.0),
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from([other[e415], other[e425], other[e435], other[e321]]) * Simd32x4::from(-1.0),
            // e423, e431, e412
            Simd32x3::from([other[e423], other[e431], other[e412]]) * Simd32x3::from(-1.0),
            // e235, e315, e125
            Simd32x3::from([other[e235], other[e315], other[e125]]) * Simd32x3::from(-1.0),
            // e4235, e4315, e4125, e3215
            self.group0(),
            // e1234
            self[e1234],
        );
    }
}
impl std::ops::Sub<VersorOdd> for Sphere {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        5        3        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        5        5        0
    //  no simd        5       11        0
    fn sub(self, other: VersorOdd) -> Self::Output {
        use crate::elements::*;
        return VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([other[e41], other[e42], other[e43], other[scalar]]) * Simd32x4::from(-1.0),
            // e23, e31, e12, e45
            Simd32x4::from([other[e23], other[e31], other[e12], other[e45]]) * Simd32x4::from(-1.0),
            // e15, e25, e35, e1234
            Simd32x4::from([other[e15] * -1.0, other[e25] * -1.0, other[e35] * -1.0, self[e1234] - other[e1234]]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([self[e4235] - other[e4235], self[e4315] - other[e4315], self[e4125] - other[e4125], self[e3215] - other[e3215]]),
        );
    }
}

impl TryFrom<AntiDualNum> for Sphere {
    type Error = String;
    fn try_from(anti_dual_num: AntiDualNum) -> Result<Self, Self::Error> {
        use crate::elements::*;
        let mut error_string = String::new();
        let mut fail = false;
        let el = anti_dual_num[1];
        if el != 0.0 {
            fail = true;
            error_string.push_str("scalar: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from AntiDualNum do not fit into Sphere { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(Sphere::from_groups(/* e4235, e4315, e4125, e3215 */ Simd32x4::from(0.0), /* e1234 */ anti_dual_num[e1234]));
    }
}

impl TryFrom<AntiMotor> for Sphere {
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
        let el = anti_motor[3];
        if el != 0.0 {
            fail = true;
            error_string.push_str("scalar: ");
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
            let mut error = "Elements from AntiMotor do not fit into Sphere { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(Sphere::from_groups(
            // e4235, e4315, e4125, e3215
            Simd32x4::from([0.0, 0.0, 0.0, anti_motor[e3215]]),
            // e1234
            0.0,
        ));
    }
}

impl TryFrom<DipoleInversion> for Sphere {
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
        if fail {
            let mut error = "Elements from DipoleInversion do not fit into Sphere { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(Sphere::from_groups(
            // e4235, e4315, e4125, e3215
            Simd32x4::from([dipole_inversion[e4235], dipole_inversion[e4315], dipole_inversion[e4125], dipole_inversion[e3215]]),
            // e1234
            dipole_inversion[e1234],
        ));
    }
}

impl TryFrom<Flector> for Sphere {
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
        let el = flector[3];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e45: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from Flector do not fit into Sphere { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(Sphere::from_groups(
            // e4235, e4315, e4125, e3215
            Simd32x4::from([flector[e4235], flector[e4315], flector[e4125], flector[e3215]]),
            // e1234
            0.0,
        ));
    }
}

impl TryFrom<MultiVector> for Sphere {
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
        if fail {
            let mut error = "Elements from MultiVector do not fit into Sphere { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(Sphere::from_groups(
            // e4235, e4315, e4125, e3215
            Simd32x4::from([multi_vector[e4235], multi_vector[e4315], multi_vector[e4125], multi_vector[e3215]]),
            // e1234
            multi_vector[e1234],
        ));
    }
}

impl TryFrom<VersorOdd> for Sphere {
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
        let el = versor_odd[3];
        if el != 0.0 {
            fail = true;
            error_string.push_str("scalar: ");
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
        if fail {
            let mut error = "Elements from VersorOdd do not fit into Sphere { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(Sphere::from_groups(
            // e4235, e4315, e4125, e3215
            Simd32x4::from([versor_odd[e4235], versor_odd[e4315], versor_odd[e4125], versor_odd[e3215]]),
            // e1234
            versor_odd[e1234],
        ));
    }
}
