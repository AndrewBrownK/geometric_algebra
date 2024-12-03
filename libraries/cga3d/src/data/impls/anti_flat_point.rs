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
// Total Implementations: 102
//
// Yes SIMD:   add/sub     mul     div
//  Minimum:         0       0       0
//   Median:         0       2       0
//  Average:         2       5       0
//  Maximum:        50      77       0
//
//  No SIMD:   add/sub     mul     div
//  Minimum:         0       0       0
//   Median:         0       4       0
//  Average:         6      11       0
//  Maximum:        96     132       0
impl std::ops::Add<AntiCircleRotor> for AntiFlatPoint {
    type Output = MultiVector;
    fn add(self, other: AntiCircleRotor) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([other[scalar], 0.0]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e15, e25, e35, e45
            Simd32x4::from([other[e15], other[e25], other[e35], other[e45]]),
            // e41, e42, e43
            other.group0(),
            // e23, e31, e12
            other.group1().truncate_to_3(),
            // e415, e425, e435, e321
            Simd32x3::from(0.0).extend_to_4(self[e321]),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            self.group0().truncate_to_3(),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(0.0),
            // e1234
            0.0,
        );
    }
}
impl std::ops::Add<AntiDipoleInversion> for AntiFlatPoint {
    type Output = AntiDipoleInversion;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        2        0        0
    // no simd        8        0        0
    fn add(self, other: AntiDipoleInversion) -> Self::Output {
        use crate::elements::*;
        return AntiDipoleInversion::from_groups(
            // e423, e431, e412
            other.group0(),
            // e415, e425, e435, e321
            other.group1() + Simd32x3::from(0.0).extend_to_4(self[e321]),
            // e235, e315, e125, e4
            Simd32x4::from([self[e235], self[e315], self[e125], 0.0]) + other.group2(),
            // e1, e2, e3, e5
            other.group3(),
        );
    }
}
impl std::ops::Add<AntiDualNum> for AntiFlatPoint {
    type Output = MultiVector;
    fn add(self, other: AntiDualNum) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([other[scalar], 0.0]),
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
            Simd32x3::from(0.0).extend_to_4(self[e321]),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            self.group0().truncate_to_3(),
            // e4235, e4315, e4125, e3215
            Simd32x3::from(0.0).extend_to_4(other[e3215]),
            // e1234
            0.0,
        );
    }
}
impl std::ops::Add<AntiFlatPoint> for AntiFlatPoint {
    type Output = AntiFlatPoint;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        1        0        0
    // no simd        4        0        0
    fn add(self, other: AntiFlatPoint) -> Self::Output {
        return AntiFlatPoint::from_groups(/* e235, e315, e125, e321 */ other.group0() + self.group0());
    }
}
impl std::ops::AddAssign<AntiFlatPoint> for AntiFlatPoint {
    fn add_assign(&mut self, other: AntiFlatPoint) {
        *self = AntiFlatPoint::from_groups(/* e235, e315, e125, e321 */ other.group0() + self.group0());
    }
}
impl std::ops::Add<AntiFlector> for AntiFlatPoint {
    type Output = AntiFlector;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        1        0        0
    // no simd        4        0        0
    fn add(self, other: AntiFlector) -> Self::Output {
        return AntiFlector::from_groups(/* e235, e315, e125, e321 */ self.group0() + other.group0(), /* e1, e2, e3, e5 */ other.group1());
    }
}
impl std::ops::Add<AntiLine> for AntiFlatPoint {
    type Output = MultiVector;
    fn add(self, other: AntiLine) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e15, e25, e35, e45
            other.group1().extend_to_4(0.0),
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12
            other.group0(),
            // e415, e425, e435, e321
            Simd32x3::from(0.0).extend_to_4(self[e321]),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            self.group0().truncate_to_3(),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(0.0),
            // e1234
            0.0,
        );
    }
}
impl std::ops::Add<AntiMotor> for AntiFlatPoint {
    type Output = MultiVector;
    fn add(self, other: AntiMotor) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([other[scalar], 0.0]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e15, e25, e35, e45
            Simd32x4::from([other[e15], other[e25], other[e35], 0.0]),
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12
            other.group0().truncate_to_3(),
            // e415, e425, e435, e321
            Simd32x3::from(0.0).extend_to_4(self[e321]),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            self.group0().truncate_to_3(),
            // e4235, e4315, e4125, e3215
            Simd32x3::from(0.0).extend_to_4(other[e3215]),
            // e1234
            0.0,
        );
    }
}
impl std::ops::Add<AntiPlane> for AntiFlatPoint {
    type Output = AntiFlector;
    fn add(self, other: AntiPlane) -> Self::Output {
        return AntiFlector::from_groups(/* e235, e315, e125, e321 */ self.group0(), /* e1, e2, e3, e5 */ other.group0());
    }
}
impl std::ops::Add<AntiScalar> for AntiFlatPoint {
    type Output = CircleRotor;
    fn add(self, other: AntiScalar) -> Self::Output {
        use crate::elements::*;
        return CircleRotor::from_groups(
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x3::from(0.0).extend_to_4(self[e321]),
            // e235, e315, e125, e12345
            Simd32x4::from([self[e235], self[e315], self[e125], other[e12345]]),
        );
    }
}
impl std::ops::Add<Circle> for AntiFlatPoint {
    type Output = Circle;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        0        0
    //    simd3        1        0        0
    // Totals...
    // yes simd        2        0        0
    //  no simd        4        0        0
    fn add(self, other: Circle) -> Self::Output {
        use crate::elements::*;
        return Circle::from_groups(
            // e423, e431, e412
            other.group0(),
            // e415, e425, e435, e321
            other.group1().truncate_to_3().extend_to_4(self[e321] + other[e321]),
            // e235, e315, e125
            other.group2() + self.group0().truncate_to_3(),
        );
    }
}
impl std::ops::Add<CircleRotor> for AntiFlatPoint {
    type Output = CircleRotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        0        0
    //    simd4        1        0        0
    // Totals...
    // yes simd        2        0        0
    //  no simd        5        0        0
    fn add(self, other: CircleRotor) -> Self::Output {
        use crate::elements::*;
        return CircleRotor::from_groups(
            // e423, e431, e412
            other.group0(),
            // e415, e425, e435, e321
            other.group1().truncate_to_3().extend_to_4(self[e321] + other[e321]),
            // e235, e315, e125, e12345
            Simd32x4::from([other[e235], other[e315], other[e125], 0.0]) + self.group0().truncate_to_3().extend_to_4(other[e12345]),
        );
    }
}
impl std::ops::Add<Dipole> for AntiFlatPoint {
    type Output = MultiVector;
    fn add(self, other: Dipole) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e15, e25, e35, e45
            other.group2().extend_to_4(other[e45]),
            // e41, e42, e43
            other.group0(),
            // e23, e31, e12
            other.group1().truncate_to_3(),
            // e415, e425, e435, e321
            Simd32x3::from(0.0).extend_to_4(self[e321]),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            self.group0().truncate_to_3(),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(0.0),
            // e1234
            0.0,
        );
    }
}
impl std::ops::Add<DipoleInversion> for AntiFlatPoint {
    type Output = MultiVector;
    fn add(self, other: DipoleInversion) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e15, e25, e35, e45
            Simd32x4::from([other[e15], other[e25], other[e35], other[e45]]),
            // e41, e42, e43
            other.group0(),
            // e23, e31, e12
            other.group1().truncate_to_3(),
            // e415, e425, e435, e321
            Simd32x3::from(0.0).extend_to_4(self[e321]),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            self.group0().truncate_to_3(),
            // e4235, e4315, e4125, e3215
            other.group3(),
            // e1234
            other[e1234],
        );
    }
}
impl std::ops::Add<DualNum> for AntiFlatPoint {
    type Output = VersorEven;
    fn add(self, other: DualNum) -> Self::Output {
        use crate::elements::*;
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x3::from(0.0).extend_to_4(other[e12345]),
            // e415, e425, e435, e321
            Simd32x3::from(0.0).extend_to_4(self[e321]),
            // e235, e315, e125, e5
            Simd32x4::from([self[e235], self[e315], self[e125], other[e5]]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
        );
    }
}
impl std::ops::Add<FlatPoint> for AntiFlatPoint {
    type Output = MultiVector;
    fn add(self, other: FlatPoint) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e15, e25, e35, e45
            other.group0(),
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x3::from(0.0).extend_to_4(self[e321]),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            self.group0().truncate_to_3(),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(0.0),
            // e1234
            0.0,
        );
    }
}
impl std::ops::Add<Flector> for AntiFlatPoint {
    type Output = MultiVector;
    fn add(self, other: Flector) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e15, e25, e35, e45
            other.group0(),
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x3::from(0.0).extend_to_4(self[e321]),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            self.group0().truncate_to_3(),
            // e4235, e4315, e4125, e3215
            other.group1(),
            // e1234
            0.0,
        );
    }
}
impl std::ops::Add<Line> for AntiFlatPoint {
    type Output = Circle;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        1        0        0
    // no simd        3        0        0
    fn add(self, other: Line) -> Self::Output {
        use crate::elements::*;
        return Circle::from_groups(
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            other.group0().extend_to_4(self[e321]),
            // e235, e315, e125
            other.group1() + self.group0().truncate_to_3(),
        );
    }
}
impl std::ops::Add<Motor> for AntiFlatPoint {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        1        0        0
    // no simd        4        0        0
    fn add(self, other: Motor) -> Self::Output {
        use crate::elements::*;
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x3::from(0.0).extend_to_4(other[e12345]),
            // e415, e425, e435, e321
            Simd32x4::from([other[e415], other[e425], other[e435], self[e321]]),
            // e235, e315, e125, e5
            Simd32x4::from([other[e235], other[e315], other[e125], 0.0]) + self.group0().truncate_to_3().extend_to_4(other[e5]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
        );
    }
}
impl std::ops::Add<MultiVector> for AntiFlatPoint {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        0        0
    //    simd3        1        0        0
    // Totals...
    // yes simd        2        0        0
    //  no simd        4        0        0
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
            other.group6().truncate_to_3().extend_to_4(self[e321] + other[e321]),
            // e423, e431, e412
            other.group7(),
            // e235, e315, e125
            other.group8() + self.group0().truncate_to_3(),
            // e4235, e4315, e4125, e3215
            other.group9(),
            // e1234
            other[e1234],
        );
    }
}
impl std::ops::Add<Plane> for AntiFlatPoint {
    type Output = MultiVector;
    fn add(self, other: Plane) -> Self::Output {
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
            Simd32x3::from(0.0).extend_to_4(self[e321]),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            self.group0().truncate_to_3(),
            // e4235, e4315, e4125, e3215
            other.group0(),
            // e1234
            0.0,
        );
    }
}
impl std::ops::Add<RoundPoint> for AntiFlatPoint {
    type Output = AntiDipoleInversion;
    fn add(self, other: RoundPoint) -> Self::Output {
        use crate::elements::*;
        return AntiDipoleInversion::from_groups(
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x3::from(0.0).extend_to_4(self[e321]),
            // e235, e315, e125, e4
            Simd32x4::from([self[e235], self[e315], self[e125], other[e4]]),
            // e1, e2, e3, e5
            Simd32x4::from([other[e1], other[e2], other[e3], other[e5]]),
        );
    }
}
impl std::ops::Add<Scalar> for AntiFlatPoint {
    type Output = MultiVector;
    fn add(self, other: Scalar) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([other[scalar], 0.0]),
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
            Simd32x3::from(0.0).extend_to_4(self[e321]),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            self.group0().truncate_to_3(),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(0.0),
            // e1234
            0.0,
        );
    }
}
impl std::ops::Add<Sphere> for AntiFlatPoint {
    type Output = MultiVector;
    fn add(self, other: Sphere) -> Self::Output {
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
            Simd32x3::from(0.0).extend_to_4(self[e321]),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            self.group0().truncate_to_3(),
            // e4235, e4315, e4125, e3215
            other.group0(),
            // e1234
            other[e1234],
        );
    }
}
impl std::ops::Add<VersorEven> for AntiFlatPoint {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        0        0
    //    simd4        1        0        0
    // Totals...
    // yes simd        2        0        0
    //  no simd        5        0        0
    fn add(self, other: VersorEven) -> Self::Output {
        use crate::elements::*;
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            other.group0(),
            // e415, e425, e435, e321
            other.group1().truncate_to_3().extend_to_4(self[e321] + other[e321]),
            // e235, e315, e125, e5
            Simd32x4::from([other[e235], other[e315], other[e125], 0.0]) + self.group0().truncate_to_3().extend_to_4(other[e5]),
            // e1, e2, e3, e4
            other.group3(),
        );
    }
}
impl std::ops::Add<VersorOdd> for AntiFlatPoint {
    type Output = MultiVector;
    fn add(self, other: VersorOdd) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([other[scalar], 0.0]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e15, e25, e35, e45
            Simd32x4::from([other[e15], other[e25], other[e35], other[e45]]),
            // e41, e42, e43
            other.group0().truncate_to_3(),
            // e23, e31, e12
            other.group1().truncate_to_3(),
            // e415, e425, e435, e321
            Simd32x3::from(0.0).extend_to_4(self[e321]),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            self.group0().truncate_to_3(),
            // e4235, e4315, e4125, e3215
            other.group3(),
            // e1234
            other[e1234],
        );
    }
}
impl std::ops::BitXor<AntiCircleRotor> for AntiFlatPoint {
    type Output = CircleRotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        5        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        3        6        0
    //  no simd        3        9        0
    fn bitxor(self, other: AntiCircleRotor) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<AntiDipoleInversion> for AntiFlatPoint {
    type Output = Sphere;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        6        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        3        8        0
    //  no simd        3       14        0
    fn bitxor(self, other: AntiDipoleInversion) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<AntiDualNum> for AntiFlatPoint {
    type Output = AntiFlatPoint;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn bitxor(self, other: AntiDualNum) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXorAssign<AntiDualNum> for AntiFlatPoint {
    fn bitxor_assign(&mut self, other: AntiDualNum) {
        *self = self.wedge(other);
    }
}
impl std::ops::BitXor<AntiFlector> for AntiFlatPoint {
    type Output = AntiDualNum;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        4        0
    fn bitxor(self, other: AntiFlector) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<AntiMotor> for AntiFlatPoint {
    type Output = AntiFlatPoint;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn bitxor(self, other: AntiMotor) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXorAssign<AntiMotor> for AntiFlatPoint {
    fn bitxor_assign(&mut self, other: AntiMotor) {
        *self = self.wedge(other);
    }
}
impl std::ops::BitXor<AntiPlane> for AntiFlatPoint {
    type Output = AntiDualNum;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        4        0
    fn bitxor(self, other: AntiPlane) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<Dipole> for AntiFlatPoint {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        4        0
    fn bitxor(self, other: Dipole) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<DipoleInversion> for AntiFlatPoint {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        4        0
    fn bitxor(self, other: DipoleInversion) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<DualNum> for AntiFlatPoint {
    type Output = AntiDualNum;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd2        0        1        0
    // Totals...
    // yes simd        0        2        0
    //  no simd        0        3        0
    fn bitxor(self, other: DualNum) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<FlatPoint> for AntiFlatPoint {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        2        0
    fn bitxor(self, other: FlatPoint) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<Flector> for AntiFlatPoint {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        2        0
    fn bitxor(self, other: Flector) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<Motor> for AntiFlatPoint {
    type Output = AntiDualNum;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd2        0        1        0
    // Totals...
    // yes simd        0        2        0
    //  no simd        0        3        0
    fn bitxor(self, other: Motor) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<MultiVector> for AntiFlatPoint {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        6       11        0
    //    simd3        0        1        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        6       14        0
    //  no simd        6       22        0
    fn bitxor(self, other: MultiVector) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<RoundPoint> for AntiFlatPoint {
    type Output = Sphere;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        6        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        3        8        0
    //  no simd        3       14        0
    fn bitxor(self, other: RoundPoint) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<Scalar> for AntiFlatPoint {
    type Output = AntiFlatPoint;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn bitxor(self, other: Scalar) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXorAssign<Scalar> for AntiFlatPoint {
    fn bitxor_assign(&mut self, other: Scalar) {
        *self = self.wedge(other);
    }
}
impl std::ops::BitXor<VersorEven> for AntiFlatPoint {
    type Output = Sphere;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        6        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        3        8        0
    //  no simd        3       14        0
    fn bitxor(self, other: VersorEven) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<VersorOdd> for AntiFlatPoint {
    type Output = CircleRotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        5        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        3        6        0
    //  no simd        3        9        0
    fn bitxor(self, other: VersorOdd) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::Mul<AntiCircleRotor> for AntiFlatPoint {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        8       19        0
    //    simd3        0        3        0
    //    simd4        6        5        0
    // Totals...
    // yes simd       14       27        0
    //  no simd       32       48        0
    fn mul(self, other: AntiCircleRotor) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiDipoleInversion> for AntiFlatPoint {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       17       28        0
    //    simd3        5        6        0
    //    simd4        3        5        0
    // Totals...
    // yes simd       25       39        0
    //  no simd       44       66        0
    fn mul(self, other: AntiDipoleInversion) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiDualNum> for AntiFlatPoint {
    type Output = AntiFlector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        2        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        3        0
    //  no simd        0        6        0
    fn mul(self, other: AntiDualNum) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiFlatPoint> for AntiFlatPoint {
    type Output = AntiMotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        2        0
    //    simd3        1        2        0
    // Totals...
    // yes simd        1        4        0
    //  no simd        3        8        0
    fn mul(self, other: AntiFlatPoint) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiFlector> for AntiFlatPoint {
    type Output = AntiMotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        8        0
    //    simd4        2        4        0
    // Totals...
    // yes simd        6       12        0
    //  no simd       12       24        0
    fn mul(self, other: AntiFlector) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiLine> for AntiFlatPoint {
    type Output = AntiFlector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        3        0
    //    simd3        0        3        0
    //    simd4        2        1        0
    // Totals...
    // yes simd        4        7        0
    //  no simd       10       16        0
    fn mul(self, other: AntiLine) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiMotor> for AntiFlatPoint {
    type Output = AntiFlector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        5        0
    //    simd3        3        4        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        6       10        0
    //  no simd       12       21        0
    fn mul(self, other: AntiMotor) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiPlane> for AntiFlatPoint {
    type Output = AntiMotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        9        0
    //    simd4        1        4        0
    // Totals...
    // yes simd        3       13        0
    //  no simd        6       25        0
    fn mul(self, other: AntiPlane) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiScalar> for AntiFlatPoint {
    type Output = FlatPoint;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        2        0
    // no simd        0        8        0
    fn mul(self, other: AntiScalar) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<Circle> for AntiFlatPoint {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4       20        0
    //    simd3        0        5        0
    //    simd4        6        4        0
    // Totals...
    // yes simd       10       29        0
    //  no simd       28       51        0
    fn mul(self, other: Circle) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<CircleRotor> for AntiFlatPoint {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        5       21        0
    //    simd3        0        6        0
    //    simd4        7        4        0
    // Totals...
    // yes simd       12       31        0
    //  no simd       33       55        0
    fn mul(self, other: CircleRotor) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<Dipole> for AntiFlatPoint {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4       16        0
    //    simd3        0        4        0
    //    simd4        6        4        0
    // Totals...
    // yes simd       10       24        0
    //  no simd       28       44        0
    fn mul(self, other: Dipole) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<DipoleInversion> for AntiFlatPoint {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        6       14        0
    //    simd3        3        5        0
    //    simd4        8        8        0
    // Totals...
    // yes simd       17       27        0
    //  no simd       47       61        0
    fn mul(self, other: DipoleInversion) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<DualNum> for AntiFlatPoint {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        0        3        0
    //  no simd        0        9        0
    fn mul(self, other: DualNum) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<FlatPoint> for AntiFlatPoint {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        2        0
    //    simd3        1        2        0
    // Totals...
    // yes simd        1        4        0
    //  no simd        3        8        0
    fn mul(self, other: FlatPoint) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<Flector> for AntiFlatPoint {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        8        0
    //    simd4        2        4        0
    // Totals...
    // yes simd        6       12        0
    //  no simd       12       24        0
    fn mul(self, other: Flector) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<Line> for AntiFlatPoint {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        3        0
    //    simd3        0        3        0
    //    simd4        2        1        0
    // Totals...
    // yes simd        4        7        0
    //  no simd       10       16        0
    fn mul(self, other: Line) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<Motor> for AntiFlatPoint {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        5        0
    //    simd3        3        4        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        6       10        0
    //  no simd       12       21        0
    fn mul(self, other: Motor) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<MultiVector> for AntiFlatPoint {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       30       51        0
    //    simd2        1        1        0
    //    simd3       12       21        0
    //    simd4        7        4        0
    // Totals...
    // yes simd       50       77        0
    //  no simd       96      132        0
    fn mul(self, other: MultiVector) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<Plane> for AntiFlatPoint {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        2        0
    //    simd4        2        5        0
    // Totals...
    // yes simd        3        7        0
    //  no simd        9       22        0
    fn mul(self, other: Plane) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<RoundPoint> for AntiFlatPoint {
    type Output = DipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        6        0
    //    simd3        2        4        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        5       12        0
    //  no simd        9       26        0
    fn mul(self, other: RoundPoint) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<Scalar> for AntiFlatPoint {
    type Output = AntiFlatPoint;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn mul(self, other: Scalar) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::MulAssign<Scalar> for AntiFlatPoint {
    fn mul_assign(&mut self, other: Scalar) {
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<Sphere> for AntiFlatPoint {
    type Output = AntiDipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        5        0
    //    simd3        2        4        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        5       11        0
    //  no simd        9       25        0
    fn mul(self, other: Sphere) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<VersorEven> for AntiFlatPoint {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       18       29        0
    //    simd3        6        7        0
    //    simd4        3        5        0
    // Totals...
    // yes simd       27       41        0
    //  no simd       48       70        0
    fn mul(self, other: VersorEven) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<VersorOdd> for AntiFlatPoint {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        7       13        0
    //    simd3        3        4        0
    //    simd4        8       10        0
    // Totals...
    // yes simd       18       27        0
    //  no simd       48       65        0
    fn mul(self, other: VersorOdd) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Neg for AntiFlatPoint {
    type Output = AntiFlatPoint;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn neg(self) -> Self::Output {
        return AntiFlatPoint::from_groups(/* e235, e315, e125, e321 */ self.group0() * Simd32x4::from(-1.0));
    }
}
impl std::ops::Not for AntiFlatPoint {
    type Output = FlatPoint;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn not(self) -> Self::Output {
        return self.right_dual();
    }
}
impl std::ops::Sub<AntiCircleRotor> for AntiFlatPoint {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd2        0        1        0
    //    simd3        0        2        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        4        0
    //  no simd        0       12        0
    fn sub(self, other: AntiCircleRotor) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([other[scalar], 1.0]) * Simd32x2::from([-1.0, 0.0]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e15, e25, e35, e45
            other.group2().truncate_to_3().extend_to_4(other[e45]) * Simd32x4::from(-1.0),
            // e41, e42, e43
            other.group0() * Simd32x3::from(-1.0),
            // e23, e31, e12
            other.group1().truncate_to_3() * Simd32x3::from(-1.0),
            // e415, e425, e435, e321
            Simd32x3::from(0.0).extend_to_4(self[e321]),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            self.group0().truncate_to_3(),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(0.0),
            // e1234
            0.0,
        );
    }
}
impl std::ops::Sub<AntiDipoleInversion> for AntiFlatPoint {
    type Output = AntiDipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        0        0
    //    simd3        1        1        0
    //    simd4        0        3        0
    // Totals...
    // yes simd        2        4        0
    //  no simd        4       15        0
    fn sub(self, other: AntiDipoleInversion) -> Self::Output {
        use crate::elements::*;
        return AntiDipoleInversion::from_groups(
            // e423, e431, e412
            other.group0() * Simd32x3::from(-1.0),
            // e415, e425, e435, e321
            other.group1().truncate_to_3().extend_to_4(self[e321] - other[e321]) * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e235, e315, e125, e4
            (self.group0().truncate_to_3() - other.group2().truncate_to_3()).extend_to_4(other[e4]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e1, e2, e3, e5
            other.group3() * Simd32x4::from(-1.0),
        );
    }
}
impl std::ops::Sub<AntiDualNum> for AntiFlatPoint {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd2        0        1        0
    // Totals...
    // yes simd        0        2        0
    //  no simd        0        3        0
    fn sub(self, other: AntiDualNum) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([other[scalar], 1.0]) * Simd32x2::from([-1.0, 0.0]),
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
            Simd32x3::from(0.0).extend_to_4(self[e321]),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            self.group0().truncate_to_3(),
            // e4235, e4315, e4125, e3215
            Simd32x3::from(0.0).extend_to_4(other[e3215] * -1.0),
            // e1234
            0.0,
        );
    }
}
impl std::ops::Sub<AntiFlatPoint> for AntiFlatPoint {
    type Output = AntiFlatPoint;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        1        0        0
    // no simd        4        0        0
    fn sub(self, other: AntiFlatPoint) -> Self::Output {
        return AntiFlatPoint::from_groups(/* e235, e315, e125, e321 */ self.group0() - other.group0());
    }
}
impl std::ops::SubAssign<AntiFlatPoint> for AntiFlatPoint {
    fn sub_assign(&mut self, other: AntiFlatPoint) {
        *self = AntiFlatPoint::from_groups(/* e235, e315, e125, e321 */ self.group0() - other.group0());
    }
}
impl std::ops::Sub<AntiFlector> for AntiFlatPoint {
    type Output = AntiFlector;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        1        1        0
    // no simd        4        4        0
    fn sub(self, other: AntiFlector) -> Self::Output {
        return AntiFlector::from_groups(
            // e235, e315, e125, e321
            self.group0() - other.group0(),
            // e1, e2, e3, e5
            other.group1() * Simd32x4::from(-1.0),
        );
    }
}
impl std::ops::Sub<AntiLine> for AntiFlatPoint {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        2        0
    // no simd        0        6        0
    fn sub(self, other: AntiLine) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e15, e25, e35, e45
            (other.group1() * Simd32x3::from(-1.0)).extend_to_4(0.0),
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12
            other.group0() * Simd32x3::from(-1.0),
            // e415, e425, e435, e321
            Simd32x3::from(0.0).extend_to_4(self[e321]),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            self.group0().truncate_to_3(),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(0.0),
            // e1234
            0.0,
        );
    }
}
impl std::ops::Sub<AntiMotor> for AntiFlatPoint {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd2        0        1        0
    //    simd3        0        2        0
    // Totals...
    // yes simd        0        4        0
    //  no simd        0        9        0
    fn sub(self, other: AntiMotor) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([other[scalar], 1.0]) * Simd32x2::from([-1.0, 0.0]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e15, e25, e35, e45
            (other.group1().truncate_to_3() * Simd32x3::from(-1.0)).extend_to_4(0.0),
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12
            other.group0().truncate_to_3() * Simd32x3::from(-1.0),
            // e415, e425, e435, e321
            Simd32x3::from(0.0).extend_to_4(self[e321]),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            self.group0().truncate_to_3(),
            // e4235, e4315, e4125, e3215
            Simd32x3::from(0.0).extend_to_4(other[e3215] * -1.0),
            // e1234
            0.0,
        );
    }
}
impl std::ops::Sub<AntiPlane> for AntiFlatPoint {
    type Output = AntiFlector;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn sub(self, other: AntiPlane) -> Self::Output {
        return AntiFlector::from_groups(/* e235, e315, e125, e321 */ self.group0(), /* e1, e2, e3, e5 */ other.group0() * Simd32x4::from(-1.0));
    }
}
impl std::ops::Sub<AntiScalar> for AntiFlatPoint {
    type Output = CircleRotor;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn sub(self, other: AntiScalar) -> Self::Output {
        use crate::elements::*;
        return CircleRotor::from_groups(
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x3::from(0.0).extend_to_4(self[e321]),
            // e235, e315, e125, e12345
            self.group0().truncate_to_3().extend_to_4(other[e12345]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
        );
    }
}
impl std::ops::Sub<Circle> for AntiFlatPoint {
    type Output = Circle;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        0        0
    //    simd3        1        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        2        2        0
    //  no simd        4        7        0
    fn sub(self, other: Circle) -> Self::Output {
        use crate::elements::*;
        return Circle::from_groups(
            // e423, e431, e412
            other.group0() * Simd32x3::from(-1.0),
            // e415, e425, e435, e321
            other.group1().truncate_to_3().extend_to_4(self[e321] - other[e321]) * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e235, e315, e125
            self.group0().truncate_to_3() - other.group2(),
        );
    }
}
impl std::ops::Sub<CircleRotor> for AntiFlatPoint {
    type Output = CircleRotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        0        0
    //    simd3        1        1        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        2        3        0
    //  no simd        4       11        0
    fn sub(self, other: CircleRotor) -> Self::Output {
        use crate::elements::*;
        return CircleRotor::from_groups(
            // e423, e431, e412
            other.group0() * Simd32x3::from(-1.0),
            // e415, e425, e435, e321
            other.group1().truncate_to_3().extend_to_4(self[e321] - other[e321]) * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e235, e315, e125, e12345
            (self.group0().truncate_to_3() - other.group2().truncate_to_3()).extend_to_4(other[e12345]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
        );
    }
}
impl std::ops::Sub<Dipole> for AntiFlatPoint {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        0        2        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        3        0
    //  no simd        0       10        0
    fn sub(self, other: Dipole) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e15, e25, e35, e45
            other.group2().extend_to_4(other[e45]) * Simd32x4::from(-1.0),
            // e41, e42, e43
            other.group0() * Simd32x3::from(-1.0),
            // e23, e31, e12
            other.group1().truncate_to_3() * Simd32x3::from(-1.0),
            // e415, e425, e435, e321
            Simd32x3::from(0.0).extend_to_4(self[e321]),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            self.group0().truncate_to_3(),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(0.0),
            // e1234
            0.0,
        );
    }
}
impl std::ops::Sub<DipoleInversion> for AntiFlatPoint {
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
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e15, e25, e35, e45
            other.group2().truncate_to_3().extend_to_4(other[e45]) * Simd32x4::from(-1.0),
            // e41, e42, e43
            other.group0() * Simd32x3::from(-1.0),
            // e23, e31, e12
            other.group1().truncate_to_3() * Simd32x3::from(-1.0),
            // e415, e425, e435, e321
            Simd32x3::from(0.0).extend_to_4(self[e321]),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            self.group0().truncate_to_3(),
            // e4235, e4315, e4125, e3215
            other.group3() * Simd32x4::from(-1.0),
            // e1234
            other[e1234] * -1.0,
        );
    }
}
impl std::ops::Sub<DualNum> for AntiFlatPoint {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        2        0
    //  no simd        0        5        0
    fn sub(self, other: DualNum) -> Self::Output {
        use crate::elements::*;
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x3::from(0.0).extend_to_4(other[e12345] * -1.0),
            // e415, e425, e435, e321
            Simd32x3::from(0.0).extend_to_4(self[e321]),
            // e235, e315, e125, e5
            self.group0().truncate_to_3().extend_to_4(other[e5]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
        );
    }
}
impl std::ops::Sub<FlatPoint> for AntiFlatPoint {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn sub(self, other: FlatPoint) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e15, e25, e35, e45
            other.group0() * Simd32x4::from(-1.0),
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x3::from(0.0).extend_to_4(self[e321]),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            self.group0().truncate_to_3(),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(0.0),
            // e1234
            0.0,
        );
    }
}
impl std::ops::Sub<Flector> for AntiFlatPoint {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        2        0
    // no simd        0        8        0
    fn sub(self, other: Flector) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e15, e25, e35, e45
            other.group0() * Simd32x4::from(-1.0),
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x3::from(0.0).extend_to_4(self[e321]),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            self.group0().truncate_to_3(),
            // e4235, e4315, e4125, e3215
            other.group1() * Simd32x4::from(-1.0),
            // e1234
            0.0,
        );
    }
}
impl std::ops::Sub<Line> for AntiFlatPoint {
    type Output = Circle;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        1        0        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        1        1        0
    //  no simd        3        4        0
    fn sub(self, other: Line) -> Self::Output {
        use crate::elements::*;
        return Circle::from_groups(
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            other.group0().extend_to_4(self[e321]) * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e235, e315, e125
            self.group0().truncate_to_3() - other.group1(),
        );
    }
}
impl std::ops::Sub<Motor> for AntiFlatPoint {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd3        1        0        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        1        3        0
    //  no simd        3        9        0
    fn sub(self, other: Motor) -> Self::Output {
        use crate::elements::*;
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x3::from(0.0).extend_to_4(other[e12345] * -1.0),
            // e415, e425, e435, e321
            other.group0().truncate_to_3().extend_to_4(self[e321]) * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e235, e315, e125, e5
            (self.group0().truncate_to_3() - other.group1().truncate_to_3()).extend_to_4(other[e5]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
        );
    }
}
impl std::ops::Sub<MultiVector> for AntiFlatPoint {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        2        0
    //    simd2        0        1        0
    //    simd3        1        3        0
    //    simd4        0        4        0
    // Totals...
    // yes simd        2       10        0
    //  no simd        4       29        0
    fn sub(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            other.group0() * Simd32x2::from(-1.0),
            // e1, e2, e3, e4
            other.group1() * Simd32x4::from(-1.0),
            // e5
            other[e5] * -1.0,
            // e15, e25, e35, e45
            other.group3() * Simd32x4::from(-1.0),
            // e41, e42, e43
            other.group4() * Simd32x3::from(-1.0),
            // e23, e31, e12
            other.group5() * Simd32x3::from(-1.0),
            // e415, e425, e435, e321
            other.group6().truncate_to_3().extend_to_4(self[e321] - other[e321]) * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e423, e431, e412
            other.group7() * Simd32x3::from(-1.0),
            // e235, e315, e125
            self.group0().truncate_to_3() - other.group8(),
            // e4235, e4315, e4125, e3215
            other.group9() * Simd32x4::from(-1.0),
            // e1234
            other[e1234] * -1.0,
        );
    }
}
impl std::ops::Sub<Plane> for AntiFlatPoint {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn sub(self, other: Plane) -> Self::Output {
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
            Simd32x3::from(0.0).extend_to_4(self[e321]),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            self.group0().truncate_to_3(),
            // e4235, e4315, e4125, e3215
            other.group0() * Simd32x4::from(-1.0),
            // e1234
            0.0,
        );
    }
}
impl std::ops::Sub<RoundPoint> for AntiFlatPoint {
    type Output = AntiDipoleInversion;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        2        0
    // no simd        0        8        0
    fn sub(self, other: RoundPoint) -> Self::Output {
        use crate::elements::*;
        return AntiDipoleInversion::from_groups(
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x3::from(0.0).extend_to_4(self[e321]),
            // e235, e315, e125, e4
            self.group0().truncate_to_3().extend_to_4(other[e4]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e1, e2, e3, e5
            other.group0().truncate_to_3().extend_to_4(other[e5]) * Simd32x4::from(-1.0),
        );
    }
}
impl std::ops::Sub<Scalar> for AntiFlatPoint {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd2        0        1        0
    // no simd        0        2        0
    fn sub(self, other: Scalar) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([other[scalar], 1.0]) * Simd32x2::from([-1.0, 0.0]),
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
            Simd32x3::from(0.0).extend_to_4(self[e321]),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            self.group0().truncate_to_3(),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(0.0),
            // e1234
            0.0,
        );
    }
}
impl std::ops::Sub<Sphere> for AntiFlatPoint {
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
            Simd32x3::from(0.0).extend_to_4(self[e321]),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            self.group0().truncate_to_3(),
            // e4235, e4315, e4125, e3215
            other.group0() * Simd32x4::from(-1.0),
            // e1234
            other[e1234] * -1.0,
        );
    }
}
impl std::ops::Sub<VersorEven> for AntiFlatPoint {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        0        0
    //    simd3        1        0        0
    //    simd4        0        4        0
    // Totals...
    // yes simd        2        4        0
    //  no simd        4       16        0
    fn sub(self, other: VersorEven) -> Self::Output {
        use crate::elements::*;
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            other.group0() * Simd32x4::from(-1.0),
            // e415, e425, e435, e321
            other.group1().truncate_to_3().extend_to_4(self[e321] - other[e321]) * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e235, e315, e125, e5
            (self.group0().truncate_to_3() - other.group2().truncate_to_3()).extend_to_4(other[e5]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e1, e2, e3, e4
            other.group3() * Simd32x4::from(-1.0),
        );
    }
}
impl std::ops::Sub<VersorOdd> for AntiFlatPoint {
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
    fn sub(self, other: VersorOdd) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([other[scalar], 1.0]) * Simd32x2::from([-1.0, 0.0]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e15, e25, e35, e45
            other.group2().truncate_to_3().extend_to_4(other[e45]) * Simd32x4::from(-1.0),
            // e41, e42, e43
            other.group0().truncate_to_3() * Simd32x3::from(-1.0),
            // e23, e31, e12
            other.group1().truncate_to_3() * Simd32x3::from(-1.0),
            // e415, e425, e435, e321
            Simd32x3::from(0.0).extend_to_4(self[e321]),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            self.group0().truncate_to_3(),
            // e4235, e4315, e4125, e3215
            other.group3() * Simd32x4::from(-1.0),
            // e1234
            other[e1234] * -1.0,
        );
    }
}

impl TryFrom<AntiDipoleInversion> for AntiFlatPoint {
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
        let el = anti_dipole_inversion[3];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e415: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = anti_dipole_inversion[4];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e425: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = anti_dipole_inversion[5];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e435: ");
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
            let mut error = "Elements from AntiDipoleInversion do not fit into AntiFlatPoint { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(AntiFlatPoint::from_groups(/* e235, e315, e125, e321 */ Simd32x4::from([
            anti_dipole_inversion[e235],
            anti_dipole_inversion[e315],
            anti_dipole_inversion[e125],
            anti_dipole_inversion[e321],
        ])));
    }
}

impl TryFrom<AntiFlector> for AntiFlatPoint {
    type Error = String;
    fn try_from(anti_flector: AntiFlector) -> Result<Self, Self::Error> {
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
            let mut error = "Elements from AntiFlector do not fit into AntiFlatPoint { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(AntiFlatPoint::from_groups(/* e235, e315, e125, e321 */ anti_flector.group0()));
    }
}

impl TryFrom<Circle> for AntiFlatPoint {
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
        let el = circle[3];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e415: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = circle[4];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e425: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = circle[5];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e435: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from Circle do not fit into AntiFlatPoint { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(AntiFlatPoint::from_groups(
            // e235, e315, e125, e321
            Simd32x4::from([circle[e235], circle[e315], circle[e125], circle[e321]]),
        ));
    }
}

impl TryFrom<CircleRotor> for AntiFlatPoint {
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
        let el = circle_rotor[3];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e415: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = circle_rotor[4];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e425: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = circle_rotor[5];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e435: ");
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
            let mut error = "Elements from CircleRotor do not fit into AntiFlatPoint { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(AntiFlatPoint::from_groups(/* e235, e315, e125, e321 */ Simd32x4::from([
            circle_rotor[e235], circle_rotor[e315], circle_rotor[e125], circle_rotor[e321],
        ])));
    }
}

impl TryFrom<Line> for AntiFlatPoint {
    type Error = String;
    fn try_from(line: Line) -> Result<Self, Self::Error> {
        use crate::elements::*;
        let mut error_string = String::new();
        let mut fail = false;
        let el = line[0];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e415: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = line[1];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e425: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = line[2];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e435: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from Line do not fit into AntiFlatPoint { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(AntiFlatPoint::from_groups(/* e235, e315, e125, e321 */ Simd32x4::from([line[e235], line[e315], line[e125], 0.0])));
    }
}

impl TryFrom<Motor> for AntiFlatPoint {
    type Error = String;
    fn try_from(motor: Motor) -> Result<Self, Self::Error> {
        use crate::elements::*;
        let mut error_string = String::new();
        let mut fail = false;
        let el = motor[0];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e415: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = motor[1];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e425: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = motor[2];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e435: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = motor[3];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e12345: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = motor[7];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e5: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from Motor do not fit into AntiFlatPoint { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(AntiFlatPoint::from_groups(
            // e235, e315, e125, e321
            Simd32x4::from([motor[e235], motor[e315], motor[e125], 0.0]),
        ));
    }
}

impl TryFrom<MultiVector> for AntiFlatPoint {
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
            let mut error = "Elements from MultiVector do not fit into AntiFlatPoint { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(AntiFlatPoint::from_groups(/* e235, e315, e125, e321 */ Simd32x4::from([
            multi_vector[e235], multi_vector[e315], multi_vector[e125], multi_vector[e321],
        ])));
    }
}

impl TryFrom<VersorEven> for AntiFlatPoint {
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
        let el = versor_even[4];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e415: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = versor_even[5];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e425: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = versor_even[6];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e435: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
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
            let mut error = "Elements from VersorEven do not fit into AntiFlatPoint { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(AntiFlatPoint::from_groups(
            // e235, e315, e125, e321
            Simd32x4::from([versor_even[e235], versor_even[e315], versor_even[e125], versor_even[e321]]),
        ));
    }
}
