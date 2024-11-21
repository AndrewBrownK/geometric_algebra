// Note on Operative Statistics:
// Operative Statistics are not a precise predictor of performance or performance comparisons.
// This is due to varying hardware capabilities and compiler optimizations.
// As always, where performance is a concern, there is no substitute for
// real measurements on real work-loads on real hardware.
// Disclaimer aside, enjoy the fun information =)
//
// Total Implementations: 23
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
impl std::ops::Div<flat_bulk> for AntiCircleRotor {
    type Output = FlatPoint;
    fn div(self, _rhs: flat_bulk) -> Self::Output {
        self.flat_bulk()
    }
}
impl FlatBulk for AntiCircleRotor {
    type Output = FlatPoint;
    fn flat_bulk(self) -> Self::Output {
        use crate::elements::*;
        return FlatPoint::from_groups(/* e15, e25, e35, e45 */ Simd32x4::from([self[e15], self[e25], self[e35], 0.0]));
    }
}
impl std::ops::Div<flat_bulk> for AntiDipoleInversion {
    type Output = AntiFlector;
    fn div(self, _rhs: flat_bulk) -> Self::Output {
        self.flat_bulk()
    }
}
impl FlatBulk for AntiDipoleInversion {
    type Output = AntiFlector;
    fn flat_bulk(self) -> Self::Output {
        use crate::elements::*;
        return AntiFlector::from_groups(
            // e235, e315, e125, e321
            Simd32x4::from([self[e235], self[e315], self[e125], 0.0]),
            // e1, e2, e3, e5
            Simd32x4::from([0.0, 0.0, 0.0, self[e5]]),
        );
    }
}
impl std::ops::Div<flat_bulk> for AntiDualNum {
    type Output = AntiDualNum;
    fn div(self, _rhs: flat_bulk) -> Self::Output {
        self.flat_bulk()
    }
}
impl std::ops::DivAssign<flat_bulk> for AntiDualNum {
    fn div_assign(&mut self, _rhs: flat_bulk) {
        *self = self.flat_bulk()
    }
}
impl FlatBulk for AntiDualNum {
    type Output = AntiDualNum;
    fn flat_bulk(self) -> Self::Output {
        use crate::elements::*;
        return AntiDualNum::from_groups(/* e3215, scalar */ Simd32x2::from([self[e3215], 0.0]));
    }
}
impl std::ops::Div<flat_bulk> for AntiFlatPoint {
    type Output = AntiFlatPoint;
    fn div(self, _rhs: flat_bulk) -> Self::Output {
        self.flat_bulk()
    }
}
impl std::ops::DivAssign<flat_bulk> for AntiFlatPoint {
    fn div_assign(&mut self, _rhs: flat_bulk) {
        *self = self.flat_bulk()
    }
}
impl FlatBulk for AntiFlatPoint {
    type Output = AntiFlatPoint;
    fn flat_bulk(self) -> Self::Output {
        use crate::elements::*;
        return AntiFlatPoint::from_groups(/* e235, e315, e125, e321 */ Simd32x4::from([self[e235], self[e315], self[e125], 0.0]));
    }
}
impl std::ops::Div<flat_bulk> for AntiFlector {
    type Output = AntiFlector;
    fn div(self, _rhs: flat_bulk) -> Self::Output {
        self.flat_bulk()
    }
}
impl std::ops::DivAssign<flat_bulk> for AntiFlector {
    fn div_assign(&mut self, _rhs: flat_bulk) {
        *self = self.flat_bulk()
    }
}
impl FlatBulk for AntiFlector {
    type Output = AntiFlector;
    fn flat_bulk(self) -> Self::Output {
        use crate::elements::*;
        return AntiFlector::from_groups(
            // e235, e315, e125, e321
            Simd32x4::from([self[e235], self[e315], self[e125], 0.0]),
            // e1, e2, e3, e5
            Simd32x4::from([0.0, 0.0, 0.0, self[e5]]),
        );
    }
}
impl std::ops::Div<flat_bulk> for AntiLine {
    type Output = FlatPoint;
    fn div(self, _rhs: flat_bulk) -> Self::Output {
        self.flat_bulk()
    }
}
impl FlatBulk for AntiLine {
    type Output = FlatPoint;
    fn flat_bulk(self) -> Self::Output {
        return FlatPoint::from_groups(/* e15, e25, e35, e45 */ crate::swizzle!(self.group1(), 0, 1, 2).extend_to_4(0.0));
    }
}
impl std::ops::Div<flat_bulk> for AntiMotor {
    type Output = Flector;
    fn div(self, _rhs: flat_bulk) -> Self::Output {
        self.flat_bulk()
    }
}
impl FlatBulk for AntiMotor {
    type Output = Flector;
    fn flat_bulk(self) -> Self::Output {
        use crate::elements::*;
        return Flector::from_groups(
            // e15, e25, e35, e45
            Simd32x4::from([self[e15], self[e25], self[e35], 0.0]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([0.0, 0.0, 0.0, self[e3215]]),
        );
    }
}
impl std::ops::Div<flat_bulk> for AntiPlane {
    type Output = DualNum;
    fn div(self, _rhs: flat_bulk) -> Self::Output {
        self.flat_bulk()
    }
}
impl FlatBulk for AntiPlane {
    type Output = DualNum;
    fn flat_bulk(self) -> Self::Output {
        use crate::elements::*;
        return DualNum::from_groups(/* e5, e12345 */ Simd32x2::from([self[e5], 0.0]));
    }
}
impl std::ops::Div<flat_bulk> for Circle {
    type Output = AntiFlatPoint;
    fn div(self, _rhs: flat_bulk) -> Self::Output {
        self.flat_bulk()
    }
}
impl FlatBulk for Circle {
    type Output = AntiFlatPoint;
    fn flat_bulk(self) -> Self::Output {
        return AntiFlatPoint::from_groups(/* e235, e315, e125, e321 */ crate::swizzle!(self.group2(), 0, 1, 2).extend_to_4(0.0));
    }
}
impl std::ops::Div<flat_bulk> for CircleRotor {
    type Output = AntiFlatPoint;
    fn div(self, _rhs: flat_bulk) -> Self::Output {
        self.flat_bulk()
    }
}
impl FlatBulk for CircleRotor {
    type Output = AntiFlatPoint;
    fn flat_bulk(self) -> Self::Output {
        use crate::elements::*;
        return AntiFlatPoint::from_groups(/* e235, e315, e125, e321 */ Simd32x4::from([self[e235], self[e315], self[e125], 0.0]));
    }
}
impl std::ops::Div<flat_bulk> for Dipole {
    type Output = FlatPoint;
    fn div(self, _rhs: flat_bulk) -> Self::Output {
        self.flat_bulk()
    }
}
impl FlatBulk for Dipole {
    type Output = FlatPoint;
    fn flat_bulk(self) -> Self::Output {
        return FlatPoint::from_groups(/* e15, e25, e35, e45 */ crate::swizzle!(self.group2(), 0, 1, 2).extend_to_4(0.0));
    }
}
impl std::ops::Div<flat_bulk> for DipoleInversion {
    type Output = Flector;
    fn div(self, _rhs: flat_bulk) -> Self::Output {
        self.flat_bulk()
    }
}
impl FlatBulk for DipoleInversion {
    type Output = Flector;
    fn flat_bulk(self) -> Self::Output {
        use crate::elements::*;
        return Flector::from_groups(
            // e15, e25, e35, e45
            Simd32x4::from([self[e15], self[e25], self[e35], 0.0]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([0.0, 0.0, 0.0, self[e3215]]),
        );
    }
}
impl std::ops::Div<flat_bulk> for DualNum {
    type Output = DualNum;
    fn div(self, _rhs: flat_bulk) -> Self::Output {
        self.flat_bulk()
    }
}
impl std::ops::DivAssign<flat_bulk> for DualNum {
    fn div_assign(&mut self, _rhs: flat_bulk) {
        *self = self.flat_bulk()
    }
}
impl FlatBulk for DualNum {
    type Output = DualNum;
    fn flat_bulk(self) -> Self::Output {
        use crate::elements::*;
        return DualNum::from_groups(/* e5, e12345 */ Simd32x2::from([self[e5], 0.0]));
    }
}
impl std::ops::Div<flat_bulk> for FlatPoint {
    type Output = FlatPoint;
    fn div(self, _rhs: flat_bulk) -> Self::Output {
        self.flat_bulk()
    }
}
impl std::ops::DivAssign<flat_bulk> for FlatPoint {
    fn div_assign(&mut self, _rhs: flat_bulk) {
        *self = self.flat_bulk()
    }
}
impl FlatBulk for FlatPoint {
    type Output = FlatPoint;
    fn flat_bulk(self) -> Self::Output {
        use crate::elements::*;
        return FlatPoint::from_groups(/* e15, e25, e35, e45 */ Simd32x4::from([self[e15], self[e25], self[e35], 0.0]));
    }
}
impl std::ops::Div<flat_bulk> for Flector {
    type Output = Flector;
    fn div(self, _rhs: flat_bulk) -> Self::Output {
        self.flat_bulk()
    }
}
impl std::ops::DivAssign<flat_bulk> for Flector {
    fn div_assign(&mut self, _rhs: flat_bulk) {
        *self = self.flat_bulk()
    }
}
impl FlatBulk for Flector {
    type Output = Flector;
    fn flat_bulk(self) -> Self::Output {
        use crate::elements::*;
        return Flector::from_groups(
            // e15, e25, e35, e45
            Simd32x4::from([self[e15], self[e25], self[e35], 0.0]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([0.0, 0.0, 0.0, self[e3215]]),
        );
    }
}
impl std::ops::Div<flat_bulk> for Line {
    type Output = AntiFlatPoint;
    fn div(self, _rhs: flat_bulk) -> Self::Output {
        self.flat_bulk()
    }
}
impl FlatBulk for Line {
    type Output = AntiFlatPoint;
    fn flat_bulk(self) -> Self::Output {
        return AntiFlatPoint::from_groups(/* e235, e315, e125, e321 */ crate::swizzle!(self.group1(), 0, 1, 2).extend_to_4(0.0));
    }
}
impl std::ops::Div<flat_bulk> for Motor {
    type Output = AntiFlector;
    fn div(self, _rhs: flat_bulk) -> Self::Output {
        self.flat_bulk()
    }
}
impl FlatBulk for Motor {
    type Output = AntiFlector;
    fn flat_bulk(self) -> Self::Output {
        use crate::elements::*;
        return AntiFlector::from_groups(
            // e235, e315, e125, e321
            Simd32x4::from([self[e235], self[e315], self[e125], 0.0]),
            // e1, e2, e3, e5
            Simd32x4::from([0.0, 0.0, 0.0, self[e5]]),
        );
    }
}
impl std::ops::Div<flat_bulk> for MultiVector {
    type Output = MultiVector;
    fn div(self, _rhs: flat_bulk) -> Self::Output {
        self.flat_bulk()
    }
}
impl std::ops::DivAssign<flat_bulk> for MultiVector {
    fn div_assign(&mut self, _rhs: flat_bulk) {
        *self = self.flat_bulk()
    }
}
impl FlatBulk for MultiVector {
    type Output = MultiVector;
    fn flat_bulk(self) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            self[e5],
            // e15, e25, e35, e45
            Simd32x4::from([self[e15], self[e25], self[e35], 0.0]),
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            self.group8(),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([0.0, 0.0, 0.0, self[e3215]]),
            // e1234
            0.0,
        );
    }
}
impl std::ops::Div<flat_bulk> for Plane {
    type Output = AntiDualNum;
    fn div(self, _rhs: flat_bulk) -> Self::Output {
        self.flat_bulk()
    }
}
impl FlatBulk for Plane {
    type Output = AntiDualNum;
    fn flat_bulk(self) -> Self::Output {
        use crate::elements::*;
        return AntiDualNum::from_groups(/* e3215, scalar */ Simd32x2::from([self[e3215], 0.0]));
    }
}
impl std::ops::Div<flat_bulk> for RoundPoint {
    type Output = DualNum;
    fn div(self, _rhs: flat_bulk) -> Self::Output {
        self.flat_bulk()
    }
}
impl FlatBulk for RoundPoint {
    type Output = DualNum;
    fn flat_bulk(self) -> Self::Output {
        use crate::elements::*;
        return DualNum::from_groups(/* e5, e12345 */ Simd32x2::from([self[e5], 0.0]));
    }
}
impl std::ops::Div<flat_bulk> for Sphere {
    type Output = AntiDualNum;
    fn div(self, _rhs: flat_bulk) -> Self::Output {
        self.flat_bulk()
    }
}
impl FlatBulk for Sphere {
    type Output = AntiDualNum;
    fn flat_bulk(self) -> Self::Output {
        use crate::elements::*;
        return AntiDualNum::from_groups(/* e3215, scalar */ Simd32x2::from([self[e3215], 0.0]));
    }
}
impl std::ops::Div<flat_bulk> for VersorEven {
    type Output = AntiFlector;
    fn div(self, _rhs: flat_bulk) -> Self::Output {
        self.flat_bulk()
    }
}
impl FlatBulk for VersorEven {
    type Output = AntiFlector;
    fn flat_bulk(self) -> Self::Output {
        use crate::elements::*;
        return AntiFlector::from_groups(
            // e235, e315, e125, e321
            Simd32x4::from([self[e235], self[e315], self[e125], 0.0]),
            // e1, e2, e3, e5
            Simd32x4::from([0.0, 0.0, 0.0, self[e5]]),
        );
    }
}
impl std::ops::Div<flat_bulk> for VersorOdd {
    type Output = Flector;
    fn div(self, _rhs: flat_bulk) -> Self::Output {
        self.flat_bulk()
    }
}
impl FlatBulk for VersorOdd {
    type Output = Flector;
    fn flat_bulk(self) -> Self::Output {
        use crate::elements::*;
        return Flector::from_groups(
            // e15, e25, e35, e45
            Simd32x4::from([self[e15], self[e25], self[e35], 0.0]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([0.0, 0.0, 0.0, self[e3215]]),
        );
    }
}
