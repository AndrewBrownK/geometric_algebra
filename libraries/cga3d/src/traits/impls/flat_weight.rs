// Note on Operative Statistics:
// Operative Statistics are not a precise predictor of performance or performance comparisons.
// This is due to varying hardware capabilities and compiler optimizations.
// As always, where performance is a concern, there is no substitute for
// real measurements on real work-loads on real hardware.
// Disclaimer aside, enjoy the fun information =)
//
// Total Implementations: 17
//
// Yes SIMD:   add/sub     mul     div
//  Minimum:         0       0       0
//   Median:         0       0       0
//  Average:         0       0       0
//  Maximum:         0       0       0
//
//  No SIMD:   add/sub     mul     div
//  Minimum:         0       0       0
//   Median:         0       0       0
//  Average:         0       0       0
//  Maximum:         0       0       0
impl std::ops::Div<flat_weight> for AntiCircleRotor {
    type Output = FlatPoint;
    fn div(self, _rhs: flat_weight) -> Self::Output {
        self.flat_weight()
    }
}
impl FlatWeight for AntiCircleRotor {
    type Output = FlatPoint;
    fn flat_weight(self) -> Self::Output {
        use crate::elements::*;
        return FlatPoint::from_groups(/* e15, e25, e35, e45 */ Simd32x4::from([0.0, 0.0, 0.0, self[e45]]));
    }
}
impl std::ops::Div<flat_weight> for AntiDipoleInversion {
    type Output = Line;
    fn div(self, _rhs: flat_weight) -> Self::Output {
        self.flat_weight()
    }
}
impl FlatWeight for AntiDipoleInversion {
    type Output = Line;
    fn flat_weight(self) -> Self::Output {
        return Line::from_groups(/* e415, e425, e435 */ self.group1().truncate_to_3(), /* e235, e315, e125 */ Simd32x3::from(0.0));
    }
}
impl std::ops::Div<flat_weight> for AntiScalar {
    type Output = AntiScalar;
    fn div(self, _rhs: flat_weight) -> Self::Output {
        self.flat_weight()
    }
}
impl std::ops::DivAssign<flat_weight> for AntiScalar {
    fn div_assign(&mut self, _rhs: flat_weight) {
        *self = self.flat_weight()
    }
}
impl FlatWeight for AntiScalar {
    type Output = AntiScalar;
    fn flat_weight(self) -> Self::Output {
        return self;
    }
}
impl std::ops::Div<flat_weight> for Circle {
    type Output = Line;
    fn div(self, _rhs: flat_weight) -> Self::Output {
        self.flat_weight()
    }
}
impl FlatWeight for Circle {
    type Output = Line;
    fn flat_weight(self) -> Self::Output {
        return Line::from_groups(/* e415, e425, e435 */ self.group1().truncate_to_3(), /* e235, e315, e125 */ Simd32x3::from(0.0));
    }
}
impl std::ops::Div<flat_weight> for CircleRotor {
    type Output = Motor;
    fn div(self, _rhs: flat_weight) -> Self::Output {
        self.flat_weight()
    }
}
impl FlatWeight for CircleRotor {
    type Output = Motor;
    fn flat_weight(self) -> Self::Output {
        use crate::elements::*;
        return Motor::from_groups(
            // e415, e425, e435, e12345
            Simd32x4::from([self[e415], self[e425], self[e435], self[e12345]]),
            // e235, e315, e125, e5
            Simd32x4::from(0.0),
        );
    }
}
impl std::ops::Div<flat_weight> for Dipole {
    type Output = FlatPoint;
    fn div(self, _rhs: flat_weight) -> Self::Output {
        self.flat_weight()
    }
}
impl FlatWeight for Dipole {
    type Output = FlatPoint;
    fn flat_weight(self) -> Self::Output {
        use crate::elements::*;
        return FlatPoint::from_groups(/* e15, e25, e35, e45 */ Simd32x4::from([0.0, 0.0, 0.0, self[e45]]));
    }
}
impl std::ops::Div<flat_weight> for DipoleInversion {
    type Output = Flector;
    fn div(self, _rhs: flat_weight) -> Self::Output {
        self.flat_weight()
    }
}
impl FlatWeight for DipoleInversion {
    type Output = Flector;
    fn flat_weight(self) -> Self::Output {
        use crate::elements::*;
        return Flector::from_groups(
            // e15, e25, e35, e45
            Simd32x4::from([0.0, 0.0, 0.0, self[e45]]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([self[e4235], self[e4315], self[e4125], 0.0]),
        );
    }
}
impl std::ops::Div<flat_weight> for DualNum {
    type Output = AntiScalar;
    fn div(self, _rhs: flat_weight) -> Self::Output {
        self.flat_weight()
    }
}
impl FlatWeight for DualNum {
    type Output = AntiScalar;
    fn flat_weight(self) -> Self::Output {
        use crate::elements::*;
        return AntiScalar::from_groups(/* e12345 */ self[e12345]);
    }
}
impl std::ops::Div<flat_weight> for FlatPoint {
    type Output = FlatPoint;
    fn div(self, _rhs: flat_weight) -> Self::Output {
        self.flat_weight()
    }
}
impl std::ops::DivAssign<flat_weight> for FlatPoint {
    fn div_assign(&mut self, _rhs: flat_weight) {
        *self = self.flat_weight()
    }
}
impl FlatWeight for FlatPoint {
    type Output = FlatPoint;
    fn flat_weight(self) -> Self::Output {
        use crate::elements::*;
        return FlatPoint::from_groups(/* e15, e25, e35, e45 */ Simd32x4::from([0.0, 0.0, 0.0, self[e45]]));
    }
}
impl std::ops::Div<flat_weight> for Flector {
    type Output = Flector;
    fn div(self, _rhs: flat_weight) -> Self::Output {
        self.flat_weight()
    }
}
impl std::ops::DivAssign<flat_weight> for Flector {
    fn div_assign(&mut self, _rhs: flat_weight) {
        *self = self.flat_weight()
    }
}
impl FlatWeight for Flector {
    type Output = Flector;
    fn flat_weight(self) -> Self::Output {
        use crate::elements::*;
        return Flector::from_groups(
            // e15, e25, e35, e45
            Simd32x4::from([0.0, 0.0, 0.0, self[e45]]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([self[e4235], self[e4315], self[e4125], 0.0]),
        );
    }
}
impl std::ops::Div<flat_weight> for Line {
    type Output = Line;
    fn div(self, _rhs: flat_weight) -> Self::Output {
        self.flat_weight()
    }
}
impl std::ops::DivAssign<flat_weight> for Line {
    fn div_assign(&mut self, _rhs: flat_weight) {
        *self = self.flat_weight()
    }
}
impl FlatWeight for Line {
    type Output = Line;
    fn flat_weight(self) -> Self::Output {
        return Line::from_groups(/* e415, e425, e435 */ self.group0(), /* e235, e315, e125 */ Simd32x3::from(0.0));
    }
}
impl std::ops::Div<flat_weight> for Motor {
    type Output = Motor;
    fn div(self, _rhs: flat_weight) -> Self::Output {
        self.flat_weight()
    }
}
impl std::ops::DivAssign<flat_weight> for Motor {
    fn div_assign(&mut self, _rhs: flat_weight) {
        *self = self.flat_weight()
    }
}
impl FlatWeight for Motor {
    type Output = Motor;
    fn flat_weight(self) -> Self::Output {
        return Motor::from_groups(/* e415, e425, e435, e12345 */ self.group0(), /* e235, e315, e125, e5 */ Simd32x4::from(0.0));
    }
}
impl std::ops::Div<flat_weight> for MultiVector {
    type Output = MultiVector;
    fn div(self, _rhs: flat_weight) -> Self::Output {
        self.flat_weight()
    }
}
impl std::ops::DivAssign<flat_weight> for MultiVector {
    fn div_assign(&mut self, _rhs: flat_weight) {
        *self = self.flat_weight()
    }
}
impl FlatWeight for MultiVector {
    type Output = MultiVector;
    fn flat_weight(self) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, self[e12345]]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e15, e25, e35, e45
            Simd32x4::from([0.0, 0.0, 0.0, self[e45]]),
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from([self[e415], self[e425], self[e435], 0.0]),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([self[e4235], self[e4315], self[e4125], 0.0]),
            // e1234
            0.0,
        );
    }
}
impl std::ops::Div<flat_weight> for Plane {
    type Output = Plane;
    fn div(self, _rhs: flat_weight) -> Self::Output {
        self.flat_weight()
    }
}
impl std::ops::DivAssign<flat_weight> for Plane {
    fn div_assign(&mut self, _rhs: flat_weight) {
        *self = self.flat_weight()
    }
}
impl FlatWeight for Plane {
    type Output = Plane;
    fn flat_weight(self) -> Self::Output {
        use crate::elements::*;
        return Plane::from_groups(/* e4235, e4315, e4125, e3215 */ Simd32x4::from([self[e4235], self[e4315], self[e4125], 0.0]));
    }
}
impl std::ops::Div<flat_weight> for Sphere {
    type Output = Plane;
    fn div(self, _rhs: flat_weight) -> Self::Output {
        self.flat_weight()
    }
}
impl FlatWeight for Sphere {
    type Output = Plane;
    fn flat_weight(self) -> Self::Output {
        use crate::elements::*;
        return Plane::from_groups(/* e4235, e4315, e4125, e3215 */ Simd32x4::from([self[e4235], self[e4315], self[e4125], 0.0]));
    }
}
impl std::ops::Div<flat_weight> for VersorEven {
    type Output = Motor;
    fn div(self, _rhs: flat_weight) -> Self::Output {
        self.flat_weight()
    }
}
impl FlatWeight for VersorEven {
    type Output = Motor;
    fn flat_weight(self) -> Self::Output {
        use crate::elements::*;
        return Motor::from_groups(
            // e415, e425, e435, e12345
            Simd32x4::from([self[e415], self[e425], self[e435], self[e12345]]),
            // e235, e315, e125, e5
            Simd32x4::from(0.0),
        );
    }
}
impl std::ops::Div<flat_weight> for VersorOdd {
    type Output = Flector;
    fn div(self, _rhs: flat_weight) -> Self::Output {
        self.flat_weight()
    }
}
impl FlatWeight for VersorOdd {
    type Output = Flector;
    fn flat_weight(self) -> Self::Output {
        use crate::elements::*;
        return Flector::from_groups(
            // e15, e25, e35, e45
            Simd32x4::from([0.0, 0.0, 0.0, self[e45]]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([self[e4235], self[e4315], self[e4125], 0.0]),
        );
    }
}
