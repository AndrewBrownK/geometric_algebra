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
//  Average:         0       2       0
//  Maximum:         0       7       0
//
//  No SIMD:   add/sub     mul     div
//  Minimum:         0       0       0
//   Median:         0       6       0
//  Average:         0       5       0
//  Maximum:         0      20       0
impl std::ops::Div<anti_reverse> for AntiCircleRotor {
    type Output = AntiCircleRotor;
    fn div(self, _rhs: anti_reverse) -> Self::Output {
        self.anti_reverse()
    }
}
impl std::ops::DivAssign<anti_reverse> for AntiCircleRotor {
    fn div_assign(&mut self, _rhs: anti_reverse) {
        *self = self.anti_reverse()
    }
}
impl AntiReverse for AntiCircleRotor {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        3        0
    //    simd3        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        5        0
    //  no simd        0       10        0
    fn anti_reverse(self) -> Self {
        use crate::elements::*;
        return AntiCircleRotor::from_groups(
            // e41, e42, e43
            Simd32x3::from([self[e41], self[e42], self[e43]]) * Simd32x3::from(-1.0),
            // e23, e31, e12, e45
            Simd32x4::from([self[e23], self[e31], self[e12], self[e45]]) * Simd32x4::from(-1.0),
            // e15, e25, e35, scalar
            Simd32x4::from([self[e15] * -1.0, self[e25] * -1.0, self[e35] * -1.0, self[scalar]]),
        );
    }
}
impl std::ops::Div<anti_reverse> for AntiDipoleInversion {
    type Output = AntiDipoleInversion;
    fn div(self, _rhs: anti_reverse) -> Self::Output {
        self.anti_reverse()
    }
}
impl std::ops::DivAssign<anti_reverse> for AntiDipoleInversion {
    fn div_assign(&mut self, _rhs: anti_reverse) {
        *self = self.anti_reverse()
    }
}
impl AntiReverse for AntiDipoleInversion {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        3        0
    //    simd3        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        5        0
    //  no simd        0       10        0
    fn anti_reverse(self) -> Self {
        use crate::elements::*;
        return AntiDipoleInversion::from_groups(
            // e423, e431, e412
            Simd32x3::from([self[e423], self[e431], self[e412]]) * Simd32x3::from(-1.0),
            // e415, e425, e435, e321
            Simd32x4::from([self[e415], self[e425], self[e435], self[e321]]) * Simd32x4::from(-1.0),
            // e235, e315, e125, e4
            Simd32x4::from([self[e235] * -1.0, self[e315] * -1.0, self[e125] * -1.0, self[e4]]),
            // e1, e2, e3, e5
            self.group3(),
        );
    }
}
impl std::ops::Div<anti_reverse> for AntiDualNum {
    type Output = AntiDualNum;
    fn div(self, _rhs: anti_reverse) -> Self::Output {
        self.anti_reverse()
    }
}
impl std::ops::DivAssign<anti_reverse> for AntiDualNum {
    fn div_assign(&mut self, _rhs: anti_reverse) {
        *self = self.anti_reverse()
    }
}
impl AntiReverse for AntiDualNum {
    fn anti_reverse(self) -> Self {
        return self;
    }
}
impl std::ops::Div<anti_reverse> for AntiFlatPoint {
    type Output = AntiFlatPoint;
    fn div(self, _rhs: anti_reverse) -> Self::Output {
        self.anti_reverse()
    }
}
impl std::ops::DivAssign<anti_reverse> for AntiFlatPoint {
    fn div_assign(&mut self, _rhs: anti_reverse) {
        *self = self.anti_reverse()
    }
}
impl AntiReverse for AntiFlatPoint {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn anti_reverse(self) -> Self {
        use crate::elements::*;
        return AntiFlatPoint::from_groups(
            // e235, e315, e125, e321
            Simd32x4::from([self[e235], self[e315], self[e125], self[e321]]) * Simd32x4::from(-1.0),
        );
    }
}
impl std::ops::Div<anti_reverse> for AntiFlector {
    type Output = AntiFlector;
    fn div(self, _rhs: anti_reverse) -> Self::Output {
        self.anti_reverse()
    }
}
impl std::ops::DivAssign<anti_reverse> for AntiFlector {
    fn div_assign(&mut self, _rhs: anti_reverse) {
        *self = self.anti_reverse()
    }
}
impl AntiReverse for AntiFlector {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn anti_reverse(self) -> Self {
        use crate::elements::*;
        return AntiFlector::from_groups(
            // e235, e315, e125, e321
            Simd32x4::from([self[e235], self[e315], self[e125], self[e321]]) * Simd32x4::from(-1.0),
            // e1, e2, e3, e5
            self.group1(),
        );
    }
}
impl std::ops::Div<anti_reverse> for AntiLine {
    type Output = AntiLine;
    fn div(self, _rhs: anti_reverse) -> Self::Output {
        self.anti_reverse()
    }
}
impl std::ops::DivAssign<anti_reverse> for AntiLine {
    fn div_assign(&mut self, _rhs: anti_reverse) {
        *self = self.anti_reverse()
    }
}
impl AntiReverse for AntiLine {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        2        0
    // no simd        0        6        0
    fn anti_reverse(self) -> Self {
        use crate::elements::*;
        return AntiLine::from_groups(
            // e23, e31, e12
            Simd32x3::from([self[e23], self[e31], self[e12]]) * Simd32x3::from(-1.0),
            // e15, e25, e35
            Simd32x3::from([self[e15], self[e25], self[e35]]) * Simd32x3::from(-1.0),
        );
    }
}
impl std::ops::Div<anti_reverse> for AntiMotor {
    type Output = AntiMotor;
    fn div(self, _rhs: anti_reverse) -> Self::Output {
        self.anti_reverse()
    }
}
impl std::ops::DivAssign<anti_reverse> for AntiMotor {
    fn div_assign(&mut self, _rhs: anti_reverse) {
        *self = self.anti_reverse()
    }
}
impl AntiReverse for AntiMotor {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        6        0
    fn anti_reverse(self) -> Self {
        use crate::elements::*;
        return AntiMotor::from_groups(
            // e23, e31, e12, scalar
            Simd32x4::from([self[e23] * -1.0, self[e31] * -1.0, self[e12] * -1.0, self[scalar]]),
            // e15, e25, e35, e3215
            Simd32x4::from([self[e15] * -1.0, self[e25] * -1.0, self[e35] * -1.0, self[e3215]]),
        );
    }
}
impl std::ops::Div<anti_reverse> for AntiPlane {
    type Output = AntiPlane;
    fn div(self, _rhs: anti_reverse) -> Self::Output {
        self.anti_reverse()
    }
}
impl std::ops::DivAssign<anti_reverse> for AntiPlane {
    fn div_assign(&mut self, _rhs: anti_reverse) {
        *self = self.anti_reverse()
    }
}
impl AntiReverse for AntiPlane {
    fn anti_reverse(self) -> Self {
        return self;
    }
}
impl std::ops::Div<anti_reverse> for AntiScalar {
    type Output = AntiScalar;
    fn div(self, _rhs: anti_reverse) -> Self::Output {
        self.anti_reverse()
    }
}
impl std::ops::DivAssign<anti_reverse> for AntiScalar {
    fn div_assign(&mut self, _rhs: anti_reverse) {
        *self = self.anti_reverse()
    }
}
impl AntiReverse for AntiScalar {
    fn anti_reverse(self) -> Self {
        return self;
    }
}
impl std::ops::Div<anti_reverse> for Circle {
    type Output = Circle;
    fn div(self, _rhs: anti_reverse) -> Self::Output {
        self.anti_reverse()
    }
}
impl std::ops::DivAssign<anti_reverse> for Circle {
    fn div_assign(&mut self, _rhs: anti_reverse) {
        *self = self.anti_reverse()
    }
}
impl AntiReverse for Circle {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        0        2        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        3        0
    //  no simd        0       10        0
    fn anti_reverse(self) -> Self {
        use crate::elements::*;
        return Circle::from_groups(
            // e423, e431, e412
            Simd32x3::from([self[e423], self[e431], self[e412]]) * Simd32x3::from(-1.0),
            // e415, e425, e435, e321
            Simd32x4::from([self[e415], self[e425], self[e435], self[e321]]) * Simd32x4::from(-1.0),
            // e235, e315, e125
            Simd32x3::from([self[e235], self[e315], self[e125]]) * Simd32x3::from(-1.0),
        );
    }
}
impl std::ops::Div<anti_reverse> for CircleRotor {
    type Output = CircleRotor;
    fn div(self, _rhs: anti_reverse) -> Self::Output {
        self.anti_reverse()
    }
}
impl std::ops::DivAssign<anti_reverse> for CircleRotor {
    fn div_assign(&mut self, _rhs: anti_reverse) {
        *self = self.anti_reverse()
    }
}
impl AntiReverse for CircleRotor {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        3        0
    //    simd3        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        5        0
    //  no simd        0       10        0
    fn anti_reverse(self) -> Self {
        use crate::elements::*;
        return CircleRotor::from_groups(
            // e423, e431, e412
            Simd32x3::from([self[e423], self[e431], self[e412]]) * Simd32x3::from(-1.0),
            // e415, e425, e435, e321
            Simd32x4::from([self[e415], self[e425], self[e435], self[e321]]) * Simd32x4::from(-1.0),
            // e235, e315, e125, e12345
            Simd32x4::from([self[e235] * -1.0, self[e315] * -1.0, self[e125] * -1.0, self[e12345]]),
        );
    }
}
impl std::ops::Div<anti_reverse> for Dipole {
    type Output = Dipole;
    fn div(self, _rhs: anti_reverse) -> Self::Output {
        self.anti_reverse()
    }
}
impl std::ops::DivAssign<anti_reverse> for Dipole {
    fn div_assign(&mut self, _rhs: anti_reverse) {
        *self = self.anti_reverse()
    }
}
impl AntiReverse for Dipole {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        0        2        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        3        0
    //  no simd        0       10        0
    fn anti_reverse(self) -> Self {
        use crate::elements::*;
        return Dipole::from_groups(
            // e41, e42, e43
            Simd32x3::from([self[e41], self[e42], self[e43]]) * Simd32x3::from(-1.0),
            // e23, e31, e12, e45
            Simd32x4::from([self[e23], self[e31], self[e12], self[e45]]) * Simd32x4::from(-1.0),
            // e15, e25, e35
            Simd32x3::from([self[e15], self[e25], self[e35]]) * Simd32x3::from(-1.0),
        );
    }
}
impl std::ops::Div<anti_reverse> for DipoleInversion {
    type Output = DipoleInversion;
    fn div(self, _rhs: anti_reverse) -> Self::Output {
        self.anti_reverse()
    }
}
impl std::ops::DivAssign<anti_reverse> for DipoleInversion {
    fn div_assign(&mut self, _rhs: anti_reverse) {
        *self = self.anti_reverse()
    }
}
impl AntiReverse for DipoleInversion {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        3        0
    //    simd3        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        5        0
    //  no simd        0       10        0
    fn anti_reverse(self) -> Self {
        use crate::elements::*;
        return DipoleInversion::from_groups(
            // e41, e42, e43
            Simd32x3::from([self[e41], self[e42], self[e43]]) * Simd32x3::from(-1.0),
            // e23, e31, e12, e45
            Simd32x4::from([self[e23], self[e31], self[e12], self[e45]]) * Simd32x4::from(-1.0),
            // e15, e25, e35, e1234
            Simd32x4::from([self[e15] * -1.0, self[e25] * -1.0, self[e35] * -1.0, self[e1234]]),
            // e4235, e4315, e4125, e3215
            self.group3(),
        );
    }
}
impl std::ops::Div<anti_reverse> for DualNum {
    type Output = DualNum;
    fn div(self, _rhs: anti_reverse) -> Self::Output {
        self.anti_reverse()
    }
}
impl std::ops::DivAssign<anti_reverse> for DualNum {
    fn div_assign(&mut self, _rhs: anti_reverse) {
        *self = self.anti_reverse()
    }
}
impl AntiReverse for DualNum {
    fn anti_reverse(self) -> Self {
        return self;
    }
}
impl std::ops::Div<anti_reverse> for FlatPoint {
    type Output = FlatPoint;
    fn div(self, _rhs: anti_reverse) -> Self::Output {
        self.anti_reverse()
    }
}
impl std::ops::DivAssign<anti_reverse> for FlatPoint {
    fn div_assign(&mut self, _rhs: anti_reverse) {
        *self = self.anti_reverse()
    }
}
impl AntiReverse for FlatPoint {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn anti_reverse(self) -> Self {
        use crate::elements::*;
        return FlatPoint::from_groups(/* e15, e25, e35, e45 */ Simd32x4::from([self[e15], self[e25], self[e35], self[e45]]) * Simd32x4::from(-1.0));
    }
}
impl std::ops::Div<anti_reverse> for Flector {
    type Output = Flector;
    fn div(self, _rhs: anti_reverse) -> Self::Output {
        self.anti_reverse()
    }
}
impl std::ops::DivAssign<anti_reverse> for Flector {
    fn div_assign(&mut self, _rhs: anti_reverse) {
        *self = self.anti_reverse()
    }
}
impl AntiReverse for Flector {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn anti_reverse(self) -> Self {
        use crate::elements::*;
        return Flector::from_groups(
            // e15, e25, e35, e45
            Simd32x4::from([self[e15], self[e25], self[e35], self[e45]]) * Simd32x4::from(-1.0),
            // e4235, e4315, e4125, e3215
            self.group1(),
        );
    }
}
impl std::ops::Div<anti_reverse> for Line {
    type Output = Line;
    fn div(self, _rhs: anti_reverse) -> Self::Output {
        self.anti_reverse()
    }
}
impl std::ops::DivAssign<anti_reverse> for Line {
    fn div_assign(&mut self, _rhs: anti_reverse) {
        *self = self.anti_reverse()
    }
}
impl AntiReverse for Line {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        2        0
    // no simd        0        6        0
    fn anti_reverse(self) -> Self {
        use crate::elements::*;
        return Line::from_groups(
            // e415, e425, e435
            Simd32x3::from([self[e415], self[e425], self[e435]]) * Simd32x3::from(-1.0),
            // e235, e315, e125
            Simd32x3::from([self[e235], self[e315], self[e125]]) * Simd32x3::from(-1.0),
        );
    }
}
impl std::ops::Div<anti_reverse> for Motor {
    type Output = Motor;
    fn div(self, _rhs: anti_reverse) -> Self::Output {
        self.anti_reverse()
    }
}
impl std::ops::DivAssign<anti_reverse> for Motor {
    fn div_assign(&mut self, _rhs: anti_reverse) {
        *self = self.anti_reverse()
    }
}
impl AntiReverse for Motor {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        6        0
    fn anti_reverse(self) -> Self {
        use crate::elements::*;
        return Motor::from_groups(
            // e415, e425, e435, e12345
            Simd32x4::from([self[e415] * -1.0, self[e425] * -1.0, self[e435] * -1.0, self[e12345]]),
            // e235, e315, e125, e5
            Simd32x4::from([self[e235] * -1.0, self[e315] * -1.0, self[e125] * -1.0, self[e5]]),
        );
    }
}
impl std::ops::Div<anti_reverse> for MultiVector {
    type Output = MultiVector;
    fn div(self, _rhs: anti_reverse) -> Self::Output {
        self.anti_reverse()
    }
}
impl std::ops::DivAssign<anti_reverse> for MultiVector {
    fn div_assign(&mut self, _rhs: anti_reverse) {
        *self = self.anti_reverse()
    }
}
impl AntiReverse for MultiVector {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        0        4        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        0        6        0
    //  no simd        0       20        0
    fn anti_reverse(self) -> Self {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e15, e25, e35, e45
            Simd32x4::from([self[e15], self[e25], self[e35], self[e45]]) * Simd32x4::from(-1.0),
            // e41, e42, e43
            Simd32x3::from([self[e41], self[e42], self[e43]]) * Simd32x3::from(-1.0),
            // e23, e31, e12
            Simd32x3::from([self[e23], self[e31], self[e12]]) * Simd32x3::from(-1.0),
            // e415, e425, e435, e321
            Simd32x4::from([self[e415], self[e425], self[e435], self[e321]]) * Simd32x4::from(-1.0),
            // e423, e431, e412
            Simd32x3::from([self[e423], self[e431], self[e412]]) * Simd32x3::from(-1.0),
            // e235, e315, e125
            Simd32x3::from([self[e235], self[e315], self[e125]]) * Simd32x3::from(-1.0),
            // e4235, e4315, e4125, e3215
            self.group9(),
            // e1234
            self[e1234],
        );
    }
}
impl std::ops::Div<anti_reverse> for Plane {
    type Output = Plane;
    fn div(self, _rhs: anti_reverse) -> Self::Output {
        self.anti_reverse()
    }
}
impl std::ops::DivAssign<anti_reverse> for Plane {
    fn div_assign(&mut self, _rhs: anti_reverse) {
        *self = self.anti_reverse()
    }
}
impl AntiReverse for Plane {
    fn anti_reverse(self) -> Self {
        return self;
    }
}
impl std::ops::Div<anti_reverse> for RoundPoint {
    type Output = RoundPoint;
    fn div(self, _rhs: anti_reverse) -> Self::Output {
        self.anti_reverse()
    }
}
impl std::ops::DivAssign<anti_reverse> for RoundPoint {
    fn div_assign(&mut self, _rhs: anti_reverse) {
        *self = self.anti_reverse()
    }
}
impl AntiReverse for RoundPoint {
    fn anti_reverse(self) -> Self {
        return self;
    }
}
impl std::ops::Div<anti_reverse> for Scalar {
    type Output = Scalar;
    fn div(self, _rhs: anti_reverse) -> Self::Output {
        self.anti_reverse()
    }
}
impl std::ops::DivAssign<anti_reverse> for Scalar {
    fn div_assign(&mut self, _rhs: anti_reverse) {
        *self = self.anti_reverse()
    }
}
impl AntiReverse for Scalar {
    fn anti_reverse(self) -> Self {
        return self;
    }
}
impl std::ops::Div<anti_reverse> for Sphere {
    type Output = Sphere;
    fn div(self, _rhs: anti_reverse) -> Self::Output {
        self.anti_reverse()
    }
}
impl std::ops::DivAssign<anti_reverse> for Sphere {
    fn div_assign(&mut self, _rhs: anti_reverse) {
        *self = self.anti_reverse()
    }
}
impl AntiReverse for Sphere {
    fn anti_reverse(self) -> Self {
        return self;
    }
}
impl std::ops::Div<anti_reverse> for VersorEven {
    type Output = VersorEven;
    fn div(self, _rhs: anti_reverse) -> Self::Output {
        self.anti_reverse()
    }
}
impl std::ops::DivAssign<anti_reverse> for VersorEven {
    fn div_assign(&mut self, _rhs: anti_reverse) {
        *self = self.anti_reverse()
    }
}
impl AntiReverse for VersorEven {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        6        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        7        0
    //  no simd        0       10        0
    fn anti_reverse(self) -> Self {
        use crate::elements::*;
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([self[e423] * -1.0, self[e431] * -1.0, self[e412] * -1.0, self[e12345]]),
            // e415, e425, e435, e321
            Simd32x4::from([self[e415], self[e425], self[e435], self[e321]]) * Simd32x4::from(-1.0),
            // e235, e315, e125, e5
            Simd32x4::from([self[e235] * -1.0, self[e315] * -1.0, self[e125] * -1.0, self[e5]]),
            // e1, e2, e3, e4
            self.group3(),
        );
    }
}
impl std::ops::Div<anti_reverse> for VersorOdd {
    type Output = VersorOdd;
    fn div(self, _rhs: anti_reverse) -> Self::Output {
        self.anti_reverse()
    }
}
impl std::ops::DivAssign<anti_reverse> for VersorOdd {
    fn div_assign(&mut self, _rhs: anti_reverse) {
        *self = self.anti_reverse()
    }
}
impl AntiReverse for VersorOdd {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        6        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        7        0
    //  no simd        0       10        0
    fn anti_reverse(self) -> Self {
        use crate::elements::*;
        return VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([self[e41] * -1.0, self[e42] * -1.0, self[e43] * -1.0, self[scalar]]),
            // e23, e31, e12, e45
            Simd32x4::from([self[e23], self[e31], self[e12], self[e45]]) * Simd32x4::from(-1.0),
            // e15, e25, e35, e1234
            Simd32x4::from([self[e15] * -1.0, self[e25] * -1.0, self[e35] * -1.0, self[e1234]]),
            // e4235, e4315, e4125, e3215
            self.group3(),
        );
    }
}
