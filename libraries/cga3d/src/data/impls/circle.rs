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
// Total Implementations: 103
//
// Yes SIMD:   add/sub     mul     div
//  Minimum:         0       0       0
//   Median:         3       2       0
//  Average:        17      21       0
//  Maximum:       227     259       0
//
//  No SIMD:   add/sub     mul     div
//  Minimum:         0       0       0
//   Median:         4       5       0
//  Average:        19      24       0
//  Maximum:       288     320       0
impl std::ops::Add<AntiCircleRotor> for Circle {
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
            self.group1(),
            // e423, e431, e412
            self.group0(),
            // e235, e315, e125
            self.group2(),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(0.0),
            // e1234
            0.0,
        );
    }
}
impl std::ops::Add<AntiDipoleInversion> for Circle {
    type Output = AntiDipoleInversion;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       10        0        0
    fn add(self, other: AntiDipoleInversion) -> Self::Output {
        use crate::elements::*;
        return AntiDipoleInversion::from_groups(
            // e423, e431, e412
            Simd32x3::from([other[e423] + self[e423], other[e431] + self[e431], other[e412] + self[e412]]),
            // e415, e425, e435, e321
            Simd32x4::from([other[e415] + self[e415], other[e425] + self[e425], other[e435] + self[e435], other[e321] + self[e321]]),
            // e235, e315, e125, e4
            Simd32x4::from([other[e235] + self[e235], other[e315] + self[e315], other[e125] + self[e125], other[e4]]),
            // e1, e2, e3, e5
            other.group3(),
        );
    }
}
impl std::ops::Add<AntiDualNum> for Circle {
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
            self.group1(),
            // e423, e431, e412
            self.group0(),
            // e235, e315, e125
            self.group2(),
            // e4235, e4315, e4125, e3215
            Simd32x3::from(0.0).extend_to_4(other[e3215]),
            // e1234
            0.0,
        );
    }
}
impl std::ops::Add<AntiFlatPoint> for Circle {
    type Output = Circle;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        4        0        0
    fn add(self, other: AntiFlatPoint) -> Self::Output {
        use crate::elements::*;
        return Circle::from_groups(
            // e423, e431, e412
            self.group0(),
            // e415, e425, e435, e321
            Simd32x4::from([self[e415], self[e425], self[e435], other[e321] + self[e321]]),
            // e235, e315, e125
            Simd32x3::from([other[e235] + self[e235], other[e315] + self[e315], other[e125] + self[e125]]),
        );
    }
}
impl std::ops::AddAssign<AntiFlatPoint> for Circle {
    fn add_assign(&mut self, other: AntiFlatPoint) {
        use crate::elements::*;
        *self = Circle::from_groups(
            // e423, e431, e412
            self.group0(),
            // e415, e425, e435, e321
            Simd32x4::from([self[e415], self[e425], self[e435], other[e321] + self[e321]]),
            // e235, e315, e125
            Simd32x3::from([other[e235] + self[e235], other[e315] + self[e315], other[e125] + self[e125]]),
        );
    }
}
impl std::ops::Add<AntiFlector> for Circle {
    type Output = AntiDipoleInversion;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        4        0        0
    fn add(self, other: AntiFlector) -> Self::Output {
        use crate::elements::*;
        return AntiDipoleInversion::from_groups(
            // e423, e431, e412
            self.group0(),
            // e415, e425, e435, e321
            Simd32x4::from([self[e415], self[e425], self[e435], other[e321] + self[e321]]),
            // e235, e315, e125, e4
            Simd32x4::from([other[e235] + self[e235], other[e315] + self[e315], other[e125] + self[e125], 0.0]),
            // e1, e2, e3, e5
            other.group1(),
        );
    }
}
impl std::ops::Add<AntiLine> for Circle {
    type Output = MultiVector;
    fn add(self, other: AntiLine) -> Self::Output {
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e15, e25, e35, e45
            crate::swizzle!(other.group1(), 0, 1, 2).extend_to_4(0.0),
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12
            other.group0(),
            // e415, e425, e435, e321
            self.group1(),
            // e423, e431, e412
            self.group0(),
            // e235, e315, e125
            self.group2(),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(0.0),
            // e1234
            0.0,
        );
    }
}
impl std::ops::Add<AntiMotor> for Circle {
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
            self.group1(),
            // e423, e431, e412
            self.group0(),
            // e235, e315, e125
            self.group2(),
            // e4235, e4315, e4125, e3215
            Simd32x3::from(0.0).extend_to_4(other[e3215]),
            // e1234
            0.0,
        );
    }
}
impl std::ops::Add<AntiPlane> for Circle {
    type Output = AntiDipoleInversion;
    fn add(self, other: AntiPlane) -> Self::Output {
        return AntiDipoleInversion::from_groups(
            // e423, e431, e412
            self.group0(),
            // e415, e425, e435, e321
            self.group1(),
            // e235, e315, e125, e4
            crate::swizzle!(self.group2(), 0, 1, 2).extend_to_4(0.0),
            // e1, e2, e3, e5
            other.group0(),
        );
    }
}
impl std::ops::Add<AntiScalar> for Circle {
    type Output = CircleRotor;
    fn add(self, other: AntiScalar) -> Self::Output {
        use crate::elements::*;
        return CircleRotor::from_groups(
            // e423, e431, e412
            self.group0(),
            // e415, e425, e435, e321
            self.group1(),
            // e235, e315, e125, e12345
            crate::swizzle!(self.group2(), 0, 1, 2).extend_to_4(other[e12345]),
        );
    }
}
impl std::ops::Add<Circle> for Circle {
    type Output = Circle;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       10        0        0
    fn add(self, other: Circle) -> Self::Output {
        use crate::elements::*;
        return Circle::from_groups(
            // e423, e431, e412
            Simd32x3::from([other[e423] + self[e423], other[e431] + self[e431], other[e412] + self[e412]]),
            // e415, e425, e435, e321
            Simd32x4::from([other[e415] + self[e415], other[e425] + self[e425], other[e435] + self[e435], other[e321] + self[e321]]),
            // e235, e315, e125
            Simd32x3::from([other[e235] + self[e235], other[e315] + self[e315], other[e125] + self[e125]]),
        );
    }
}
impl std::ops::AddAssign<Circle> for Circle {
    fn add_assign(&mut self, other: Circle) {
        use crate::elements::*;
        *self = Circle::from_groups(
            // e423, e431, e412
            Simd32x3::from([other[e423] + self[e423], other[e431] + self[e431], other[e412] + self[e412]]),
            // e415, e425, e435, e321
            Simd32x4::from([other[e415] + self[e415], other[e425] + self[e425], other[e435] + self[e435], other[e321] + self[e321]]),
            // e235, e315, e125
            Simd32x3::from([other[e235] + self[e235], other[e315] + self[e315], other[e125] + self[e125]]),
        );
    }
}
impl std::ops::Add<CircleRotor> for Circle {
    type Output = CircleRotor;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       10        0        0
    fn add(self, other: CircleRotor) -> Self::Output {
        use crate::elements::*;
        return CircleRotor::from_groups(
            // e423, e431, e412
            Simd32x3::from([self[e423] + other[e423], self[e431] + other[e431], self[e412] + other[e412]]),
            // e415, e425, e435, e321
            Simd32x4::from([self[e415] + other[e415], self[e425] + other[e425], self[e435] + other[e435], self[e321] + other[e321]]),
            // e235, e315, e125, e12345
            Simd32x4::from([self[e235] + other[e235], self[e315] + other[e315], self[e125] + other[e125], other[e12345]]),
        );
    }
}
impl std::ops::Add<Dipole> for Circle {
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
            crate::swizzle!(other.group2(), 0, 1, 2).extend_to_4(other[e45]),
            // e41, e42, e43
            other.group0(),
            // e23, e31, e12
            other.group1().truncate_to_3(),
            // e415, e425, e435, e321
            self.group1(),
            // e423, e431, e412
            self.group0(),
            // e235, e315, e125
            self.group2(),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(0.0),
            // e1234
            0.0,
        );
    }
}
impl std::ops::Add<DipoleInversion> for Circle {
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
            self.group1(),
            // e423, e431, e412
            self.group0(),
            // e235, e315, e125
            self.group2(),
            // e4235, e4315, e4125, e3215
            other.group3(),
            // e1234
            other[e1234],
        );
    }
}
impl std::ops::Add<DualNum> for Circle {
    type Output = VersorEven;
    fn add(self, other: DualNum) -> Self::Output {
        use crate::elements::*;
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            crate::swizzle!(self.group0(), 0, 1, 2).extend_to_4(other[e12345]),
            // e415, e425, e435, e321
            self.group1(),
            // e235, e315, e125, e5
            crate::swizzle!(self.group2(), 0, 1, 2).extend_to_4(other[e5]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
        );
    }
}
impl std::ops::Add<FlatPoint> for Circle {
    type Output = MultiVector;
    fn add(self, other: FlatPoint) -> Self::Output {
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
            self.group1(),
            // e423, e431, e412
            self.group0(),
            // e235, e315, e125
            self.group2(),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(0.0),
            // e1234
            0.0,
        );
    }
}
impl std::ops::Add<Flector> for Circle {
    type Output = MultiVector;
    fn add(self, other: Flector) -> Self::Output {
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
            self.group1(),
            // e423, e431, e412
            self.group0(),
            // e235, e315, e125
            self.group2(),
            // e4235, e4315, e4125, e3215
            other.group1(),
            // e1234
            0.0,
        );
    }
}
impl std::ops::Add<Line> for Circle {
    type Output = Circle;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        6        0        0
    fn add(self, other: Line) -> Self::Output {
        use crate::elements::*;
        return Circle::from_groups(
            // e423, e431, e412
            self.group0(),
            // e415, e425, e435, e321
            Simd32x4::from([self[e415] + other[e415], self[e425] + other[e425], self[e435] + other[e435], self[e321]]),
            // e235, e315, e125
            Simd32x3::from([self[e235] + other[e235], self[e315] + other[e315], self[e125] + other[e125]]),
        );
    }
}
impl std::ops::AddAssign<Line> for Circle {
    fn add_assign(&mut self, other: Line) {
        use crate::elements::*;
        *self = Circle::from_groups(
            // e423, e431, e412
            self.group0(),
            // e415, e425, e435, e321
            Simd32x4::from([self[e415] + other[e415], self[e425] + other[e425], self[e435] + other[e435], self[e321]]),
            // e235, e315, e125
            Simd32x3::from([self[e235] + other[e235], self[e315] + other[e315], self[e125] + other[e125]]),
        );
    }
}
impl std::ops::Add<Motor> for Circle {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        6        0        0
    fn add(self, other: Motor) -> Self::Output {
        use crate::elements::*;
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            crate::swizzle!(self.group0(), 0, 1, 2).extend_to_4(other[e12345]),
            // e415, e425, e435, e321
            Simd32x4::from([self[e415] + other[e415], self[e425] + other[e425], self[e435] + other[e435], self[e321]]),
            // e235, e315, e125, e5
            Simd32x4::from([self[e235] + other[e235], self[e315] + other[e315], self[e125] + other[e125], other[e5]]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
        );
    }
}
impl std::ops::Add<MultiVector> for Circle {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       10        0        0
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
            Simd32x4::from([self[e415] + other[e415], self[e425] + other[e425], self[e435] + other[e435], self[e321] + other[e321]]),
            // e423, e431, e412
            Simd32x3::from([self[e423] + other[e423], self[e431] + other[e431], self[e412] + other[e412]]),
            // e235, e315, e125
            Simd32x3::from([self[e235] + other[e235], self[e315] + other[e315], self[e125] + other[e125]]),
            // e4235, e4315, e4125, e3215
            other.group9(),
            // e1234
            other[e1234],
        );
    }
}
impl std::ops::Add<Plane> for Circle {
    type Output = MultiVector;
    fn add(self, other: Plane) -> Self::Output {
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
            self.group1(),
            // e423, e431, e412
            self.group0(),
            // e235, e315, e125
            self.group2(),
            // e4235, e4315, e4125, e3215
            other.group0(),
            // e1234
            0.0,
        );
    }
}
impl std::ops::Add<RoundPoint> for Circle {
    type Output = AntiDipoleInversion;
    fn add(self, other: RoundPoint) -> Self::Output {
        use crate::elements::*;
        return AntiDipoleInversion::from_groups(
            // e423, e431, e412
            self.group0(),
            // e415, e425, e435, e321
            self.group1(),
            // e235, e315, e125, e4
            crate::swizzle!(self.group2(), 0, 1, 2).extend_to_4(other[e4]),
            // e1, e2, e3, e5
            Simd32x4::from([other[e1], other[e2], other[e3], other[e5]]),
        );
    }
}
impl std::ops::Add<Scalar> for Circle {
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
            self.group1(),
            // e423, e431, e412
            self.group0(),
            // e235, e315, e125
            self.group2(),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(0.0),
            // e1234
            0.0,
        );
    }
}
impl std::ops::Add<Sphere> for Circle {
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
            self.group1(),
            // e423, e431, e412
            self.group0(),
            // e235, e315, e125
            self.group2(),
            // e4235, e4315, e4125, e3215
            other.group0(),
            // e1234
            other[e1234],
        );
    }
}
impl std::ops::Add<VersorEven> for Circle {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       10        0        0
    fn add(self, other: VersorEven) -> Self::Output {
        use crate::elements::*;
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([self[e423] + other[e423], self[e431] + other[e431], self[e412] + other[e412], other[e12345]]),
            // e415, e425, e435, e321
            Simd32x4::from([self[e415] + other[e415], self[e425] + other[e425], self[e435] + other[e435], self[e321] + other[e321]]),
            // e235, e315, e125, e5
            Simd32x4::from([self[e235] + other[e235], self[e315] + other[e315], self[e125] + other[e125], other[e5]]),
            // e1, e2, e3, e4
            other.group3(),
        );
    }
}
impl std::ops::Add<VersorOdd> for Circle {
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
            self.group1(),
            // e423, e431, e412
            self.group0(),
            // e235, e315, e125
            self.group2(),
            // e4235, e4315, e4125, e3215
            other.group3(),
            // e1234
            other[e1234],
        );
    }
}
impl std::ops::BitXor<AntiCircleRotor> for Circle {
    type Output = CircleRotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        9       13        0
    //    simd3        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        9       15        0
    //  no simd        9       20        0
    fn bitxor(self, other: AntiCircleRotor) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<AntiDipoleInversion> for Circle {
    type Output = Sphere;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       11       16        0
    //    simd4        1        1        0
    // Totals...
    // yes simd       12       17        0
    //  no simd       15       20        0
    fn bitxor(self, other: AntiDipoleInversion) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<AntiDualNum> for Circle {
    type Output = Circle;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        0        2        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        3        0
    //  no simd        0       10        0
    fn bitxor(self, other: AntiDualNum) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXorAssign<AntiDualNum> for Circle {
    fn bitxor_assign(&mut self, other: AntiDualNum) {
        *self = self.wedge(other);
    }
}
impl std::ops::BitXor<AntiFlector> for Circle {
    type Output = Sphere;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        7       12        0
    //    simd4        1        1        0
    // Totals...
    // yes simd        8       13        0
    //  no simd       11       16        0
    fn bitxor(self, other: AntiFlector) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<AntiLine> for Circle {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        5        6        0
    fn bitxor(self, other: AntiLine) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<AntiMotor> for Circle {
    type Output = CircleRotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        5        9        0
    //    simd3        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        5       11        0
    //  no simd        5       16        0
    fn bitxor(self, other: AntiMotor) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<AntiPlane> for Circle {
    type Output = Sphere;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        7       12        0
    //    simd4        1        1        0
    // Totals...
    // yes simd        8       13        0
    //  no simd       11       16        0
    fn bitxor(self, other: AntiPlane) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<Dipole> for Circle {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        9       10        0
    fn bitxor(self, other: Dipole) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<DipoleInversion> for Circle {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        9       10        0
    fn bitxor(self, other: DipoleInversion) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<DualNum> for Circle {
    type Output = Plane;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn bitxor(self, other: DualNum) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<FlatPoint> for Circle {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        4        0
    fn bitxor(self, other: FlatPoint) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<Flector> for Circle {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        4        0
    fn bitxor(self, other: Flector) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<Motor> for Circle {
    type Output = Plane;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn bitxor(self, other: Motor) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<MultiVector> for Circle {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       20       26        0
    //    simd3        0        2        0
    //    simd4        1        2        0
    // Totals...
    // yes simd       21       30        0
    //  no simd       24       40        0
    fn bitxor(self, other: MultiVector) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<RoundPoint> for Circle {
    type Output = Sphere;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       11       16        0
    //    simd4        1        1        0
    // Totals...
    // yes simd       12       17        0
    //  no simd       15       20        0
    fn bitxor(self, other: RoundPoint) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<Scalar> for Circle {
    type Output = Circle;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        0        2        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        3        0
    //  no simd        0       10        0
    fn bitxor(self, other: Scalar) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXorAssign<Scalar> for Circle {
    fn bitxor_assign(&mut self, other: Scalar) {
        *self = self.wedge(other);
    }
}
impl std::ops::BitXor<VersorEven> for Circle {
    type Output = Sphere;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       11       16        0
    //    simd4        1        1        0
    // Totals...
    // yes simd       12       17        0
    //  no simd       15       20        0
    fn bitxor(self, other: VersorEven) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<VersorOdd> for Circle {
    type Output = CircleRotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        9       13        0
    //    simd3        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        9       15        0
    //  no simd        9       20        0
    fn bitxor(self, other: VersorOdd) -> Self::Output {
        return self.wedge(other);
    }
}

impl From<AntiFlatPoint> for Circle {
    fn from(from_anti_flat_point: AntiFlatPoint) -> Self {
        use crate::elements::*;
        return Circle::from_groups(
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x3::from(0.0).extend_to_4(from_anti_flat_point[e321]),
            // e235, e315, e125
            from_anti_flat_point.group0().truncate_to_3(),
        );
    }
}

impl From<Line> for Circle {
    fn from(from_line: Line) -> Self {
        use crate::elements::*;
        return Circle::from_groups(
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from([from_line[e415], from_line[e425], from_line[e435], 0.0]),
            // e235, e315, e125
            from_line.group1(),
        );
    }
}
impl std::ops::Mul<AntiCircleRotor> for Circle {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       90      106        0
    //    simd4        1        1        0
    // Totals...
    // yes simd       91      107        0
    //  no simd       94      110        0
    fn mul(self, other: AntiCircleRotor) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiDipoleInversion> for Circle {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32      114      130        0
    //    simd4        5        5        0
    // Totals...
    // yes simd      119      135        0
    //  no simd      134      150        0
    fn mul(self, other: AntiDipoleInversion) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiDualNum> for Circle {
    type Output = AntiDipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        4        0
    //    simd3        1        4        0
    //    simd4        1        2        0
    // Totals...
    // yes simd        2       10        0
    //  no simd        7       24        0
    fn mul(self, other: AntiDualNum) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiFlatPoint> for Circle {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       19       34        0
    //    simd3        2        2        0
    // Totals...
    // yes simd       21       36        0
    //  no simd       25       40        0
    fn mul(self, other: AntiFlatPoint) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiFlector> for Circle {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       60       76        0
    //    simd4        1        1        0
    // Totals...
    // yes simd       61       77        0
    //  no simd       64       80        0
    fn mul(self, other: AntiFlector) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiLine> for Circle {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       44       60        0
    fn mul(self, other: AntiLine) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiMotor> for Circle {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       56       72        0
    //    simd4        2        2        0
    // Totals...
    // yes simd       58       74        0
    //  no simd       64       80        0
    fn mul(self, other: AntiMotor) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiPlane> for Circle {
    type Output = DipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       21       36        0
    //    simd4        1        1        0
    // Totals...
    // yes simd       22       37        0
    //  no simd       25       40        0
    fn mul(self, other: AntiPlane) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiScalar> for Circle {
    type Output = Dipole;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        0        4        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        0        6        0
    //  no simd        0       20        0
    fn mul(self, other: AntiScalar) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<Circle> for Circle {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       80       96        0
    //    simd4        1        1        0
    // Totals...
    // yes simd       81       97        0
    //  no simd       84      100        0
    fn mul(self, other: Circle) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<CircleRotor> for Circle {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       90      106        0
    //    simd4        1        1        0
    // Totals...
    // yes simd       91      107        0
    //  no simd       94      110        0
    fn mul(self, other: CircleRotor) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<Dipole> for Circle {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       84      100        0
    fn mul(self, other: Dipole) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<DipoleInversion> for Circle {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32      114      130        0
    //    simd4        5        5        0
    // Totals...
    // yes simd      119      135        0
    //  no simd      134      150        0
    fn mul(self, other: DipoleInversion) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<DualNum> for Circle {
    type Output = DipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd3        2        6        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        2        8        0
    //  no simd        6       23        0
    fn mul(self, other: DualNum) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<FlatPoint> for Circle {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       22       37        0
    //    simd3        1        1        0
    // Totals...
    // yes simd       23       38        0
    //  no simd       25       40        0
    fn mul(self, other: FlatPoint) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<Flector> for Circle {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       60       76        0
    //    simd4        1        1        0
    // Totals...
    // yes simd       61       77        0
    //  no simd       64       80        0
    fn mul(self, other: Flector) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<Line> for Circle {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       44       60        0
    fn mul(self, other: Line) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<Motor> for Circle {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       60       76        0
    //    simd4        1        1        0
    // Totals...
    // yes simd       61       77        0
    //  no simd       64       80        0
    fn mul(self, other: Motor) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<MultiVector> for Circle {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32      198      230        0
    //    simd2        1        1        0
    //    simd3       24       24        0
    //    simd4        4        4        0
    // Totals...
    // yes simd      227      259        0
    //  no simd      288      320        0
    fn mul(self, other: MultiVector) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<Plane> for Circle {
    type Output = AntiDipoleInversion;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       25       40        0
    fn mul(self, other: Plane) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<RoundPoint> for Circle {
    type Output = DipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       28       43        0
    //    simd3        1        1        0
    //    simd4        1        1        0
    // Totals...
    // yes simd       30       45        0
    //  no simd       35       50        0
    fn mul(self, other: RoundPoint) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<Scalar> for Circle {
    type Output = Circle;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        0        2        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        3        0
    //  no simd        0       10        0
    fn mul(self, other: Scalar) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::MulAssign<Scalar> for Circle {
    fn mul_assign(&mut self, other: Scalar) {
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<Sphere> for Circle {
    type Output = AntiDipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       32       47        0
    //    simd3        1        1        0
    // Totals...
    // yes simd       33       48        0
    //  no simd       35       50        0
    fn mul(self, other: Sphere) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<VersorEven> for Circle {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32      124      140        0
    //    simd4        5        5        0
    // Totals...
    // yes simd      129      145        0
    //  no simd      144      160        0
    fn mul(self, other: VersorEven) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<VersorOdd> for Circle {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32      108      124        0
    //    simd4        9        9        0
    // Totals...
    // yes simd      117      133        0
    //  no simd      144      160        0
    fn mul(self, other: VersorOdd) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Neg for Circle {
    type Output = Circle;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        0        2        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        3        0
    //  no simd        0       10        0
    fn neg(self) -> Self::Output {
        return Circle::from_groups(
            // e423, e431, e412
            self.group0() * Simd32x3::from(-1.0),
            // e415, e425, e435, e321
            self.group1() * Simd32x4::from(-1.0),
            // e235, e315, e125
            self.group2() * Simd32x3::from(-1.0),
        );
    }
}
impl std::ops::Not for Circle {
    type Output = Dipole;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn not(self) -> Self::Output {
        return self.right_dual();
    }
}
impl std::ops::Sub<AntiCircleRotor> for Circle {
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
            Simd32x4::from([other[e15], other[e25], other[e35], other[e45]]) * Simd32x4::from(-1.0),
            // e41, e42, e43
            other.group0() * Simd32x3::from(-1.0),
            // e23, e31, e12
            other.group1().truncate_to_3() * Simd32x3::from(-1.0),
            // e415, e425, e435, e321
            self.group1(),
            // e423, e431, e412
            self.group0(),
            // e235, e315, e125
            self.group2(),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(0.0),
            // e1234
            0.0,
        );
    }
}
impl std::ops::Sub<AntiDipoleInversion> for Circle {
    type Output = AntiDipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       10        0        0
    //    simd4        0        2        0
    // Totals...
    // yes simd       10        2        0
    //  no simd       10        8        0
    fn sub(self, other: AntiDipoleInversion) -> Self::Output {
        use crate::elements::*;
        return AntiDipoleInversion::from_groups(
            // e423, e431, e412
            Simd32x3::from([self[e423] - other[e423], self[e431] - other[e431], self[e412] - other[e412]]),
            // e415, e425, e435, e321
            Simd32x4::from([self[e415] - other[e415], self[e425] - other[e425], self[e435] - other[e435], self[e321] - other[e321]]),
            // e235, e315, e125, e4
            Simd32x4::from([self[e235] - other[e235], self[e315] - other[e315], self[e125] - other[e125], other[e4]]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e1, e2, e3, e5
            other.group3() * Simd32x4::from(-1.0),
        );
    }
}
impl std::ops::Sub<AntiDualNum> for Circle {
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
            self.group1(),
            // e423, e431, e412
            self.group0(),
            // e235, e315, e125
            self.group2(),
            // e4235, e4315, e4125, e3215
            Simd32x3::from(0.0).extend_to_4(other[e3215] * -1.0),
            // e1234
            0.0,
        );
    }
}
impl std::ops::Sub<AntiFlatPoint> for Circle {
    type Output = Circle;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        4        0        0
    fn sub(self, other: AntiFlatPoint) -> Self::Output {
        use crate::elements::*;
        return Circle::from_groups(
            // e423, e431, e412
            self.group0(),
            // e415, e425, e435, e321
            Simd32x4::from([self[e415], self[e425], self[e435], self[e321] - other[e321]]),
            // e235, e315, e125
            Simd32x3::from([self[e235] - other[e235], self[e315] - other[e315], self[e125] - other[e125]]),
        );
    }
}
impl std::ops::SubAssign<AntiFlatPoint> for Circle {
    fn sub_assign(&mut self, other: AntiFlatPoint) {
        use crate::elements::*;
        *self = Circle::from_groups(
            // e423, e431, e412
            self.group0(),
            // e415, e425, e435, e321
            Simd32x4::from([self[e415], self[e425], self[e435], self[e321] - other[e321]]),
            // e235, e315, e125
            Simd32x3::from([self[e235] - other[e235], self[e315] - other[e315], self[e125] - other[e125]]),
        );
    }
}
impl std::ops::Sub<AntiFlector> for Circle {
    type Output = AntiDipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        0        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        4        1        0
    //  no simd        4        4        0
    fn sub(self, other: AntiFlector) -> Self::Output {
        use crate::elements::*;
        return AntiDipoleInversion::from_groups(
            // e423, e431, e412
            self.group0(),
            // e415, e425, e435, e321
            Simd32x4::from([self[e415], self[e425], self[e435], self[e321] - other[e321]]),
            // e235, e315, e125, e4
            Simd32x4::from([self[e235] - other[e235], self[e315] - other[e315], self[e125] - other[e125], 0.0]),
            // e1, e2, e3, e5
            other.group1() * Simd32x4::from(-1.0),
        );
    }
}
impl std::ops::Sub<AntiLine> for Circle {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        2        0
    //  no simd        0        7        0
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
            Simd32x4::from([other[e15], other[e25], other[e35], 1.0]) * Simd32x4::from([-1.0, -1.0, -1.0, 0.0]),
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12
            other.group0() * Simd32x3::from(-1.0),
            // e415, e425, e435, e321
            self.group1(),
            // e423, e431, e412
            self.group0(),
            // e235, e315, e125
            self.group2(),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(0.0),
            // e1234
            0.0,
        );
    }
}
impl std::ops::Sub<AntiMotor> for Circle {
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
            Simd32x4::from([other[e15], other[e25], other[e35], 1.0]) * Simd32x4::from([-1.0, -1.0, -1.0, 0.0]),
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12
            other.group0().truncate_to_3() * Simd32x3::from(-1.0),
            // e415, e425, e435, e321
            self.group1(),
            // e423, e431, e412
            self.group0(),
            // e235, e315, e125
            self.group2(),
            // e4235, e4315, e4125, e3215
            Simd32x3::from(0.0).extend_to_4(other[e3215] * -1.0),
            // e1234
            0.0,
        );
    }
}
impl std::ops::Sub<AntiPlane> for Circle {
    type Output = AntiDipoleInversion;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn sub(self, other: AntiPlane) -> Self::Output {
        return AntiDipoleInversion::from_groups(
            // e423, e431, e412
            self.group0(),
            // e415, e425, e435, e321
            self.group1(),
            // e235, e315, e125, e4
            crate::swizzle!(self.group2(), 0, 1, 2).extend_to_4(0.0),
            // e1, e2, e3, e5
            other.group0() * Simd32x4::from(-1.0),
        );
    }
}
impl std::ops::Sub<AntiScalar> for Circle {
    type Output = CircleRotor;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn sub(self, other: AntiScalar) -> Self::Output {
        use crate::elements::*;
        return CircleRotor::from_groups(
            // e423, e431, e412
            self.group0(),
            // e415, e425, e435, e321
            self.group1(),
            // e235, e315, e125, e12345
            crate::swizzle!(self.group2(), 0, 1, 2).extend_to_4(other[e12345] * -1.0),
        );
    }
}
impl std::ops::Sub<Circle> for Circle {
    type Output = Circle;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       10        0        0
    fn sub(self, other: Circle) -> Self::Output {
        use crate::elements::*;
        return Circle::from_groups(
            // e423, e431, e412
            Simd32x3::from([self[e423] - other[e423], self[e431] - other[e431], self[e412] - other[e412]]),
            // e415, e425, e435, e321
            Simd32x4::from([self[e415] - other[e415], self[e425] - other[e425], self[e435] - other[e435], self[e321] - other[e321]]),
            // e235, e315, e125
            Simd32x3::from([self[e235] - other[e235], self[e315] - other[e315], self[e125] - other[e125]]),
        );
    }
}
impl std::ops::SubAssign<Circle> for Circle {
    fn sub_assign(&mut self, other: Circle) {
        use crate::elements::*;
        *self = Circle::from_groups(
            // e423, e431, e412
            Simd32x3::from([self[e423] - other[e423], self[e431] - other[e431], self[e412] - other[e412]]),
            // e415, e425, e435, e321
            Simd32x4::from([self[e415] - other[e415], self[e425] - other[e425], self[e435] - other[e435], self[e321] - other[e321]]),
            // e235, e315, e125
            Simd32x3::from([self[e235] - other[e235], self[e315] - other[e315], self[e125] - other[e125]]),
        );
    }
}
impl std::ops::Sub<CircleRotor> for Circle {
    type Output = CircleRotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       10        0        0
    //    simd4        0        1        0
    // Totals...
    // yes simd       10        1        0
    //  no simd       10        4        0
    fn sub(self, other: CircleRotor) -> Self::Output {
        use crate::elements::*;
        return CircleRotor::from_groups(
            // e423, e431, e412
            Simd32x3::from([self[e423] - other[e423], self[e431] - other[e431], self[e412] - other[e412]]),
            // e415, e425, e435, e321
            Simd32x4::from([self[e415] - other[e415], self[e425] - other[e425], self[e435] - other[e435], self[e321] - other[e321]]),
            // e235, e315, e125, e12345
            Simd32x4::from([self[e235] - other[e235], self[e315] - other[e315], self[e125] - other[e125], other[e12345]]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
        );
    }
}
impl std::ops::Sub<Dipole> for Circle {
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
            Simd32x4::from([other[e15], other[e25], other[e35], other[e45]]) * Simd32x4::from(-1.0),
            // e41, e42, e43
            other.group0() * Simd32x3::from(-1.0),
            // e23, e31, e12
            other.group1().truncate_to_3() * Simd32x3::from(-1.0),
            // e415, e425, e435, e321
            self.group1(),
            // e423, e431, e412
            self.group0(),
            // e235, e315, e125
            self.group2(),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(0.0),
            // e1234
            0.0,
        );
    }
}
impl std::ops::Sub<DipoleInversion> for Circle {
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
            Simd32x4::from([other[e15], other[e25], other[e35], other[e45]]) * Simd32x4::from(-1.0),
            // e41, e42, e43
            other.group0() * Simd32x3::from(-1.0),
            // e23, e31, e12
            other.group1().truncate_to_3() * Simd32x3::from(-1.0),
            // e415, e425, e435, e321
            self.group1(),
            // e423, e431, e412
            self.group0(),
            // e235, e315, e125
            self.group2(),
            // e4235, e4315, e4125, e3215
            other.group3() * Simd32x4::from(-1.0),
            // e1234
            other[e1234] * -1.0,
        );
    }
}
impl std::ops::Sub<DualNum> for Circle {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        2        0
    fn sub(self, other: DualNum) -> Self::Output {
        use crate::elements::*;
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            crate::swizzle!(self.group0(), 0, 1, 2).extend_to_4(other[e12345] * -1.0),
            // e415, e425, e435, e321
            self.group1(),
            // e235, e315, e125, e5
            crate::swizzle!(self.group2(), 0, 1, 2).extend_to_4(other[e5] * -1.0),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
        );
    }
}
impl std::ops::Sub<FlatPoint> for Circle {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn sub(self, other: FlatPoint) -> Self::Output {
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
            self.group1(),
            // e423, e431, e412
            self.group0(),
            // e235, e315, e125
            self.group2(),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(0.0),
            // e1234
            0.0,
        );
    }
}
impl std::ops::Sub<Flector> for Circle {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        2        0
    // no simd        0        8        0
    fn sub(self, other: Flector) -> Self::Output {
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
            self.group1(),
            // e423, e431, e412
            self.group0(),
            // e235, e315, e125
            self.group2(),
            // e4235, e4315, e4125, e3215
            other.group1() * Simd32x4::from(-1.0),
            // e1234
            0.0,
        );
    }
}
impl std::ops::Sub<Line> for Circle {
    type Output = Circle;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        6        0        0
    fn sub(self, other: Line) -> Self::Output {
        use crate::elements::*;
        return Circle::from_groups(
            // e423, e431, e412
            self.group0(),
            // e415, e425, e435, e321
            Simd32x4::from([self[e415] - other[e415], self[e425] - other[e425], self[e435] - other[e435], self[e321]]),
            // e235, e315, e125
            Simd32x3::from([self[e235] - other[e235], self[e315] - other[e315], self[e125] - other[e125]]),
        );
    }
}
impl std::ops::SubAssign<Line> for Circle {
    fn sub_assign(&mut self, other: Line) {
        use crate::elements::*;
        *self = Circle::from_groups(
            // e423, e431, e412
            self.group0(),
            // e415, e425, e435, e321
            Simd32x4::from([self[e415] - other[e415], self[e425] - other[e425], self[e435] - other[e435], self[e321]]),
            // e235, e315, e125
            Simd32x3::from([self[e235] - other[e235], self[e315] - other[e315], self[e125] - other[e125]]),
        );
    }
}
impl std::ops::Sub<Motor> for Circle {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        6        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        6        2        0
    //  no simd        6        5        0
    fn sub(self, other: Motor) -> Self::Output {
        use crate::elements::*;
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            crate::swizzle!(self.group0(), 0, 1, 2).extend_to_4(other[e12345] * -1.0),
            // e415, e425, e435, e321
            Simd32x4::from([self[e415] - other[e415], self[e425] - other[e425], self[e435] - other[e435], self[e321]]),
            // e235, e315, e125, e5
            Simd32x4::from([self[e235] - other[e235], self[e315] - other[e315], self[e125] - other[e125], other[e5]]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
        );
    }
}
impl std::ops::Sub<MultiVector> for Circle {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       10        2        0
    //    simd2        0        1        0
    //    simd3        0        2        0
    //    simd4        0        3        0
    // Totals...
    // yes simd       10        8        0
    //  no simd       10       22        0
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
            Simd32x4::from([self[e415] - other[e415], self[e425] - other[e425], self[e435] - other[e435], self[e321] - other[e321]]),
            // e423, e431, e412
            Simd32x3::from([self[e423] - other[e423], self[e431] - other[e431], self[e412] - other[e412]]),
            // e235, e315, e125
            Simd32x3::from([self[e235] - other[e235], self[e315] - other[e315], self[e125] - other[e125]]),
            // e4235, e4315, e4125, e3215
            other.group9() * Simd32x4::from(-1.0),
            // e1234
            other[e1234] * -1.0,
        );
    }
}
impl std::ops::Sub<Plane> for Circle {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn sub(self, other: Plane) -> Self::Output {
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
            self.group1(),
            // e423, e431, e412
            self.group0(),
            // e235, e315, e125
            self.group2(),
            // e4235, e4315, e4125, e3215
            other.group0() * Simd32x4::from(-1.0),
            // e1234
            0.0,
        );
    }
}
impl std::ops::Sub<RoundPoint> for Circle {
    type Output = AntiDipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        2        0
    //  no simd        0        5        0
    fn sub(self, other: RoundPoint) -> Self::Output {
        use crate::elements::*;
        return AntiDipoleInversion::from_groups(
            // e423, e431, e412
            self.group0(),
            // e415, e425, e435, e321
            self.group1(),
            // e235, e315, e125, e4
            crate::swizzle!(self.group2(), 0, 1, 2).extend_to_4(other[e4] * -1.0),
            // e1, e2, e3, e5
            Simd32x4::from([other[e1], other[e2], other[e3], other[e5]]) * Simd32x4::from(-1.0),
        );
    }
}
impl std::ops::Sub<Scalar> for Circle {
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
            self.group1(),
            // e423, e431, e412
            self.group0(),
            // e235, e315, e125
            self.group2(),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(0.0),
            // e1234
            0.0,
        );
    }
}
impl std::ops::Sub<Sphere> for Circle {
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
            self.group1(),
            // e423, e431, e412
            self.group0(),
            // e235, e315, e125
            self.group2(),
            // e4235, e4315, e4125, e3215
            other.group0() * Simd32x4::from(-1.0),
            // e1234
            other[e1234] * -1.0,
        );
    }
}
impl std::ops::Sub<VersorEven> for Circle {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       10        0        0
    //    simd4        0        3        0
    // Totals...
    // yes simd       10        3        0
    //  no simd       10       12        0
    fn sub(self, other: VersorEven) -> Self::Output {
        use crate::elements::*;
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([self[e423] - other[e423], self[e431] - other[e431], self[e412] - other[e412], other[e12345]]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e415, e425, e435, e321
            Simd32x4::from([self[e415] - other[e415], self[e425] - other[e425], self[e435] - other[e435], self[e321] - other[e321]]),
            // e235, e315, e125, e5
            Simd32x4::from([self[e235] - other[e235], self[e315] - other[e315], self[e125] - other[e125], other[e5]]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e1, e2, e3, e4
            other.group3() * Simd32x4::from(-1.0),
        );
    }
}
impl std::ops::Sub<VersorOdd> for Circle {
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
            Simd32x4::from([other[e15], other[e25], other[e35], other[e45]]) * Simd32x4::from(-1.0),
            // e41, e42, e43
            other.group0().truncate_to_3() * Simd32x3::from(-1.0),
            // e23, e31, e12
            other.group1().truncate_to_3() * Simd32x3::from(-1.0),
            // e415, e425, e435, e321
            self.group1(),
            // e423, e431, e412
            self.group0(),
            // e235, e315, e125
            self.group2(),
            // e4235, e4315, e4125, e3215
            other.group3() * Simd32x4::from(-1.0),
            // e1234
            other[e1234] * -1.0,
        );
    }
}

impl TryFrom<AntiDipoleInversion> for Circle {
    type Error = String;
    fn try_from(anti_dipole_inversion: AntiDipoleInversion) -> Result<Self, Self::Error> {
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
            let mut error = "Elements from AntiDipoleInversion do not fit into Circle { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(Circle::from_groups(
            // e423, e431, e412
            anti_dipole_inversion.group0(),
            // e415, e425, e435, e321
            anti_dipole_inversion.group1(),
            // e235, e315, e125
            anti_dipole_inversion.group2().truncate_to_3(),
        ));
    }
}

impl TryFrom<AntiFlector> for Circle {
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
            let mut error = "Elements from AntiFlector do not fit into Circle { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(Circle::from_groups(
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x3::from(0.0).extend_to_4(anti_flector[e321]),
            // e235, e315, e125
            anti_flector.group0().truncate_to_3(),
        ));
    }
}

impl TryFrom<CircleRotor> for Circle {
    type Error = String;
    fn try_from(circle_rotor: CircleRotor) -> Result<Self, Self::Error> {
        let mut error_string = String::new();
        let mut fail = false;
        let el = circle_rotor[10];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e12345: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from CircleRotor do not fit into Circle { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(Circle::from_groups(
            // e423, e431, e412
            circle_rotor.group0(),
            // e415, e425, e435, e321
            circle_rotor.group1(),
            // e235, e315, e125
            circle_rotor.group2().truncate_to_3(),
        ));
    }
}

impl TryFrom<Motor> for Circle {
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
        let el = motor[7];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e5: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from Motor do not fit into Circle { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(Circle::from_groups(
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from([motor[e415], motor[e425], motor[e435], 0.0]),
            // e235, e315, e125
            motor.group1().truncate_to_3(),
        ));
    }
}

impl TryFrom<MultiVector> for Circle {
    type Error = String;
    fn try_from(multi_vector: MultiVector) -> Result<Self, Self::Error> {
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
            let mut error = "Elements from MultiVector do not fit into Circle { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(Circle::from_groups(
            // e423, e431, e412
            multi_vector.group7(),
            // e415, e425, e435, e321
            multi_vector.group6(),
            // e235, e315, e125
            multi_vector.group8(),
        ));
    }
}

impl TryFrom<VersorEven> for Circle {
    type Error = String;
    fn try_from(versor_even: VersorEven) -> Result<Self, Self::Error> {
        let mut error_string = String::new();
        let mut fail = false;
        let el = versor_even[3];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e12345: ");
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
            let mut error = "Elements from VersorEven do not fit into Circle { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(Circle::from_groups(
            // e423, e431, e412
            versor_even.group0().truncate_to_3(),
            // e415, e425, e435, e321
            versor_even.group1(),
            // e235, e315, e125
            versor_even.group2().truncate_to_3(),
        ));
    }
}
