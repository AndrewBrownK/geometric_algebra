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
impl std::ops::Div<right_dual> for AntiCircleRotor {
    type Output = CircleRotor;
    fn div(self, _rhs: right_dual) -> Self::Output {
        self.right_dual()
    }
}
impl RightDual for AntiCircleRotor {
    type Output = CircleRotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        6        0
    //    simd3        0        1        0
    // Totals...
    // yes simd        0        7        0
    //  no simd        0        9        0
    fn right_dual(self) -> Self::Output {
        return CircleRotor::from_groups(
            // e423, e431, e412
            self.group0() * Simd32x3::from(-1.0),
            // e415, e425, e435, e321
            Simd32x4::from([self.group1()[0] * -1.0, self.group1()[1] * -1.0, self.group1()[2] * -1.0, self.group1()[3]]),
            // e235, e315, e125, e12345
            Simd32x4::from([self.group2()[0] * -1.0, self.group2()[1] * -1.0, self.group2()[2] * -1.0, self.group2()[3]]),
        );
    }
}
impl std::ops::Div<right_dual> for AntiDipoleInversion {
    type Output = DipoleInversion;
    fn div(self, _rhs: right_dual) -> Self::Output {
        self.right_dual()
    }
}
impl RightDual for AntiDipoleInversion {
    type Output = DipoleInversion;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        3        0
    fn right_dual(self) -> Self::Output {
        return DipoleInversion::from_groups(
            // e41, e42, e43
            self.group0(),
            // e23, e31, e12, e45
            Simd32x4::from([self.group1()[0], self.group1()[1], self.group1()[2], self.group1()[3] * -1.0]),
            // e15, e25, e35, e1234
            Simd32x4::from([self.group2()[0], self.group2()[1], self.group2()[2], self.group2()[3] * -1.0]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([self.group3()[0], self.group3()[1], self.group3()[2], self.group3()[3] * -1.0]),
        );
    }
}
impl std::ops::Div<right_dual> for AntiDualNum {
    type Output = AntiDualNum;
    fn div(self, _rhs: right_dual) -> Self::Output {
        self.right_dual()
    }
}
impl std::ops::DivAssign<right_dual> for AntiDualNum {
    fn div_assign(&mut self, _rhs: right_dual) {
        *self = self.right_dual()
    }
}
impl RightDual for AntiDualNum {
    type Output = AntiDualNum;
    fn right_dual(self) -> Self::Output {
        return self;
    }
}
impl std::ops::Div<right_dual> for AntiFlatPoint {
    type Output = FlatPoint;
    fn div(self, _rhs: right_dual) -> Self::Output {
        self.right_dual()
    }
}
impl RightDual for AntiFlatPoint {
    type Output = FlatPoint;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn right_dual(self) -> Self::Output {
        return FlatPoint::from_groups(
            // e15, e25, e35, e45
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], self.group0()[3] * -1.0]),
        );
    }
}
impl std::ops::Div<right_dual> for AntiFlector {
    type Output = Flector;
    fn div(self, _rhs: right_dual) -> Self::Output {
        self.right_dual()
    }
}
impl RightDual for AntiFlector {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        2        0
    fn right_dual(self) -> Self::Output {
        return Flector::from_groups(
            // e15, e25, e35, e45
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], self.group0()[3] * -1.0]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([self.group1()[0], self.group1()[1], self.group1()[2], self.group1()[3] * -1.0]),
        );
    }
}
impl std::ops::Div<right_dual> for AntiLine {
    type Output = Line;
    fn div(self, _rhs: right_dual) -> Self::Output {
        self.right_dual()
    }
}
impl RightDual for AntiLine {
    type Output = Line;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        2        0
    // no simd        0        6        0
    fn right_dual(self) -> Self::Output {
        return Line::from_groups(
            // e415, e425, e435
            self.group0() * Simd32x3::from(-1.0),
            // e235, e315, e125
            self.group1() * Simd32x3::from(-1.0),
        );
    }
}
impl std::ops::Div<right_dual> for AntiMotor {
    type Output = Motor;
    fn div(self, _rhs: right_dual) -> Self::Output {
        self.right_dual()
    }
}
impl RightDual for AntiMotor {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        6        0
    fn right_dual(self) -> Self::Output {
        return Motor::from_groups(
            // e415, e425, e435, e12345
            Simd32x4::from([self.group0()[0] * -1.0, self.group0()[1] * -1.0, self.group0()[2] * -1.0, self.group0()[3]]),
            // e235, e315, e125, e5
            Simd32x4::from([self.group1()[0] * -1.0, self.group1()[1] * -1.0, self.group1()[2] * -1.0, self.group1()[3]]),
        );
    }
}
impl std::ops::Div<right_dual> for AntiPlane {
    type Output = Plane;
    fn div(self, _rhs: right_dual) -> Self::Output {
        self.right_dual()
    }
}
impl RightDual for AntiPlane {
    type Output = Plane;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn right_dual(self) -> Self::Output {
        return Plane::from_groups(
            // e4235, e4315, e4125, e3215
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], self.group0()[3] * -1.0]),
        );
    }
}
impl std::ops::Div<right_dual> for AntiScalar {
    type Output = Scalar;
    fn div(self, _rhs: right_dual) -> Self::Output {
        self.right_dual()
    }
}
impl RightDual for AntiScalar {
    type Output = Scalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn right_dual(self) -> Self::Output {
        use crate::elements::*;
        return Scalar::from_groups(/* scalar */ self[e12345] * -1.0);
    }
}
impl std::ops::Div<right_dual> for Circle {
    type Output = Dipole;
    fn div(self, _rhs: right_dual) -> Self::Output {
        self.right_dual()
    }
}
impl RightDual for Circle {
    type Output = Dipole;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn right_dual(self) -> Self::Output {
        return Dipole::from_groups(
            // e41, e42, e43
            self.group0(),
            // e23, e31, e12, e45
            Simd32x4::from([self.group1()[0], self.group1()[1], self.group1()[2], self.group1()[3] * -1.0]),
            // e15, e25, e35
            self.group2(),
        );
    }
}
impl std::ops::Div<right_dual> for CircleRotor {
    type Output = AntiCircleRotor;
    fn div(self, _rhs: right_dual) -> Self::Output {
        self.right_dual()
    }
}
impl RightDual for CircleRotor {
    type Output = AntiCircleRotor;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        2        0
    fn right_dual(self) -> Self::Output {
        return AntiCircleRotor::from_groups(
            // e41, e42, e43
            self.group0(),
            // e23, e31, e12, e45
            Simd32x4::from([self.group1()[0], self.group1()[1], self.group1()[2], self.group1()[3] * -1.0]),
            // e15, e25, e35, scalar
            Simd32x4::from([self.group2()[0], self.group2()[1], self.group2()[2], self.group2()[3] * -1.0]),
        );
    }
}
impl std::ops::Div<right_dual> for Dipole {
    type Output = Circle;
    fn div(self, _rhs: right_dual) -> Self::Output {
        self.right_dual()
    }
}
impl RightDual for Dipole {
    type Output = Circle;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        3        0
    //    simd3        0        2        0
    // Totals...
    // yes simd        0        5        0
    //  no simd        0        9        0
    fn right_dual(self) -> Self::Output {
        return Circle::from_groups(
            // e423, e431, e412
            self.group0() * Simd32x3::from(-1.0),
            // e415, e425, e435, e321
            Simd32x4::from([self.group1()[0] * -1.0, self.group1()[1] * -1.0, self.group1()[2] * -1.0, self.group1()[3]]),
            // e235, e315, e125
            self.group2() * Simd32x3::from(-1.0),
        );
    }
}
impl std::ops::Div<right_dual> for DipoleInversion {
    type Output = AntiDipoleInversion;
    fn div(self, _rhs: right_dual) -> Self::Output {
        self.right_dual()
    }
}
impl RightDual for DipoleInversion {
    type Output = AntiDipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        9        0
    //    simd3        0        1        0
    // Totals...
    // yes simd        0       10        0
    //  no simd        0       12        0
    fn right_dual(self) -> Self::Output {
        return AntiDipoleInversion::from_groups(
            // e423, e431, e412
            self.group0() * Simd32x3::from(-1.0),
            // e415, e425, e435, e321
            Simd32x4::from([self.group1()[0] * -1.0, self.group1()[1] * -1.0, self.group1()[2] * -1.0, self.group1()[3]]),
            // e235, e315, e125, e4
            Simd32x4::from([self.group2()[0] * -1.0, self.group2()[1] * -1.0, self.group2()[2] * -1.0, self.group2()[3]]),
            // e1, e2, e3, e5
            Simd32x4::from([self.group3()[0] * -1.0, self.group3()[1] * -1.0, self.group3()[2] * -1.0, self.group3()[3]]),
        );
    }
}
impl std::ops::Div<right_dual> for DualNum {
    type Output = AntiDualNum;
    fn div(self, _rhs: right_dual) -> Self::Output {
        self.right_dual()
    }
}
impl RightDual for DualNum {
    type Output = AntiDualNum;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd2        0        1        0
    // no simd        0        2        0
    fn right_dual(self) -> Self::Output {
        return AntiDualNum::from_groups(/* e1234, scalar */ self.group0() * Simd32x2::from(-1.0));
    }
}
impl std::ops::Div<right_dual> for FlatPoint {
    type Output = AntiFlatPoint;
    fn div(self, _rhs: right_dual) -> Self::Output {
        self.right_dual()
    }
}
impl RightDual for FlatPoint {
    type Output = AntiFlatPoint;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        3        0
    fn right_dual(self) -> Self::Output {
        return AntiFlatPoint::from_groups(
            // e235, e315, e125, e321
            Simd32x4::from([self.group0()[0] * -1.0, self.group0()[1] * -1.0, self.group0()[2] * -1.0, self.group0()[3]]),
        );
    }
}
impl std::ops::Div<right_dual> for Flector {
    type Output = AntiFlector;
    fn div(self, _rhs: right_dual) -> Self::Output {
        self.right_dual()
    }
}
impl RightDual for Flector {
    type Output = AntiFlector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        6        0
    fn right_dual(self) -> Self::Output {
        return AntiFlector::from_groups(
            // e235, e315, e125, e321
            Simd32x4::from([self.group0()[0] * -1.0, self.group0()[1] * -1.0, self.group0()[2] * -1.0, self.group0()[3]]),
            // e1, e2, e3, e5
            Simd32x4::from([self.group1()[0] * -1.0, self.group1()[1] * -1.0, self.group1()[2] * -1.0, self.group1()[3]]),
        );
    }
}
impl std::ops::Div<right_dual> for Line {
    type Output = Line;
    fn div(self, _rhs: right_dual) -> Self::Output {
        self.right_dual()
    }
}
impl std::ops::DivAssign<right_dual> for Line {
    fn div_assign(&mut self, _rhs: right_dual) {
        *self = self.right_dual()
    }
}
impl RightDual for Line {
    type Output = Line;
    fn right_dual(self) -> Self::Output {
        return self;
    }
}
impl std::ops::Div<right_dual> for Motor {
    type Output = AntiMotor;
    fn div(self, _rhs: right_dual) -> Self::Output {
        self.right_dual()
    }
}
impl RightDual for Motor {
    type Output = AntiMotor;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        2        0
    fn right_dual(self) -> Self::Output {
        return AntiMotor::from_groups(
            // e23, e31, e12, scalar
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], self.group0()[3] * -1.0]),
            // e15, e25, e35, e3215
            Simd32x4::from([self.group1()[0], self.group1()[1], self.group1()[2], self.group1()[3] * -1.0]),
        );
    }
}
impl std::ops::Div<right_dual> for MultiVector {
    type Output = MultiVector;
    fn div(self, _rhs: right_dual) -> Self::Output {
        self.right_dual()
    }
}
impl std::ops::DivAssign<right_dual> for MultiVector {
    fn div_assign(&mut self, _rhs: right_dual) {
        *self = self.right_dual()
    }
}
impl RightDual for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0       10        0
    //    simd3        0        2        0
    // Totals...
    // yes simd        0       12        0
    //  no simd        0       16        0
    fn right_dual(self) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self.group0()[1] * -1.0, self.group0()[0]]),
            // e1, e2, e3, e4
            Simd32x4::from([self.group9()[0] * -1.0, self.group9()[1] * -1.0, self.group9()[2] * -1.0, self[e45]]),
            // e5
            self.group9()[3],
            // e15, e25, e35, e45
            Simd32x4::from([self.group8()[0], self.group8()[1], self.group8()[2], self.group6()[3] * -1.0]),
            // e41, e42, e43
            self.group7(),
            // e23, e31, e12
            Simd32x3::from([self.group6()[0], self.group6()[1], self.group6()[2]]),
            // e415, e425, e435, e321
            Simd32x4::from([self.group5()[0] * -1.0, self.group5()[1] * -1.0, self.group5()[2] * -1.0, self.group3()[3]]),
            // e423, e431, e412
            self.group4() * Simd32x3::from(-1.0),
            // e235, e315, e125
            Simd32x3::from([self.group3()[0], self.group3()[1], self.group3()[2]]) * Simd32x3::from(-1.0),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([self.group1()[0], self.group1()[1], self.group1()[2], self[e1] * -1.0]),
            // e1234
            self.group1()[3] * -1.0,
        );
    }
}
impl std::ops::Div<right_dual> for Plane {
    type Output = AntiPlane;
    fn div(self, _rhs: right_dual) -> Self::Output {
        self.right_dual()
    }
}
impl RightDual for Plane {
    type Output = AntiPlane;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        3        0
    fn right_dual(self) -> Self::Output {
        return AntiPlane::from_groups(
            // e1, e2, e3, e5
            Simd32x4::from([self.group0()[0] * -1.0, self.group0()[1] * -1.0, self.group0()[2] * -1.0, self.group0()[3]]),
        );
    }
}
impl std::ops::Div<right_dual> for RoundPoint {
    type Output = Sphere;
    fn div(self, _rhs: right_dual) -> Self::Output {
        self.right_dual()
    }
}
impl RightDual for RoundPoint {
    type Output = Sphere;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        2        0
    fn right_dual(self) -> Self::Output {
        use crate::elements::*;
        return Sphere::from_groups(
            // e4235, e4315, e4125, e3215
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], self[e2] * -1.0]),
            // e1234
            self.group0()[3] * -1.0,
        );
    }
}
impl std::ops::Div<right_dual> for Scalar {
    type Output = Scalar;
    fn div(self, _rhs: right_dual) -> Self::Output {
        self.right_dual()
    }
}
impl std::ops::DivAssign<right_dual> for Scalar {
    fn div_assign(&mut self, _rhs: right_dual) {
        *self = self.right_dual()
    }
}
impl RightDual for Scalar {
    type Output = Scalar;
    fn right_dual(self) -> Self::Output {
        return self;
    }
}
impl std::ops::Div<right_dual> for Sphere {
    type Output = RoundPoint;
    fn div(self, _rhs: right_dual) -> Self::Output {
        self.right_dual()
    }
}
impl RightDual for Sphere {
    type Output = RoundPoint;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        3        0
    fn right_dual(self) -> Self::Output {
        use crate::elements::*;
        return RoundPoint::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from([self.group0()[0] * -1.0, self.group0()[1] * -1.0, self.group0()[2] * -1.0, self[e4315]]),
            // e5
            self.group0()[3],
        );
    }
}
impl std::ops::Div<right_dual> for VersorEven {
    type Output = VersorOdd;
    fn div(self, _rhs: right_dual) -> Self::Output {
        self.right_dual()
    }
}
impl RightDual for VersorEven {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        4        0
    fn right_dual(self) -> Self::Output {
        return VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], self.group0()[3] * -1.0]),
            // e23, e31, e12, e45
            Simd32x4::from([self.group1()[0], self.group1()[1], self.group1()[2], self.group1()[3] * -1.0]),
            // e15, e25, e35, e1234
            Simd32x4::from([self.group2()[0], self.group2()[1], self.group2()[2], self.group3()[3] * -1.0]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([self.group3()[0], self.group3()[1], self.group3()[2], self.group2()[3] * -1.0]),
        );
    }
}
impl std::ops::Div<right_dual> for VersorOdd {
    type Output = VersorEven;
    fn div(self, _rhs: right_dual) -> Self::Output {
        self.right_dual()
    }
}
impl RightDual for VersorOdd {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0       12        0
    fn right_dual(self) -> Self::Output {
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([self.group0()[0] * -1.0, self.group0()[1] * -1.0, self.group0()[2] * -1.0, self.group0()[3]]),
            // e415, e425, e435, e321
            Simd32x4::from([self.group1()[0] * -1.0, self.group1()[1] * -1.0, self.group1()[2] * -1.0, self.group1()[3]]),
            // e235, e315, e125, e5
            Simd32x4::from([self.group2()[0] * -1.0, self.group2()[1] * -1.0, self.group2()[2] * -1.0, self.group3()[3]]),
            // e1, e2, e3, e4
            Simd32x4::from([self.group3()[0] * -1.0, self.group3()[1] * -1.0, self.group3()[2] * -1.0, self.group2()[3]]),
        );
    }
}
