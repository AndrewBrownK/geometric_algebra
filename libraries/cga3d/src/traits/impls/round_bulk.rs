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
impl std::ops::Div<round_bulk> for AntiCircleRotor {
    type Output = AntiMotor;
    fn div(self, _rhs: round_bulk) -> Self::Output {
        self.round_bulk()
    }
}
impl RoundBulk for AntiCircleRotor {
    type Output = AntiMotor;
    fn round_bulk(self) -> Self::Output {
        use crate::elements::*;
        return AntiMotor::from_groups(
            // e23, e31, e12, scalar
            Simd32x4::from([self[e23], self[e31], self[e12], self[scalar]]),
            // e15, e25, e35, e3215
            Simd32x4::from(0.0),
        );
    }
}
impl std::ops::Div<round_bulk> for AntiDipoleInversion {
    type Output = AntiFlector;
    fn div(self, _rhs: round_bulk) -> Self::Output {
        self.round_bulk()
    }
}
impl RoundBulk for AntiDipoleInversion {
    type Output = AntiFlector;
    fn round_bulk(self) -> Self::Output {
        use crate::elements::*;
        return AntiFlector::from_groups(
            // e235, e315, e125, e321
            Simd32x3::from(0.0).extend_to_4(self[e321]),
            // e1, e2, e3, e5
            Simd32x4::from([self[e1], self[e2], self[e3], 0.0]),
        );
    }
}
impl std::ops::Div<round_bulk> for AntiDualNum {
    type Output = Scalar;
    fn div(self, _rhs: round_bulk) -> Self::Output {
        self.round_bulk()
    }
}
impl RoundBulk for AntiDualNum {
    type Output = Scalar;
    fn round_bulk(self) -> Self::Output {
        use crate::elements::*;
        return Scalar::from_groups(/* scalar */ self[scalar]);
    }
}
impl std::ops::Div<round_bulk> for AntiFlatPoint {
    type Output = AntiFlatPoint;
    fn div(self, _rhs: round_bulk) -> Self::Output {
        self.round_bulk()
    }
}
impl std::ops::DivAssign<round_bulk> for AntiFlatPoint {
    fn div_assign(&mut self, _rhs: round_bulk) {
        *self = self.round_bulk()
    }
}
impl RoundBulk for AntiFlatPoint {
    type Output = AntiFlatPoint;
    fn round_bulk(self) -> Self::Output {
        use crate::elements::*;
        return AntiFlatPoint::from_groups(/* e235, e315, e125, e321 */ Simd32x3::from(0.0).extend_to_4(self[e321]));
    }
}
impl std::ops::Div<round_bulk> for AntiFlector {
    type Output = AntiFlector;
    fn div(self, _rhs: round_bulk) -> Self::Output {
        self.round_bulk()
    }
}
impl std::ops::DivAssign<round_bulk> for AntiFlector {
    fn div_assign(&mut self, _rhs: round_bulk) {
        *self = self.round_bulk()
    }
}
impl RoundBulk for AntiFlector {
    type Output = AntiFlector;
    fn round_bulk(self) -> Self::Output {
        use crate::elements::*;
        return AntiFlector::from_groups(
            // e235, e315, e125, e321
            Simd32x3::from(0.0).extend_to_4(self[e321]),
            // e1, e2, e3, e5
            Simd32x4::from([self[e1], self[e2], self[e3], 0.0]),
        );
    }
}
impl std::ops::Div<round_bulk> for AntiLine {
    type Output = AntiLine;
    fn div(self, _rhs: round_bulk) -> Self::Output {
        self.round_bulk()
    }
}
impl std::ops::DivAssign<round_bulk> for AntiLine {
    fn div_assign(&mut self, _rhs: round_bulk) {
        *self = self.round_bulk()
    }
}
impl RoundBulk for AntiLine {
    type Output = AntiLine;
    fn round_bulk(self) -> Self::Output {
        return AntiLine::from_groups(/* e23, e31, e12 */ self.group0(), /* e15, e25, e35 */ Simd32x3::from(0.0));
    }
}
impl std::ops::Div<round_bulk> for AntiMotor {
    type Output = AntiMotor;
    fn div(self, _rhs: round_bulk) -> Self::Output {
        self.round_bulk()
    }
}
impl std::ops::DivAssign<round_bulk> for AntiMotor {
    fn div_assign(&mut self, _rhs: round_bulk) {
        *self = self.round_bulk()
    }
}
impl RoundBulk for AntiMotor {
    type Output = AntiMotor;
    fn round_bulk(self) -> Self::Output {
        return AntiMotor::from_groups(/* e23, e31, e12, scalar */ self.group0(), /* e15, e25, e35, e3215 */ Simd32x4::from(0.0));
    }
}
impl std::ops::Div<round_bulk> for AntiPlane {
    type Output = AntiPlane;
    fn div(self, _rhs: round_bulk) -> Self::Output {
        self.round_bulk()
    }
}
impl std::ops::DivAssign<round_bulk> for AntiPlane {
    fn div_assign(&mut self, _rhs: round_bulk) {
        *self = self.round_bulk()
    }
}
impl RoundBulk for AntiPlane {
    type Output = AntiPlane;
    fn round_bulk(self) -> Self::Output {
        use crate::elements::*;
        return AntiPlane::from_groups(/* e1, e2, e3, e5 */ Simd32x4::from([self[e1], self[e2], self[e3], 0.0]));
    }
}
impl std::ops::Div<round_bulk> for Circle {
    type Output = AntiFlatPoint;
    fn div(self, _rhs: round_bulk) -> Self::Output {
        self.round_bulk()
    }
}
impl RoundBulk for Circle {
    type Output = AntiFlatPoint;
    fn round_bulk(self) -> Self::Output {
        use crate::elements::*;
        return AntiFlatPoint::from_groups(/* e235, e315, e125, e321 */ Simd32x3::from(0.0).extend_to_4(self[e321]));
    }
}
impl std::ops::Div<round_bulk> for CircleRotor {
    type Output = AntiFlatPoint;
    fn div(self, _rhs: round_bulk) -> Self::Output {
        self.round_bulk()
    }
}
impl RoundBulk for CircleRotor {
    type Output = AntiFlatPoint;
    fn round_bulk(self) -> Self::Output {
        use crate::elements::*;
        return AntiFlatPoint::from_groups(/* e235, e315, e125, e321 */ Simd32x3::from(0.0).extend_to_4(self[e321]));
    }
}
impl std::ops::Div<round_bulk> for Dipole {
    type Output = AntiLine;
    fn div(self, _rhs: round_bulk) -> Self::Output {
        self.round_bulk()
    }
}
impl RoundBulk for Dipole {
    type Output = AntiLine;
    fn round_bulk(self) -> Self::Output {
        return AntiLine::from_groups(/* e23, e31, e12 */ self.group1().truncate_to_3(), /* e15, e25, e35 */ Simd32x3::from(0.0));
    }
}
impl std::ops::Div<round_bulk> for DipoleInversion {
    type Output = AntiLine;
    fn div(self, _rhs: round_bulk) -> Self::Output {
        self.round_bulk()
    }
}
impl RoundBulk for DipoleInversion {
    type Output = AntiLine;
    fn round_bulk(self) -> Self::Output {
        return AntiLine::from_groups(/* e23, e31, e12 */ self.group1().truncate_to_3(), /* e15, e25, e35 */ Simd32x3::from(0.0));
    }
}
impl std::ops::Div<round_bulk> for MultiVector {
    type Output = MultiVector;
    fn div(self, _rhs: round_bulk) -> Self::Output {
        self.round_bulk()
    }
}
impl std::ops::DivAssign<round_bulk> for MultiVector {
    fn div_assign(&mut self, _rhs: round_bulk) {
        *self = self.round_bulk()
    }
}
impl RoundBulk for MultiVector {
    type Output = MultiVector;
    fn round_bulk(self) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self[scalar], 0.0]),
            // e1, e2, e3, e4
            Simd32x4::from([self[e1], self[e2], self[e3], 0.0]),
            // e5
            0.0,
            // e15, e25, e35, e45
            Simd32x4::from(0.0),
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            Simd32x3::from(0.0).extend_to_4(self[e321]),
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
impl std::ops::Div<round_bulk> for RoundPoint {
    type Output = AntiPlane;
    fn div(self, _rhs: round_bulk) -> Self::Output {
        self.round_bulk()
    }
}
impl RoundBulk for RoundPoint {
    type Output = AntiPlane;
    fn round_bulk(self) -> Self::Output {
        use crate::elements::*;
        return AntiPlane::from_groups(/* e1, e2, e3, e5 */ Simd32x4::from([self[e1], self[e2], self[e3], 0.0]));
    }
}
impl std::ops::Div<round_bulk> for Scalar {
    type Output = Scalar;
    fn div(self, _rhs: round_bulk) -> Self::Output {
        self.round_bulk()
    }
}
impl std::ops::DivAssign<round_bulk> for Scalar {
    fn div_assign(&mut self, _rhs: round_bulk) {
        *self = self.round_bulk()
    }
}
impl RoundBulk for Scalar {
    type Output = Scalar;
    fn round_bulk(self) -> Self::Output {
        return self;
    }
}
impl std::ops::Div<round_bulk> for VersorEven {
    type Output = AntiFlector;
    fn div(self, _rhs: round_bulk) -> Self::Output {
        self.round_bulk()
    }
}
impl RoundBulk for VersorEven {
    type Output = AntiFlector;
    fn round_bulk(self) -> Self::Output {
        use crate::elements::*;
        return AntiFlector::from_groups(
            // e235, e315, e125, e321
            Simd32x3::from(0.0).extend_to_4(self[e321]),
            // e1, e2, e3, e5
            Simd32x4::from([self[e1], self[e2], self[e3], 0.0]),
        );
    }
}
impl std::ops::Div<round_bulk> for VersorOdd {
    type Output = AntiMotor;
    fn div(self, _rhs: round_bulk) -> Self::Output {
        self.round_bulk()
    }
}
impl RoundBulk for VersorOdd {
    type Output = AntiMotor;
    fn round_bulk(self) -> Self::Output {
        use crate::elements::*;
        return AntiMotor::from_groups(
            // e23, e31, e12, scalar
            Simd32x4::from([self[e23], self[e31], self[e12], self[scalar]]),
            // e15, e25, e35, e3215
            Simd32x4::from(0.0),
        );
    }
}
