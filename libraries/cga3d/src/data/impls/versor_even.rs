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
// Total Implementations: 113
//
// Yes SIMD:   add/sub     mul     div
//  Minimum:         0       0       0
//   Median:         6       2       0
//  Average:        26      30       0
//  Maximum:       358     390       0
//
//  No SIMD:   add/sub     mul     div
//  Minimum:         0       0       0
//   Median:         6       7       0
//  Average:        32      38       0
//  Maximum:       480     512       0
impl std::ops::Add<AntiCircleRotor> for VersorEven {
    type Output = MultiVector;
    fn add(self, other: AntiCircleRotor) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([other[scalar], self[e12345]]),
            // e1, e2, e3, e4
            self.group3(),
            // e5
            self[e5],
            // e15, e25, e35, e45
            Simd32x4::from([other[e15], other[e25], other[e35], other[e45]]),
            // e41, e42, e43
            other.group0(),
            // e23, e31, e12
            Simd32x3::from([other[e23], other[e31], other[e12]]),
            // e415, e425, e435, e321
            self.group1(),
            // e423, e431, e412
            Simd32x3::from([self[e423], self[e431], self[e412]]),
            // e235, e315, e125
            Simd32x3::from([self[e235], self[e315], self[e125]]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(0.0),
            // e1234
            0.0,
        );
    }
}
impl std::ops::Add<AntiDipoleInversion> for VersorEven {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       15        0        0
    fn add(self, other: AntiDipoleInversion) -> Self::Output {
        use crate::elements::*;
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([other[e423] + self[e423], other[e431] + self[e431], other[e412] + self[e412], self[e12345]]),
            // e415, e425, e435, e321
            Simd32x4::from([other[e415] + self[e415], other[e425] + self[e425], other[e435] + self[e435], other[e321] + self[e321]]),
            // e235, e315, e125, e5
            Simd32x4::from([other[e235] + self[e235], other[e315] + self[e315], other[e125] + self[e125], other[e5] + self[e5]]),
            // e1, e2, e3, e4
            Simd32x4::from([other[e1] + self[e1], other[e2] + self[e2], other[e3] + self[e3], other[e4] + self[e4]]),
        );
    }
}
impl std::ops::AddAssign<AntiDipoleInversion> for VersorEven {
    fn add_assign(&mut self, other: AntiDipoleInversion) {
        use crate::elements::*;
        *self = VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([other[e423] + self[e423], other[e431] + self[e431], other[e412] + self[e412], self[e12345]]),
            // e415, e425, e435, e321
            Simd32x4::from([other[e415] + self[e415], other[e425] + self[e425], other[e435] + self[e435], other[e321] + self[e321]]),
            // e235, e315, e125, e5
            Simd32x4::from([other[e235] + self[e235], other[e315] + self[e315], other[e125] + self[e125], other[e5] + self[e5]]),
            // e1, e2, e3, e4
            Simd32x4::from([other[e1] + self[e1], other[e2] + self[e2], other[e3] + self[e3], other[e4] + self[e4]]),
        );
    }
}
impl std::ops::Add<AntiDualNum> for VersorEven {
    type Output = MultiVector;
    fn add(self, other: AntiDualNum) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([other[scalar], self[e12345]]),
            // e1, e2, e3, e4
            self.group3(),
            // e5
            self[e5],
            // e15, e25, e35, e45
            Simd32x4::from(0.0),
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            self.group1(),
            // e423, e431, e412
            Simd32x3::from([self[e423], self[e431], self[e412]]),
            // e235, e315, e125
            Simd32x3::from([self[e235], self[e315], self[e125]]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([0.0, 0.0, 0.0, other[e3215]]),
            // e1234
            0.0,
        );
    }
}
impl std::ops::Add<AntiFlatPoint> for VersorEven {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        4        0        0
    fn add(self, other: AntiFlatPoint) -> Self::Output {
        use crate::elements::*;
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            self.group0(),
            // e415, e425, e435, e321
            Simd32x4::from([self[e415], self[e425], self[e435], other[e321] + self[e321]]),
            // e235, e315, e125, e5
            Simd32x4::from([other[e235] + self[e235], other[e315] + self[e315], other[e125] + self[e125], self[e5]]),
            // e1, e2, e3, e4
            self.group3(),
        );
    }
}
impl std::ops::AddAssign<AntiFlatPoint> for VersorEven {
    fn add_assign(&mut self, other: AntiFlatPoint) {
        use crate::elements::*;
        *self = VersorEven::from_groups(
            // e423, e431, e412, e12345
            self.group0(),
            // e415, e425, e435, e321
            Simd32x4::from([self[e415], self[e425], self[e435], other[e321] + self[e321]]),
            // e235, e315, e125, e5
            Simd32x4::from([other[e235] + self[e235], other[e315] + self[e315], other[e125] + self[e125], self[e5]]),
            // e1, e2, e3, e4
            self.group3(),
        );
    }
}
impl std::ops::Add<AntiFlector> for VersorEven {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        8        0        0
    fn add(self, other: AntiFlector) -> Self::Output {
        use crate::elements::*;
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            self.group0(),
            // e415, e425, e435, e321
            Simd32x4::from([self[e415], self[e425], self[e435], other[e321] + self[e321]]),
            // e235, e315, e125, e5
            Simd32x4::from([other[e235] + self[e235], other[e315] + self[e315], other[e125] + self[e125], other[e5] + self[e5]]),
            // e1, e2, e3, e4
            Simd32x4::from([other[e1] + self[e1], other[e2] + self[e2], other[e3] + self[e3], self[e4]]),
        );
    }
}
impl std::ops::AddAssign<AntiFlector> for VersorEven {
    fn add_assign(&mut self, other: AntiFlector) {
        use crate::elements::*;
        *self = VersorEven::from_groups(
            // e423, e431, e412, e12345
            self.group0(),
            // e415, e425, e435, e321
            Simd32x4::from([self[e415], self[e425], self[e435], other[e321] + self[e321]]),
            // e235, e315, e125, e5
            Simd32x4::from([other[e235] + self[e235], other[e315] + self[e315], other[e125] + self[e125], other[e5] + self[e5]]),
            // e1, e2, e3, e4
            Simd32x4::from([other[e1] + self[e1], other[e2] + self[e2], other[e3] + self[e3], self[e4]]),
        );
    }
}
impl std::ops::Add<AntiLine> for VersorEven {
    type Output = MultiVector;
    fn add(self, other: AntiLine) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, self[e12345]]),
            // e1, e2, e3, e4
            self.group3(),
            // e5
            self[e5],
            // e15, e25, e35, e45
            crate::swizzle!(other.group1(), 0, 1, 2).extend_to_4(0.0),
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12
            other.group0(),
            // e415, e425, e435, e321
            self.group1(),
            // e423, e431, e412
            Simd32x3::from([self[e423], self[e431], self[e412]]),
            // e235, e315, e125
            Simd32x3::from([self[e235], self[e315], self[e125]]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(0.0),
            // e1234
            0.0,
        );
    }
}
impl std::ops::Add<AntiMotor> for VersorEven {
    type Output = MultiVector;
    fn add(self, other: AntiMotor) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([other[scalar], self[e12345]]),
            // e1, e2, e3, e4
            self.group3(),
            // e5
            self[e5],
            // e15, e25, e35, e45
            Simd32x4::from([other[e15], other[e25], other[e35], 0.0]),
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from([other[e23], other[e31], other[e12]]),
            // e415, e425, e435, e321
            self.group1(),
            // e423, e431, e412
            Simd32x3::from([self[e423], self[e431], self[e412]]),
            // e235, e315, e125
            Simd32x3::from([self[e235], self[e315], self[e125]]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([0.0, 0.0, 0.0, other[e3215]]),
            // e1234
            0.0,
        );
    }
}
impl std::ops::Add<AntiPlane> for VersorEven {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        4        0        0
    fn add(self, other: AntiPlane) -> Self::Output {
        use crate::elements::*;
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            self.group0(),
            // e415, e425, e435, e321
            self.group1(),
            // e235, e315, e125, e5
            Simd32x4::from([self[e235], self[e315], self[e125], other[e5] + self[e5]]),
            // e1, e2, e3, e4
            Simd32x4::from([other[e1] + self[e1], other[e2] + self[e2], other[e3] + self[e3], self[e4]]),
        );
    }
}
impl std::ops::AddAssign<AntiPlane> for VersorEven {
    fn add_assign(&mut self, other: AntiPlane) {
        use crate::elements::*;
        *self = VersorEven::from_groups(
            // e423, e431, e412, e12345
            self.group0(),
            // e415, e425, e435, e321
            self.group1(),
            // e235, e315, e125, e5
            Simd32x4::from([self[e235], self[e315], self[e125], other[e5] + self[e5]]),
            // e1, e2, e3, e4
            Simd32x4::from([other[e1] + self[e1], other[e2] + self[e2], other[e3] + self[e3], self[e4]]),
        );
    }
}
impl std::ops::Add<AntiScalar> for VersorEven {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        1        0        0
    fn add(self, other: AntiScalar) -> Self::Output {
        use crate::elements::*;
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([self[e423], self[e431], self[e412], other[e12345] + self[e12345]]),
            // e415, e425, e435, e321
            self.group1(),
            // e235, e315, e125, e5
            self.group2(),
            // e1, e2, e3, e4
            self.group3(),
        );
    }
}
impl std::ops::AddAssign<AntiScalar> for VersorEven {
    fn add_assign(&mut self, other: AntiScalar) {
        use crate::elements::*;
        *self = VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([self[e423], self[e431], self[e412], other[e12345] + self[e12345]]),
            // e415, e425, e435, e321
            self.group1(),
            // e235, e315, e125, e5
            self.group2(),
            // e1, e2, e3, e4
            self.group3(),
        );
    }
}
impl std::ops::Add<Circle> for VersorEven {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       10        0        0
    fn add(self, other: Circle) -> Self::Output {
        use crate::elements::*;
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([other[e423] + self[e423], other[e431] + self[e431], other[e412] + self[e412], self[e12345]]),
            // e415, e425, e435, e321
            Simd32x4::from([other[e415] + self[e415], other[e425] + self[e425], other[e435] + self[e435], other[e321] + self[e321]]),
            // e235, e315, e125, e5
            Simd32x4::from([other[e235] + self[e235], other[e315] + self[e315], other[e125] + self[e125], self[e5]]),
            // e1, e2, e3, e4
            self.group3(),
        );
    }
}
impl std::ops::AddAssign<Circle> for VersorEven {
    fn add_assign(&mut self, other: Circle) {
        use crate::elements::*;
        *self = VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([other[e423] + self[e423], other[e431] + self[e431], other[e412] + self[e412], self[e12345]]),
            // e415, e425, e435, e321
            Simd32x4::from([other[e415] + self[e415], other[e425] + self[e425], other[e435] + self[e435], other[e321] + self[e321]]),
            // e235, e315, e125, e5
            Simd32x4::from([other[e235] + self[e235], other[e315] + self[e315], other[e125] + self[e125], self[e5]]),
            // e1, e2, e3, e4
            self.group3(),
        );
    }
}
impl std::ops::Add<CircleRotor> for VersorEven {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       11        0        0
    fn add(self, other: CircleRotor) -> Self::Output {
        use crate::elements::*;
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([other[e423] + self[e423], other[e431] + self[e431], other[e412] + self[e412], other[e12345] + self[e12345]]),
            // e415, e425, e435, e321
            Simd32x4::from([other[e415] + self[e415], other[e425] + self[e425], other[e435] + self[e435], other[e321] + self[e321]]),
            // e235, e315, e125, e5
            Simd32x4::from([other[e235] + self[e235], other[e315] + self[e315], other[e125] + self[e125], self[e5]]),
            // e1, e2, e3, e4
            self.group3(),
        );
    }
}
impl std::ops::AddAssign<CircleRotor> for VersorEven {
    fn add_assign(&mut self, other: CircleRotor) {
        use crate::elements::*;
        *self = VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([other[e423] + self[e423], other[e431] + self[e431], other[e412] + self[e412], other[e12345] + self[e12345]]),
            // e415, e425, e435, e321
            Simd32x4::from([other[e415] + self[e415], other[e425] + self[e425], other[e435] + self[e435], other[e321] + self[e321]]),
            // e235, e315, e125, e5
            Simd32x4::from([other[e235] + self[e235], other[e315] + self[e315], other[e125] + self[e125], self[e5]]),
            // e1, e2, e3, e4
            self.group3(),
        );
    }
}
impl std::ops::Add<Dipole> for VersorEven {
    type Output = MultiVector;
    fn add(self, other: Dipole) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, self[e12345]]),
            // e1, e2, e3, e4
            self.group3(),
            // e5
            self[e5],
            // e15, e25, e35, e45
            crate::swizzle!(other.group2(), 0, 1, 2).extend_to_4(other[e45]),
            // e41, e42, e43
            other.group0(),
            // e23, e31, e12
            Simd32x3::from([other[e23], other[e31], other[e12]]),
            // e415, e425, e435, e321
            self.group1(),
            // e423, e431, e412
            Simd32x3::from([self[e423], self[e431], self[e412]]),
            // e235, e315, e125
            Simd32x3::from([self[e235], self[e315], self[e125]]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(0.0),
            // e1234
            0.0,
        );
    }
}
impl std::ops::Add<DipoleInversion> for VersorEven {
    type Output = MultiVector;
    fn add(self, other: DipoleInversion) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, self[e12345]]),
            // e1, e2, e3, e4
            self.group3(),
            // e5
            self[e5],
            // e15, e25, e35, e45
            Simd32x4::from([other[e15], other[e25], other[e35], other[e45]]),
            // e41, e42, e43
            other.group0(),
            // e23, e31, e12
            Simd32x3::from([other[e23], other[e31], other[e12]]),
            // e415, e425, e435, e321
            self.group1(),
            // e423, e431, e412
            Simd32x3::from([self[e423], self[e431], self[e412]]),
            // e235, e315, e125
            Simd32x3::from([self[e235], self[e315], self[e125]]),
            // e4235, e4315, e4125, e3215
            other.group3(),
            // e1234
            other[e1234],
        );
    }
}
impl std::ops::Add<DualNum> for VersorEven {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        0        0
    fn add(self, other: DualNum) -> Self::Output {
        use crate::elements::*;
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([self[e423], self[e431], self[e412], other[e12345] + self[e12345]]),
            // e415, e425, e435, e321
            self.group1(),
            // e235, e315, e125, e5
            Simd32x4::from([self[e235], self[e315], self[e125], other[e5] + self[e5]]),
            // e1, e2, e3, e4
            self.group3(),
        );
    }
}
impl std::ops::AddAssign<DualNum> for VersorEven {
    fn add_assign(&mut self, other: DualNum) {
        use crate::elements::*;
        *self = VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([self[e423], self[e431], self[e412], other[e12345] + self[e12345]]),
            // e415, e425, e435, e321
            self.group1(),
            // e235, e315, e125, e5
            Simd32x4::from([self[e235], self[e315], self[e125], other[e5] + self[e5]]),
            // e1, e2, e3, e4
            self.group3(),
        );
    }
}
impl std::ops::Add<FlatPoint> for VersorEven {
    type Output = MultiVector;
    fn add(self, other: FlatPoint) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, self[e12345]]),
            // e1, e2, e3, e4
            self.group3(),
            // e5
            self[e5],
            // e15, e25, e35, e45
            other.group0(),
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            self.group1(),
            // e423, e431, e412
            Simd32x3::from([self[e423], self[e431], self[e412]]),
            // e235, e315, e125
            Simd32x3::from([self[e235], self[e315], self[e125]]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(0.0),
            // e1234
            0.0,
        );
    }
}
impl std::ops::Add<Flector> for VersorEven {
    type Output = MultiVector;
    fn add(self, other: Flector) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, self[e12345]]),
            // e1, e2, e3, e4
            self.group3(),
            // e5
            self[e5],
            // e15, e25, e35, e45
            other.group0(),
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            self.group1(),
            // e423, e431, e412
            Simd32x3::from([self[e423], self[e431], self[e412]]),
            // e235, e315, e125
            Simd32x3::from([self[e235], self[e315], self[e125]]),
            // e4235, e4315, e4125, e3215
            other.group1(),
            // e1234
            0.0,
        );
    }
}
impl std::ops::Add<Line> for VersorEven {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        6        0        0
    fn add(self, other: Line) -> Self::Output {
        use crate::elements::*;
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            self.group0(),
            // e415, e425, e435, e321
            Simd32x4::from([other[e415] + self[e415], other[e425] + self[e425], other[e435] + self[e435], self[e321]]),
            // e235, e315, e125, e5
            Simd32x4::from([other[e235] + self[e235], other[e315] + self[e315], other[e125] + self[e125], self[e5]]),
            // e1, e2, e3, e4
            self.group3(),
        );
    }
}
impl std::ops::AddAssign<Line> for VersorEven {
    fn add_assign(&mut self, other: Line) {
        use crate::elements::*;
        *self = VersorEven::from_groups(
            // e423, e431, e412, e12345
            self.group0(),
            // e415, e425, e435, e321
            Simd32x4::from([other[e415] + self[e415], other[e425] + self[e425], other[e435] + self[e435], self[e321]]),
            // e235, e315, e125, e5
            Simd32x4::from([other[e235] + self[e235], other[e315] + self[e315], other[e125] + self[e125], self[e5]]),
            // e1, e2, e3, e4
            self.group3(),
        );
    }
}
impl std::ops::Add<Motor> for VersorEven {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        8        0        0
    fn add(self, other: Motor) -> Self::Output {
        use crate::elements::*;
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([self[e423], self[e431], self[e412], other[e12345] + self[e12345]]),
            // e415, e425, e435, e321
            Simd32x4::from([other[e415] + self[e415], other[e425] + self[e425], other[e435] + self[e435], self[e321]]),
            // e235, e315, e125, e5
            Simd32x4::from([other[e235] + self[e235], other[e315] + self[e315], other[e125] + self[e125], other[e5] + self[e5]]),
            // e1, e2, e3, e4
            self.group3(),
        );
    }
}
impl std::ops::AddAssign<Motor> for VersorEven {
    fn add_assign(&mut self, other: Motor) {
        use crate::elements::*;
        *self = VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([self[e423], self[e431], self[e412], other[e12345] + self[e12345]]),
            // e415, e425, e435, e321
            Simd32x4::from([other[e415] + self[e415], other[e425] + self[e425], other[e435] + self[e435], self[e321]]),
            // e235, e315, e125, e5
            Simd32x4::from([other[e235] + self[e235], other[e315] + self[e315], other[e125] + self[e125], other[e5] + self[e5]]),
            // e1, e2, e3, e4
            self.group3(),
        );
    }
}
impl std::ops::Add<MultiVector> for VersorEven {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       16        0        0
    fn add(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([other[scalar], other[e12345] + self[e12345]]),
            // e1, e2, e3, e4
            Simd32x4::from([other[e1] + self[e1], other[e2] + self[e2], other[e3] + self[e3], other[e4] + self[e4]]),
            // e5
            other[e5] + self[e5],
            // e15, e25, e35, e45
            other.group3(),
            // e41, e42, e43
            other.group4(),
            // e23, e31, e12
            other.group5(),
            // e415, e425, e435, e321
            Simd32x4::from([other[e415] + self[e415], other[e425] + self[e425], other[e435] + self[e435], other[e321] + self[e321]]),
            // e423, e431, e412
            Simd32x3::from([other[e423] + self[e423], other[e431] + self[e431], other[e412] + self[e412]]),
            // e235, e315, e125
            Simd32x3::from([other[e235] + self[e235], other[e315] + self[e315], other[e125] + self[e125]]),
            // e4235, e4315, e4125, e3215
            other.group9(),
            // e1234
            other[e1234],
        );
    }
}
impl std::ops::Add<Plane> for VersorEven {
    type Output = MultiVector;
    fn add(self, other: Plane) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, self[e12345]]),
            // e1, e2, e3, e4
            self.group3(),
            // e5
            self[e5],
            // e15, e25, e35, e45
            Simd32x4::from(0.0),
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            self.group1(),
            // e423, e431, e412
            Simd32x3::from([self[e423], self[e431], self[e412]]),
            // e235, e315, e125
            Simd32x3::from([self[e235], self[e315], self[e125]]),
            // e4235, e4315, e4125, e3215
            other.group0(),
            // e1234
            0.0,
        );
    }
}
impl std::ops::Add<RoundPoint> for VersorEven {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        5        0        0
    fn add(self, other: RoundPoint) -> Self::Output {
        use crate::elements::*;
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            self.group0(),
            // e415, e425, e435, e321
            self.group1(),
            // e235, e315, e125, e5
            Simd32x4::from([self[e235], self[e315], self[e125], other[e5] + self[e5]]),
            // e1, e2, e3, e4
            Simd32x4::from([other[e1] + self[e1], other[e2] + self[e2], other[e3] + self[e3], other[e4] + self[e4]]),
        );
    }
}
impl std::ops::AddAssign<RoundPoint> for VersorEven {
    fn add_assign(&mut self, other: RoundPoint) {
        use crate::elements::*;
        *self = VersorEven::from_groups(
            // e423, e431, e412, e12345
            self.group0(),
            // e415, e425, e435, e321
            self.group1(),
            // e235, e315, e125, e5
            Simd32x4::from([self[e235], self[e315], self[e125], other[e5] + self[e5]]),
            // e1, e2, e3, e4
            Simd32x4::from([other[e1] + self[e1], other[e2] + self[e2], other[e3] + self[e3], other[e4] + self[e4]]),
        );
    }
}
impl std::ops::Add<Scalar> for VersorEven {
    type Output = MultiVector;
    fn add(self, other: Scalar) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([other[scalar], self[e12345]]),
            // e1, e2, e3, e4
            self.group3(),
            // e5
            self[e5],
            // e15, e25, e35, e45
            Simd32x4::from(0.0),
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            self.group1(),
            // e423, e431, e412
            Simd32x3::from([self[e423], self[e431], self[e412]]),
            // e235, e315, e125
            Simd32x3::from([self[e235], self[e315], self[e125]]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(0.0),
            // e1234
            0.0,
        );
    }
}
impl std::ops::Add<Sphere> for VersorEven {
    type Output = MultiVector;
    fn add(self, other: Sphere) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, self[e12345]]),
            // e1, e2, e3, e4
            self.group3(),
            // e5
            self[e5],
            // e15, e25, e35, e45
            Simd32x4::from(0.0),
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            self.group1(),
            // e423, e431, e412
            Simd32x3::from([self[e423], self[e431], self[e412]]),
            // e235, e315, e125
            Simd32x3::from([self[e235], self[e315], self[e125]]),
            // e4235, e4315, e4125, e3215
            other.group0(),
            // e1234
            other[e1234],
        );
    }
}
impl std::ops::Add<VersorEven> for VersorEven {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       16        0        0
    fn add(self, other: VersorEven) -> Self::Output {
        use crate::elements::*;
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([other[e423] + self[e423], other[e431] + self[e431], other[e412] + self[e412], other[e12345] + self[e12345]]),
            // e415, e425, e435, e321
            Simd32x4::from([other[e415] + self[e415], other[e425] + self[e425], other[e435] + self[e435], other[e321] + self[e321]]),
            // e235, e315, e125, e5
            Simd32x4::from([other[e235] + self[e235], other[e315] + self[e315], other[e125] + self[e125], other[e5] + self[e5]]),
            // e1, e2, e3, e4
            Simd32x4::from([other[e1] + self[e1], other[e2] + self[e2], other[e3] + self[e3], other[e4] + self[e4]]),
        );
    }
}
impl std::ops::AddAssign<VersorEven> for VersorEven {
    fn add_assign(&mut self, other: VersorEven) {
        use crate::elements::*;
        *self = VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([other[e423] + self[e423], other[e431] + self[e431], other[e412] + self[e412], other[e12345] + self[e12345]]),
            // e415, e425, e435, e321
            Simd32x4::from([other[e415] + self[e415], other[e425] + self[e425], other[e435] + self[e435], other[e321] + self[e321]]),
            // e235, e315, e125, e5
            Simd32x4::from([other[e235] + self[e235], other[e315] + self[e315], other[e125] + self[e125], other[e5] + self[e5]]),
            // e1, e2, e3, e4
            Simd32x4::from([other[e1] + self[e1], other[e2] + self[e2], other[e3] + self[e3], other[e4] + self[e4]]),
        );
    }
}
impl std::ops::Add<VersorOdd> for VersorEven {
    type Output = MultiVector;
    fn add(self, other: VersorOdd) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([other[scalar], self[e12345]]),
            // e1, e2, e3, e4
            self.group3(),
            // e5
            self[e5],
            // e15, e25, e35, e45
            Simd32x4::from([other[e15], other[e25], other[e35], other[e45]]),
            // e41, e42, e43
            Simd32x3::from([other[e41], other[e42], other[e43]]),
            // e23, e31, e12
            Simd32x3::from([other[e23], other[e31], other[e12]]),
            // e415, e425, e435, e321
            self.group1(),
            // e423, e431, e412
            Simd32x3::from([self[e423], self[e431], self[e412]]),
            // e235, e315, e125
            Simd32x3::from([self[e235], self[e315], self[e125]]),
            // e4235, e4315, e4125, e3215
            other.group3(),
            // e1234
            other[e1234],
        );
    }
}
impl std::ops::BitXor<AntiCircleRotor> for VersorEven {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       32       44        0
    //    simd4        2        3        0
    // Totals...
    // yes simd       34       47        0
    //  no simd       40       56        0
    fn bitxor(self, other: AntiCircleRotor) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXorAssign<AntiCircleRotor> for VersorEven {
    fn bitxor_assign(&mut self, other: AntiCircleRotor) {
        *self = self.wedge(other);
    }
}
impl std::ops::BitXor<AntiDipoleInversion> for VersorEven {
    type Output = DipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       34       46        0
    //    simd3        1        2        0
    //    simd4        2        2        0
    // Totals...
    // yes simd       37       50        0
    //  no simd       45       60        0
    fn bitxor(self, other: AntiDipoleInversion) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<AntiDualNum> for VersorEven {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        5        0
    //    simd4        0        3        0
    // Totals...
    // yes simd        1        8        0
    //  no simd        1       17        0
    fn bitxor(self, other: AntiDualNum) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXorAssign<AntiDualNum> for VersorEven {
    fn bitxor_assign(&mut self, other: AntiDualNum) {
        *self = self.wedge(other);
    }
}
impl std::ops::BitXor<AntiFlatPoint> for VersorEven {
    type Output = Sphere;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        8        0
    fn bitxor(self, other: AntiFlatPoint) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<AntiFlector> for VersorEven {
    type Output = DipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       21       33        0
    //    simd3        0        1        0
    //    simd4        1        1        0
    // Totals...
    // yes simd       22       35        0
    //  no simd       25       40        0
    fn bitxor(self, other: AntiFlector) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<AntiLine> for VersorEven {
    type Output = CircleRotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       13       21        0
    //    simd3        0        1        0
    // Totals...
    // yes simd       13       22        0
    //  no simd       13       24        0
    fn bitxor(self, other: AntiLine) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<AntiMotor> for VersorEven {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       16       25        0
    //    simd4        3        4        0
    // Totals...
    // yes simd       19       29        0
    //  no simd       28       41        0
    fn bitxor(self, other: AntiMotor) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXorAssign<AntiMotor> for VersorEven {
    fn bitxor_assign(&mut self, other: AntiMotor) {
        *self = self.wedge(other);
    }
}
impl std::ops::BitXor<AntiPlane> for VersorEven {
    type Output = DipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       13       25        0
    //    simd3        0        1        0
    //    simd4        1        1        0
    // Totals...
    // yes simd       14       27        0
    //  no simd       17       32        0
    fn bitxor(self, other: AntiPlane) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<Circle> for VersorEven {
    type Output = Sphere;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       11       16        0
    //    simd4        1        1        0
    // Totals...
    // yes simd       12       17        0
    //  no simd       15       20        0
    fn bitxor(self, other: Circle) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<CircleRotor> for VersorEven {
    type Output = Sphere;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       11       16        0
    //    simd4        1        1        0
    // Totals...
    // yes simd       12       17        0
    //  no simd       15       20        0
    fn bitxor(self, other: CircleRotor) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<Dipole> for VersorEven {
    type Output = CircleRotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       26       37        0
    //    simd3        1        1        0
    // Totals...
    // yes simd       27       38        0
    //  no simd       29       40        0
    fn bitxor(self, other: Dipole) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<DipoleInversion> for VersorEven {
    type Output = CircleRotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       27       38        0
    //    simd3        1        1        0
    //    simd4        1        1        0
    // Totals...
    // yes simd       29       40        0
    //  no simd       34       45        0
    fn bitxor(self, other: DipoleInversion) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<DualNum> for VersorEven {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        2        0
    // no simd        0        8        0
    fn bitxor(self, other: DualNum) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<FlatPoint> for VersorEven {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        5       12        0
    //    simd4        1        1        0
    // Totals...
    // yes simd        6       13        0
    //  no simd        9       16        0
    fn bitxor(self, other: FlatPoint) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<Flector> for VersorEven {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        8       12        0
    //    simd4        2        2        0
    // Totals...
    // yes simd       10       14        0
    //  no simd       16       20        0
    fn bitxor(self, other: Flector) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<Line> for VersorEven {
    type Output = Plane;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        8       12        0
    fn bitxor(self, other: Line) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<Motor> for VersorEven {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        8       12        0
    //    simd4        1        2        0
    // Totals...
    // yes simd        9       14        0
    //  no simd       12       20        0
    fn bitxor(self, other: Motor) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<MultiVector> for VersorEven {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       59       79        0
    //    simd3        5        6        0
    //    simd4        4        6        0
    // Totals...
    // yes simd       68       91        0
    //  no simd       90      121        0
    fn bitxor(self, other: MultiVector) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<Plane> for VersorEven {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        4        0
    fn bitxor(self, other: Plane) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<RoundPoint> for VersorEven {
    type Output = DipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       18       30        0
    //    simd3        1        2        0
    //    simd4        1        1        0
    // Totals...
    // yes simd       20       33        0
    //  no simd       25       40        0
    fn bitxor(self, other: RoundPoint) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<Scalar> for VersorEven {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        4        0
    // no simd        0       16        0
    fn bitxor(self, other: Scalar) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXorAssign<Scalar> for VersorEven {
    fn bitxor_assign(&mut self, other: Scalar) {
        *self = self.wedge(other);
    }
}
impl std::ops::BitXor<Sphere> for VersorEven {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        4        5        0
    fn bitxor(self, other: Sphere) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<VersorEven> for VersorEven {
    type Output = DipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       34       46        0
    //    simd3        1        2        0
    //    simd4        2        2        0
    // Totals...
    // yes simd       37       50        0
    //  no simd       45       60        0
    fn bitxor(self, other: VersorEven) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<VersorOdd> for VersorEven {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       33       45        0
    //    simd4        3        4        0
    // Totals...
    // yes simd       36       49        0
    //  no simd       45       61        0
    fn bitxor(self, other: VersorOdd) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXorAssign<VersorOdd> for VersorEven {
    fn bitxor_assign(&mut self, other: VersorOdd) {
        *self = self.wedge(other);
    }
}

impl From<AntiDipoleInversion> for VersorEven {
    fn from(from_anti_dipole_inversion: AntiDipoleInversion) -> Self {
        use crate::elements::*;
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([from_anti_dipole_inversion[e423], from_anti_dipole_inversion[e431], from_anti_dipole_inversion[e412], 0.0]),
            // e415, e425, e435, e321
            from_anti_dipole_inversion.group1(),
            // e235, e315, e125, e5
            Simd32x4::from([
                from_anti_dipole_inversion[e235],
                from_anti_dipole_inversion[e315],
                from_anti_dipole_inversion[e125],
                from_anti_dipole_inversion[e5],
            ]),
            // e1, e2, e3, e4
            Simd32x4::from([
                from_anti_dipole_inversion[e1],
                from_anti_dipole_inversion[e2],
                from_anti_dipole_inversion[e3],
                from_anti_dipole_inversion[e4],
            ]),
        );
    }
}

impl From<AntiFlatPoint> for VersorEven {
    fn from(from_anti_flat_point: AntiFlatPoint) -> Self {
        use crate::elements::*;
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from([0.0, 0.0, 0.0, from_anti_flat_point[e321]]),
            // e235, e315, e125, e5
            Simd32x4::from([from_anti_flat_point[e235], from_anti_flat_point[e315], from_anti_flat_point[e125], 0.0]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
        );
    }
}

impl From<AntiFlector> for VersorEven {
    fn from(from_anti_flector: AntiFlector) -> Self {
        use crate::elements::*;
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from([0.0, 0.0, 0.0, from_anti_flector[e321]]),
            // e235, e315, e125, e5
            Simd32x4::from([from_anti_flector[e235], from_anti_flector[e315], from_anti_flector[e125], from_anti_flector[e5]]),
            // e1, e2, e3, e4
            Simd32x4::from([from_anti_flector[e1], from_anti_flector[e2], from_anti_flector[e3], 0.0]),
        );
    }
}

impl From<AntiPlane> for VersorEven {
    fn from(from_anti_plane: AntiPlane) -> Self {
        use crate::elements::*;
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e235, e315, e125, e5
            Simd32x4::from([0.0, 0.0, 0.0, from_anti_plane[e5]]),
            // e1, e2, e3, e4
            Simd32x4::from([from_anti_plane[e1], from_anti_plane[e2], from_anti_plane[e3], 0.0]),
        );
    }
}

impl From<AntiScalar> for VersorEven {
    fn from(from_anti_scalar: AntiScalar) -> Self {
        use crate::elements::*;
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([0.0, 0.0, 0.0, from_anti_scalar[e12345]]),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e235, e315, e125, e5
            Simd32x4::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
        );
    }
}

impl From<Circle> for VersorEven {
    fn from(from_circle: Circle) -> Self {
        use crate::elements::*;
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([from_circle[e423], from_circle[e431], from_circle[e412], 0.0]),
            // e415, e425, e435, e321
            from_circle.group1(),
            // e235, e315, e125, e5
            Simd32x4::from([from_circle[e235], from_circle[e315], from_circle[e125], 0.0]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
        );
    }
}

impl From<CircleRotor> for VersorEven {
    fn from(from_circle_rotor: CircleRotor) -> Self {
        use crate::elements::*;
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([from_circle_rotor[e423], from_circle_rotor[e431], from_circle_rotor[e412], from_circle_rotor[e12345]]),
            // e415, e425, e435, e321
            from_circle_rotor.group1(),
            // e235, e315, e125, e5
            Simd32x4::from([from_circle_rotor[e235], from_circle_rotor[e315], from_circle_rotor[e125], 0.0]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
        );
    }
}

impl From<DualNum> for VersorEven {
    fn from(from_dual_num: DualNum) -> Self {
        use crate::elements::*;
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([0.0, 0.0, 0.0, from_dual_num[e12345]]),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e235, e315, e125, e5
            Simd32x4::from([0.0, 0.0, 0.0, from_dual_num[e5]]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
        );
    }
}

impl From<Line> for VersorEven {
    fn from(from_line: Line) -> Self {
        use crate::elements::*;
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from([from_line[e415], from_line[e425], from_line[e435], 0.0]),
            // e235, e315, e125, e5
            Simd32x4::from([from_line[e235], from_line[e315], from_line[e125], 0.0]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
        );
    }
}

impl From<Motor> for VersorEven {
    fn from(from_motor: Motor) -> Self {
        use crate::elements::*;
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([0.0, 0.0, 0.0, from_motor[e12345]]),
            // e415, e425, e435, e321
            Simd32x4::from([from_motor[e415], from_motor[e425], from_motor[e435], 0.0]),
            // e235, e315, e125, e5
            from_motor.group1(),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
        );
    }
}

impl From<RoundPoint> for VersorEven {
    fn from(from_round_point: RoundPoint) -> Self {
        use crate::elements::*;
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e235, e315, e125, e5
            Simd32x4::from([0.0, 0.0, 0.0, from_round_point[e5]]),
            // e1, e2, e3, e4
            from_round_point.group0(),
        );
    }
}
impl std::ops::Mul<AntiCircleRotor> for VersorEven {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32      108      124        0
    //    simd4       13       13        0
    // Totals...
    // yes simd      121      137        0
    //  no simd      160      176        0
    fn mul(self, other: AntiCircleRotor) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::MulAssign<AntiCircleRotor> for VersorEven {
    fn mul_assign(&mut self, other: AntiCircleRotor) {
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiDipoleInversion> for VersorEven {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32      184      200        0
    //    simd4       10       10        0
    // Totals...
    // yes simd      194      210        0
    //  no simd      224      240        0
    fn mul(self, other: AntiDipoleInversion) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiDualNum> for VersorEven {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4       12        0
    //    simd4        3        5        0
    // Totals...
    // yes simd        7       17        0
    //  no simd       16       32        0
    fn mul(self, other: AntiDualNum) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::MulAssign<AntiDualNum> for VersorEven {
    fn mul_assign(&mut self, other: AntiDualNum) {
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiFlatPoint> for VersorEven {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       48       64        0
    fn mul(self, other: AntiFlatPoint) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiFlector> for VersorEven {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32      108      124        0
    //    simd4        1        1        0
    // Totals...
    // yes simd      109      125        0
    //  no simd      112      128        0
    fn mul(self, other: AntiFlector) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiLine> for VersorEven {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       56       72        0
    //    simd4        6        6        0
    // Totals...
    // yes simd       62       78        0
    //  no simd       80       96        0
    fn mul(self, other: AntiLine) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::MulAssign<AntiLine> for VersorEven {
    fn mul_assign(&mut self, other: AntiLine) {
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiMotor> for VersorEven {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       56       72        0
    //    simd4       14       14        0
    // Totals...
    // yes simd       70       86        0
    //  no simd      112      128        0
    fn mul(self, other: AntiMotor) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::MulAssign<AntiMotor> for VersorEven {
    fn mul_assign(&mut self, other: AntiMotor) {
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiPlane> for VersorEven {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       44       60        0
    //    simd4        1        1        0
    // Totals...
    // yes simd       45       61        0
    //  no simd       48       64        0
    fn mul(self, other: AntiPlane) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiScalar> for VersorEven {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        8        0
    // no simd        0       32        0
    fn mul(self, other: AntiScalar) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<Circle> for VersorEven {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32      136      152        0
    //    simd4        2        2        0
    // Totals...
    // yes simd      138      154        0
    //  no simd      144      160        0
    fn mul(self, other: Circle) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<CircleRotor> for VersorEven {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32      144      160        0
    //    simd4        4        4        0
    // Totals...
    // yes simd      148      164        0
    //  no simd      160      176        0
    fn mul(self, other: CircleRotor) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<Dipole> for VersorEven {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32      108      124        0
    //    simd4        9        9        0
    // Totals...
    // yes simd      117      133        0
    //  no simd      144      160        0
    fn mul(self, other: Dipole) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::MulAssign<Dipole> for VersorEven {
    fn mul_assign(&mut self, other: Dipole) {
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<DipoleInversion> for VersorEven {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32      152      168        0
    //    simd4       18       18        0
    // Totals...
    // yes simd      170      186        0
    //  no simd      224      240        0
    fn mul(self, other: DipoleInversion) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::MulAssign<DipoleInversion> for VersorEven {
    fn mul_assign(&mut self, other: DipoleInversion) {
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<DualNum> for VersorEven {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       12       28        0
    //    simd4        1        3        0
    // Totals...
    // yes simd       13       31        0
    //  no simd       16       40        0
    fn mul(self, other: DualNum) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<FlatPoint> for VersorEven {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       20       32        0
    //    simd4        7        8        0
    // Totals...
    // yes simd       27       40        0
    //  no simd       48       64        0
    fn mul(self, other: FlatPoint) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::MulAssign<FlatPoint> for VersorEven {
    fn mul_assign(&mut self, other: FlatPoint) {
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<Flector> for VersorEven {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       60       76        0
    //    simd4       13       13        0
    // Totals...
    // yes simd       73       89        0
    //  no simd      112      128        0
    fn mul(self, other: Flector) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::MulAssign<Flector> for VersorEven {
    fn mul_assign(&mut self, other: Flector) {
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<Line> for VersorEven {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       80       96        0
    fn mul(self, other: Line) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<Motor> for VersorEven {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       96      112        0
    //    simd4        4        4        0
    // Totals...
    // yes simd      100      116        0
    //  no simd      112      128        0
    fn mul(self, other: Motor) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<MultiVector> for VersorEven {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32      302      334        0
    //    simd2        4        4        0
    //    simd3       38       38        0
    //    simd4       14       14        0
    // Totals...
    // yes simd      358      390        0
    //  no simd      480      512        0
    fn mul(self, other: MultiVector) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<Plane> for VersorEven {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       32       48        0
    //    simd4        4        4        0
    // Totals...
    // yes simd       36       52        0
    //  no simd       48       64        0
    fn mul(self, other: Plane) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::MulAssign<Plane> for VersorEven {
    fn mul_assign(&mut self, other: Plane) {
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<RoundPoint> for VersorEven {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       52       68        0
    //    simd4        3        3        0
    // Totals...
    // yes simd       55       71        0
    //  no simd       64       80        0
    fn mul(self, other: RoundPoint) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<Scalar> for VersorEven {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        4        0
    // no simd        0       16        0
    fn mul(self, other: Scalar) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::MulAssign<Scalar> for VersorEven {
    fn mul_assign(&mut self, other: Scalar) {
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<Sphere> for VersorEven {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       44       60        0
    //    simd4        5        5        0
    // Totals...
    // yes simd       49       65        0
    //  no simd       64       80        0
    fn mul(self, other: Sphere) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::MulAssign<Sphere> for VersorEven {
    fn mul_assign(&mut self, other: Sphere) {
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<VersorEven> for VersorEven {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32      192      208        0
    //    simd4       12       12        0
    // Totals...
    // yes simd      204      220        0
    //  no simd      240      256        0
    fn mul(self, other: VersorEven) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<VersorOdd> for VersorEven {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32      152      168        0
    //    simd4       22       22        0
    // Totals...
    // yes simd      174      190        0
    //  no simd      240      256        0
    fn mul(self, other: VersorOdd) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::MulAssign<VersorOdd> for VersorEven {
    fn mul_assign(&mut self, other: VersorOdd) {
        *self = self.geometric_product(other);
    }
}
impl std::ops::Neg for VersorEven {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        4        0
    // no simd        0       16        0
    fn neg(self) -> Self::Output {
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            self.group0() * Simd32x4::from(-1.0),
            // e415, e425, e435, e321
            self.group1() * Simd32x4::from(-1.0),
            // e235, e315, e125, e5
            self.group2() * Simd32x4::from(-1.0),
            // e1, e2, e3, e4
            self.group3() * Simd32x4::from(-1.0),
        );
    }
}
impl std::ops::Not for VersorEven {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        4        0
    // no simd        0       16        0
    fn not(self) -> Self::Output {
        return self.right_dual();
    }
}
impl std::ops::Sub<AntiCircleRotor> for VersorEven {
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
            Simd32x2::from([other[scalar], self[e12345]]) * Simd32x2::from([-1.0, 1.0]),
            // e1, e2, e3, e4
            self.group3(),
            // e5
            self[e5],
            // e15, e25, e35, e45
            Simd32x4::from([other[e15], other[e25], other[e35], other[e45]]) * Simd32x4::from(-1.0),
            // e41, e42, e43
            other.group0() * Simd32x3::from(-1.0),
            // e23, e31, e12
            Simd32x3::from([other[e23], other[e31], other[e12]]) * Simd32x3::from(-1.0),
            // e415, e425, e435, e321
            self.group1(),
            // e423, e431, e412
            Simd32x3::from([self[e423], self[e431], self[e412]]),
            // e235, e315, e125
            Simd32x3::from([self[e235], self[e315], self[e125]]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(0.0),
            // e1234
            0.0,
        );
    }
}
impl std::ops::Sub<AntiDipoleInversion> for VersorEven {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       15        0        0
    fn sub(self, other: AntiDipoleInversion) -> Self::Output {
        use crate::elements::*;
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([self[e423] - other[e423], self[e431] - other[e431], self[e412] - other[e412], self[e12345]]),
            // e415, e425, e435, e321
            Simd32x4::from([self[e415] - other[e415], self[e425] - other[e425], self[e435] - other[e435], self[e321] - other[e321]]),
            // e235, e315, e125, e5
            Simd32x4::from([self[e235] - other[e235], self[e315] - other[e315], self[e125] - other[e125], self[e5] - other[e5]]),
            // e1, e2, e3, e4
            Simd32x4::from([self[e1] - other[e1], self[e2] - other[e2], self[e3] - other[e3], self[e4] - other[e4]]),
        );
    }
}
impl std::ops::SubAssign<AntiDipoleInversion> for VersorEven {
    fn sub_assign(&mut self, other: AntiDipoleInversion) {
        use crate::elements::*;
        *self = VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([self[e423] - other[e423], self[e431] - other[e431], self[e412] - other[e412], self[e12345]]),
            // e415, e425, e435, e321
            Simd32x4::from([self[e415] - other[e415], self[e425] - other[e425], self[e435] - other[e435], self[e321] - other[e321]]),
            // e235, e315, e125, e5
            Simd32x4::from([self[e235] - other[e235], self[e315] - other[e315], self[e125] - other[e125], self[e5] - other[e5]]),
            // e1, e2, e3, e4
            Simd32x4::from([self[e1] - other[e1], self[e2] - other[e2], self[e3] - other[e3], self[e4] - other[e4]]),
        );
    }
}
impl std::ops::Sub<AntiDualNum> for VersorEven {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd2        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        2        0
    //  no simd        0        6        0
    fn sub(self, other: AntiDualNum) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([other[scalar], self[e12345]]) * Simd32x2::from([-1.0, 1.0]),
            // e1, e2, e3, e4
            self.group3(),
            // e5
            self[e5],
            // e15, e25, e35, e45
            Simd32x4::from(0.0),
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            self.group1(),
            // e423, e431, e412
            Simd32x3::from([self[e423], self[e431], self[e412]]),
            // e235, e315, e125
            Simd32x3::from([self[e235], self[e315], self[e125]]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([1.0, 1.0, 1.0, other[e3215]]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]),
            // e1234
            0.0,
        );
    }
}
impl std::ops::Sub<AntiFlatPoint> for VersorEven {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        4        0        0
    fn sub(self, other: AntiFlatPoint) -> Self::Output {
        use crate::elements::*;
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            self.group0(),
            // e415, e425, e435, e321
            Simd32x4::from([self[e415], self[e425], self[e435], self[e321] - other[e321]]),
            // e235, e315, e125, e5
            Simd32x4::from([self[e235] - other[e235], self[e315] - other[e315], self[e125] - other[e125], self[e5]]),
            // e1, e2, e3, e4
            self.group3(),
        );
    }
}
impl std::ops::SubAssign<AntiFlatPoint> for VersorEven {
    fn sub_assign(&mut self, other: AntiFlatPoint) {
        use crate::elements::*;
        *self = VersorEven::from_groups(
            // e423, e431, e412, e12345
            self.group0(),
            // e415, e425, e435, e321
            Simd32x4::from([self[e415], self[e425], self[e435], self[e321] - other[e321]]),
            // e235, e315, e125, e5
            Simd32x4::from([self[e235] - other[e235], self[e315] - other[e315], self[e125] - other[e125], self[e5]]),
            // e1, e2, e3, e4
            self.group3(),
        );
    }
}
impl std::ops::Sub<AntiFlector> for VersorEven {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        8        0        0
    fn sub(self, other: AntiFlector) -> Self::Output {
        use crate::elements::*;
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            self.group0(),
            // e415, e425, e435, e321
            Simd32x4::from([self[e415], self[e425], self[e435], self[e321] - other[e321]]),
            // e235, e315, e125, e5
            Simd32x4::from([self[e235] - other[e235], self[e315] - other[e315], self[e125] - other[e125], self[e5] - other[e5]]),
            // e1, e2, e3, e4
            Simd32x4::from([self[e1] - other[e1], self[e2] - other[e2], self[e3] - other[e3], self[e4]]),
        );
    }
}
impl std::ops::SubAssign<AntiFlector> for VersorEven {
    fn sub_assign(&mut self, other: AntiFlector) {
        use crate::elements::*;
        *self = VersorEven::from_groups(
            // e423, e431, e412, e12345
            self.group0(),
            // e415, e425, e435, e321
            Simd32x4::from([self[e415], self[e425], self[e435], self[e321] - other[e321]]),
            // e235, e315, e125, e5
            Simd32x4::from([self[e235] - other[e235], self[e315] - other[e315], self[e125] - other[e125], self[e5] - other[e5]]),
            // e1, e2, e3, e4
            Simd32x4::from([self[e1] - other[e1], self[e2] - other[e2], self[e3] - other[e3], self[e4]]),
        );
    }
}
impl std::ops::Sub<AntiLine> for VersorEven {
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
            Simd32x2::from([0.0, self[e12345]]),
            // e1, e2, e3, e4
            self.group3(),
            // e5
            self[e5],
            // e15, e25, e35, e45
            Simd32x4::from([other[e15], other[e25], other[e35], 1.0]) * Simd32x4::from([-1.0, -1.0, -1.0, 0.0]),
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12
            other.group0() * Simd32x3::from(-1.0),
            // e415, e425, e435, e321
            self.group1(),
            // e423, e431, e412
            Simd32x3::from([self[e423], self[e431], self[e412]]),
            // e235, e315, e125
            Simd32x3::from([self[e235], self[e315], self[e125]]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(0.0),
            // e1234
            0.0,
        );
    }
}
impl std::ops::Sub<AntiMotor> for VersorEven {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd2        0        1        0
    //    simd3        0        1        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        0        4        0
    //  no simd        0       13        0
    fn sub(self, other: AntiMotor) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([other[scalar], self[e12345]]) * Simd32x2::from([-1.0, 1.0]),
            // e1, e2, e3, e4
            self.group3(),
            // e5
            self[e5],
            // e15, e25, e35, e45
            Simd32x4::from([other[e15], other[e25], other[e35], 1.0]) * Simd32x4::from([-1.0, -1.0, -1.0, 0.0]),
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from([other[e23], other[e31], other[e12]]) * Simd32x3::from(-1.0),
            // e415, e425, e435, e321
            self.group1(),
            // e423, e431, e412
            Simd32x3::from([self[e423], self[e431], self[e412]]),
            // e235, e315, e125
            Simd32x3::from([self[e235], self[e315], self[e125]]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([1.0, 1.0, 1.0, other[e3215]]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]),
            // e1234
            0.0,
        );
    }
}
impl std::ops::Sub<AntiPlane> for VersorEven {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        4        0        0
    fn sub(self, other: AntiPlane) -> Self::Output {
        use crate::elements::*;
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            self.group0(),
            // e415, e425, e435, e321
            self.group1(),
            // e235, e315, e125, e5
            Simd32x4::from([self[e235], self[e315], self[e125], self[e5] - other[e5]]),
            // e1, e2, e3, e4
            Simd32x4::from([self[e1] - other[e1], self[e2] - other[e2], self[e3] - other[e3], self[e4]]),
        );
    }
}
impl std::ops::SubAssign<AntiPlane> for VersorEven {
    fn sub_assign(&mut self, other: AntiPlane) {
        use crate::elements::*;
        *self = VersorEven::from_groups(
            // e423, e431, e412, e12345
            self.group0(),
            // e415, e425, e435, e321
            self.group1(),
            // e235, e315, e125, e5
            Simd32x4::from([self[e235], self[e315], self[e125], self[e5] - other[e5]]),
            // e1, e2, e3, e4
            Simd32x4::from([self[e1] - other[e1], self[e2] - other[e2], self[e3] - other[e3], self[e4]]),
        );
    }
}
impl std::ops::Sub<AntiScalar> for VersorEven {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        1        0        0
    fn sub(self, other: AntiScalar) -> Self::Output {
        use crate::elements::*;
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([self[e423], self[e431], self[e412], self[e12345] - other[e12345]]),
            // e415, e425, e435, e321
            self.group1(),
            // e235, e315, e125, e5
            self.group2(),
            // e1, e2, e3, e4
            self.group3(),
        );
    }
}
impl std::ops::SubAssign<AntiScalar> for VersorEven {
    fn sub_assign(&mut self, other: AntiScalar) {
        use crate::elements::*;
        *self = VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([self[e423], self[e431], self[e412], self[e12345] - other[e12345]]),
            // e415, e425, e435, e321
            self.group1(),
            // e235, e315, e125, e5
            self.group2(),
            // e1, e2, e3, e4
            self.group3(),
        );
    }
}
impl std::ops::Sub<Circle> for VersorEven {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       10        0        0
    fn sub(self, other: Circle) -> Self::Output {
        use crate::elements::*;
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([self[e423] - other[e423], self[e431] - other[e431], self[e412] - other[e412], self[e12345]]),
            // e415, e425, e435, e321
            Simd32x4::from([self[e415] - other[e415], self[e425] - other[e425], self[e435] - other[e435], self[e321] - other[e321]]),
            // e235, e315, e125, e5
            Simd32x4::from([self[e235] - other[e235], self[e315] - other[e315], self[e125] - other[e125], self[e5]]),
            // e1, e2, e3, e4
            self.group3(),
        );
    }
}
impl std::ops::SubAssign<Circle> for VersorEven {
    fn sub_assign(&mut self, other: Circle) {
        use crate::elements::*;
        *self = VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([self[e423] - other[e423], self[e431] - other[e431], self[e412] - other[e412], self[e12345]]),
            // e415, e425, e435, e321
            Simd32x4::from([self[e415] - other[e415], self[e425] - other[e425], self[e435] - other[e435], self[e321] - other[e321]]),
            // e235, e315, e125, e5
            Simd32x4::from([self[e235] - other[e235], self[e315] - other[e315], self[e125] - other[e125], self[e5]]),
            // e1, e2, e3, e4
            self.group3(),
        );
    }
}
impl std::ops::Sub<CircleRotor> for VersorEven {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       11        0        0
    fn sub(self, other: CircleRotor) -> Self::Output {
        use crate::elements::*;
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([self[e423] - other[e423], self[e431] - other[e431], self[e412] - other[e412], self[e12345] - other[e12345]]),
            // e415, e425, e435, e321
            Simd32x4::from([self[e415] - other[e415], self[e425] - other[e425], self[e435] - other[e435], self[e321] - other[e321]]),
            // e235, e315, e125, e5
            Simd32x4::from([self[e235] - other[e235], self[e315] - other[e315], self[e125] - other[e125], self[e5]]),
            // e1, e2, e3, e4
            self.group3(),
        );
    }
}
impl std::ops::SubAssign<CircleRotor> for VersorEven {
    fn sub_assign(&mut self, other: CircleRotor) {
        use crate::elements::*;
        *self = VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([self[e423] - other[e423], self[e431] - other[e431], self[e412] - other[e412], self[e12345] - other[e12345]]),
            // e415, e425, e435, e321
            Simd32x4::from([self[e415] - other[e415], self[e425] - other[e425], self[e435] - other[e435], self[e321] - other[e321]]),
            // e235, e315, e125, e5
            Simd32x4::from([self[e235] - other[e235], self[e315] - other[e315], self[e125] - other[e125], self[e5]]),
            // e1, e2, e3, e4
            self.group3(),
        );
    }
}
impl std::ops::Sub<Dipole> for VersorEven {
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
            Simd32x2::from([0.0, self[e12345]]),
            // e1, e2, e3, e4
            self.group3(),
            // e5
            self[e5],
            // e15, e25, e35, e45
            Simd32x4::from([other[e15], other[e25], other[e35], other[e45]]) * Simd32x4::from(-1.0),
            // e41, e42, e43
            other.group0() * Simd32x3::from(-1.0),
            // e23, e31, e12
            Simd32x3::from([other[e23], other[e31], other[e12]]) * Simd32x3::from(-1.0),
            // e415, e425, e435, e321
            self.group1(),
            // e423, e431, e412
            Simd32x3::from([self[e423], self[e431], self[e412]]),
            // e235, e315, e125
            Simd32x3::from([self[e235], self[e315], self[e125]]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(0.0),
            // e1234
            0.0,
        );
    }
}
impl std::ops::Sub<DipoleInversion> for VersorEven {
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
            Simd32x2::from([0.0, self[e12345]]),
            // e1, e2, e3, e4
            self.group3(),
            // e5
            self[e5],
            // e15, e25, e35, e45
            Simd32x4::from([other[e15], other[e25], other[e35], other[e45]]) * Simd32x4::from(-1.0),
            // e41, e42, e43
            other.group0() * Simd32x3::from(-1.0),
            // e23, e31, e12
            Simd32x3::from([other[e23], other[e31], other[e12]]) * Simd32x3::from(-1.0),
            // e415, e425, e435, e321
            self.group1(),
            // e423, e431, e412
            Simd32x3::from([self[e423], self[e431], self[e412]]),
            // e235, e315, e125
            Simd32x3::from([self[e235], self[e315], self[e125]]),
            // e4235, e4315, e4125, e3215
            other.group3() * Simd32x4::from(-1.0),
            // e1234
            other[e1234] * -1.0,
        );
    }
}
impl std::ops::Sub<DualNum> for VersorEven {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        0        0
    fn sub(self, other: DualNum) -> Self::Output {
        use crate::elements::*;
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([self[e423], self[e431], self[e412], self[e12345] - other[e12345]]),
            // e415, e425, e435, e321
            self.group1(),
            // e235, e315, e125, e5
            Simd32x4::from([self[e235], self[e315], self[e125], self[e5] - other[e5]]),
            // e1, e2, e3, e4
            self.group3(),
        );
    }
}
impl std::ops::SubAssign<DualNum> for VersorEven {
    fn sub_assign(&mut self, other: DualNum) {
        use crate::elements::*;
        *self = VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([self[e423], self[e431], self[e412], self[e12345] - other[e12345]]),
            // e415, e425, e435, e321
            self.group1(),
            // e235, e315, e125, e5
            Simd32x4::from([self[e235], self[e315], self[e125], self[e5] - other[e5]]),
            // e1, e2, e3, e4
            self.group3(),
        );
    }
}
impl std::ops::Sub<FlatPoint> for VersorEven {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn sub(self, other: FlatPoint) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, self[e12345]]),
            // e1, e2, e3, e4
            self.group3(),
            // e5
            self[e5],
            // e15, e25, e35, e45
            other.group0() * Simd32x4::from(-1.0),
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            self.group1(),
            // e423, e431, e412
            Simd32x3::from([self[e423], self[e431], self[e412]]),
            // e235, e315, e125
            Simd32x3::from([self[e235], self[e315], self[e125]]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(0.0),
            // e1234
            0.0,
        );
    }
}
impl std::ops::Sub<Flector> for VersorEven {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        2        0
    // no simd        0        8        0
    fn sub(self, other: Flector) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, self[e12345]]),
            // e1, e2, e3, e4
            self.group3(),
            // e5
            self[e5],
            // e15, e25, e35, e45
            other.group0() * Simd32x4::from(-1.0),
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            self.group1(),
            // e423, e431, e412
            Simd32x3::from([self[e423], self[e431], self[e412]]),
            // e235, e315, e125
            Simd32x3::from([self[e235], self[e315], self[e125]]),
            // e4235, e4315, e4125, e3215
            other.group1() * Simd32x4::from(-1.0),
            // e1234
            0.0,
        );
    }
}
impl std::ops::Sub<Line> for VersorEven {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        6        0        0
    fn sub(self, other: Line) -> Self::Output {
        use crate::elements::*;
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            self.group0(),
            // e415, e425, e435, e321
            Simd32x4::from([self[e415] - other[e415], self[e425] - other[e425], self[e435] - other[e435], self[e321]]),
            // e235, e315, e125, e5
            Simd32x4::from([self[e235] - other[e235], self[e315] - other[e315], self[e125] - other[e125], self[e5]]),
            // e1, e2, e3, e4
            self.group3(),
        );
    }
}
impl std::ops::SubAssign<Line> for VersorEven {
    fn sub_assign(&mut self, other: Line) {
        use crate::elements::*;
        *self = VersorEven::from_groups(
            // e423, e431, e412, e12345
            self.group0(),
            // e415, e425, e435, e321
            Simd32x4::from([self[e415] - other[e415], self[e425] - other[e425], self[e435] - other[e435], self[e321]]),
            // e235, e315, e125, e5
            Simd32x4::from([self[e235] - other[e235], self[e315] - other[e315], self[e125] - other[e125], self[e5]]),
            // e1, e2, e3, e4
            self.group3(),
        );
    }
}
impl std::ops::Sub<Motor> for VersorEven {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        8        0        0
    fn sub(self, other: Motor) -> Self::Output {
        use crate::elements::*;
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([self[e423], self[e431], self[e412], self[e12345] - other[e12345]]),
            // e415, e425, e435, e321
            Simd32x4::from([self[e415] - other[e415], self[e425] - other[e425], self[e435] - other[e435], self[e321]]),
            // e235, e315, e125, e5
            Simd32x4::from([self[e235] - other[e235], self[e315] - other[e315], self[e125] - other[e125], self[e5] - other[e5]]),
            // e1, e2, e3, e4
            self.group3(),
        );
    }
}
impl std::ops::SubAssign<Motor> for VersorEven {
    fn sub_assign(&mut self, other: Motor) {
        use crate::elements::*;
        *self = VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([self[e423], self[e431], self[e412], self[e12345] - other[e12345]]),
            // e415, e425, e435, e321
            Simd32x4::from([self[e415] - other[e415], self[e425] - other[e425], self[e435] - other[e435], self[e321]]),
            // e235, e315, e125, e5
            Simd32x4::from([self[e235] - other[e235], self[e315] - other[e315], self[e125] - other[e125], self[e5] - other[e5]]),
            // e1, e2, e3, e4
            self.group3(),
        );
    }
}
impl std::ops::Sub<MultiVector> for VersorEven {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       16        1        0
    //    simd2        0        1        0
    //    simd3        0        2        0
    //    simd4        0        2        0
    // Totals...
    // yes simd       16        6        0
    //  no simd       16       17        0
    fn sub(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([other[scalar], self[e12345] - other[e12345]]) * Simd32x2::from([-1.0, 1.0]),
            // e1, e2, e3, e4
            Simd32x4::from([self[e1] - other[e1], self[e2] - other[e2], self[e3] - other[e3], self[e4] - other[e4]]),
            // e5
            self[e5] - other[e5],
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
impl std::ops::Sub<Plane> for VersorEven {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn sub(self, other: Plane) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, self[e12345]]),
            // e1, e2, e3, e4
            self.group3(),
            // e5
            self[e5],
            // e15, e25, e35, e45
            Simd32x4::from(0.0),
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            self.group1(),
            // e423, e431, e412
            Simd32x3::from([self[e423], self[e431], self[e412]]),
            // e235, e315, e125
            Simd32x3::from([self[e235], self[e315], self[e125]]),
            // e4235, e4315, e4125, e3215
            other.group0() * Simd32x4::from(-1.0),
            // e1234
            0.0,
        );
    }
}
impl std::ops::Sub<RoundPoint> for VersorEven {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        5        0        0
    fn sub(self, other: RoundPoint) -> Self::Output {
        use crate::elements::*;
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            self.group0(),
            // e415, e425, e435, e321
            self.group1(),
            // e235, e315, e125, e5
            Simd32x4::from([self[e235], self[e315], self[e125], self[e5] - other[e5]]),
            // e1, e2, e3, e4
            Simd32x4::from([self[e1] - other[e1], self[e2] - other[e2], self[e3] - other[e3], self[e4] - other[e4]]),
        );
    }
}
impl std::ops::SubAssign<RoundPoint> for VersorEven {
    fn sub_assign(&mut self, other: RoundPoint) {
        use crate::elements::*;
        *self = VersorEven::from_groups(
            // e423, e431, e412, e12345
            self.group0(),
            // e415, e425, e435, e321
            self.group1(),
            // e235, e315, e125, e5
            Simd32x4::from([self[e235], self[e315], self[e125], self[e5] - other[e5]]),
            // e1, e2, e3, e4
            Simd32x4::from([self[e1] - other[e1], self[e2] - other[e2], self[e3] - other[e3], self[e4] - other[e4]]),
        );
    }
}
impl std::ops::Sub<Scalar> for VersorEven {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd2        0        1        0
    // no simd        0        2        0
    fn sub(self, other: Scalar) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([other[scalar], self[e12345]]) * Simd32x2::from([-1.0, 1.0]),
            // e1, e2, e3, e4
            self.group3(),
            // e5
            self[e5],
            // e15, e25, e35, e45
            Simd32x4::from(0.0),
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            self.group1(),
            // e423, e431, e412
            Simd32x3::from([self[e423], self[e431], self[e412]]),
            // e235, e315, e125
            Simd32x3::from([self[e235], self[e315], self[e125]]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(0.0),
            // e1234
            0.0,
        );
    }
}
impl std::ops::Sub<Sphere> for VersorEven {
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
            Simd32x2::from([0.0, self[e12345]]),
            // e1, e2, e3, e4
            self.group3(),
            // e5
            self[e5],
            // e15, e25, e35, e45
            Simd32x4::from(0.0),
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            self.group1(),
            // e423, e431, e412
            Simd32x3::from([self[e423], self[e431], self[e412]]),
            // e235, e315, e125
            Simd32x3::from([self[e235], self[e315], self[e125]]),
            // e4235, e4315, e4125, e3215
            other.group0() * Simd32x4::from(-1.0),
            // e1234
            other[e1234] * -1.0,
        );
    }
}
impl std::ops::Sub<VersorEven> for VersorEven {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       16        0        0
    fn sub(self, other: VersorEven) -> Self::Output {
        use crate::elements::*;
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([self[e423] - other[e423], self[e431] - other[e431], self[e412] - other[e412], self[e12345] - other[e12345]]),
            // e415, e425, e435, e321
            Simd32x4::from([self[e415] - other[e415], self[e425] - other[e425], self[e435] - other[e435], self[e321] - other[e321]]),
            // e235, e315, e125, e5
            Simd32x4::from([self[e235] - other[e235], self[e315] - other[e315], self[e125] - other[e125], self[e5] - other[e5]]),
            // e1, e2, e3, e4
            Simd32x4::from([self[e1] - other[e1], self[e2] - other[e2], self[e3] - other[e3], self[e4] - other[e4]]),
        );
    }
}
impl std::ops::SubAssign<VersorEven> for VersorEven {
    fn sub_assign(&mut self, other: VersorEven) {
        use crate::elements::*;
        *self = VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([self[e423] - other[e423], self[e431] - other[e431], self[e412] - other[e412], self[e12345] - other[e12345]]),
            // e415, e425, e435, e321
            Simd32x4::from([self[e415] - other[e415], self[e425] - other[e425], self[e435] - other[e435], self[e321] - other[e321]]),
            // e235, e315, e125, e5
            Simd32x4::from([self[e235] - other[e235], self[e315] - other[e315], self[e125] - other[e125], self[e5] - other[e5]]),
            // e1, e2, e3, e4
            Simd32x4::from([self[e1] - other[e1], self[e2] - other[e2], self[e3] - other[e3], self[e4] - other[e4]]),
        );
    }
}
impl std::ops::Sub<VersorOdd> for VersorEven {
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
            Simd32x2::from([other[scalar], self[e12345]]) * Simd32x2::from([-1.0, 1.0]),
            // e1, e2, e3, e4
            self.group3(),
            // e5
            self[e5],
            // e15, e25, e35, e45
            Simd32x4::from([other[e15], other[e25], other[e35], other[e45]]) * Simd32x4::from(-1.0),
            // e41, e42, e43
            Simd32x3::from([other[e41], other[e42], other[e43]]) * Simd32x3::from(-1.0),
            // e23, e31, e12
            Simd32x3::from([other[e23], other[e31], other[e12]]) * Simd32x3::from(-1.0),
            // e415, e425, e435, e321
            self.group1(),
            // e423, e431, e412
            Simd32x3::from([self[e423], self[e431], self[e412]]),
            // e235, e315, e125
            Simd32x3::from([self[e235], self[e315], self[e125]]),
            // e4235, e4315, e4125, e3215
            other.group3() * Simd32x4::from(-1.0),
            // e1234
            other[e1234] * -1.0,
        );
    }
}

impl TryFrom<MultiVector> for VersorEven {
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
            let mut error = "Elements from MultiVector do not fit into VersorEven { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([multi_vector[e423], multi_vector[e431], multi_vector[e412], multi_vector[e12345]]),
            // e415, e425, e435, e321
            multi_vector.group6(),
            // e235, e315, e125, e5
            Simd32x4::from([multi_vector[e235], multi_vector[e315], multi_vector[e125], multi_vector[e5]]),
            // e1, e2, e3, e4
            multi_vector.group1(),
        ));
    }
}
