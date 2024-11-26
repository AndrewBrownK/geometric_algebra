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
//   Median:         0       1       0
//  Average:         0       1       0
//  Maximum:         0       6       0
//
//  No SIMD:   add/sub     mul     div
//  Minimum:         0       0       0
//   Median:         0       4       0
//  Average:         0       5       0
//  Maximum:         0      17       0
impl std::ops::Div<conjugation> for AntiCircleRotor {
    type Output = AntiCircleRotor;
    fn div(self, _rhs: conjugation) -> Self::Output {
        self.conjugation()
    }
}
impl std::ops::DivAssign<conjugation> for AntiCircleRotor {
    fn div_assign(&mut self, _rhs: conjugation) {
        *self = self.conjugation()
    }
}
impl Conjugation for AntiCircleRotor {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        0        1        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        0        3        0
    //  no simd        0       11        0
    fn conjugation(self) -> Self {
        return AntiCircleRotor::from_groups(
            // e41, e42, e43
            self.group0() * Simd32x3::from(-1.0),
            // e23, e31, e12, e45
            self.group1() * Simd32x4::from(-1.0),
            // e15, e25, e35, scalar
            self.group2() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
        );
    }
}
impl std::ops::Div<conjugation> for AntiDipoleInversion {
    type Output = AntiDipoleInversion;
    fn div(self, _rhs: conjugation) -> Self::Output {
        self.conjugation()
    }
}
impl std::ops::DivAssign<conjugation> for AntiDipoleInversion {
    fn div_assign(&mut self, _rhs: conjugation) {
        *self = self.conjugation()
    }
}
impl Conjugation for AntiDipoleInversion {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        2        0
    // no simd        0        8        0
    fn conjugation(self) -> Self {
        return AntiDipoleInversion::from_groups(
            // e423, e431, e412
            self.group0(),
            // e415, e425, e435, e321
            self.group1(),
            // e235, e315, e125, e4
            self.group2() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e1, e2, e3, e5
            self.group3() * Simd32x4::from(-1.0),
        );
    }
}
impl std::ops::Div<conjugation> for AntiDualNum {
    type Output = AntiDualNum;
    fn div(self, _rhs: conjugation) -> Self::Output {
        self.conjugation()
    }
}
impl std::ops::DivAssign<conjugation> for AntiDualNum {
    fn div_assign(&mut self, _rhs: conjugation) {
        *self = self.conjugation()
    }
}
impl Conjugation for AntiDualNum {
    fn conjugation(self) -> Self {
        return self;
    }
}
impl std::ops::Div<conjugation> for AntiFlatPoint {
    type Output = AntiFlatPoint;
    fn div(self, _rhs: conjugation) -> Self::Output {
        self.conjugation()
    }
}
impl std::ops::DivAssign<conjugation> for AntiFlatPoint {
    fn div_assign(&mut self, _rhs: conjugation) {
        *self = self.conjugation()
    }
}
impl Conjugation for AntiFlatPoint {
    fn conjugation(self) -> Self {
        return self;
    }
}
impl std::ops::Div<conjugation> for AntiFlector {
    type Output = AntiFlector;
    fn div(self, _rhs: conjugation) -> Self::Output {
        self.conjugation()
    }
}
impl std::ops::DivAssign<conjugation> for AntiFlector {
    fn div_assign(&mut self, _rhs: conjugation) {
        *self = self.conjugation()
    }
}
impl Conjugation for AntiFlector {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn conjugation(self) -> Self {
        return AntiFlector::from_groups(/* e235, e315, e125, e321 */ self.group0(), /* e1, e2, e3, e5 */ self.group1() * Simd32x4::from(-1.0));
    }
}
impl std::ops::Div<conjugation> for AntiLine {
    type Output = AntiLine;
    fn div(self, _rhs: conjugation) -> Self::Output {
        self.conjugation()
    }
}
impl std::ops::DivAssign<conjugation> for AntiLine {
    fn div_assign(&mut self, _rhs: conjugation) {
        *self = self.conjugation()
    }
}
impl Conjugation for AntiLine {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        2        0
    // no simd        0        6        0
    fn conjugation(self) -> Self {
        return AntiLine::from_groups(
            // e23, e31, e12
            self.group0() * Simd32x3::from(-1.0),
            // e15, e25, e35
            self.group1() * Simd32x3::from(-1.0),
        );
    }
}
impl std::ops::Div<conjugation> for AntiMotor {
    type Output = AntiMotor;
    fn div(self, _rhs: conjugation) -> Self::Output {
        self.conjugation()
    }
}
impl std::ops::DivAssign<conjugation> for AntiMotor {
    fn div_assign(&mut self, _rhs: conjugation) {
        *self = self.conjugation()
    }
}
impl Conjugation for AntiMotor {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        2        0
    // no simd        0        8        0
    fn conjugation(self) -> Self {
        return AntiMotor::from_groups(
            // e23, e31, e12, scalar
            self.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e15, e25, e35, e3215
            self.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
        );
    }
}
impl std::ops::Div<conjugation> for AntiPlane {
    type Output = AntiPlane;
    fn div(self, _rhs: conjugation) -> Self::Output {
        self.conjugation()
    }
}
impl std::ops::DivAssign<conjugation> for AntiPlane {
    fn div_assign(&mut self, _rhs: conjugation) {
        *self = self.conjugation()
    }
}
impl Conjugation for AntiPlane {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn conjugation(self) -> Self {
        return AntiPlane::from_groups(/* e1, e2, e3, e5 */ self.group0() * Simd32x4::from(-1.0));
    }
}
impl std::ops::Div<conjugation> for AntiScalar {
    type Output = AntiScalar;
    fn div(self, _rhs: conjugation) -> Self::Output {
        self.conjugation()
    }
}
impl std::ops::DivAssign<conjugation> for AntiScalar {
    fn div_assign(&mut self, _rhs: conjugation) {
        *self = self.conjugation()
    }
}
impl Conjugation for AntiScalar {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn conjugation(self) -> Self {
        use crate::elements::*;
        return AntiScalar::from_groups(/* e12345 */ self[e12345] * -1.0);
    }
}
impl std::ops::Div<conjugation> for Circle {
    type Output = Circle;
    fn div(self, _rhs: conjugation) -> Self::Output {
        self.conjugation()
    }
}
impl std::ops::DivAssign<conjugation> for Circle {
    fn div_assign(&mut self, _rhs: conjugation) {
        *self = self.conjugation()
    }
}
impl Conjugation for Circle {
    fn conjugation(self) -> Self {
        return self;
    }
}
impl std::ops::Div<conjugation> for CircleRotor {
    type Output = CircleRotor;
    fn div(self, _rhs: conjugation) -> Self::Output {
        self.conjugation()
    }
}
impl std::ops::DivAssign<conjugation> for CircleRotor {
    fn div_assign(&mut self, _rhs: conjugation) {
        *self = self.conjugation()
    }
}
impl Conjugation for CircleRotor {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn conjugation(self) -> Self {
        return CircleRotor::from_groups(
            // e423, e431, e412
            self.group0(),
            // e415, e425, e435, e321
            self.group1(),
            // e235, e315, e125, e12345
            self.group2() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
        );
    }
}
impl std::ops::Div<conjugation> for Dipole {
    type Output = Dipole;
    fn div(self, _rhs: conjugation) -> Self::Output {
        self.conjugation()
    }
}
impl std::ops::DivAssign<conjugation> for Dipole {
    fn div_assign(&mut self, _rhs: conjugation) {
        *self = self.conjugation()
    }
}
impl Conjugation for Dipole {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        0        2        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        3        0
    //  no simd        0       10        0
    fn conjugation(self) -> Self {
        return Dipole::from_groups(
            // e41, e42, e43
            self.group0() * Simd32x3::from(-1.0),
            // e23, e31, e12, e45
            self.group1() * Simd32x4::from(-1.0),
            // e15, e25, e35
            self.group2() * Simd32x3::from(-1.0),
        );
    }
}
impl std::ops::Div<conjugation> for DipoleInversion {
    type Output = DipoleInversion;
    fn div(self, _rhs: conjugation) -> Self::Output {
        self.conjugation()
    }
}
impl std::ops::DivAssign<conjugation> for DipoleInversion {
    fn div_assign(&mut self, _rhs: conjugation) {
        *self = self.conjugation()
    }
}
impl Conjugation for DipoleInversion {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        0        1        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        0        3        0
    //  no simd        0       11        0
    fn conjugation(self) -> Self {
        return DipoleInversion::from_groups(
            // e41, e42, e43
            self.group0() * Simd32x3::from(-1.0),
            // e23, e31, e12, e45
            self.group1() * Simd32x4::from(-1.0),
            // e15, e25, e35, e1234
            self.group2() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e4235, e4315, e4125, e3215
            self.group3(),
        );
    }
}
impl std::ops::Div<conjugation> for DualNum {
    type Output = DualNum;
    fn div(self, _rhs: conjugation) -> Self::Output {
        self.conjugation()
    }
}
impl std::ops::DivAssign<conjugation> for DualNum {
    fn div_assign(&mut self, _rhs: conjugation) {
        *self = self.conjugation()
    }
}
impl Conjugation for DualNum {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd2        0        1        0
    // no simd        0        2        0
    fn conjugation(self) -> Self {
        return DualNum::from_groups(/* e5, e12345 */ self.group0() * Simd32x2::from(-1.0));
    }
}
impl std::ops::Div<conjugation> for FlatPoint {
    type Output = FlatPoint;
    fn div(self, _rhs: conjugation) -> Self::Output {
        self.conjugation()
    }
}
impl std::ops::DivAssign<conjugation> for FlatPoint {
    fn div_assign(&mut self, _rhs: conjugation) {
        *self = self.conjugation()
    }
}
impl Conjugation for FlatPoint {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn conjugation(self) -> Self {
        return FlatPoint::from_groups(/* e15, e25, e35, e45 */ self.group0() * Simd32x4::from(-1.0));
    }
}
impl std::ops::Div<conjugation> for Flector {
    type Output = Flector;
    fn div(self, _rhs: conjugation) -> Self::Output {
        self.conjugation()
    }
}
impl std::ops::DivAssign<conjugation> for Flector {
    fn div_assign(&mut self, _rhs: conjugation) {
        *self = self.conjugation()
    }
}
impl Conjugation for Flector {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn conjugation(self) -> Self {
        return Flector::from_groups(/* e15, e25, e35, e45 */ self.group0() * Simd32x4::from(-1.0), /* e4235, e4315, e4125, e3215 */ self.group1());
    }
}
impl std::ops::Div<conjugation> for Line {
    type Output = Line;
    fn div(self, _rhs: conjugation) -> Self::Output {
        self.conjugation()
    }
}
impl std::ops::DivAssign<conjugation> for Line {
    fn div_assign(&mut self, _rhs: conjugation) {
        *self = self.conjugation()
    }
}
impl Conjugation for Line {
    fn conjugation(self) -> Self {
        return self;
    }
}
impl std::ops::Div<conjugation> for Motor {
    type Output = Motor;
    fn div(self, _rhs: conjugation) -> Self::Output {
        self.conjugation()
    }
}
impl std::ops::DivAssign<conjugation> for Motor {
    fn div_assign(&mut self, _rhs: conjugation) {
        *self = self.conjugation()
    }
}
impl Conjugation for Motor {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        2        0
    // no simd        0        8        0
    fn conjugation(self) -> Self {
        return Motor::from_groups(
            // e415, e425, e435, e12345
            self.group0() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e235, e315, e125, e5
            self.group1() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
        );
    }
}
impl std::ops::Div<conjugation> for MultiVector {
    type Output = MultiVector;
    fn div(self, _rhs: conjugation) -> Self::Output {
        self.conjugation()
    }
}
impl std::ops::DivAssign<conjugation> for MultiVector {
    fn div_assign(&mut self, _rhs: conjugation) {
        *self = self.conjugation()
    }
}
impl Conjugation for MultiVector {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd2        0        1        0
    //    simd3        0        2        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        0        6        0
    //  no simd        0       17        0
    fn conjugation(self) -> Self {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            self.group0() * Simd32x2::from([1.0, -1.0]),
            // e1, e2, e3, e4
            self.group1() * Simd32x4::from(-1.0),
            // e5
            self[e5] * -1.0,
            // e15, e25, e35, e45
            self.group3() * Simd32x4::from(-1.0),
            // e41, e42, e43
            self.group4() * Simd32x3::from(-1.0),
            // e23, e31, e12
            self.group5() * Simd32x3::from(-1.0),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e4235, e4315, e4125, e3215
            self.group9(),
            // e1234
            self[e1234],
        );
    }
}
impl std::ops::Div<conjugation> for Plane {
    type Output = Plane;
    fn div(self, _rhs: conjugation) -> Self::Output {
        self.conjugation()
    }
}
impl std::ops::DivAssign<conjugation> for Plane {
    fn div_assign(&mut self, _rhs: conjugation) {
        *self = self.conjugation()
    }
}
impl Conjugation for Plane {
    fn conjugation(self) -> Self {
        return self;
    }
}
impl std::ops::Div<conjugation> for RoundPoint {
    type Output = RoundPoint;
    fn div(self, _rhs: conjugation) -> Self::Output {
        self.conjugation()
    }
}
impl std::ops::DivAssign<conjugation> for RoundPoint {
    fn div_assign(&mut self, _rhs: conjugation) {
        *self = self.conjugation()
    }
}
impl Conjugation for RoundPoint {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        2        0
    //  no simd        0        5        0
    fn conjugation(self) -> Self {
        use crate::elements::*;
        return RoundPoint::from_groups(/* e1, e2, e3, e4 */ self.group0() * Simd32x4::from(-1.0), /* e5 */ self[e5] * -1.0);
    }
}
impl std::ops::Div<conjugation> for Scalar {
    type Output = Scalar;
    fn div(self, _rhs: conjugation) -> Self::Output {
        self.conjugation()
    }
}
impl std::ops::DivAssign<conjugation> for Scalar {
    fn div_assign(&mut self, _rhs: conjugation) {
        *self = self.conjugation()
    }
}
impl Conjugation for Scalar {
    fn conjugation(self) -> Self {
        return self;
    }
}
impl std::ops::Div<conjugation> for Sphere {
    type Output = Sphere;
    fn div(self, _rhs: conjugation) -> Self::Output {
        self.conjugation()
    }
}
impl std::ops::DivAssign<conjugation> for Sphere {
    fn div_assign(&mut self, _rhs: conjugation) {
        *self = self.conjugation()
    }
}
impl Conjugation for Sphere {
    fn conjugation(self) -> Self {
        return self;
    }
}
impl std::ops::Div<conjugation> for VersorEven {
    type Output = VersorEven;
    fn div(self, _rhs: conjugation) -> Self::Output {
        self.conjugation()
    }
}
impl std::ops::DivAssign<conjugation> for VersorEven {
    fn div_assign(&mut self, _rhs: conjugation) {
        *self = self.conjugation()
    }
}
impl Conjugation for VersorEven {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        3        0
    // no simd        0       12        0
    fn conjugation(self) -> Self {
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            self.group0() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e415, e425, e435, e321
            self.group1(),
            // e235, e315, e125, e5
            self.group2() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e1, e2, e3, e4
            self.group3() * Simd32x4::from(-1.0),
        );
    }
}
impl std::ops::Div<conjugation> for VersorOdd {
    type Output = VersorOdd;
    fn div(self, _rhs: conjugation) -> Self::Output {
        self.conjugation()
    }
}
impl std::ops::DivAssign<conjugation> for VersorOdd {
    fn div_assign(&mut self, _rhs: conjugation) {
        *self = self.conjugation()
    }
}
impl Conjugation for VersorOdd {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        3        0
    // no simd        0       12        0
    fn conjugation(self) -> Self {
        return VersorOdd::from_groups(
            // e41, e42, e43, scalar
            self.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e23, e31, e12, e45
            self.group1() * Simd32x4::from(-1.0),
            // e15, e25, e35, e1234
            self.group2() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e4235, e4315, e4125, e3215
            self.group3(),
        );
    }
}
