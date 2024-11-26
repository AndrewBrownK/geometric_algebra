// Note on Operative Statistics:
// Operative Statistics are not a precise predictor of performance or performance comparisons.
// This is due to varying hardware capabilities and compiler optimizations.
// As always, where performance is a concern, there is no substitute for
// real measurements on real work-loads on real hardware.
// Disclaimer aside, enjoy the fun information =)
//
// Total Implementations: 95
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
//  Maximum:         0      20       0
impl std::ops::Div<anti_reverse> for AntiCircleOnOrigin {
    type Output = AntiCircleOnOrigin;
    fn div(self, _rhs: anti_reverse) -> Self::Output {
        self.anti_reverse()
    }
}
impl std::ops::DivAssign<anti_reverse> for AntiCircleOnOrigin {
    fn div_assign(&mut self, _rhs: anti_reverse) {
        *self = self.anti_reverse()
    }
}
impl AntiReverse for AntiCircleOnOrigin {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        2        0
    // no simd        0        6        0
    fn anti_reverse(self) -> Self {
        return AntiCircleOnOrigin::from_groups(
            // e41, e42, e43
            self.group0() * Simd32x3::from(-1.0),
            // e23, e31, e12
            self.group1() * Simd32x3::from(-1.0),
        );
    }
}
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
    //    simd3        0        1        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        0        3        0
    //  no simd        0       11        0
    fn anti_reverse(self) -> Self {
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
impl std::ops::Div<anti_reverse> for AntiCircleRotorAligningOrigin {
    type Output = AntiCircleRotorAligningOrigin;
    fn div(self, _rhs: anti_reverse) -> Self::Output {
        self.anti_reverse()
    }
}
impl std::ops::DivAssign<anti_reverse> for AntiCircleRotorAligningOrigin {
    fn div_assign(&mut self, _rhs: anti_reverse) {
        *self = self.anti_reverse()
    }
}
impl AntiReverse for AntiCircleRotorAligningOrigin {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        0        2        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        3        0
    //  no simd        0       10        0
    fn anti_reverse(self) -> Self {
        return AntiCircleRotorAligningOrigin::from_groups(
            // e41, e42, e43
            self.group0() * Simd32x3::from(-1.0),
            // e23, e31, e12
            self.group1() * Simd32x3::from(-1.0),
            // e15, e25, e35, scalar
            self.group2() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
        );
    }
}
impl std::ops::Div<anti_reverse> for AntiCircleRotorAligningOriginAtInfinity {
    type Output = AntiCircleRotorAligningOriginAtInfinity;
    fn div(self, _rhs: anti_reverse) -> Self::Output {
        self.anti_reverse()
    }
}
impl std::ops::DivAssign<anti_reverse> for AntiCircleRotorAligningOriginAtInfinity {
    fn div_assign(&mut self, _rhs: anti_reverse) {
        *self = self.anti_reverse()
    }
}
impl AntiReverse for AntiCircleRotorAligningOriginAtInfinity {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        2        0
    //  no simd        0        7        0
    fn anti_reverse(self) -> Self {
        return AntiCircleRotorAligningOriginAtInfinity::from_groups(
            // e23, e31, e12
            self.group0() * Simd32x3::from(-1.0),
            // e15, e25, e35, scalar
            self.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
        );
    }
}
impl std::ops::Div<anti_reverse> for AntiCircleRotorAtInfinity {
    type Output = AntiCircleRotorAtInfinity;
    fn div(self, _rhs: anti_reverse) -> Self::Output {
        self.anti_reverse()
    }
}
impl std::ops::DivAssign<anti_reverse> for AntiCircleRotorAtInfinity {
    fn div_assign(&mut self, _rhs: anti_reverse) {
        *self = self.anti_reverse()
    }
}
impl AntiReverse for AntiCircleRotorAtInfinity {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        2        0
    // no simd        0        8        0
    fn anti_reverse(self) -> Self {
        return AntiCircleRotorAtInfinity::from_groups(
            // e23, e31, e12, e45
            self.group0() * Simd32x4::from(-1.0),
            // e15, e25, e35, scalar
            self.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
        );
    }
}
impl std::ops::Div<anti_reverse> for AntiCircleRotorOnOrigin {
    type Output = AntiCircleRotorOnOrigin;
    fn div(self, _rhs: anti_reverse) -> Self::Output {
        self.anti_reverse()
    }
}
impl std::ops::DivAssign<anti_reverse> for AntiCircleRotorOnOrigin {
    fn div_assign(&mut self, _rhs: anti_reverse) {
        *self = self.anti_reverse()
    }
}
impl AntiReverse for AntiCircleRotorOnOrigin {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        2        0
    //  no simd        0        7        0
    fn anti_reverse(self) -> Self {
        return AntiCircleRotorOnOrigin::from_groups(
            // e41, e42, e43, scalar
            self.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e23, e31, e12
            self.group1() * Simd32x3::from(-1.0),
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
    //    simd3        0        1        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        0        3        0
    //  no simd        0       11        0
    fn anti_reverse(self) -> Self {
        return AntiDipoleInversion::from_groups(
            // e423, e431, e412
            self.group0() * Simd32x3::from(-1.0),
            // e415, e425, e435, e321
            self.group1() * Simd32x4::from(-1.0),
            // e235, e315, e125, e4
            self.group2() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e1, e2, e3, e5
            self.group3(),
        );
    }
}
impl std::ops::Div<anti_reverse> for AntiDipoleInversionAtInfinity {
    type Output = AntiDipoleInversionAtInfinity;
    fn div(self, _rhs: anti_reverse) -> Self::Output {
        self.anti_reverse()
    }
}
impl std::ops::DivAssign<anti_reverse> for AntiDipoleInversionAtInfinity {
    fn div_assign(&mut self, _rhs: anti_reverse) {
        *self = self.anti_reverse()
    }
}
impl AntiReverse for AntiDipoleInversionAtInfinity {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        2        0
    //  no simd        0        7        0
    fn anti_reverse(self) -> Self {
        return AntiDipoleInversionAtInfinity::from_groups(
            // e415, e425, e435, e321
            self.group0() * Simd32x4::from(-1.0),
            // e235, e315, e125
            self.group1() * Simd32x3::from(-1.0),
            // e1, e2, e3, e5
            self.group2(),
        );
    }
}
impl std::ops::Div<anti_reverse> for AntiDipoleInversionOnOrigin {
    type Output = AntiDipoleInversionOnOrigin;
    fn div(self, _rhs: anti_reverse) -> Self::Output {
        self.anti_reverse()
    }
}
impl std::ops::DivAssign<anti_reverse> for AntiDipoleInversionOnOrigin {
    fn div_assign(&mut self, _rhs: anti_reverse) {
        *self = self.anti_reverse()
    }
}
impl AntiReverse for AntiDipoleInversionOnOrigin {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn anti_reverse(self) -> Self {
        return AntiDipoleInversionOnOrigin::from_groups(/* e423, e431, e412, e321 */ self.group0() * Simd32x4::from(-1.0), /* e4, e1, e2, e3 */ self.group1());
    }
}
impl std::ops::Div<anti_reverse> for AntiDipoleInversionOrthogonalOrigin {
    type Output = AntiDipoleInversionOrthogonalOrigin;
    fn div(self, _rhs: anti_reverse) -> Self::Output {
        self.anti_reverse()
    }
}
impl std::ops::DivAssign<anti_reverse> for AntiDipoleInversionOrthogonalOrigin {
    fn div_assign(&mut self, _rhs: anti_reverse) {
        *self = self.anti_reverse()
    }
}
impl AntiReverse for AntiDipoleInversionOrthogonalOrigin {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        0        1        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        0        3        0
    //  no simd        0       11        0
    fn anti_reverse(self) -> Self {
        return AntiDipoleInversionOrthogonalOrigin::from_groups(
            // e423, e431, e412, e5
            self.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e415, e425, e435
            self.group1() * Simd32x3::from(-1.0),
            // e235, e315, e125, e4
            self.group2() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
        );
    }
}
impl std::ops::Div<anti_reverse> for AntiDipoleOnOrigin {
    type Output = AntiDipoleOnOrigin;
    fn div(self, _rhs: anti_reverse) -> Self::Output {
        self.anti_reverse()
    }
}
impl std::ops::DivAssign<anti_reverse> for AntiDipoleOnOrigin {
    fn div_assign(&mut self, _rhs: anti_reverse) {
        *self = self.anti_reverse()
    }
}
impl AntiReverse for AntiDipoleOnOrigin {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn anti_reverse(self) -> Self {
        return AntiDipoleOnOrigin::from_groups(/* e423, e431, e412, e321 */ self.group0() * Simd32x4::from(-1.0));
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
impl std::ops::Div<anti_reverse> for AntiFlatOrigin {
    type Output = AntiFlatOrigin;
    fn div(self, _rhs: anti_reverse) -> Self::Output {
        self.anti_reverse()
    }
}
impl std::ops::DivAssign<anti_reverse> for AntiFlatOrigin {
    fn div_assign(&mut self, _rhs: anti_reverse) {
        *self = self.anti_reverse()
    }
}
impl AntiReverse for AntiFlatOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn anti_reverse(self) -> Self {
        use crate::elements::*;
        return AntiFlatOrigin::from_groups(/* e321 */ self[e321] * -1.0);
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
        return AntiFlatPoint::from_groups(/* e235, e315, e125, e321 */ self.group0() * Simd32x4::from(-1.0));
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
        return AntiFlector::from_groups(/* e235, e315, e125, e321 */ self.group0() * Simd32x4::from(-1.0), /* e1, e2, e3, e5 */ self.group1());
    }
}
impl std::ops::Div<anti_reverse> for AntiFlectorOnOrigin {
    type Output = AntiFlectorOnOrigin;
    fn div(self, _rhs: anti_reverse) -> Self::Output {
        self.anti_reverse()
    }
}
impl std::ops::DivAssign<anti_reverse> for AntiFlectorOnOrigin {
    fn div_assign(&mut self, _rhs: anti_reverse) {
        *self = self.anti_reverse()
    }
}
impl AntiReverse for AntiFlectorOnOrigin {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn anti_reverse(self) -> Self {
        return AntiFlectorOnOrigin::from_groups(/* e321, e1, e2, e3 */ self.group0() * Simd32x4::from([-1.0, 1.0, 1.0, 1.0]));
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
        return AntiLine::from_groups(
            // e23, e31, e12
            self.group0() * Simd32x3::from(-1.0),
            // e15, e25, e35
            self.group1() * Simd32x3::from(-1.0),
        );
    }
}
impl std::ops::Div<anti_reverse> for AntiLineOnOrigin {
    type Output = AntiLineOnOrigin;
    fn div(self, _rhs: anti_reverse) -> Self::Output {
        self.anti_reverse()
    }
}
impl std::ops::DivAssign<anti_reverse> for AntiLineOnOrigin {
    fn div_assign(&mut self, _rhs: anti_reverse) {
        *self = self.anti_reverse()
    }
}
impl AntiReverse for AntiLineOnOrigin {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        1        0
    // no simd        0        3        0
    fn anti_reverse(self) -> Self {
        return AntiLineOnOrigin::from_groups(/* e23, e31, e12 */ self.group0() * Simd32x3::from(-1.0));
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
    //          add/sub      mul      div
    //   simd4        0        2        0
    // no simd        0        8        0
    fn anti_reverse(self) -> Self {
        return AntiMotor::from_groups(
            // e23, e31, e12, scalar
            self.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e15, e25, e35, e3215
            self.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
        );
    }
}
impl std::ops::Div<anti_reverse> for AntiMotorOnOrigin {
    type Output = AntiMotorOnOrigin;
    fn div(self, _rhs: anti_reverse) -> Self::Output {
        self.anti_reverse()
    }
}
impl std::ops::DivAssign<anti_reverse> for AntiMotorOnOrigin {
    fn div_assign(&mut self, _rhs: anti_reverse) {
        *self = self.anti_reverse()
    }
}
impl AntiReverse for AntiMotorOnOrigin {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn anti_reverse(self) -> Self {
        return AntiMotorOnOrigin::from_groups(/* e23, e31, e12, scalar */ self.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]));
    }
}
impl std::ops::Div<anti_reverse> for AntiMysteryCircleRotor {
    type Output = AntiMysteryCircleRotor;
    fn div(self, _rhs: anti_reverse) -> Self::Output {
        self.anti_reverse()
    }
}
impl std::ops::DivAssign<anti_reverse> for AntiMysteryCircleRotor {
    fn div_assign(&mut self, _rhs: anti_reverse) {
        *self = self.anti_reverse()
    }
}
impl AntiReverse for AntiMysteryCircleRotor {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn anti_reverse(self) -> Self {
        use crate::elements::*;
        return AntiMysteryCircleRotor::from_groups(/* e23, e31, e12, e45 */ self.group0() * Simd32x4::from(-1.0), /* scalar */ self[scalar]);
    }
}
impl std::ops::Div<anti_reverse> for AntiMysteryDipoleInversion {
    type Output = AntiMysteryDipoleInversion;
    fn div(self, _rhs: anti_reverse) -> Self::Output {
        self.anti_reverse()
    }
}
impl std::ops::DivAssign<anti_reverse> for AntiMysteryDipoleInversion {
    fn div_assign(&mut self, _rhs: anti_reverse) {
        *self = self.anti_reverse()
    }
}
impl AntiReverse for AntiMysteryDipoleInversion {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn anti_reverse(self) -> Self {
        return AntiMysteryDipoleInversion::from_groups(/* e415, e425, e435, e321 */ self.group0() * Simd32x4::from(-1.0), /* e1, e2, e3 */ self.group1());
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
impl std::ops::Div<anti_reverse> for AntiPlaneOnOrigin {
    type Output = AntiPlaneOnOrigin;
    fn div(self, _rhs: anti_reverse) -> Self::Output {
        self.anti_reverse()
    }
}
impl std::ops::DivAssign<anti_reverse> for AntiPlaneOnOrigin {
    fn div_assign(&mut self, _rhs: anti_reverse) {
        *self = self.anti_reverse()
    }
}
impl AntiReverse for AntiPlaneOnOrigin {
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
impl std::ops::Div<anti_reverse> for AntiSphereOnOrigin {
    type Output = AntiSphereOnOrigin;
    fn div(self, _rhs: anti_reverse) -> Self::Output {
        self.anti_reverse()
    }
}
impl std::ops::DivAssign<anti_reverse> for AntiSphereOnOrigin {
    fn div_assign(&mut self, _rhs: anti_reverse) {
        *self = self.anti_reverse()
    }
}
impl AntiReverse for AntiSphereOnOrigin {
    fn anti_reverse(self) -> Self {
        return self;
    }
}
impl std::ops::Div<anti_reverse> for AntiVersorEvenOnOrigin {
    type Output = AntiVersorEvenOnOrigin;
    fn div(self, _rhs: anti_reverse) -> Self::Output {
        self.anti_reverse()
    }
}
impl std::ops::DivAssign<anti_reverse> for AntiVersorEvenOnOrigin {
    fn div_assign(&mut self, _rhs: anti_reverse) {
        *self = self.anti_reverse()
    }
}
impl AntiReverse for AntiVersorEvenOnOrigin {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        2        0
    // no simd        0        8        0
    fn anti_reverse(self) -> Self {
        return AntiVersorEvenOnOrigin::from_groups(
            // e41, e42, e43, scalar
            self.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e23, e31, e12, e1234
            self.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
        );
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
impl std::ops::Div<anti_reverse> for CircleAligningOrigin {
    type Output = CircleAligningOrigin;
    fn div(self, _rhs: anti_reverse) -> Self::Output {
        self.anti_reverse()
    }
}
impl std::ops::DivAssign<anti_reverse> for CircleAligningOrigin {
    fn div_assign(&mut self, _rhs: anti_reverse) {
        *self = self.anti_reverse()
    }
}
impl AntiReverse for CircleAligningOrigin {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        3        0
    // no simd        0        9        0
    fn anti_reverse(self) -> Self {
        return CircleAligningOrigin::from_groups(
            // e423, e431, e412
            self.group0() * Simd32x3::from(-1.0),
            // e415, e425, e435
            self.group1() * Simd32x3::from(-1.0),
            // e235, e315, e125
            self.group2() * Simd32x3::from(-1.0),
        );
    }
}
impl std::ops::Div<anti_reverse> for CircleAtInfinity {
    type Output = CircleAtInfinity;
    fn div(self, _rhs: anti_reverse) -> Self::Output {
        self.anti_reverse()
    }
}
impl std::ops::DivAssign<anti_reverse> for CircleAtInfinity {
    fn div_assign(&mut self, _rhs: anti_reverse) {
        *self = self.anti_reverse()
    }
}
impl AntiReverse for CircleAtInfinity {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        2        0
    //  no simd        0        7        0
    fn anti_reverse(self) -> Self {
        return CircleAtInfinity::from_groups(
            // e415, e425, e435, e321
            self.group0() * Simd32x4::from(-1.0),
            // e235, e315, e125
            self.group1() * Simd32x3::from(-1.0),
        );
    }
}
impl std::ops::Div<anti_reverse> for CircleAtOrigin {
    type Output = CircleAtOrigin;
    fn div(self, _rhs: anti_reverse) -> Self::Output {
        self.anti_reverse()
    }
}
impl std::ops::DivAssign<anti_reverse> for CircleAtOrigin {
    fn div_assign(&mut self, _rhs: anti_reverse) {
        *self = self.anti_reverse()
    }
}
impl AntiReverse for CircleAtOrigin {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        2        0
    // no simd        0        6        0
    fn anti_reverse(self) -> Self {
        return CircleAtOrigin::from_groups(
            // e423, e431, e412
            self.group0() * Simd32x3::from(-1.0),
            // e235, e315, e125
            self.group1() * Simd32x3::from(-1.0),
        );
    }
}
impl std::ops::Div<anti_reverse> for CircleOnOrigin {
    type Output = CircleOnOrigin;
    fn div(self, _rhs: anti_reverse) -> Self::Output {
        self.anti_reverse()
    }
}
impl std::ops::DivAssign<anti_reverse> for CircleOnOrigin {
    fn div_assign(&mut self, _rhs: anti_reverse) {
        *self = self.anti_reverse()
    }
}
impl AntiReverse for CircleOnOrigin {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        2        0
    // no simd        0        6        0
    fn anti_reverse(self) -> Self {
        return CircleOnOrigin::from_groups(
            // e423, e431, e412
            self.group0() * Simd32x3::from(-1.0),
            // e415, e425, e435
            self.group1() * Simd32x3::from(-1.0),
        );
    }
}
impl std::ops::Div<anti_reverse> for CircleOrthogonalOrigin {
    type Output = CircleOrthogonalOrigin;
    fn div(self, _rhs: anti_reverse) -> Self::Output {
        self.anti_reverse()
    }
}
impl std::ops::DivAssign<anti_reverse> for CircleOrthogonalOrigin {
    fn div_assign(&mut self, _rhs: anti_reverse) {
        *self = self.anti_reverse()
    }
}
impl AntiReverse for CircleOrthogonalOrigin {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        2        0
    //  no simd        0        7        0
    fn anti_reverse(self) -> Self {
        return CircleOrthogonalOrigin::from_groups(
            // e423, e431, e412, e321
            self.group0() * Simd32x4::from(-1.0),
            // e235, e315, e125
            self.group1() * Simd32x3::from(-1.0),
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
    //    simd3        0        1        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        0        3        0
    //  no simd        0       11        0
    fn anti_reverse(self) -> Self {
        return CircleRotor::from_groups(
            // e423, e431, e412
            self.group0() * Simd32x3::from(-1.0),
            // e415, e425, e435, e321
            self.group1() * Simd32x4::from(-1.0),
            // e235, e315, e125, e12345
            self.group2() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
        );
    }
}
impl std::ops::Div<anti_reverse> for CircleRotorAligningOrigin {
    type Output = CircleRotorAligningOrigin;
    fn div(self, _rhs: anti_reverse) -> Self::Output {
        self.anti_reverse()
    }
}
impl std::ops::DivAssign<anti_reverse> for CircleRotorAligningOrigin {
    fn div_assign(&mut self, _rhs: anti_reverse) {
        *self = self.anti_reverse()
    }
}
impl AntiReverse for CircleRotorAligningOrigin {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        0        2        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        3        0
    //  no simd        0       10        0
    fn anti_reverse(self) -> Self {
        return CircleRotorAligningOrigin::from_groups(
            // e423, e431, e412
            self.group0() * Simd32x3::from(-1.0),
            // e415, e425, e435
            self.group1() * Simd32x3::from(-1.0),
            // e235, e315, e125, e12345
            self.group2() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
        );
    }
}
impl std::ops::Div<anti_reverse> for CircleRotorAligningOriginAtInfinity {
    type Output = CircleRotorAligningOriginAtInfinity;
    fn div(self, _rhs: anti_reverse) -> Self::Output {
        self.anti_reverse()
    }
}
impl std::ops::DivAssign<anti_reverse> for CircleRotorAligningOriginAtInfinity {
    fn div_assign(&mut self, _rhs: anti_reverse) {
        *self = self.anti_reverse()
    }
}
impl AntiReverse for CircleRotorAligningOriginAtInfinity {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        2        0
    //  no simd        0        7        0
    fn anti_reverse(self) -> Self {
        return CircleRotorAligningOriginAtInfinity::from_groups(
            // e415, e425, e435
            self.group0() * Simd32x3::from(-1.0),
            // e235, e315, e125, e12345
            self.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
        );
    }
}
impl std::ops::Div<anti_reverse> for CircleRotorAtInfinity {
    type Output = CircleRotorAtInfinity;
    fn div(self, _rhs: anti_reverse) -> Self::Output {
        self.anti_reverse()
    }
}
impl std::ops::DivAssign<anti_reverse> for CircleRotorAtInfinity {
    fn div_assign(&mut self, _rhs: anti_reverse) {
        *self = self.anti_reverse()
    }
}
impl AntiReverse for CircleRotorAtInfinity {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        2        0
    // no simd        0        8        0
    fn anti_reverse(self) -> Self {
        return CircleRotorAtInfinity::from_groups(
            // e415, e425, e435, e321
            self.group0() * Simd32x4::from(-1.0),
            // e235, e315, e125, e12345
            self.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
        );
    }
}
impl std::ops::Div<anti_reverse> for CircleRotorOnOrigin {
    type Output = CircleRotorOnOrigin;
    fn div(self, _rhs: anti_reverse) -> Self::Output {
        self.anti_reverse()
    }
}
impl std::ops::DivAssign<anti_reverse> for CircleRotorOnOrigin {
    fn div_assign(&mut self, _rhs: anti_reverse) {
        *self = self.anti_reverse()
    }
}
impl AntiReverse for CircleRotorOnOrigin {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        2        0
    //  no simd        0        7        0
    fn anti_reverse(self) -> Self {
        return CircleRotorOnOrigin::from_groups(
            // e423, e431, e412, e12345
            self.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e415, e425, e435
            self.group1() * Simd32x3::from(-1.0),
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
impl std::ops::Div<anti_reverse> for DipoleAligningOrigin {
    type Output = DipoleAligningOrigin;
    fn div(self, _rhs: anti_reverse) -> Self::Output {
        self.anti_reverse()
    }
}
impl std::ops::DivAssign<anti_reverse> for DipoleAligningOrigin {
    fn div_assign(&mut self, _rhs: anti_reverse) {
        *self = self.anti_reverse()
    }
}
impl AntiReverse for DipoleAligningOrigin {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        2        0
    //  no simd        0        7        0
    fn anti_reverse(self) -> Self {
        return DipoleAligningOrigin::from_groups(
            // e41, e42, e43, e45
            self.group0() * Simd32x4::from(-1.0),
            // e15, e25, e35
            self.group1() * Simd32x3::from(-1.0),
        );
    }
}
impl std::ops::Div<anti_reverse> for DipoleAtInfinity {
    type Output = DipoleAtInfinity;
    fn div(self, _rhs: anti_reverse) -> Self::Output {
        self.anti_reverse()
    }
}
impl std::ops::DivAssign<anti_reverse> for DipoleAtInfinity {
    fn div_assign(&mut self, _rhs: anti_reverse) {
        *self = self.anti_reverse()
    }
}
impl AntiReverse for DipoleAtInfinity {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        2        0
    //  no simd        0        7        0
    fn anti_reverse(self) -> Self {
        return DipoleAtInfinity::from_groups(
            // e23, e31, e12, e45
            self.group0() * Simd32x4::from(-1.0),
            // e15, e25, e35
            self.group1() * Simd32x3::from(-1.0),
        );
    }
}
impl std::ops::Div<anti_reverse> for DipoleAtOrigin {
    type Output = DipoleAtOrigin;
    fn div(self, _rhs: anti_reverse) -> Self::Output {
        self.anti_reverse()
    }
}
impl std::ops::DivAssign<anti_reverse> for DipoleAtOrigin {
    fn div_assign(&mut self, _rhs: anti_reverse) {
        *self = self.anti_reverse()
    }
}
impl AntiReverse for DipoleAtOrigin {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        2        0
    // no simd        0        6        0
    fn anti_reverse(self) -> Self {
        return DipoleAtOrigin::from_groups(
            // e41, e42, e43
            self.group0() * Simd32x3::from(-1.0),
            // e15, e25, e35
            self.group1() * Simd32x3::from(-1.0),
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
    //    simd3        0        1        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        0        3        0
    //  no simd        0       11        0
    fn anti_reverse(self) -> Self {
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
impl std::ops::Div<anti_reverse> for DipoleInversionAligningOrigin {
    type Output = DipoleInversionAligningOrigin;
    fn div(self, _rhs: anti_reverse) -> Self::Output {
        self.anti_reverse()
    }
}
impl std::ops::DivAssign<anti_reverse> for DipoleInversionAligningOrigin {
    fn div_assign(&mut self, _rhs: anti_reverse) {
        *self = self.anti_reverse()
    }
}
impl AntiReverse for DipoleInversionAligningOrigin {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        2        0
    // no simd        0        8        0
    fn anti_reverse(self) -> Self {
        return DipoleInversionAligningOrigin::from_groups(
            // e41, e42, e43, e45
            self.group0() * Simd32x4::from(-1.0),
            // e15, e25, e35, e1234
            self.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e4235, e4315, e4125, e3215
            self.group2(),
        );
    }
}
impl std::ops::Div<anti_reverse> for DipoleInversionAtInfinity {
    type Output = DipoleInversionAtInfinity;
    fn div(self, _rhs: anti_reverse) -> Self::Output {
        self.anti_reverse()
    }
}
impl std::ops::DivAssign<anti_reverse> for DipoleInversionAtInfinity {
    fn div_assign(&mut self, _rhs: anti_reverse) {
        *self = self.anti_reverse()
    }
}
impl AntiReverse for DipoleInversionAtInfinity {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        2        0
    //  no simd        0        7        0
    fn anti_reverse(self) -> Self {
        return DipoleInversionAtInfinity::from_groups(
            // e23, e31, e12, e45
            self.group0() * Simd32x4::from(-1.0),
            // e15, e25, e35
            self.group1() * Simd32x3::from(-1.0),
            // e4235, e4315, e4125, e3215
            self.group2(),
        );
    }
}
impl std::ops::Div<anti_reverse> for DipoleInversionAtOrigin {
    type Output = DipoleInversionAtOrigin;
    fn div(self, _rhs: anti_reverse) -> Self::Output {
        self.anti_reverse()
    }
}
impl std::ops::DivAssign<anti_reverse> for DipoleInversionAtOrigin {
    fn div_assign(&mut self, _rhs: anti_reverse) {
        *self = self.anti_reverse()
    }
}
impl AntiReverse for DipoleInversionAtOrigin {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        2        0
    // no simd        0        8        0
    fn anti_reverse(self) -> Self {
        return DipoleInversionAtOrigin::from_groups(
            // e41, e42, e43, e3215
            self.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e15, e25, e35, e1234
            self.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
        );
    }
}
impl std::ops::Div<anti_reverse> for DipoleInversionOnOrigin {
    type Output = DipoleInversionOnOrigin;
    fn div(self, _rhs: anti_reverse) -> Self::Output {
        self.anti_reverse()
    }
}
impl std::ops::DivAssign<anti_reverse> for DipoleInversionOnOrigin {
    fn div_assign(&mut self, _rhs: anti_reverse) {
        *self = self.anti_reverse()
    }
}
impl AntiReverse for DipoleInversionOnOrigin {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn anti_reverse(self) -> Self {
        return DipoleInversionOnOrigin::from_groups(/* e41, e42, e43, e45 */ self.group0() * Simd32x4::from(-1.0), /* e1234, e4235, e4315, e4125 */ self.group1());
    }
}
impl std::ops::Div<anti_reverse> for DipoleInversionOrthogonalOrigin {
    type Output = DipoleInversionOrthogonalOrigin;
    fn div(self, _rhs: anti_reverse) -> Self::Output {
        self.anti_reverse()
    }
}
impl std::ops::DivAssign<anti_reverse> for DipoleInversionOrthogonalOrigin {
    fn div_assign(&mut self, _rhs: anti_reverse) {
        *self = self.anti_reverse()
    }
}
impl AntiReverse for DipoleInversionOrthogonalOrigin {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        0        1        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        0        3        0
    //  no simd        0       11        0
    fn anti_reverse(self) -> Self {
        return DipoleInversionOrthogonalOrigin::from_groups(
            // e41, e42, e43, e3215
            self.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e23, e31, e12
            self.group1() * Simd32x3::from(-1.0),
            // e15, e25, e35, e1234
            self.group2() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
        );
    }
}
impl std::ops::Div<anti_reverse> for DipoleOnOrigin {
    type Output = DipoleOnOrigin;
    fn div(self, _rhs: anti_reverse) -> Self::Output {
        self.anti_reverse()
    }
}
impl std::ops::DivAssign<anti_reverse> for DipoleOnOrigin {
    fn div_assign(&mut self, _rhs: anti_reverse) {
        *self = self.anti_reverse()
    }
}
impl AntiReverse for DipoleOnOrigin {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn anti_reverse(self) -> Self {
        return DipoleOnOrigin::from_groups(/* e41, e42, e43, e45 */ self.group0() * Simd32x4::from(-1.0));
    }
}
impl std::ops::Div<anti_reverse> for DipoleOrthogonalOrigin {
    type Output = DipoleOrthogonalOrigin;
    fn div(self, _rhs: anti_reverse) -> Self::Output {
        self.anti_reverse()
    }
}
impl std::ops::DivAssign<anti_reverse> for DipoleOrthogonalOrigin {
    fn div_assign(&mut self, _rhs: anti_reverse) {
        *self = self.anti_reverse()
    }
}
impl AntiReverse for DipoleOrthogonalOrigin {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        3        0
    // no simd        0        9        0
    fn anti_reverse(self) -> Self {
        return DipoleOrthogonalOrigin::from_groups(
            // e41, e42, e43
            self.group0() * Simd32x3::from(-1.0),
            // e23, e31, e12
            self.group1() * Simd32x3::from(-1.0),
            // e15, e25, e35
            self.group2() * Simd32x3::from(-1.0),
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
impl std::ops::Div<anti_reverse> for FlatOrigin {
    type Output = FlatOrigin;
    fn div(self, _rhs: anti_reverse) -> Self::Output {
        self.anti_reverse()
    }
}
impl std::ops::DivAssign<anti_reverse> for FlatOrigin {
    fn div_assign(&mut self, _rhs: anti_reverse) {
        *self = self.anti_reverse()
    }
}
impl AntiReverse for FlatOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn anti_reverse(self) -> Self {
        use crate::elements::*;
        return FlatOrigin::from_groups(/* e45 */ self[e45] * -1.0);
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
        return FlatPoint::from_groups(/* e15, e25, e35, e45 */ self.group0() * Simd32x4::from(-1.0));
    }
}
impl std::ops::Div<anti_reverse> for FlatPointAtInfinity {
    type Output = FlatPointAtInfinity;
    fn div(self, _rhs: anti_reverse) -> Self::Output {
        self.anti_reverse()
    }
}
impl std::ops::DivAssign<anti_reverse> for FlatPointAtInfinity {
    fn div_assign(&mut self, _rhs: anti_reverse) {
        *self = self.anti_reverse()
    }
}
impl AntiReverse for FlatPointAtInfinity {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        1        0
    // no simd        0        3        0
    fn anti_reverse(self) -> Self {
        return FlatPointAtInfinity::from_groups(/* e15, e25, e35 */ self.group0() * Simd32x3::from(-1.0));
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
        return Flector::from_groups(/* e15, e25, e35, e45 */ self.group0() * Simd32x4::from(-1.0), /* e4235, e4315, e4125, e3215 */ self.group1());
    }
}
impl std::ops::Div<anti_reverse> for FlectorAtInfinity {
    type Output = FlectorAtInfinity;
    fn div(self, _rhs: anti_reverse) -> Self::Output {
        self.anti_reverse()
    }
}
impl std::ops::DivAssign<anti_reverse> for FlectorAtInfinity {
    fn div_assign(&mut self, _rhs: anti_reverse) {
        *self = self.anti_reverse()
    }
}
impl AntiReverse for FlectorAtInfinity {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn anti_reverse(self) -> Self {
        return FlectorAtInfinity::from_groups(/* e15, e25, e35, e3215 */ self.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]));
    }
}
impl std::ops::Div<anti_reverse> for FlectorOnOrigin {
    type Output = FlectorOnOrigin;
    fn div(self, _rhs: anti_reverse) -> Self::Output {
        self.anti_reverse()
    }
}
impl std::ops::DivAssign<anti_reverse> for FlectorOnOrigin {
    fn div_assign(&mut self, _rhs: anti_reverse) {
        *self = self.anti_reverse()
    }
}
impl AntiReverse for FlectorOnOrigin {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn anti_reverse(self) -> Self {
        return FlectorOnOrigin::from_groups(/* e45, e4235, e4315, e4125 */ self.group0() * Simd32x4::from([-1.0, 1.0, 1.0, 1.0]));
    }
}
impl std::ops::Div<anti_reverse> for Horizon {
    type Output = Horizon;
    fn div(self, _rhs: anti_reverse) -> Self::Output {
        self.anti_reverse()
    }
}
impl std::ops::DivAssign<anti_reverse> for Horizon {
    fn div_assign(&mut self, _rhs: anti_reverse) {
        *self = self.anti_reverse()
    }
}
impl AntiReverse for Horizon {
    fn anti_reverse(self) -> Self {
        return self;
    }
}
impl std::ops::Div<anti_reverse> for Infinity {
    type Output = Infinity;
    fn div(self, _rhs: anti_reverse) -> Self::Output {
        self.anti_reverse()
    }
}
impl std::ops::DivAssign<anti_reverse> for Infinity {
    fn div_assign(&mut self, _rhs: anti_reverse) {
        *self = self.anti_reverse()
    }
}
impl AntiReverse for Infinity {
    fn anti_reverse(self) -> Self {
        return self;
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
        return Line::from_groups(
            // e415, e425, e435
            self.group0() * Simd32x3::from(-1.0),
            // e235, e315, e125
            self.group1() * Simd32x3::from(-1.0),
        );
    }
}
impl std::ops::Div<anti_reverse> for LineAtInfinity {
    type Output = LineAtInfinity;
    fn div(self, _rhs: anti_reverse) -> Self::Output {
        self.anti_reverse()
    }
}
impl std::ops::DivAssign<anti_reverse> for LineAtInfinity {
    fn div_assign(&mut self, _rhs: anti_reverse) {
        *self = self.anti_reverse()
    }
}
impl AntiReverse for LineAtInfinity {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        1        0
    // no simd        0        3        0
    fn anti_reverse(self) -> Self {
        return LineAtInfinity::from_groups(/* e235, e315, e125 */ self.group0() * Simd32x3::from(-1.0));
    }
}
impl std::ops::Div<anti_reverse> for LineOnOrigin {
    type Output = LineOnOrigin;
    fn div(self, _rhs: anti_reverse) -> Self::Output {
        self.anti_reverse()
    }
}
impl std::ops::DivAssign<anti_reverse> for LineOnOrigin {
    fn div_assign(&mut self, _rhs: anti_reverse) {
        *self = self.anti_reverse()
    }
}
impl AntiReverse for LineOnOrigin {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        1        0
    // no simd        0        3        0
    fn anti_reverse(self) -> Self {
        return LineOnOrigin::from_groups(/* e415, e425, e435 */ self.group0() * Simd32x3::from(-1.0));
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
    //          add/sub      mul      div
    //   simd4        0        2        0
    // no simd        0        8        0
    fn anti_reverse(self) -> Self {
        return Motor::from_groups(
            // e415, e425, e435, e12345
            self.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e235, e315, e125, e5
            self.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
        );
    }
}
impl std::ops::Div<anti_reverse> for MotorAtInfinity {
    type Output = MotorAtInfinity;
    fn div(self, _rhs: anti_reverse) -> Self::Output {
        self.anti_reverse()
    }
}
impl std::ops::DivAssign<anti_reverse> for MotorAtInfinity {
    fn div_assign(&mut self, _rhs: anti_reverse) {
        *self = self.anti_reverse()
    }
}
impl AntiReverse for MotorAtInfinity {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn anti_reverse(self) -> Self {
        return MotorAtInfinity::from_groups(/* e235, e315, e125, e5 */ self.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]));
    }
}
impl std::ops::Div<anti_reverse> for MotorOnOrigin {
    type Output = MotorOnOrigin;
    fn div(self, _rhs: anti_reverse) -> Self::Output {
        self.anti_reverse()
    }
}
impl std::ops::DivAssign<anti_reverse> for MotorOnOrigin {
    fn div_assign(&mut self, _rhs: anti_reverse) {
        *self = self.anti_reverse()
    }
}
impl AntiReverse for MotorOnOrigin {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn anti_reverse(self) -> Self {
        return MotorOnOrigin::from_groups(/* e415, e425, e435, e12345 */ self.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]));
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
            // e41, e42, e43, e45
            self.group3() * Simd32x4::from(-1.0),
            // e15, e25, e35
            self.group4() * Simd32x3::from(-1.0),
            // e23, e31, e12
            self.group5() * Simd32x3::from(-1.0),
            // e415, e425, e435, e321
            self.group6() * Simd32x4::from(-1.0),
            // e423, e431, e412
            self.group7() * Simd32x3::from(-1.0),
            // e235, e315, e125
            self.group8() * Simd32x3::from(-1.0),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Div<anti_reverse> for MysteryCircle {
    type Output = MysteryCircle;
    fn div(self, _rhs: anti_reverse) -> Self::Output {
        self.anti_reverse()
    }
}
impl std::ops::DivAssign<anti_reverse> for MysteryCircle {
    fn div_assign(&mut self, _rhs: anti_reverse) {
        *self = self.anti_reverse()
    }
}
impl AntiReverse for MysteryCircle {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn anti_reverse(self) -> Self {
        return MysteryCircle::from_groups(/* e415, e425, e435, e321 */ self.group0() * Simd32x4::from(-1.0));
    }
}
impl std::ops::Div<anti_reverse> for MysteryCircleRotor {
    type Output = MysteryCircleRotor;
    fn div(self, _rhs: anti_reverse) -> Self::Output {
        self.anti_reverse()
    }
}
impl std::ops::DivAssign<anti_reverse> for MysteryCircleRotor {
    fn div_assign(&mut self, _rhs: anti_reverse) {
        *self = self.anti_reverse()
    }
}
impl AntiReverse for MysteryCircleRotor {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn anti_reverse(self) -> Self {
        use crate::elements::*;
        return MysteryCircleRotor::from_groups(/* e415, e425, e435, e321 */ self.group0() * Simd32x4::from(-1.0), /* e12345 */ self[e12345]);
    }
}
impl std::ops::Div<anti_reverse> for MysteryDipole {
    type Output = MysteryDipole;
    fn div(self, _rhs: anti_reverse) -> Self::Output {
        self.anti_reverse()
    }
}
impl std::ops::DivAssign<anti_reverse> for MysteryDipole {
    fn div_assign(&mut self, _rhs: anti_reverse) {
        *self = self.anti_reverse()
    }
}
impl AntiReverse for MysteryDipole {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn anti_reverse(self) -> Self {
        return MysteryDipole::from_groups(/* e23, e31, e12, e45 */ self.group0() * Simd32x4::from(-1.0));
    }
}
impl std::ops::Div<anti_reverse> for MysteryDipoleInversion {
    type Output = MysteryDipoleInversion;
    fn div(self, _rhs: anti_reverse) -> Self::Output {
        self.anti_reverse()
    }
}
impl std::ops::DivAssign<anti_reverse> for MysteryDipoleInversion {
    fn div_assign(&mut self, _rhs: anti_reverse) {
        *self = self.anti_reverse()
    }
}
impl AntiReverse for MysteryDipoleInversion {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn anti_reverse(self) -> Self {
        return MysteryDipoleInversion::from_groups(/* e23, e31, e12, e45 */ self.group0() * Simd32x4::from(-1.0), /* e4235, e4315, e4125 */ self.group1());
    }
}
impl std::ops::Div<anti_reverse> for MysteryVersorEven {
    type Output = MysteryVersorEven;
    fn div(self, _rhs: anti_reverse) -> Self::Output {
        self.anti_reverse()
    }
}
impl std::ops::DivAssign<anti_reverse> for MysteryVersorEven {
    fn div_assign(&mut self, _rhs: anti_reverse) {
        *self = self.anti_reverse()
    }
}
impl AntiReverse for MysteryVersorEven {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn anti_reverse(self) -> Self {
        return MysteryVersorEven::from_groups(/* e12345, e1, e2, e3 */ self.group0(), /* e415, e425, e435, e321 */ self.group1() * Simd32x4::from(-1.0));
    }
}
impl std::ops::Div<anti_reverse> for MysteryVersorOdd {
    type Output = MysteryVersorOdd;
    fn div(self, _rhs: anti_reverse) -> Self::Output {
        self.anti_reverse()
    }
}
impl std::ops::DivAssign<anti_reverse> for MysteryVersorOdd {
    fn div_assign(&mut self, _rhs: anti_reverse) {
        *self = self.anti_reverse()
    }
}
impl AntiReverse for MysteryVersorOdd {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn anti_reverse(self) -> Self {
        return MysteryVersorOdd::from_groups(/* scalar, e4235, e4315, e4125 */ self.group0(), /* e23, e31, e12, e45 */ self.group1() * Simd32x4::from(-1.0));
    }
}
impl std::ops::Div<anti_reverse> for NullCircleAtOrigin {
    type Output = NullCircleAtOrigin;
    fn div(self, _rhs: anti_reverse) -> Self::Output {
        self.anti_reverse()
    }
}
impl std::ops::DivAssign<anti_reverse> for NullCircleAtOrigin {
    fn div_assign(&mut self, _rhs: anti_reverse) {
        *self = self.anti_reverse()
    }
}
impl AntiReverse for NullCircleAtOrigin {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        1        0
    // no simd        0        3        0
    fn anti_reverse(self) -> Self {
        return NullCircleAtOrigin::from_groups(/* e423, e431, e412 */ self.group0() * Simd32x3::from(-1.0));
    }
}
impl std::ops::Div<anti_reverse> for NullDipoleAtOrigin {
    type Output = NullDipoleAtOrigin;
    fn div(self, _rhs: anti_reverse) -> Self::Output {
        self.anti_reverse()
    }
}
impl std::ops::DivAssign<anti_reverse> for NullDipoleAtOrigin {
    fn div_assign(&mut self, _rhs: anti_reverse) {
        *self = self.anti_reverse()
    }
}
impl AntiReverse for NullDipoleAtOrigin {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        1        0
    // no simd        0        3        0
    fn anti_reverse(self) -> Self {
        return NullDipoleAtOrigin::from_groups(/* e41, e42, e43 */ self.group0() * Simd32x3::from(-1.0));
    }
}
impl std::ops::Div<anti_reverse> for NullDipoleInversionAtOrigin {
    type Output = NullDipoleInversionAtOrigin;
    fn div(self, _rhs: anti_reverse) -> Self::Output {
        self.anti_reverse()
    }
}
impl std::ops::DivAssign<anti_reverse> for NullDipoleInversionAtOrigin {
    fn div_assign(&mut self, _rhs: anti_reverse) {
        *self = self.anti_reverse()
    }
}
impl AntiReverse for NullDipoleInversionAtOrigin {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn anti_reverse(self) -> Self {
        return NullDipoleInversionAtOrigin::from_groups(/* e41, e42, e43, e1234 */ self.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]));
    }
}
impl std::ops::Div<anti_reverse> for NullSphereAtOrigin {
    type Output = NullSphereAtOrigin;
    fn div(self, _rhs: anti_reverse) -> Self::Output {
        self.anti_reverse()
    }
}
impl std::ops::DivAssign<anti_reverse> for NullSphereAtOrigin {
    fn div_assign(&mut self, _rhs: anti_reverse) {
        *self = self.anti_reverse()
    }
}
impl AntiReverse for NullSphereAtOrigin {
    fn anti_reverse(self) -> Self {
        return self;
    }
}
impl std::ops::Div<anti_reverse> for NullVersorEvenAtOrigin {
    type Output = NullVersorEvenAtOrigin;
    fn div(self, _rhs: anti_reverse) -> Self::Output {
        self.anti_reverse()
    }
}
impl std::ops::DivAssign<anti_reverse> for NullVersorEvenAtOrigin {
    fn div_assign(&mut self, _rhs: anti_reverse) {
        *self = self.anti_reverse()
    }
}
impl AntiReverse for NullVersorEvenAtOrigin {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn anti_reverse(self) -> Self {
        return NullVersorEvenAtOrigin::from_groups(/* e423, e431, e412, e4 */ self.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]));
    }
}
impl std::ops::Div<anti_reverse> for Origin {
    type Output = Origin;
    fn div(self, _rhs: anti_reverse) -> Self::Output {
        self.anti_reverse()
    }
}
impl std::ops::DivAssign<anti_reverse> for Origin {
    fn div_assign(&mut self, _rhs: anti_reverse) {
        *self = self.anti_reverse()
    }
}
impl AntiReverse for Origin {
    fn anti_reverse(self) -> Self {
        return self;
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
impl std::ops::Div<anti_reverse> for PlaneOnOrigin {
    type Output = PlaneOnOrigin;
    fn div(self, _rhs: anti_reverse) -> Self::Output {
        self.anti_reverse()
    }
}
impl std::ops::DivAssign<anti_reverse> for PlaneOnOrigin {
    fn div_assign(&mut self, _rhs: anti_reverse) {
        *self = self.anti_reverse()
    }
}
impl AntiReverse for PlaneOnOrigin {
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
impl std::ops::Div<anti_reverse> for RoundPointAtOrigin {
    type Output = RoundPointAtOrigin;
    fn div(self, _rhs: anti_reverse) -> Self::Output {
        self.anti_reverse()
    }
}
impl std::ops::DivAssign<anti_reverse> for RoundPointAtOrigin {
    fn div_assign(&mut self, _rhs: anti_reverse) {
        *self = self.anti_reverse()
    }
}
impl AntiReverse for RoundPointAtOrigin {
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
impl std::ops::Div<anti_reverse> for SphereAtOrigin {
    type Output = SphereAtOrigin;
    fn div(self, _rhs: anti_reverse) -> Self::Output {
        self.anti_reverse()
    }
}
impl std::ops::DivAssign<anti_reverse> for SphereAtOrigin {
    fn div_assign(&mut self, _rhs: anti_reverse) {
        *self = self.anti_reverse()
    }
}
impl AntiReverse for SphereAtOrigin {
    fn anti_reverse(self) -> Self {
        return self;
    }
}
impl std::ops::Div<anti_reverse> for SphereOnOrigin {
    type Output = SphereOnOrigin;
    fn div(self, _rhs: anti_reverse) -> Self::Output {
        self.anti_reverse()
    }
}
impl std::ops::DivAssign<anti_reverse> for SphereOnOrigin {
    fn div_assign(&mut self, _rhs: anti_reverse) {
        *self = self.anti_reverse()
    }
}
impl AntiReverse for SphereOnOrigin {
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
    //          add/sub      mul      div
    //   simd4        0        3        0
    // no simd        0       12        0
    fn anti_reverse(self) -> Self {
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            self.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e415, e425, e435, e321
            self.group1() * Simd32x4::from(-1.0),
            // e235, e315, e125, e5
            self.group2() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e1, e2, e3, e4
            self.group3(),
        );
    }
}
impl std::ops::Div<anti_reverse> for VersorEvenAligningOrigin {
    type Output = VersorEvenAligningOrigin;
    fn div(self, _rhs: anti_reverse) -> Self::Output {
        self.anti_reverse()
    }
}
impl std::ops::DivAssign<anti_reverse> for VersorEvenAligningOrigin {
    fn div_assign(&mut self, _rhs: anti_reverse) {
        *self = self.anti_reverse()
    }
}
impl AntiReverse for VersorEvenAligningOrigin {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        3        0
    // no simd        0       12        0
    fn anti_reverse(self) -> Self {
        return VersorEvenAligningOrigin::from_groups(
            // e423, e431, e412, e12345
            self.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e415, e425, e435, e4
            self.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e235, e315, e125, e5
            self.group2() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
        );
    }
}
impl std::ops::Div<anti_reverse> for VersorEvenAtInfinity {
    type Output = VersorEvenAtInfinity;
    fn div(self, _rhs: anti_reverse) -> Self::Output {
        self.anti_reverse()
    }
}
impl std::ops::DivAssign<anti_reverse> for VersorEvenAtInfinity {
    fn div_assign(&mut self, _rhs: anti_reverse) {
        *self = self.anti_reverse()
    }
}
impl AntiReverse for VersorEvenAtInfinity {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        2        0
    // no simd        0        8        0
    fn anti_reverse(self) -> Self {
        return VersorEvenAtInfinity::from_groups(
            // e12345, e1, e2, e3
            self.group0(),
            // e415, e425, e435, e321
            self.group1() * Simd32x4::from(-1.0),
            // e235, e315, e125, e5
            self.group2() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
        );
    }
}
impl std::ops::Div<anti_reverse> for VersorEvenAtOrigin {
    type Output = VersorEvenAtOrigin;
    fn div(self, _rhs: anti_reverse) -> Self::Output {
        self.anti_reverse()
    }
}
impl std::ops::DivAssign<anti_reverse> for VersorEvenAtOrigin {
    fn div_assign(&mut self, _rhs: anti_reverse) {
        *self = self.anti_reverse()
    }
}
impl AntiReverse for VersorEvenAtOrigin {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        2        0
    // no simd        0        8        0
    fn anti_reverse(self) -> Self {
        return VersorEvenAtOrigin::from_groups(
            // e423, e431, e412, e4
            self.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e235, e315, e125, e5
            self.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
        );
    }
}
impl std::ops::Div<anti_reverse> for VersorEvenOnOrigin {
    type Output = VersorEvenOnOrigin;
    fn div(self, _rhs: anti_reverse) -> Self::Output {
        self.anti_reverse()
    }
}
impl std::ops::DivAssign<anti_reverse> for VersorEvenOnOrigin {
    fn div_assign(&mut self, _rhs: anti_reverse) {
        *self = self.anti_reverse()
    }
}
impl AntiReverse for VersorEvenOnOrigin {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        2        0
    // no simd        0        8        0
    fn anti_reverse(self) -> Self {
        return VersorEvenOnOrigin::from_groups(
            // e423, e431, e412, e12345
            self.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e415, e425, e435, e4
            self.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
        );
    }
}
impl std::ops::Div<anti_reverse> for VersorEvenOrthogonalOrigin {
    type Output = VersorEvenOrthogonalOrigin;
    fn div(self, _rhs: anti_reverse) -> Self::Output {
        self.anti_reverse()
    }
}
impl std::ops::DivAssign<anti_reverse> for VersorEvenOrthogonalOrigin {
    fn div_assign(&mut self, _rhs: anti_reverse) {
        *self = self.anti_reverse()
    }
}
impl AntiReverse for VersorEvenOrthogonalOrigin {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        2        0
    // no simd        0        8        0
    fn anti_reverse(self) -> Self {
        return VersorEvenOrthogonalOrigin::from_groups(
            // e423, e431, e412, e321
            self.group0() * Simd32x4::from(-1.0),
            // e235, e315, e125, e5
            self.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e1, e2, e3, e4
            self.group2(),
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
    //          add/sub      mul      div
    //   simd4        0        3        0
    // no simd        0       12        0
    fn anti_reverse(self) -> Self {
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
impl std::ops::Div<anti_reverse> for VersorOddAtInfinity {
    type Output = VersorOddAtInfinity;
    fn div(self, _rhs: anti_reverse) -> Self::Output {
        self.anti_reverse()
    }
}
impl std::ops::DivAssign<anti_reverse> for VersorOddAtInfinity {
    fn div_assign(&mut self, _rhs: anti_reverse) {
        *self = self.anti_reverse()
    }
}
impl AntiReverse for VersorOddAtInfinity {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        2        0
    // no simd        0        8        0
    fn anti_reverse(self) -> Self {
        return VersorOddAtInfinity::from_groups(
            // scalar, e15, e25, e35
            self.group0() * Simd32x4::from([1.0, -1.0, -1.0, -1.0]),
            // e23, e31, e12, e45
            self.group1() * Simd32x4::from(-1.0),
            // e4235, e4315, e4125, e3215
            self.group2(),
        );
    }
}
impl std::ops::Div<anti_reverse> for VersorOddOrthogonalOrigin {
    type Output = VersorOddOrthogonalOrigin;
    fn div(self, _rhs: anti_reverse) -> Self::Output {
        self.anti_reverse()
    }
}
impl std::ops::DivAssign<anti_reverse> for VersorOddOrthogonalOrigin {
    fn div_assign(&mut self, _rhs: anti_reverse) {
        *self = self.anti_reverse()
    }
}
impl AntiReverse for VersorOddOrthogonalOrigin {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        3        0
    // no simd        0       12        0
    fn anti_reverse(self) -> Self {
        return VersorOddOrthogonalOrigin::from_groups(
            // e41, e42, e43, scalar
            self.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e23, e31, e12, e3215
            self.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e15, e25, e35, e1234
            self.group2() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
        );
    }
}
