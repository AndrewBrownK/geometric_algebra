// Note on Operative Statistics:
// Operative Statistics are not a precise predictor of performance or performance comparisons.
// This is due to varying hardware capabilities and compiler optimizations.
// As always, where performance is a concern, there is no substitute for
// real measurements on real work-loads on real hardware.
// Disclaimer aside, enjoy the fun information =)
//
// Total Implementations: 11
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
impl std::ops::Div<round_weight> for AntiCircleRotor {
    type Output = Dipole;
    fn div(self, _rhs: round_weight) -> Self::Output {
        self.round_weight()
    }
}
impl RoundWeight for AntiCircleRotor {
    type Output = Dipole;
    fn round_weight(self) -> Self::Output {
        return Dipole::from_groups(
            // e41, e42, e43
            self.group0(),
            // e23, e31, e12, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
        );
    }
}
impl std::ops::Div<round_weight> for AntiDipoleInversion {
    type Output = AntiDipoleInversion;
    fn div(self, _rhs: round_weight) -> Self::Output {
        self.round_weight()
    }
}
impl std::ops::DivAssign<round_weight> for AntiDipoleInversion {
    fn div_assign(&mut self, _rhs: round_weight) {
        *self = self.round_weight()
    }
}
impl RoundWeight for AntiDipoleInversion {
    type Output = AntiDipoleInversion;
    fn round_weight(self) -> Self::Output {
        use crate::elements::*;
        return AntiDipoleInversion::from_groups(
            // e423, e431, e412
            self.group0(),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e235, e315, e125, e4
            Simd32x3::from(0.0).with_w(self[e4]),
            // e1, e2, e3, e5
            Simd32x4::from(0.0),
        );
    }
}
impl std::ops::Div<round_weight> for Circle {
    type Output = Circle;
    fn div(self, _rhs: round_weight) -> Self::Output {
        self.round_weight()
    }
}
impl std::ops::DivAssign<round_weight> for Circle {
    fn div_assign(&mut self, _rhs: round_weight) {
        *self = self.round_weight()
    }
}
impl RoundWeight for Circle {
    type Output = Circle;
    fn round_weight(self) -> Self::Output {
        return Circle::from_groups(
            // e423, e431, e412
            self.group0(),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
        );
    }
}
impl std::ops::Div<round_weight> for CircleRotor {
    type Output = Circle;
    fn div(self, _rhs: round_weight) -> Self::Output {
        self.round_weight()
    }
}
impl RoundWeight for CircleRotor {
    type Output = Circle;
    fn round_weight(self) -> Self::Output {
        return Circle::from_groups(
            // e423, e431, e412
            self.group0(),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
        );
    }
}
impl std::ops::Div<round_weight> for Dipole {
    type Output = Dipole;
    fn div(self, _rhs: round_weight) -> Self::Output {
        self.round_weight()
    }
}
impl std::ops::DivAssign<round_weight> for Dipole {
    fn div_assign(&mut self, _rhs: round_weight) {
        *self = self.round_weight()
    }
}
impl RoundWeight for Dipole {
    type Output = Dipole;
    fn round_weight(self) -> Self::Output {
        return Dipole::from_groups(
            // e41, e42, e43
            self.group0(),
            // e23, e31, e12, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
        );
    }
}
impl std::ops::Div<round_weight> for DipoleInversion {
    type Output = DipoleInversion;
    fn div(self, _rhs: round_weight) -> Self::Output {
        self.round_weight()
    }
}
impl std::ops::DivAssign<round_weight> for DipoleInversion {
    fn div_assign(&mut self, _rhs: round_weight) {
        *self = self.round_weight()
    }
}
impl RoundWeight for DipoleInversion {
    type Output = DipoleInversion;
    fn round_weight(self) -> Self::Output {
        use crate::elements::*;
        return DipoleInversion::from_groups(
            // e41, e42, e43
            self.group0(),
            // e23, e31, e12, e45
            Simd32x4::from(0.0),
            // e15, e25, e35, e1234
            Simd32x3::from(0.0).with_w(self[e1234]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(0.0),
        );
    }
}
impl std::ops::Div<round_weight> for MultiVector {
    type Output = MultiVector;
    fn div(self, _rhs: round_weight) -> Self::Output {
        self.round_weight()
    }
}
impl std::ops::DivAssign<round_weight> for MultiVector {
    fn div_assign(&mut self, _rhs: round_weight) {
        *self = self.round_weight()
    }
}
impl RoundWeight for MultiVector {
    type Output = MultiVector;
    fn round_weight(self) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x3::from(0.0).with_w(self[e4]),
            // e5
            0.0,
            // e15, e25, e35, e45
            Simd32x4::from(0.0),
            // e41, e42, e43
            self.group4(),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(0.0),
            // e1234
            self[e1234],
        );
    }
}
impl std::ops::Div<round_weight> for RoundPoint {
    type Output = RoundPoint;
    fn div(self, _rhs: round_weight) -> Self::Output {
        self.round_weight()
    }
}
impl std::ops::DivAssign<round_weight> for RoundPoint {
    fn div_assign(&mut self, _rhs: round_weight) {
        *self = self.round_weight()
    }
}
impl RoundWeight for RoundPoint {
    type Output = RoundPoint;
    fn round_weight(self) -> Self::Output {
        use crate::elements::*;
        return RoundPoint::from_groups(/* e1, e2, e3, e4 */ Simd32x3::from(0.0).with_w(self[e4]), /* e5 */ 0.0);
    }
}
impl std::ops::Div<round_weight> for Sphere {
    type Output = Sphere;
    fn div(self, _rhs: round_weight) -> Self::Output {
        self.round_weight()
    }
}
impl std::ops::DivAssign<round_weight> for Sphere {
    fn div_assign(&mut self, _rhs: round_weight) {
        *self = self.round_weight()
    }
}
impl RoundWeight for Sphere {
    type Output = Sphere;
    fn round_weight(self) -> Self::Output {
        use crate::elements::*;
        return Sphere::from_groups(/* e4235, e4315, e4125, e3215 */ Simd32x4::from(0.0), /* e1234 */ self[e1234]);
    }
}
impl std::ops::Div<round_weight> for VersorEven {
    type Output = AntiDipoleInversion;
    fn div(self, _rhs: round_weight) -> Self::Output {
        self.round_weight()
    }
}
impl RoundWeight for VersorEven {
    type Output = AntiDipoleInversion;
    fn round_weight(self) -> Self::Output {
        use crate::elements::*;
        return AntiDipoleInversion::from_groups(
            // e423, e431, e412
            self.group0().xyz(),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e235, e315, e125, e4
            Simd32x3::from(0.0).with_w(self[e4]),
            // e1, e2, e3, e5
            Simd32x4::from(0.0),
        );
    }
}
impl std::ops::Div<round_weight> for VersorOdd {
    type Output = DipoleInversion;
    fn div(self, _rhs: round_weight) -> Self::Output {
        self.round_weight()
    }
}
impl RoundWeight for VersorOdd {
    type Output = DipoleInversion;
    fn round_weight(self) -> Self::Output {
        use crate::elements::*;
        return DipoleInversion::from_groups(
            // e41, e42, e43
            self.group0().xyz(),
            // e23, e31, e12, e45
            Simd32x4::from(0.0),
            // e15, e25, e35, e1234
            Simd32x3::from(0.0).with_w(self[e1234]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(0.0),
        );
    }
}
