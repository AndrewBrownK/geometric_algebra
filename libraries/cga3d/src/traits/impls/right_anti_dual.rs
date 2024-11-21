// Note on Operative Statistics:
// Operative Statistics are not a precise predictor of performance or performance comparisons.
// This is due to varying hardware capabilities and compiler optimizations.
// As always, where performance is a concern, there is no substitute for
// real measurements on real work-loads on real hardware.
// Disclaimer aside, enjoy the fun information =)
//
// Total Implementations: 25
//
// Yes SIMD:   add/sub     mul     div
//  Minimum:         0       0       0
//   Median:         0       2       0
//  Average:         0       3       0
//  Maximum:         0      12       0
//
//  No SIMD:   add/sub     mul     div
//  Minimum:         0       0       0
//   Median:         0       3       0
//  Average:         0       4       0
//  Maximum:         0      16       0
impl std::ops::Div<right_anti_dual> for AntiCircleRotor {
    type Output = CircleRotor;
    fn div(self, _rhs: right_anti_dual) -> Self::Output {
        self.right_anti_dual()
    }
}
impl RightAntiDual for AntiCircleRotor {
    type Output = CircleRotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        6        0
    //    simd3        0        1        0
    // Totals...
    // yes simd        0        7        0
    //  no simd        0        9        0
    fn right_anti_dual(self) -> Self::Output {
        use crate::elements::*;
        return CircleRotor::from_groups(
            // e423, e431, e412
            Simd32x3::from([self[e41], self[e42], self[e43]]) * Simd32x3::from(-1.0),
            // e415, e425, e435, e321
            Simd32x4::from([self[e23] * -1.0, self[e31] * -1.0, self[e12] * -1.0, self[e45]]),
            // e235, e315, e125, e12345
            Simd32x4::from([self[e15] * -1.0, self[e25] * -1.0, self[e35] * -1.0, self[scalar]]),
        );
    }
}
impl std::ops::Div<right_anti_dual> for AntiDipoleInversion {
    type Output = DipoleInversion;
    fn div(self, _rhs: right_anti_dual) -> Self::Output {
        self.right_anti_dual()
    }
}
impl RightAntiDual for AntiDipoleInversion {
    type Output = DipoleInversion;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        3        0
    fn right_anti_dual(self) -> Self::Output {
        use crate::elements::*;
        return DipoleInversion::from_groups(
            // e41, e42, e43
            self.group0(),
            // e23, e31, e12, e45
            Simd32x4::from([self[e415], self[e425], self[e435], self[e321] * -1.0]),
            // e15, e25, e35, e1234
            Simd32x4::from([self[e235], self[e315], self[e125], self[e4] * -1.0]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([self[e1], self[e2], self[e3], self[e5] * -1.0]),
        );
    }
}
impl std::ops::Div<right_anti_dual> for AntiDualNum {
    type Output = AntiDualNum;
    fn div(self, _rhs: right_anti_dual) -> Self::Output {
        self.right_anti_dual()
    }
}
impl std::ops::DivAssign<right_anti_dual> for AntiDualNum {
    fn div_assign(&mut self, _rhs: right_anti_dual) {
        *self = self.right_anti_dual()
    }
}
impl RightAntiDual for AntiDualNum {
    type Output = AntiDualNum;
    fn right_anti_dual(self) -> Self::Output {
        return self;
    }
}
impl std::ops::Div<right_anti_dual> for AntiFlatPoint {
    type Output = FlatPoint;
    fn div(self, _rhs: right_anti_dual) -> Self::Output {
        self.right_anti_dual()
    }
}
impl RightAntiDual for AntiFlatPoint {
    type Output = FlatPoint;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn right_anti_dual(self) -> Self::Output {
        use crate::elements::*;
        return FlatPoint::from_groups(/* e15, e25, e35, e45 */ Simd32x4::from([self[e235], self[e315], self[e125], self[e321] * -1.0]));
    }
}
impl std::ops::Div<right_anti_dual> for AntiFlector {
    type Output = Flector;
    fn div(self, _rhs: right_anti_dual) -> Self::Output {
        self.right_anti_dual()
    }
}
impl RightAntiDual for AntiFlector {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        2        0
    fn right_anti_dual(self) -> Self::Output {
        use crate::elements::*;
        return Flector::from_groups(
            // e15, e25, e35, e45
            Simd32x4::from([self[e235], self[e315], self[e125], self[e321] * -1.0]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([self[e1], self[e2], self[e3], self[e5] * -1.0]),
        );
    }
}
impl std::ops::Div<right_anti_dual> for AntiLine {
    type Output = Line;
    fn div(self, _rhs: right_anti_dual) -> Self::Output {
        self.right_anti_dual()
    }
}
impl RightAntiDual for AntiLine {
    type Output = Line;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        2        0
    // no simd        0        6        0
    fn right_anti_dual(self) -> Self::Output {
        use crate::elements::*;
        return Line::from_groups(
            // e415, e425, e435
            Simd32x3::from([self[e23], self[e31], self[e12]]) * Simd32x3::from(-1.0),
            // e235, e315, e125
            Simd32x3::from([self[e15], self[e25], self[e35]]) * Simd32x3::from(-1.0),
        );
    }
}
impl std::ops::Div<right_anti_dual> for AntiMotor {
    type Output = Motor;
    fn div(self, _rhs: right_anti_dual) -> Self::Output {
        self.right_anti_dual()
    }
}
impl RightAntiDual for AntiMotor {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        6        0
    fn right_anti_dual(self) -> Self::Output {
        use crate::elements::*;
        return Motor::from_groups(
            // e415, e425, e435, e12345
            Simd32x4::from([self[e23] * -1.0, self[e31] * -1.0, self[e12] * -1.0, self[scalar]]),
            // e235, e315, e125, e5
            Simd32x4::from([self[e15] * -1.0, self[e25] * -1.0, self[e35] * -1.0, self[e3215]]),
        );
    }
}
impl std::ops::Div<right_anti_dual> for AntiPlane {
    type Output = Plane;
    fn div(self, _rhs: right_anti_dual) -> Self::Output {
        self.right_anti_dual()
    }
}
impl RightAntiDual for AntiPlane {
    type Output = Plane;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn right_anti_dual(self) -> Self::Output {
        use crate::elements::*;
        return Plane::from_groups(/* e4235, e4315, e4125, e3215 */ Simd32x4::from([self[e1], self[e2], self[e3], self[e5] * -1.0]));
    }
}
impl std::ops::Div<right_anti_dual> for AntiScalar {
    type Output = Scalar;
    fn div(self, _rhs: right_anti_dual) -> Self::Output {
        self.right_anti_dual()
    }
}
impl RightAntiDual for AntiScalar {
    type Output = Scalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn right_anti_dual(self) -> Self::Output {
        use crate::elements::*;
        return Scalar::from_groups(/* scalar */ self[e12345] * -1.0);
    }
}
impl std::ops::Div<right_anti_dual> for Circle {
    type Output = Dipole;
    fn div(self, _rhs: right_anti_dual) -> Self::Output {
        self.right_anti_dual()
    }
}
impl RightAntiDual for Circle {
    type Output = Dipole;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn right_anti_dual(self) -> Self::Output {
        use crate::elements::*;
        return Dipole::from_groups(
            // e41, e42, e43
            self.group0(),
            // e23, e31, e12, e45
            Simd32x4::from([self[e415], self[e425], self[e435], self[e321] * -1.0]),
            // e15, e25, e35
            self.group2(),
        );
    }
}
impl std::ops::Div<right_anti_dual> for CircleRotor {
    type Output = AntiCircleRotor;
    fn div(self, _rhs: right_anti_dual) -> Self::Output {
        self.right_anti_dual()
    }
}
impl RightAntiDual for CircleRotor {
    type Output = AntiCircleRotor;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        2        0
    fn right_anti_dual(self) -> Self::Output {
        use crate::elements::*;
        return AntiCircleRotor::from_groups(
            // e41, e42, e43
            self.group0(),
            // e23, e31, e12, e45
            Simd32x4::from([self[e415], self[e425], self[e435], self[e321] * -1.0]),
            // e15, e25, e35, scalar
            Simd32x4::from([self[e235], self[e315], self[e125], self[e12345] * -1.0]),
        );
    }
}
impl std::ops::Div<right_anti_dual> for Dipole {
    type Output = Circle;
    fn div(self, _rhs: right_anti_dual) -> Self::Output {
        self.right_anti_dual()
    }
}
impl RightAntiDual for Dipole {
    type Output = Circle;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        3        0
    //    simd3        0        2        0
    // Totals...
    // yes simd        0        5        0
    //  no simd        0        9        0
    fn right_anti_dual(self) -> Self::Output {
        use crate::elements::*;
        return Circle::from_groups(
            // e423, e431, e412
            Simd32x3::from([self[e41], self[e42], self[e43]]) * Simd32x3::from(-1.0),
            // e415, e425, e435, e321
            Simd32x4::from([self[e23] * -1.0, self[e31] * -1.0, self[e12] * -1.0, self[e45]]),
            // e235, e315, e125
            Simd32x3::from([self[e15], self[e25], self[e35]]) * Simd32x3::from(-1.0),
        );
    }
}
impl std::ops::Div<right_anti_dual> for DipoleInversion {
    type Output = AntiDipoleInversion;
    fn div(self, _rhs: right_anti_dual) -> Self::Output {
        self.right_anti_dual()
    }
}
impl RightAntiDual for DipoleInversion {
    type Output = AntiDipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        9        0
    //    simd3        0        1        0
    // Totals...
    // yes simd        0       10        0
    //  no simd        0       12        0
    fn right_anti_dual(self) -> Self::Output {
        use crate::elements::*;
        return AntiDipoleInversion::from_groups(
            // e423, e431, e412
            Simd32x3::from([self[e41], self[e42], self[e43]]) * Simd32x3::from(-1.0),
            // e415, e425, e435, e321
            Simd32x4::from([self[e23] * -1.0, self[e31] * -1.0, self[e12] * -1.0, self[e45]]),
            // e235, e315, e125, e4
            Simd32x4::from([self[e15] * -1.0, self[e25] * -1.0, self[e35] * -1.0, self[e1234]]),
            // e1, e2, e3, e5
            Simd32x4::from([self[e4235] * -1.0, self[e4315] * -1.0, self[e4125] * -1.0, self[e3215]]),
        );
    }
}
impl std::ops::Div<right_anti_dual> for DualNum {
    type Output = AntiDualNum;
    fn div(self, _rhs: right_anti_dual) -> Self::Output {
        self.right_anti_dual()
    }
}
impl RightAntiDual for DualNum {
    type Output = AntiDualNum;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd2        0        1        0
    // no simd        0        2        0
    fn right_anti_dual(self) -> Self::Output {
        use crate::elements::*;
        return AntiDualNum::from_groups(/* e1234, scalar */ Simd32x2::from([self[e4], self[e12345]]) * Simd32x2::from(-1.0));
    }
}
impl std::ops::Div<right_anti_dual> for FlatPoint {
    type Output = AntiFlatPoint;
    fn div(self, _rhs: right_anti_dual) -> Self::Output {
        self.right_anti_dual()
    }
}
impl RightAntiDual for FlatPoint {
    type Output = AntiFlatPoint;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        3        0
    fn right_anti_dual(self) -> Self::Output {
        use crate::elements::*;
        return AntiFlatPoint::from_groups(/* e235, e315, e125, e321 */ Simd32x4::from([self[e15] * -1.0, self[e25] * -1.0, self[e35] * -1.0, self[e45]]));
    }
}
impl std::ops::Div<right_anti_dual> for Flector {
    type Output = AntiFlector;
    fn div(self, _rhs: right_anti_dual) -> Self::Output {
        self.right_anti_dual()
    }
}
impl RightAntiDual for Flector {
    type Output = AntiFlector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        6        0
    fn right_anti_dual(self) -> Self::Output {
        use crate::elements::*;
        return AntiFlector::from_groups(
            // e235, e315, e125, e321
            Simd32x4::from([self[e15] * -1.0, self[e25] * -1.0, self[e35] * -1.0, self[e45]]),
            // e1, e2, e3, e5
            Simd32x4::from([self[e4235] * -1.0, self[e4315] * -1.0, self[e4125] * -1.0, self[e3215]]),
        );
    }
}
impl std::ops::Div<right_anti_dual> for Line {
    type Output = Line;
    fn div(self, _rhs: right_anti_dual) -> Self::Output {
        self.right_anti_dual()
    }
}
impl std::ops::DivAssign<right_anti_dual> for Line {
    fn div_assign(&mut self, _rhs: right_anti_dual) {
        *self = self.right_anti_dual()
    }
}
impl RightAntiDual for Line {
    type Output = Line;
    fn right_anti_dual(self) -> Self::Output {
        return self;
    }
}
impl std::ops::Div<right_anti_dual> for Motor {
    type Output = AntiMotor;
    fn div(self, _rhs: right_anti_dual) -> Self::Output {
        self.right_anti_dual()
    }
}
impl RightAntiDual for Motor {
    type Output = AntiMotor;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        2        0
    fn right_anti_dual(self) -> Self::Output {
        use crate::elements::*;
        return AntiMotor::from_groups(
            // e23, e31, e12, scalar
            Simd32x4::from([self[e415], self[e425], self[e435], self[e12345] * -1.0]),
            // e15, e25, e35, e3215
            Simd32x4::from([self[e235], self[e315], self[e125], self[e5] * -1.0]),
        );
    }
}
impl std::ops::Div<right_anti_dual> for MultiVector {
    type Output = MultiVector;
    fn div(self, _rhs: right_anti_dual) -> Self::Output {
        self.right_anti_dual()
    }
}
impl std::ops::DivAssign<right_anti_dual> for MultiVector {
    fn div_assign(&mut self, _rhs: right_anti_dual) {
        *self = self.right_anti_dual()
    }
}
impl RightAntiDual for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0       10        0
    //    simd3        0        2        0
    // Totals...
    // yes simd        0       12        0
    //  no simd        0       16        0
    fn right_anti_dual(self) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self[e12345] * -1.0, self[scalar]]),
            // e1, e2, e3, e4
            Simd32x4::from([self[e4235] * -1.0, self[e4315] * -1.0, self[e4125] * -1.0, self[e1234]]),
            // e5
            self[e3215],
            // e15, e25, e35, e45
            crate::swizzle!(self.group8(), 0, 1, 2).extend_to_4((self[e321] * -1.0)),
            // e41, e42, e43
            self.group7(),
            // e23, e31, e12
            Simd32x3::from([self[e415], self[e425], self[e435]]),
            // e415, e425, e435, e321
            Simd32x4::from([self[e23] * -1.0, self[e31] * -1.0, self[e12] * -1.0, self[e45]]),
            // e423, e431, e412
            Simd32x3::from([self[e41], self[e42], self[e43]]) * Simd32x3::from(-1.0),
            // e235, e315, e125
            Simd32x3::from([self[e15], self[e25], self[e35]]) * Simd32x3::from(-1.0),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([self[e1], self[e2], self[e3], self[e5] * -1.0]),
            // e1234
            self[e4] * -1.0,
        );
    }
}
impl std::ops::Div<right_anti_dual> for Plane {
    type Output = AntiPlane;
    fn div(self, _rhs: right_anti_dual) -> Self::Output {
        self.right_anti_dual()
    }
}
impl RightAntiDual for Plane {
    type Output = AntiPlane;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        3        0
    fn right_anti_dual(self) -> Self::Output {
        use crate::elements::*;
        return AntiPlane::from_groups(/* e1, e2, e3, e5 */ Simd32x4::from([self[e4235] * -1.0, self[e4315] * -1.0, self[e4125] * -1.0, self[e3215]]));
    }
}
impl std::ops::Div<right_anti_dual> for RoundPoint {
    type Output = Sphere;
    fn div(self, _rhs: right_anti_dual) -> Self::Output {
        self.right_anti_dual()
    }
}
impl RightAntiDual for RoundPoint {
    type Output = Sphere;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        2        0
    fn right_anti_dual(self) -> Self::Output {
        use crate::elements::*;
        return Sphere::from_groups(
            // e4235, e4315, e4125, e3215
            Simd32x4::from([self[e1], self[e2], self[e3], self[e5] * -1.0]),
            // e1234
            self[e4] * -1.0,
        );
    }
}
impl std::ops::Div<right_anti_dual> for Scalar {
    type Output = Scalar;
    fn div(self, _rhs: right_anti_dual) -> Self::Output {
        self.right_anti_dual()
    }
}
impl std::ops::DivAssign<right_anti_dual> for Scalar {
    fn div_assign(&mut self, _rhs: right_anti_dual) {
        *self = self.right_anti_dual()
    }
}
impl RightAntiDual for Scalar {
    type Output = Scalar;
    fn right_anti_dual(self) -> Self::Output {
        return self;
    }
}
impl std::ops::Div<right_anti_dual> for Sphere {
    type Output = RoundPoint;
    fn div(self, _rhs: right_anti_dual) -> Self::Output {
        self.right_anti_dual()
    }
}
impl RightAntiDual for Sphere {
    type Output = RoundPoint;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        3        0
    fn right_anti_dual(self) -> Self::Output {
        use crate::elements::*;
        return RoundPoint::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from([self[e4235] * -1.0, self[e4315] * -1.0, self[e4125] * -1.0, self[e1234]]),
            // e5
            self[e3215],
        );
    }
}
impl std::ops::Div<right_anti_dual> for VersorEven {
    type Output = VersorOdd;
    fn div(self, _rhs: right_anti_dual) -> Self::Output {
        self.right_anti_dual()
    }
}
impl RightAntiDual for VersorEven {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        4        0
    fn right_anti_dual(self) -> Self::Output {
        use crate::elements::*;
        return VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([self[e423], self[e431], self[e412], self[e12345] * -1.0]),
            // e23, e31, e12, e45
            Simd32x4::from([self[e415], self[e425], self[e435], self[e321] * -1.0]),
            // e15, e25, e35, e1234
            Simd32x4::from([self[e235], self[e315], self[e125], self[e4] * -1.0]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([self[e1], self[e2], self[e3], self[e5] * -1.0]),
        );
    }
}
impl std::ops::Div<right_anti_dual> for VersorOdd {
    type Output = VersorEven;
    fn div(self, _rhs: right_anti_dual) -> Self::Output {
        self.right_anti_dual()
    }
}
impl RightAntiDual for VersorOdd {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0       12        0
    fn right_anti_dual(self) -> Self::Output {
        use crate::elements::*;
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([self[e41] * -1.0, self[e42] * -1.0, self[e43] * -1.0, self[scalar]]),
            // e415, e425, e435, e321
            Simd32x4::from([self[e23] * -1.0, self[e31] * -1.0, self[e12] * -1.0, self[e45]]),
            // e235, e315, e125, e5
            Simd32x4::from([self[e15] * -1.0, self[e25] * -1.0, self[e35] * -1.0, self[e3215]]),
            // e1, e2, e3, e4
            Simd32x4::from([self[e4235] * -1.0, self[e4315] * -1.0, self[e4125] * -1.0, self[e1234]]),
        );
    }
}
