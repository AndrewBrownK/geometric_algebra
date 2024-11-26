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
//  Maximum:         0       8       0
//
//  No SIMD:   add/sub     mul     div
//  Minimum:         0       0       0
//   Median:         0       4       0
//  Average:         0       4       0
//  Maximum:         0      22       0
impl std::ops::Div<conformal_conjugate> for AntiCircleOnOrigin {
    type Output = AntiCircleOnOrigin;
    fn div(self, _rhs: conformal_conjugate) -> Self::Output {
        self.conformal_conjugate()
    }
}
impl std::ops::DivAssign<conformal_conjugate> for AntiCircleOnOrigin {
    fn div_assign(&mut self, _rhs: conformal_conjugate) {
        *self = self.conformal_conjugate()
    }
}
impl ConformalConjugate for AntiCircleOnOrigin {
    fn conformal_conjugate(self) -> Self {
        return self;
    }
}
impl std::ops::Div<conformal_conjugate> for AntiCircleRotor {
    type Output = AntiCircleRotor;
    fn div(self, _rhs: conformal_conjugate) -> Self::Output {
        self.conformal_conjugate()
    }
}
impl std::ops::DivAssign<conformal_conjugate> for AntiCircleRotor {
    fn div_assign(&mut self, _rhs: conformal_conjugate) {
        *self = self.conformal_conjugate()
    }
}
impl ConformalConjugate for AntiCircleRotor {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        2        0
    // no simd        0        8        0
    fn conformal_conjugate(self) -> Self {
        return AntiCircleRotor::from_groups(
            // e41, e42, e43
            self.group0(),
            // e23, e31, e12, e45
            self.group1() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e15, e25, e35, scalar
            self.group2() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
        );
    }
}
impl std::ops::Div<conformal_conjugate> for AntiCircleRotorAligningOrigin {
    type Output = AntiCircleRotorAligningOrigin;
    fn div(self, _rhs: conformal_conjugate) -> Self::Output {
        self.conformal_conjugate()
    }
}
impl std::ops::DivAssign<conformal_conjugate> for AntiCircleRotorAligningOrigin {
    fn div_assign(&mut self, _rhs: conformal_conjugate) {
        *self = self.conformal_conjugate()
    }
}
impl ConformalConjugate for AntiCircleRotorAligningOrigin {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn conformal_conjugate(self) -> Self {
        return AntiCircleRotorAligningOrigin::from_groups(
            // e41, e42, e43
            self.group0(),
            // e23, e31, e12
            self.group1(),
            // e15, e25, e35, scalar
            self.group2() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
        );
    }
}
impl std::ops::Div<conformal_conjugate> for AntiCircleRotorAligningOriginAtInfinity {
    type Output = AntiCircleRotorAligningOriginAtInfinity;
    fn div(self, _rhs: conformal_conjugate) -> Self::Output {
        self.conformal_conjugate()
    }
}
impl std::ops::DivAssign<conformal_conjugate> for AntiCircleRotorAligningOriginAtInfinity {
    fn div_assign(&mut self, _rhs: conformal_conjugate) {
        *self = self.conformal_conjugate()
    }
}
impl ConformalConjugate for AntiCircleRotorAligningOriginAtInfinity {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn conformal_conjugate(self) -> Self {
        return AntiCircleRotorAligningOriginAtInfinity::from_groups(
            // e23, e31, e12
            self.group0(),
            // e15, e25, e35, scalar
            self.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
        );
    }
}
impl std::ops::Div<conformal_conjugate> for AntiCircleRotorAtInfinity {
    type Output = AntiCircleRotorAtInfinity;
    fn div(self, _rhs: conformal_conjugate) -> Self::Output {
        self.conformal_conjugate()
    }
}
impl std::ops::DivAssign<conformal_conjugate> for AntiCircleRotorAtInfinity {
    fn div_assign(&mut self, _rhs: conformal_conjugate) {
        *self = self.conformal_conjugate()
    }
}
impl ConformalConjugate for AntiCircleRotorAtInfinity {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        2        0
    // no simd        0        8        0
    fn conformal_conjugate(self) -> Self {
        return AntiCircleRotorAtInfinity::from_groups(
            // e23, e31, e12, e45
            self.group0() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e15, e25, e35, scalar
            self.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
        );
    }
}
impl std::ops::Div<conformal_conjugate> for AntiCircleRotorOnOrigin {
    type Output = AntiCircleRotorOnOrigin;
    fn div(self, _rhs: conformal_conjugate) -> Self::Output {
        self.conformal_conjugate()
    }
}
impl std::ops::DivAssign<conformal_conjugate> for AntiCircleRotorOnOrigin {
    fn div_assign(&mut self, _rhs: conformal_conjugate) {
        *self = self.conformal_conjugate()
    }
}
impl ConformalConjugate for AntiCircleRotorOnOrigin {
    fn conformal_conjugate(self) -> Self {
        return self;
    }
}
impl std::ops::Div<conformal_conjugate> for AntiDipoleInversion {
    type Output = AntiDipoleInversion;
    fn div(self, _rhs: conformal_conjugate) -> Self::Output {
        self.conformal_conjugate()
    }
}
impl std::ops::DivAssign<conformal_conjugate> for AntiDipoleInversion {
    fn div_assign(&mut self, _rhs: conformal_conjugate) {
        *self = self.conformal_conjugate()
    }
}
impl ConformalConjugate for AntiDipoleInversion {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        3        0
    // no simd        0       12        0
    fn conformal_conjugate(self) -> Self {
        return AntiDipoleInversion::from_groups(
            // e423, e431, e412
            self.group0(),
            // e415, e425, e435, e321
            self.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e235, e315, e125, e4
            self.group2() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e1, e2, e3, e5
            self.group3() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
        );
    }
}
impl std::ops::Div<conformal_conjugate> for AntiDipoleInversionAtInfinity {
    type Output = AntiDipoleInversionAtInfinity;
    fn div(self, _rhs: conformal_conjugate) -> Self::Output {
        self.conformal_conjugate()
    }
}
impl std::ops::DivAssign<conformal_conjugate> for AntiDipoleInversionAtInfinity {
    fn div_assign(&mut self, _rhs: conformal_conjugate) {
        *self = self.conformal_conjugate()
    }
}
impl ConformalConjugate for AntiDipoleInversionAtInfinity {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        0        1        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        0        3        0
    //  no simd        0       11        0
    fn conformal_conjugate(self) -> Self {
        return AntiDipoleInversionAtInfinity::from_groups(
            // e415, e425, e435, e321
            self.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e235, e315, e125
            self.group1() * Simd32x3::from(-1.0),
            // e1, e2, e3, e5
            self.group2() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
        );
    }
}
impl std::ops::Div<conformal_conjugate> for AntiDipoleInversionOnOrigin {
    type Output = AntiDipoleInversionOnOrigin;
    fn div(self, _rhs: conformal_conjugate) -> Self::Output {
        self.conformal_conjugate()
    }
}
impl std::ops::DivAssign<conformal_conjugate> for AntiDipoleInversionOnOrigin {
    fn div_assign(&mut self, _rhs: conformal_conjugate) {
        *self = self.conformal_conjugate()
    }
}
impl ConformalConjugate for AntiDipoleInversionOnOrigin {
    fn conformal_conjugate(self) -> Self {
        return self;
    }
}
impl std::ops::Div<conformal_conjugate> for AntiDipoleInversionOrthogonalOrigin {
    type Output = AntiDipoleInversionOrthogonalOrigin;
    fn div(self, _rhs: conformal_conjugate) -> Self::Output {
        self.conformal_conjugate()
    }
}
impl std::ops::DivAssign<conformal_conjugate> for AntiDipoleInversionOrthogonalOrigin {
    fn div_assign(&mut self, _rhs: conformal_conjugate) {
        *self = self.conformal_conjugate()
    }
}
impl ConformalConjugate for AntiDipoleInversionOrthogonalOrigin {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        0        1        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        0        3        0
    //  no simd        0       11        0
    fn conformal_conjugate(self) -> Self {
        return AntiDipoleInversionOrthogonalOrigin::from_groups(
            // e423, e431, e412, e5
            self.group0() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e415, e425, e435
            self.group1() * Simd32x3::from(-1.0),
            // e235, e315, e125, e4
            self.group2() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
        );
    }
}
impl std::ops::Div<conformal_conjugate> for AntiDipoleOnOrigin {
    type Output = AntiDipoleOnOrigin;
    fn div(self, _rhs: conformal_conjugate) -> Self::Output {
        self.conformal_conjugate()
    }
}
impl std::ops::DivAssign<conformal_conjugate> for AntiDipoleOnOrigin {
    fn div_assign(&mut self, _rhs: conformal_conjugate) {
        *self = self.conformal_conjugate()
    }
}
impl ConformalConjugate for AntiDipoleOnOrigin {
    fn conformal_conjugate(self) -> Self {
        return self;
    }
}
impl std::ops::Div<conformal_conjugate> for AntiDualNum {
    type Output = AntiDualNum;
    fn div(self, _rhs: conformal_conjugate) -> Self::Output {
        self.conformal_conjugate()
    }
}
impl std::ops::DivAssign<conformal_conjugate> for AntiDualNum {
    fn div_assign(&mut self, _rhs: conformal_conjugate) {
        *self = self.conformal_conjugate()
    }
}
impl ConformalConjugate for AntiDualNum {
    fn conformal_conjugate(self) -> Self {
        return self;
    }
}
impl std::ops::Div<conformal_conjugate> for AntiFlatOrigin {
    type Output = AntiFlatOrigin;
    fn div(self, _rhs: conformal_conjugate) -> Self::Output {
        self.conformal_conjugate()
    }
}
impl std::ops::DivAssign<conformal_conjugate> for AntiFlatOrigin {
    fn div_assign(&mut self, _rhs: conformal_conjugate) {
        *self = self.conformal_conjugate()
    }
}
impl ConformalConjugate for AntiFlatOrigin {
    fn conformal_conjugate(self) -> Self {
        return self;
    }
}
impl std::ops::Div<conformal_conjugate> for AntiFlatPoint {
    type Output = AntiFlatPoint;
    fn div(self, _rhs: conformal_conjugate) -> Self::Output {
        self.conformal_conjugate()
    }
}
impl std::ops::DivAssign<conformal_conjugate> for AntiFlatPoint {
    fn div_assign(&mut self, _rhs: conformal_conjugate) {
        *self = self.conformal_conjugate()
    }
}
impl ConformalConjugate for AntiFlatPoint {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn conformal_conjugate(self) -> Self {
        return AntiFlatPoint::from_groups(/* e235, e315, e125, e321 */ self.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]));
    }
}
impl std::ops::Div<conformal_conjugate> for AntiFlector {
    type Output = AntiFlector;
    fn div(self, _rhs: conformal_conjugate) -> Self::Output {
        self.conformal_conjugate()
    }
}
impl std::ops::DivAssign<conformal_conjugate> for AntiFlector {
    fn div_assign(&mut self, _rhs: conformal_conjugate) {
        *self = self.conformal_conjugate()
    }
}
impl ConformalConjugate for AntiFlector {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        2        0
    // no simd        0        8        0
    fn conformal_conjugate(self) -> Self {
        return AntiFlector::from_groups(
            // e235, e315, e125, e321
            self.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e1, e2, e3, e5
            self.group1() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
        );
    }
}
impl std::ops::Div<conformal_conjugate> for AntiFlectorOnOrigin {
    type Output = AntiFlectorOnOrigin;
    fn div(self, _rhs: conformal_conjugate) -> Self::Output {
        self.conformal_conjugate()
    }
}
impl std::ops::DivAssign<conformal_conjugate> for AntiFlectorOnOrigin {
    fn div_assign(&mut self, _rhs: conformal_conjugate) {
        *self = self.conformal_conjugate()
    }
}
impl ConformalConjugate for AntiFlectorOnOrigin {
    fn conformal_conjugate(self) -> Self {
        return self;
    }
}
impl std::ops::Div<conformal_conjugate> for AntiLine {
    type Output = AntiLine;
    fn div(self, _rhs: conformal_conjugate) -> Self::Output {
        self.conformal_conjugate()
    }
}
impl std::ops::DivAssign<conformal_conjugate> for AntiLine {
    fn div_assign(&mut self, _rhs: conformal_conjugate) {
        *self = self.conformal_conjugate()
    }
}
impl ConformalConjugate for AntiLine {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        1        0
    // no simd        0        3        0
    fn conformal_conjugate(self) -> Self {
        return AntiLine::from_groups(/* e23, e31, e12 */ self.group0(), /* e15, e25, e35 */ self.group1() * Simd32x3::from(-1.0));
    }
}
impl std::ops::Div<conformal_conjugate> for AntiLineOnOrigin {
    type Output = AntiLineOnOrigin;
    fn div(self, _rhs: conformal_conjugate) -> Self::Output {
        self.conformal_conjugate()
    }
}
impl std::ops::DivAssign<conformal_conjugate> for AntiLineOnOrigin {
    fn div_assign(&mut self, _rhs: conformal_conjugate) {
        *self = self.conformal_conjugate()
    }
}
impl ConformalConjugate for AntiLineOnOrigin {
    fn conformal_conjugate(self) -> Self {
        return self;
    }
}
impl std::ops::Div<conformal_conjugate> for AntiMotor {
    type Output = AntiMotor;
    fn div(self, _rhs: conformal_conjugate) -> Self::Output {
        self.conformal_conjugate()
    }
}
impl std::ops::DivAssign<conformal_conjugate> for AntiMotor {
    fn div_assign(&mut self, _rhs: conformal_conjugate) {
        *self = self.conformal_conjugate()
    }
}
impl ConformalConjugate for AntiMotor {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn conformal_conjugate(self) -> Self {
        return AntiMotor::from_groups(/* e23, e31, e12, scalar */ self.group0(), /* e15, e25, e35, e3215 */ self.group1() * Simd32x4::from(-1.0));
    }
}
impl std::ops::Div<conformal_conjugate> for AntiMotorOnOrigin {
    type Output = AntiMotorOnOrigin;
    fn div(self, _rhs: conformal_conjugate) -> Self::Output {
        self.conformal_conjugate()
    }
}
impl std::ops::DivAssign<conformal_conjugate> for AntiMotorOnOrigin {
    fn div_assign(&mut self, _rhs: conformal_conjugate) {
        *self = self.conformal_conjugate()
    }
}
impl ConformalConjugate for AntiMotorOnOrigin {
    fn conformal_conjugate(self) -> Self {
        return self;
    }
}
impl std::ops::Div<conformal_conjugate> for AntiMysteryCircleRotor {
    type Output = AntiMysteryCircleRotor;
    fn div(self, _rhs: conformal_conjugate) -> Self::Output {
        self.conformal_conjugate()
    }
}
impl std::ops::DivAssign<conformal_conjugate> for AntiMysteryCircleRotor {
    fn div_assign(&mut self, _rhs: conformal_conjugate) {
        *self = self.conformal_conjugate()
    }
}
impl ConformalConjugate for AntiMysteryCircleRotor {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn conformal_conjugate(self) -> Self {
        use crate::elements::*;
        return AntiMysteryCircleRotor::from_groups(/* e23, e31, e12, e45 */ self.group0() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]), /* scalar */ self[scalar]);
    }
}
impl std::ops::Div<conformal_conjugate> for AntiMysteryDipoleInversion {
    type Output = AntiMysteryDipoleInversion;
    fn div(self, _rhs: conformal_conjugate) -> Self::Output {
        self.conformal_conjugate()
    }
}
impl std::ops::DivAssign<conformal_conjugate> for AntiMysteryDipoleInversion {
    fn div_assign(&mut self, _rhs: conformal_conjugate) {
        *self = self.conformal_conjugate()
    }
}
impl ConformalConjugate for AntiMysteryDipoleInversion {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn conformal_conjugate(self) -> Self {
        return AntiMysteryDipoleInversion::from_groups(
            // e415, e425, e435, e321
            self.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e1, e2, e3
            self.group1(),
        );
    }
}
impl std::ops::Div<conformal_conjugate> for AntiPlane {
    type Output = AntiPlane;
    fn div(self, _rhs: conformal_conjugate) -> Self::Output {
        self.conformal_conjugate()
    }
}
impl std::ops::DivAssign<conformal_conjugate> for AntiPlane {
    fn div_assign(&mut self, _rhs: conformal_conjugate) {
        *self = self.conformal_conjugate()
    }
}
impl ConformalConjugate for AntiPlane {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn conformal_conjugate(self) -> Self {
        return AntiPlane::from_groups(/* e1, e2, e3, e5 */ self.group0() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]));
    }
}
impl std::ops::Div<conformal_conjugate> for AntiPlaneOnOrigin {
    type Output = AntiPlaneOnOrigin;
    fn div(self, _rhs: conformal_conjugate) -> Self::Output {
        self.conformal_conjugate()
    }
}
impl std::ops::DivAssign<conformal_conjugate> for AntiPlaneOnOrigin {
    fn div_assign(&mut self, _rhs: conformal_conjugate) {
        *self = self.conformal_conjugate()
    }
}
impl ConformalConjugate for AntiPlaneOnOrigin {
    fn conformal_conjugate(self) -> Self {
        return self;
    }
}
impl std::ops::Div<conformal_conjugate> for AntiScalar {
    type Output = AntiScalar;
    fn div(self, _rhs: conformal_conjugate) -> Self::Output {
        self.conformal_conjugate()
    }
}
impl std::ops::DivAssign<conformal_conjugate> for AntiScalar {
    fn div_assign(&mut self, _rhs: conformal_conjugate) {
        *self = self.conformal_conjugate()
    }
}
impl ConformalConjugate for AntiScalar {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn conformal_conjugate(self) -> Self {
        use crate::elements::*;
        return AntiScalar::from_groups(/* e12345 */ self[e12345] * -1.0);
    }
}
impl std::ops::Div<conformal_conjugate> for AntiSphereOnOrigin {
    type Output = AntiSphereOnOrigin;
    fn div(self, _rhs: conformal_conjugate) -> Self::Output {
        self.conformal_conjugate()
    }
}
impl std::ops::DivAssign<conformal_conjugate> for AntiSphereOnOrigin {
    fn div_assign(&mut self, _rhs: conformal_conjugate) {
        *self = self.conformal_conjugate()
    }
}
impl ConformalConjugate for AntiSphereOnOrigin {
    fn conformal_conjugate(self) -> Self {
        return self;
    }
}
impl std::ops::Div<conformal_conjugate> for AntiVersorEvenOnOrigin {
    type Output = AntiVersorEvenOnOrigin;
    fn div(self, _rhs: conformal_conjugate) -> Self::Output {
        self.conformal_conjugate()
    }
}
impl std::ops::DivAssign<conformal_conjugate> for AntiVersorEvenOnOrigin {
    fn div_assign(&mut self, _rhs: conformal_conjugate) {
        *self = self.conformal_conjugate()
    }
}
impl ConformalConjugate for AntiVersorEvenOnOrigin {
    fn conformal_conjugate(self) -> Self {
        return self;
    }
}
impl std::ops::Div<conformal_conjugate> for Circle {
    type Output = Circle;
    fn div(self, _rhs: conformal_conjugate) -> Self::Output {
        self.conformal_conjugate()
    }
}
impl std::ops::DivAssign<conformal_conjugate> for Circle {
    fn div_assign(&mut self, _rhs: conformal_conjugate) {
        *self = self.conformal_conjugate()
    }
}
impl ConformalConjugate for Circle {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        2        0
    //  no simd        0        7        0
    fn conformal_conjugate(self) -> Self {
        return Circle::from_groups(
            // e423, e431, e412
            self.group0(),
            // e415, e425, e435, e321
            self.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e235, e315, e125
            self.group2() * Simd32x3::from(-1.0),
        );
    }
}
impl std::ops::Div<conformal_conjugate> for CircleAligningOrigin {
    type Output = CircleAligningOrigin;
    fn div(self, _rhs: conformal_conjugate) -> Self::Output {
        self.conformal_conjugate()
    }
}
impl std::ops::DivAssign<conformal_conjugate> for CircleAligningOrigin {
    fn div_assign(&mut self, _rhs: conformal_conjugate) {
        *self = self.conformal_conjugate()
    }
}
impl ConformalConjugate for CircleAligningOrigin {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        2        0
    // no simd        0        6        0
    fn conformal_conjugate(self) -> Self {
        return CircleAligningOrigin::from_groups(
            // e423, e431, e412
            self.group0(),
            // e415, e425, e435
            self.group1() * Simd32x3::from(-1.0),
            // e235, e315, e125
            self.group2() * Simd32x3::from(-1.0),
        );
    }
}
impl std::ops::Div<conformal_conjugate> for CircleAtInfinity {
    type Output = CircleAtInfinity;
    fn div(self, _rhs: conformal_conjugate) -> Self::Output {
        self.conformal_conjugate()
    }
}
impl std::ops::DivAssign<conformal_conjugate> for CircleAtInfinity {
    fn div_assign(&mut self, _rhs: conformal_conjugate) {
        *self = self.conformal_conjugate()
    }
}
impl ConformalConjugate for CircleAtInfinity {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        2        0
    //  no simd        0        7        0
    fn conformal_conjugate(self) -> Self {
        return CircleAtInfinity::from_groups(
            // e415, e425, e435, e321
            self.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e235, e315, e125
            self.group1() * Simd32x3::from(-1.0),
        );
    }
}
impl std::ops::Div<conformal_conjugate> for CircleAtOrigin {
    type Output = CircleAtOrigin;
    fn div(self, _rhs: conformal_conjugate) -> Self::Output {
        self.conformal_conjugate()
    }
}
impl std::ops::DivAssign<conformal_conjugate> for CircleAtOrigin {
    fn div_assign(&mut self, _rhs: conformal_conjugate) {
        *self = self.conformal_conjugate()
    }
}
impl ConformalConjugate for CircleAtOrigin {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        1        0
    // no simd        0        3        0
    fn conformal_conjugate(self) -> Self {
        return CircleAtOrigin::from_groups(/* e423, e431, e412 */ self.group0(), /* e235, e315, e125 */ self.group1() * Simd32x3::from(-1.0));
    }
}
impl std::ops::Div<conformal_conjugate> for CircleOnOrigin {
    type Output = CircleOnOrigin;
    fn div(self, _rhs: conformal_conjugate) -> Self::Output {
        self.conformal_conjugate()
    }
}
impl std::ops::DivAssign<conformal_conjugate> for CircleOnOrigin {
    fn div_assign(&mut self, _rhs: conformal_conjugate) {
        *self = self.conformal_conjugate()
    }
}
impl ConformalConjugate for CircleOnOrigin {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        1        0
    // no simd        0        3        0
    fn conformal_conjugate(self) -> Self {
        return CircleOnOrigin::from_groups(/* e423, e431, e412 */ self.group0(), /* e415, e425, e435 */ self.group1() * Simd32x3::from(-1.0));
    }
}
impl std::ops::Div<conformal_conjugate> for CircleOrthogonalOrigin {
    type Output = CircleOrthogonalOrigin;
    fn div(self, _rhs: conformal_conjugate) -> Self::Output {
        self.conformal_conjugate()
    }
}
impl std::ops::DivAssign<conformal_conjugate> for CircleOrthogonalOrigin {
    fn div_assign(&mut self, _rhs: conformal_conjugate) {
        *self = self.conformal_conjugate()
    }
}
impl ConformalConjugate for CircleOrthogonalOrigin {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        1        0
    // no simd        0        3        0
    fn conformal_conjugate(self) -> Self {
        return CircleOrthogonalOrigin::from_groups(/* e423, e431, e412, e321 */ self.group0(), /* e235, e315, e125 */ self.group1() * Simd32x3::from(-1.0));
    }
}
impl std::ops::Div<conformal_conjugate> for CircleRotor {
    type Output = CircleRotor;
    fn div(self, _rhs: conformal_conjugate) -> Self::Output {
        self.conformal_conjugate()
    }
}
impl std::ops::DivAssign<conformal_conjugate> for CircleRotor {
    fn div_assign(&mut self, _rhs: conformal_conjugate) {
        *self = self.conformal_conjugate()
    }
}
impl ConformalConjugate for CircleRotor {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        2        0
    // no simd        0        8        0
    fn conformal_conjugate(self) -> Self {
        return CircleRotor::from_groups(
            // e423, e431, e412
            self.group0(),
            // e415, e425, e435, e321
            self.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e235, e315, e125, e12345
            self.group2() * Simd32x4::from(-1.0),
        );
    }
}
impl std::ops::Div<conformal_conjugate> for CircleRotorAligningOrigin {
    type Output = CircleRotorAligningOrigin;
    fn div(self, _rhs: conformal_conjugate) -> Self::Output {
        self.conformal_conjugate()
    }
}
impl std::ops::DivAssign<conformal_conjugate> for CircleRotorAligningOrigin {
    fn div_assign(&mut self, _rhs: conformal_conjugate) {
        *self = self.conformal_conjugate()
    }
}
impl ConformalConjugate for CircleRotorAligningOrigin {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        2        0
    //  no simd        0        7        0
    fn conformal_conjugate(self) -> Self {
        return CircleRotorAligningOrigin::from_groups(
            // e423, e431, e412
            self.group0(),
            // e415, e425, e435
            self.group1() * Simd32x3::from(-1.0),
            // e235, e315, e125, e12345
            self.group2() * Simd32x4::from(-1.0),
        );
    }
}
impl std::ops::Div<conformal_conjugate> for CircleRotorAligningOriginAtInfinity {
    type Output = CircleRotorAligningOriginAtInfinity;
    fn div(self, _rhs: conformal_conjugate) -> Self::Output {
        self.conformal_conjugate()
    }
}
impl std::ops::DivAssign<conformal_conjugate> for CircleRotorAligningOriginAtInfinity {
    fn div_assign(&mut self, _rhs: conformal_conjugate) {
        *self = self.conformal_conjugate()
    }
}
impl ConformalConjugate for CircleRotorAligningOriginAtInfinity {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        2        0
    //  no simd        0        7        0
    fn conformal_conjugate(self) -> Self {
        return CircleRotorAligningOriginAtInfinity::from_groups(
            // e415, e425, e435
            self.group0() * Simd32x3::from(-1.0),
            // e235, e315, e125, e12345
            self.group1() * Simd32x4::from(-1.0),
        );
    }
}
impl std::ops::Div<conformal_conjugate> for CircleRotorAtInfinity {
    type Output = CircleRotorAtInfinity;
    fn div(self, _rhs: conformal_conjugate) -> Self::Output {
        self.conformal_conjugate()
    }
}
impl std::ops::DivAssign<conformal_conjugate> for CircleRotorAtInfinity {
    fn div_assign(&mut self, _rhs: conformal_conjugate) {
        *self = self.conformal_conjugate()
    }
}
impl ConformalConjugate for CircleRotorAtInfinity {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        2        0
    // no simd        0        8        0
    fn conformal_conjugate(self) -> Self {
        return CircleRotorAtInfinity::from_groups(
            // e415, e425, e435, e321
            self.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e235, e315, e125, e12345
            self.group1() * Simd32x4::from(-1.0),
        );
    }
}
impl std::ops::Div<conformal_conjugate> for CircleRotorOnOrigin {
    type Output = CircleRotorOnOrigin;
    fn div(self, _rhs: conformal_conjugate) -> Self::Output {
        self.conformal_conjugate()
    }
}
impl std::ops::DivAssign<conformal_conjugate> for CircleRotorOnOrigin {
    fn div_assign(&mut self, _rhs: conformal_conjugate) {
        *self = self.conformal_conjugate()
    }
}
impl ConformalConjugate for CircleRotorOnOrigin {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        2        0
    //  no simd        0        7        0
    fn conformal_conjugate(self) -> Self {
        return CircleRotorOnOrigin::from_groups(
            // e423, e431, e412, e12345
            self.group0() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e415, e425, e435
            self.group1() * Simd32x3::from(-1.0),
        );
    }
}
impl std::ops::Div<conformal_conjugate> for Dipole {
    type Output = Dipole;
    fn div(self, _rhs: conformal_conjugate) -> Self::Output {
        self.conformal_conjugate()
    }
}
impl std::ops::DivAssign<conformal_conjugate> for Dipole {
    fn div_assign(&mut self, _rhs: conformal_conjugate) {
        *self = self.conformal_conjugate()
    }
}
impl ConformalConjugate for Dipole {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        2        0
    //  no simd        0        7        0
    fn conformal_conjugate(self) -> Self {
        return Dipole::from_groups(
            // e41, e42, e43
            self.group0(),
            // e23, e31, e12, e45
            self.group1() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e15, e25, e35
            self.group2() * Simd32x3::from(-1.0),
        );
    }
}
impl std::ops::Div<conformal_conjugate> for DipoleAligningOrigin {
    type Output = DipoleAligningOrigin;
    fn div(self, _rhs: conformal_conjugate) -> Self::Output {
        self.conformal_conjugate()
    }
}
impl std::ops::DivAssign<conformal_conjugate> for DipoleAligningOrigin {
    fn div_assign(&mut self, _rhs: conformal_conjugate) {
        *self = self.conformal_conjugate()
    }
}
impl ConformalConjugate for DipoleAligningOrigin {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        2        0
    //  no simd        0        7        0
    fn conformal_conjugate(self) -> Self {
        return DipoleAligningOrigin::from_groups(
            // e41, e42, e43, e45
            self.group0() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e15, e25, e35
            self.group1() * Simd32x3::from(-1.0),
        );
    }
}
impl std::ops::Div<conformal_conjugate> for DipoleAtInfinity {
    type Output = DipoleAtInfinity;
    fn div(self, _rhs: conformal_conjugate) -> Self::Output {
        self.conformal_conjugate()
    }
}
impl std::ops::DivAssign<conformal_conjugate> for DipoleAtInfinity {
    fn div_assign(&mut self, _rhs: conformal_conjugate) {
        *self = self.conformal_conjugate()
    }
}
impl ConformalConjugate for DipoleAtInfinity {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        2        0
    //  no simd        0        7        0
    fn conformal_conjugate(self) -> Self {
        return DipoleAtInfinity::from_groups(
            // e23, e31, e12, e45
            self.group0() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e15, e25, e35
            self.group1() * Simd32x3::from(-1.0),
        );
    }
}
impl std::ops::Div<conformal_conjugate> for DipoleAtOrigin {
    type Output = DipoleAtOrigin;
    fn div(self, _rhs: conformal_conjugate) -> Self::Output {
        self.conformal_conjugate()
    }
}
impl std::ops::DivAssign<conformal_conjugate> for DipoleAtOrigin {
    fn div_assign(&mut self, _rhs: conformal_conjugate) {
        *self = self.conformal_conjugate()
    }
}
impl ConformalConjugate for DipoleAtOrigin {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        1        0
    // no simd        0        3        0
    fn conformal_conjugate(self) -> Self {
        return DipoleAtOrigin::from_groups(/* e41, e42, e43 */ self.group0(), /* e15, e25, e35 */ self.group1() * Simd32x3::from(-1.0));
    }
}
impl std::ops::Div<conformal_conjugate> for DipoleInversion {
    type Output = DipoleInversion;
    fn div(self, _rhs: conformal_conjugate) -> Self::Output {
        self.conformal_conjugate()
    }
}
impl std::ops::DivAssign<conformal_conjugate> for DipoleInversion {
    fn div_assign(&mut self, _rhs: conformal_conjugate) {
        *self = self.conformal_conjugate()
    }
}
impl ConformalConjugate for DipoleInversion {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        3        0
    // no simd        0       12        0
    fn conformal_conjugate(self) -> Self {
        return DipoleInversion::from_groups(
            // e41, e42, e43
            self.group0(),
            // e23, e31, e12, e45
            self.group1() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e15, e25, e35, e1234
            self.group2() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e4235, e4315, e4125, e3215
            self.group3() * Simd32x4::from(-1.0),
        );
    }
}
impl std::ops::Div<conformal_conjugate> for DipoleInversionAligningOrigin {
    type Output = DipoleInversionAligningOrigin;
    fn div(self, _rhs: conformal_conjugate) -> Self::Output {
        self.conformal_conjugate()
    }
}
impl std::ops::DivAssign<conformal_conjugate> for DipoleInversionAligningOrigin {
    fn div_assign(&mut self, _rhs: conformal_conjugate) {
        *self = self.conformal_conjugate()
    }
}
impl ConformalConjugate for DipoleInversionAligningOrigin {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        3        0
    // no simd        0       12        0
    fn conformal_conjugate(self) -> Self {
        return DipoleInversionAligningOrigin::from_groups(
            // e41, e42, e43, e45
            self.group0() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e15, e25, e35, e1234
            self.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e4235, e4315, e4125, e3215
            self.group2() * Simd32x4::from(-1.0),
        );
    }
}
impl std::ops::Div<conformal_conjugate> for DipoleInversionAtInfinity {
    type Output = DipoleInversionAtInfinity;
    fn div(self, _rhs: conformal_conjugate) -> Self::Output {
        self.conformal_conjugate()
    }
}
impl std::ops::DivAssign<conformal_conjugate> for DipoleInversionAtInfinity {
    fn div_assign(&mut self, _rhs: conformal_conjugate) {
        *self = self.conformal_conjugate()
    }
}
impl ConformalConjugate for DipoleInversionAtInfinity {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        0        1        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        0        3        0
    //  no simd        0       11        0
    fn conformal_conjugate(self) -> Self {
        return DipoleInversionAtInfinity::from_groups(
            // e23, e31, e12, e45
            self.group0() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e15, e25, e35
            self.group1() * Simd32x3::from(-1.0),
            // e4235, e4315, e4125, e3215
            self.group2() * Simd32x4::from(-1.0),
        );
    }
}
impl std::ops::Div<conformal_conjugate> for DipoleInversionAtOrigin {
    type Output = DipoleInversionAtOrigin;
    fn div(self, _rhs: conformal_conjugate) -> Self::Output {
        self.conformal_conjugate()
    }
}
impl std::ops::DivAssign<conformal_conjugate> for DipoleInversionAtOrigin {
    fn div_assign(&mut self, _rhs: conformal_conjugate) {
        *self = self.conformal_conjugate()
    }
}
impl ConformalConjugate for DipoleInversionAtOrigin {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        2        0
    // no simd        0        8        0
    fn conformal_conjugate(self) -> Self {
        return DipoleInversionAtOrigin::from_groups(
            // e41, e42, e43, e3215
            self.group0() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e15, e25, e35, e1234
            self.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
        );
    }
}
impl std::ops::Div<conformal_conjugate> for DipoleInversionOnOrigin {
    type Output = DipoleInversionOnOrigin;
    fn div(self, _rhs: conformal_conjugate) -> Self::Output {
        self.conformal_conjugate()
    }
}
impl std::ops::DivAssign<conformal_conjugate> for DipoleInversionOnOrigin {
    fn div_assign(&mut self, _rhs: conformal_conjugate) {
        *self = self.conformal_conjugate()
    }
}
impl ConformalConjugate for DipoleInversionOnOrigin {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        2        0
    // no simd        0        8        0
    fn conformal_conjugate(self) -> Self {
        return DipoleInversionOnOrigin::from_groups(
            // e41, e42, e43, e45
            self.group0() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e1234, e4235, e4315, e4125
            self.group1() * Simd32x4::from([1.0, -1.0, -1.0, -1.0]),
        );
    }
}
impl std::ops::Div<conformal_conjugate> for DipoleInversionOrthogonalOrigin {
    type Output = DipoleInversionOrthogonalOrigin;
    fn div(self, _rhs: conformal_conjugate) -> Self::Output {
        self.conformal_conjugate()
    }
}
impl std::ops::DivAssign<conformal_conjugate> for DipoleInversionOrthogonalOrigin {
    fn div_assign(&mut self, _rhs: conformal_conjugate) {
        *self = self.conformal_conjugate()
    }
}
impl ConformalConjugate for DipoleInversionOrthogonalOrigin {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        2        0
    // no simd        0        8        0
    fn conformal_conjugate(self) -> Self {
        return DipoleInversionOrthogonalOrigin::from_groups(
            // e41, e42, e43, e3215
            self.group0() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e23, e31, e12
            self.group1(),
            // e15, e25, e35, e1234
            self.group2() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
        );
    }
}
impl std::ops::Div<conformal_conjugate> for DipoleOnOrigin {
    type Output = DipoleOnOrigin;
    fn div(self, _rhs: conformal_conjugate) -> Self::Output {
        self.conformal_conjugate()
    }
}
impl std::ops::DivAssign<conformal_conjugate> for DipoleOnOrigin {
    fn div_assign(&mut self, _rhs: conformal_conjugate) {
        *self = self.conformal_conjugate()
    }
}
impl ConformalConjugate for DipoleOnOrigin {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn conformal_conjugate(self) -> Self {
        return DipoleOnOrigin::from_groups(/* e41, e42, e43, e45 */ self.group0() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]));
    }
}
impl std::ops::Div<conformal_conjugate> for DipoleOrthogonalOrigin {
    type Output = DipoleOrthogonalOrigin;
    fn div(self, _rhs: conformal_conjugate) -> Self::Output {
        self.conformal_conjugate()
    }
}
impl std::ops::DivAssign<conformal_conjugate> for DipoleOrthogonalOrigin {
    fn div_assign(&mut self, _rhs: conformal_conjugate) {
        *self = self.conformal_conjugate()
    }
}
impl ConformalConjugate for DipoleOrthogonalOrigin {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        1        0
    // no simd        0        3        0
    fn conformal_conjugate(self) -> Self {
        return DipoleOrthogonalOrigin::from_groups(
            // e41, e42, e43
            self.group0(),
            // e23, e31, e12
            self.group1(),
            // e15, e25, e35
            self.group2() * Simd32x3::from(-1.0),
        );
    }
}
impl std::ops::Div<conformal_conjugate> for DualNum {
    type Output = DualNum;
    fn div(self, _rhs: conformal_conjugate) -> Self::Output {
        self.conformal_conjugate()
    }
}
impl std::ops::DivAssign<conformal_conjugate> for DualNum {
    fn div_assign(&mut self, _rhs: conformal_conjugate) {
        *self = self.conformal_conjugate()
    }
}
impl ConformalConjugate for DualNum {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd2        0        1        0
    // no simd        0        2        0
    fn conformal_conjugate(self) -> Self {
        return DualNum::from_groups(/* e4, e12345 */ self.group0() * Simd32x2::from([1.0, -1.0]));
    }
}
impl std::ops::Div<conformal_conjugate> for FlatOrigin {
    type Output = FlatOrigin;
    fn div(self, _rhs: conformal_conjugate) -> Self::Output {
        self.conformal_conjugate()
    }
}
impl std::ops::DivAssign<conformal_conjugate> for FlatOrigin {
    fn div_assign(&mut self, _rhs: conformal_conjugate) {
        *self = self.conformal_conjugate()
    }
}
impl ConformalConjugate for FlatOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn conformal_conjugate(self) -> Self {
        use crate::elements::*;
        return FlatOrigin::from_groups(/* e45 */ self[e45] * -1.0);
    }
}
impl std::ops::Div<conformal_conjugate> for FlatPoint {
    type Output = FlatPoint;
    fn div(self, _rhs: conformal_conjugate) -> Self::Output {
        self.conformal_conjugate()
    }
}
impl std::ops::DivAssign<conformal_conjugate> for FlatPoint {
    fn div_assign(&mut self, _rhs: conformal_conjugate) {
        *self = self.conformal_conjugate()
    }
}
impl ConformalConjugate for FlatPoint {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn conformal_conjugate(self) -> Self {
        return FlatPoint::from_groups(/* e15, e25, e35, e45 */ self.group0() * Simd32x4::from(-1.0));
    }
}
impl std::ops::Div<conformal_conjugate> for FlatPointAtInfinity {
    type Output = FlatPointAtInfinity;
    fn div(self, _rhs: conformal_conjugate) -> Self::Output {
        self.conformal_conjugate()
    }
}
impl std::ops::DivAssign<conformal_conjugate> for FlatPointAtInfinity {
    fn div_assign(&mut self, _rhs: conformal_conjugate) {
        *self = self.conformal_conjugate()
    }
}
impl ConformalConjugate for FlatPointAtInfinity {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        1        0
    // no simd        0        3        0
    fn conformal_conjugate(self) -> Self {
        return FlatPointAtInfinity::from_groups(/* e15, e25, e35 */ self.group0() * Simd32x3::from(-1.0));
    }
}
impl std::ops::Div<conformal_conjugate> for Flector {
    type Output = Flector;
    fn div(self, _rhs: conformal_conjugate) -> Self::Output {
        self.conformal_conjugate()
    }
}
impl std::ops::DivAssign<conformal_conjugate> for Flector {
    fn div_assign(&mut self, _rhs: conformal_conjugate) {
        *self = self.conformal_conjugate()
    }
}
impl ConformalConjugate for Flector {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        2        0
    // no simd        0        8        0
    fn conformal_conjugate(self) -> Self {
        return Flector::from_groups(
            // e15, e25, e35, e45
            self.group0() * Simd32x4::from(-1.0),
            // e4235, e4315, e4125, e3215
            self.group1() * Simd32x4::from(-1.0),
        );
    }
}
impl std::ops::Div<conformal_conjugate> for FlectorAtInfinity {
    type Output = FlectorAtInfinity;
    fn div(self, _rhs: conformal_conjugate) -> Self::Output {
        self.conformal_conjugate()
    }
}
impl std::ops::DivAssign<conformal_conjugate> for FlectorAtInfinity {
    fn div_assign(&mut self, _rhs: conformal_conjugate) {
        *self = self.conformal_conjugate()
    }
}
impl ConformalConjugate for FlectorAtInfinity {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn conformal_conjugate(self) -> Self {
        return FlectorAtInfinity::from_groups(/* e15, e25, e35, e3215 */ self.group0() * Simd32x4::from(-1.0));
    }
}
impl std::ops::Div<conformal_conjugate> for FlectorOnOrigin {
    type Output = FlectorOnOrigin;
    fn div(self, _rhs: conformal_conjugate) -> Self::Output {
        self.conformal_conjugate()
    }
}
impl std::ops::DivAssign<conformal_conjugate> for FlectorOnOrigin {
    fn div_assign(&mut self, _rhs: conformal_conjugate) {
        *self = self.conformal_conjugate()
    }
}
impl ConformalConjugate for FlectorOnOrigin {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn conformal_conjugate(self) -> Self {
        return FlectorOnOrigin::from_groups(/* e45, e4235, e4315, e4125 */ self.group0() * Simd32x4::from(-1.0));
    }
}
impl std::ops::Div<conformal_conjugate> for Horizon {
    type Output = Horizon;
    fn div(self, _rhs: conformal_conjugate) -> Self::Output {
        self.conformal_conjugate()
    }
}
impl std::ops::DivAssign<conformal_conjugate> for Horizon {
    fn div_assign(&mut self, _rhs: conformal_conjugate) {
        *self = self.conformal_conjugate()
    }
}
impl ConformalConjugate for Horizon {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn conformal_conjugate(self) -> Self {
        use crate::elements::*;
        return Horizon::from_groups(/* e3215 */ self[e3215] * -1.0);
    }
}
impl std::ops::Div<conformal_conjugate> for Infinity {
    type Output = Infinity;
    fn div(self, _rhs: conformal_conjugate) -> Self::Output {
        self.conformal_conjugate()
    }
}
impl std::ops::DivAssign<conformal_conjugate> for Infinity {
    fn div_assign(&mut self, _rhs: conformal_conjugate) {
        *self = self.conformal_conjugate()
    }
}
impl ConformalConjugate for Infinity {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn conformal_conjugate(self) -> Self {
        use crate::elements::*;
        return Infinity::from_groups(/* e5 */ self[e5] * -1.0);
    }
}
impl std::ops::Div<conformal_conjugate> for Line {
    type Output = Line;
    fn div(self, _rhs: conformal_conjugate) -> Self::Output {
        self.conformal_conjugate()
    }
}
impl std::ops::DivAssign<conformal_conjugate> for Line {
    fn div_assign(&mut self, _rhs: conformal_conjugate) {
        *self = self.conformal_conjugate()
    }
}
impl ConformalConjugate for Line {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        2        0
    // no simd        0        6        0
    fn conformal_conjugate(self) -> Self {
        return Line::from_groups(
            // e415, e425, e435
            self.group0() * Simd32x3::from(-1.0),
            // e235, e315, e125
            self.group1() * Simd32x3::from(-1.0),
        );
    }
}
impl std::ops::Div<conformal_conjugate> for LineAtInfinity {
    type Output = LineAtInfinity;
    fn div(self, _rhs: conformal_conjugate) -> Self::Output {
        self.conformal_conjugate()
    }
}
impl std::ops::DivAssign<conformal_conjugate> for LineAtInfinity {
    fn div_assign(&mut self, _rhs: conformal_conjugate) {
        *self = self.conformal_conjugate()
    }
}
impl ConformalConjugate for LineAtInfinity {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        1        0
    // no simd        0        3        0
    fn conformal_conjugate(self) -> Self {
        return LineAtInfinity::from_groups(/* e235, e315, e125 */ self.group0() * Simd32x3::from(-1.0));
    }
}
impl std::ops::Div<conformal_conjugate> for LineOnOrigin {
    type Output = LineOnOrigin;
    fn div(self, _rhs: conformal_conjugate) -> Self::Output {
        self.conformal_conjugate()
    }
}
impl std::ops::DivAssign<conformal_conjugate> for LineOnOrigin {
    fn div_assign(&mut self, _rhs: conformal_conjugate) {
        *self = self.conformal_conjugate()
    }
}
impl ConformalConjugate for LineOnOrigin {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        1        0
    // no simd        0        3        0
    fn conformal_conjugate(self) -> Self {
        return LineOnOrigin::from_groups(/* e415, e425, e435 */ self.group0() * Simd32x3::from(-1.0));
    }
}
impl std::ops::Div<conformal_conjugate> for Motor {
    type Output = Motor;
    fn div(self, _rhs: conformal_conjugate) -> Self::Output {
        self.conformal_conjugate()
    }
}
impl std::ops::DivAssign<conformal_conjugate> for Motor {
    fn div_assign(&mut self, _rhs: conformal_conjugate) {
        *self = self.conformal_conjugate()
    }
}
impl ConformalConjugate for Motor {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        2        0
    // no simd        0        8        0
    fn conformal_conjugate(self) -> Self {
        return Motor::from_groups(
            // e415, e425, e435, e12345
            self.group0() * Simd32x4::from(-1.0),
            // e235, e315, e125, e5
            self.group1() * Simd32x4::from(-1.0),
        );
    }
}
impl std::ops::Div<conformal_conjugate> for MotorAtInfinity {
    type Output = MotorAtInfinity;
    fn div(self, _rhs: conformal_conjugate) -> Self::Output {
        self.conformal_conjugate()
    }
}
impl std::ops::DivAssign<conformal_conjugate> for MotorAtInfinity {
    fn div_assign(&mut self, _rhs: conformal_conjugate) {
        *self = self.conformal_conjugate()
    }
}
impl ConformalConjugate for MotorAtInfinity {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn conformal_conjugate(self) -> Self {
        return MotorAtInfinity::from_groups(/* e235, e315, e125, e5 */ self.group0() * Simd32x4::from(-1.0));
    }
}
impl std::ops::Div<conformal_conjugate> for MotorOnOrigin {
    type Output = MotorOnOrigin;
    fn div(self, _rhs: conformal_conjugate) -> Self::Output {
        self.conformal_conjugate()
    }
}
impl std::ops::DivAssign<conformal_conjugate> for MotorOnOrigin {
    fn div_assign(&mut self, _rhs: conformal_conjugate) {
        *self = self.conformal_conjugate()
    }
}
impl ConformalConjugate for MotorOnOrigin {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn conformal_conjugate(self) -> Self {
        return MotorOnOrigin::from_groups(/* e415, e425, e435, e12345 */ self.group0() * Simd32x4::from(-1.0));
    }
}
impl std::ops::Div<conformal_conjugate> for MultiVector {
    type Output = MultiVector;
    fn div(self, _rhs: conformal_conjugate) -> Self::Output {
        self.conformal_conjugate()
    }
}
impl std::ops::DivAssign<conformal_conjugate> for MultiVector {
    fn div_assign(&mut self, _rhs: conformal_conjugate) {
        *self = self.conformal_conjugate()
    }
}
impl ConformalConjugate for MultiVector {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        2        0
    //    simd2        0        1        0
    //    simd3        0        2        0
    //    simd4        0        3        0
    // Totals...
    // yes simd        0        8        0
    //  no simd        0       22        0
    fn conformal_conjugate(self) -> Self {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            self.group0() * Simd32x2::from([1.0, -1.0]),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5] * -1.0,
            // e41, e42, e43, e45
            self.group3() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e15, e25, e35
            self.group4() * Simd32x3::from(-1.0),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            self.group6() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8() * Simd32x3::from(-1.0),
            // e1234, e4235, e4315, e4125
            self.group9() * Simd32x4::from([1.0, -1.0, -1.0, -1.0]),
            // e3215
            self[e3215] * -1.0,
        );
    }
}
impl std::ops::Div<conformal_conjugate> for MysteryCircle {
    type Output = MysteryCircle;
    fn div(self, _rhs: conformal_conjugate) -> Self::Output {
        self.conformal_conjugate()
    }
}
impl std::ops::DivAssign<conformal_conjugate> for MysteryCircle {
    fn div_assign(&mut self, _rhs: conformal_conjugate) {
        *self = self.conformal_conjugate()
    }
}
impl ConformalConjugate for MysteryCircle {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn conformal_conjugate(self) -> Self {
        return MysteryCircle::from_groups(/* e415, e425, e435, e321 */ self.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]));
    }
}
impl std::ops::Div<conformal_conjugate> for MysteryCircleRotor {
    type Output = MysteryCircleRotor;
    fn div(self, _rhs: conformal_conjugate) -> Self::Output {
        self.conformal_conjugate()
    }
}
impl std::ops::DivAssign<conformal_conjugate> for MysteryCircleRotor {
    fn div_assign(&mut self, _rhs: conformal_conjugate) {
        *self = self.conformal_conjugate()
    }
}
impl ConformalConjugate for MysteryCircleRotor {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        2        0
    //  no simd        0        5        0
    fn conformal_conjugate(self) -> Self {
        use crate::elements::*;
        return MysteryCircleRotor::from_groups(
            // e415, e425, e435, e321
            self.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e12345
            self[e12345] * -1.0,
        );
    }
}
impl std::ops::Div<conformal_conjugate> for MysteryDipole {
    type Output = MysteryDipole;
    fn div(self, _rhs: conformal_conjugate) -> Self::Output {
        self.conformal_conjugate()
    }
}
impl std::ops::DivAssign<conformal_conjugate> for MysteryDipole {
    fn div_assign(&mut self, _rhs: conformal_conjugate) {
        *self = self.conformal_conjugate()
    }
}
impl ConformalConjugate for MysteryDipole {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn conformal_conjugate(self) -> Self {
        return MysteryDipole::from_groups(/* e23, e31, e12, e45 */ self.group0() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]));
    }
}
impl std::ops::Div<conformal_conjugate> for MysteryDipoleInversion {
    type Output = MysteryDipoleInversion;
    fn div(self, _rhs: conformal_conjugate) -> Self::Output {
        self.conformal_conjugate()
    }
}
impl std::ops::DivAssign<conformal_conjugate> for MysteryDipoleInversion {
    fn div_assign(&mut self, _rhs: conformal_conjugate) {
        *self = self.conformal_conjugate()
    }
}
impl ConformalConjugate for MysteryDipoleInversion {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        2        0
    //  no simd        0        7        0
    fn conformal_conjugate(self) -> Self {
        return MysteryDipoleInversion::from_groups(
            // e23, e31, e12, e45
            self.group0() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e4235, e4315, e4125
            self.group1() * Simd32x3::from(-1.0),
        );
    }
}
impl std::ops::Div<conformal_conjugate> for MysteryVersorEven {
    type Output = MysteryVersorEven;
    fn div(self, _rhs: conformal_conjugate) -> Self::Output {
        self.conformal_conjugate()
    }
}
impl std::ops::DivAssign<conformal_conjugate> for MysteryVersorEven {
    fn div_assign(&mut self, _rhs: conformal_conjugate) {
        *self = self.conformal_conjugate()
    }
}
impl ConformalConjugate for MysteryVersorEven {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        2        0
    // no simd        0        8        0
    fn conformal_conjugate(self) -> Self {
        return MysteryVersorEven::from_groups(
            // e12345, e1, e2, e3
            self.group0() * Simd32x4::from([-1.0, 1.0, 1.0, 1.0]),
            // e415, e425, e435, e321
            self.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
        );
    }
}
impl std::ops::Div<conformal_conjugate> for MysteryVersorOdd {
    type Output = MysteryVersorOdd;
    fn div(self, _rhs: conformal_conjugate) -> Self::Output {
        self.conformal_conjugate()
    }
}
impl std::ops::DivAssign<conformal_conjugate> for MysteryVersorOdd {
    fn div_assign(&mut self, _rhs: conformal_conjugate) {
        *self = self.conformal_conjugate()
    }
}
impl ConformalConjugate for MysteryVersorOdd {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        2        0
    // no simd        0        8        0
    fn conformal_conjugate(self) -> Self {
        return MysteryVersorOdd::from_groups(
            // scalar, e4235, e4315, e4125
            self.group0() * Simd32x4::from([1.0, -1.0, -1.0, -1.0]),
            // e23, e31, e12, e45
            self.group1() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
        );
    }
}
impl std::ops::Div<conformal_conjugate> for NullCircleAtOrigin {
    type Output = NullCircleAtOrigin;
    fn div(self, _rhs: conformal_conjugate) -> Self::Output {
        self.conformal_conjugate()
    }
}
impl std::ops::DivAssign<conformal_conjugate> for NullCircleAtOrigin {
    fn div_assign(&mut self, _rhs: conformal_conjugate) {
        *self = self.conformal_conjugate()
    }
}
impl ConformalConjugate for NullCircleAtOrigin {
    fn conformal_conjugate(self) -> Self {
        return self;
    }
}
impl std::ops::Div<conformal_conjugate> for NullDipoleAtOrigin {
    type Output = NullDipoleAtOrigin;
    fn div(self, _rhs: conformal_conjugate) -> Self::Output {
        self.conformal_conjugate()
    }
}
impl std::ops::DivAssign<conformal_conjugate> for NullDipoleAtOrigin {
    fn div_assign(&mut self, _rhs: conformal_conjugate) {
        *self = self.conformal_conjugate()
    }
}
impl ConformalConjugate for NullDipoleAtOrigin {
    fn conformal_conjugate(self) -> Self {
        return self;
    }
}
impl std::ops::Div<conformal_conjugate> for NullDipoleInversionAtOrigin {
    type Output = NullDipoleInversionAtOrigin;
    fn div(self, _rhs: conformal_conjugate) -> Self::Output {
        self.conformal_conjugate()
    }
}
impl std::ops::DivAssign<conformal_conjugate> for NullDipoleInversionAtOrigin {
    fn div_assign(&mut self, _rhs: conformal_conjugate) {
        *self = self.conformal_conjugate()
    }
}
impl ConformalConjugate for NullDipoleInversionAtOrigin {
    fn conformal_conjugate(self) -> Self {
        return self;
    }
}
impl std::ops::Div<conformal_conjugate> for NullSphereAtOrigin {
    type Output = NullSphereAtOrigin;
    fn div(self, _rhs: conformal_conjugate) -> Self::Output {
        self.conformal_conjugate()
    }
}
impl std::ops::DivAssign<conformal_conjugate> for NullSphereAtOrigin {
    fn div_assign(&mut self, _rhs: conformal_conjugate) {
        *self = self.conformal_conjugate()
    }
}
impl ConformalConjugate for NullSphereAtOrigin {
    fn conformal_conjugate(self) -> Self {
        return self;
    }
}
impl std::ops::Div<conformal_conjugate> for NullVersorEvenAtOrigin {
    type Output = NullVersorEvenAtOrigin;
    fn div(self, _rhs: conformal_conjugate) -> Self::Output {
        self.conformal_conjugate()
    }
}
impl std::ops::DivAssign<conformal_conjugate> for NullVersorEvenAtOrigin {
    fn div_assign(&mut self, _rhs: conformal_conjugate) {
        *self = self.conformal_conjugate()
    }
}
impl ConformalConjugate for NullVersorEvenAtOrigin {
    fn conformal_conjugate(self) -> Self {
        return self;
    }
}
impl std::ops::Div<conformal_conjugate> for Origin {
    type Output = Origin;
    fn div(self, _rhs: conformal_conjugate) -> Self::Output {
        self.conformal_conjugate()
    }
}
impl std::ops::DivAssign<conformal_conjugate> for Origin {
    fn div_assign(&mut self, _rhs: conformal_conjugate) {
        *self = self.conformal_conjugate()
    }
}
impl ConformalConjugate for Origin {
    fn conformal_conjugate(self) -> Self {
        return self;
    }
}
impl std::ops::Div<conformal_conjugate> for Plane {
    type Output = Plane;
    fn div(self, _rhs: conformal_conjugate) -> Self::Output {
        self.conformal_conjugate()
    }
}
impl std::ops::DivAssign<conformal_conjugate> for Plane {
    fn div_assign(&mut self, _rhs: conformal_conjugate) {
        *self = self.conformal_conjugate()
    }
}
impl ConformalConjugate for Plane {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn conformal_conjugate(self) -> Self {
        return Plane::from_groups(/* e4235, e4315, e4125, e3215 */ self.group0() * Simd32x4::from(-1.0));
    }
}
impl std::ops::Div<conformal_conjugate> for PlaneOnOrigin {
    type Output = PlaneOnOrigin;
    fn div(self, _rhs: conformal_conjugate) -> Self::Output {
        self.conformal_conjugate()
    }
}
impl std::ops::DivAssign<conformal_conjugate> for PlaneOnOrigin {
    fn div_assign(&mut self, _rhs: conformal_conjugate) {
        *self = self.conformal_conjugate()
    }
}
impl ConformalConjugate for PlaneOnOrigin {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        1        0
    // no simd        0        3        0
    fn conformal_conjugate(self) -> Self {
        return PlaneOnOrigin::from_groups(/* e4235, e4315, e4125 */ self.group0() * Simd32x3::from(-1.0));
    }
}
impl std::ops::Div<conformal_conjugate> for RoundPoint {
    type Output = RoundPoint;
    fn div(self, _rhs: conformal_conjugate) -> Self::Output {
        self.conformal_conjugate()
    }
}
impl std::ops::DivAssign<conformal_conjugate> for RoundPoint {
    fn div_assign(&mut self, _rhs: conformal_conjugate) {
        *self = self.conformal_conjugate()
    }
}
impl ConformalConjugate for RoundPoint {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn conformal_conjugate(self) -> Self {
        use crate::elements::*;
        return RoundPoint::from_groups(/* e1, e2, e3, e4 */ self.group0(), /* e5 */ self[e5] * -1.0);
    }
}
impl std::ops::Div<conformal_conjugate> for RoundPointAtOrigin {
    type Output = RoundPointAtOrigin;
    fn div(self, _rhs: conformal_conjugate) -> Self::Output {
        self.conformal_conjugate()
    }
}
impl std::ops::DivAssign<conformal_conjugate> for RoundPointAtOrigin {
    fn div_assign(&mut self, _rhs: conformal_conjugate) {
        *self = self.conformal_conjugate()
    }
}
impl ConformalConjugate for RoundPointAtOrigin {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd2        0        1        0
    // no simd        0        2        0
    fn conformal_conjugate(self) -> Self {
        return RoundPointAtOrigin::from_groups(/* e4, e5 */ self.group0() * Simd32x2::from([1.0, -1.0]));
    }
}
impl std::ops::Div<conformal_conjugate> for Scalar {
    type Output = Scalar;
    fn div(self, _rhs: conformal_conjugate) -> Self::Output {
        self.conformal_conjugate()
    }
}
impl std::ops::DivAssign<conformal_conjugate> for Scalar {
    fn div_assign(&mut self, _rhs: conformal_conjugate) {
        *self = self.conformal_conjugate()
    }
}
impl ConformalConjugate for Scalar {
    fn conformal_conjugate(self) -> Self {
        return self;
    }
}
impl std::ops::Div<conformal_conjugate> for Sphere {
    type Output = Sphere;
    fn div(self, _rhs: conformal_conjugate) -> Self::Output {
        self.conformal_conjugate()
    }
}
impl std::ops::DivAssign<conformal_conjugate> for Sphere {
    fn div_assign(&mut self, _rhs: conformal_conjugate) {
        *self = self.conformal_conjugate()
    }
}
impl ConformalConjugate for Sphere {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn conformal_conjugate(self) -> Self {
        use crate::elements::*;
        return Sphere::from_groups(/* e4235, e4315, e4125, e3215 */ self.group0() * Simd32x4::from(-1.0), /* e1234 */ self[e1234]);
    }
}
impl std::ops::Div<conformal_conjugate> for SphereAtOrigin {
    type Output = SphereAtOrigin;
    fn div(self, _rhs: conformal_conjugate) -> Self::Output {
        self.conformal_conjugate()
    }
}
impl std::ops::DivAssign<conformal_conjugate> for SphereAtOrigin {
    fn div_assign(&mut self, _rhs: conformal_conjugate) {
        *self = self.conformal_conjugate()
    }
}
impl ConformalConjugate for SphereAtOrigin {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd2        0        1        0
    // no simd        0        2        0
    fn conformal_conjugate(self) -> Self {
        return SphereAtOrigin::from_groups(/* e3215, e1234 */ self.group0() * Simd32x2::from([-1.0, 1.0]));
    }
}
impl std::ops::Div<conformal_conjugate> for SphereOnOrigin {
    type Output = SphereOnOrigin;
    fn div(self, _rhs: conformal_conjugate) -> Self::Output {
        self.conformal_conjugate()
    }
}
impl std::ops::DivAssign<conformal_conjugate> for SphereOnOrigin {
    fn div_assign(&mut self, _rhs: conformal_conjugate) {
        *self = self.conformal_conjugate()
    }
}
impl ConformalConjugate for SphereOnOrigin {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn conformal_conjugate(self) -> Self {
        return SphereOnOrigin::from_groups(/* e4235, e4315, e4125, e1234 */ self.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]));
    }
}
impl std::ops::Div<conformal_conjugate> for VersorEven {
    type Output = VersorEven;
    fn div(self, _rhs: conformal_conjugate) -> Self::Output {
        self.conformal_conjugate()
    }
}
impl std::ops::DivAssign<conformal_conjugate> for VersorEven {
    fn div_assign(&mut self, _rhs: conformal_conjugate) {
        *self = self.conformal_conjugate()
    }
}
impl ConformalConjugate for VersorEven {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        3        0
    // no simd        0       12        0
    fn conformal_conjugate(self) -> Self {
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            self.group0() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e415, e425, e435, e321
            self.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e235, e315, e125, e5
            self.group2() * Simd32x4::from(-1.0),
            // e1, e2, e3, e4
            self.group3(),
        );
    }
}
impl std::ops::Div<conformal_conjugate> for VersorEvenAligningOrigin {
    type Output = VersorEvenAligningOrigin;
    fn div(self, _rhs: conformal_conjugate) -> Self::Output {
        self.conformal_conjugate()
    }
}
impl std::ops::DivAssign<conformal_conjugate> for VersorEvenAligningOrigin {
    fn div_assign(&mut self, _rhs: conformal_conjugate) {
        *self = self.conformal_conjugate()
    }
}
impl ConformalConjugate for VersorEvenAligningOrigin {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        3        0
    // no simd        0       12        0
    fn conformal_conjugate(self) -> Self {
        return VersorEvenAligningOrigin::from_groups(
            // e423, e431, e412, e12345
            self.group0() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e415, e425, e435, e4
            self.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e235, e315, e125, e5
            self.group2() * Simd32x4::from(-1.0),
        );
    }
}
impl std::ops::Div<conformal_conjugate> for VersorEvenAtInfinity {
    type Output = VersorEvenAtInfinity;
    fn div(self, _rhs: conformal_conjugate) -> Self::Output {
        self.conformal_conjugate()
    }
}
impl std::ops::DivAssign<conformal_conjugate> for VersorEvenAtInfinity {
    fn div_assign(&mut self, _rhs: conformal_conjugate) {
        *self = self.conformal_conjugate()
    }
}
impl ConformalConjugate for VersorEvenAtInfinity {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        3        0
    // no simd        0       12        0
    fn conformal_conjugate(self) -> Self {
        return VersorEvenAtInfinity::from_groups(
            // e12345, e1, e2, e3
            self.group0() * Simd32x4::from([-1.0, 1.0, 1.0, 1.0]),
            // e415, e425, e435, e321
            self.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e235, e315, e125, e5
            self.group2() * Simd32x4::from(-1.0),
        );
    }
}
impl std::ops::Div<conformal_conjugate> for VersorEvenAtOrigin {
    type Output = VersorEvenAtOrigin;
    fn div(self, _rhs: conformal_conjugate) -> Self::Output {
        self.conformal_conjugate()
    }
}
impl std::ops::DivAssign<conformal_conjugate> for VersorEvenAtOrigin {
    fn div_assign(&mut self, _rhs: conformal_conjugate) {
        *self = self.conformal_conjugate()
    }
}
impl ConformalConjugate for VersorEvenAtOrigin {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn conformal_conjugate(self) -> Self {
        return VersorEvenAtOrigin::from_groups(/* e423, e431, e412, e4 */ self.group0(), /* e235, e315, e125, e5 */ self.group1() * Simd32x4::from(-1.0));
    }
}
impl std::ops::Div<conformal_conjugate> for VersorEvenOnOrigin {
    type Output = VersorEvenOnOrigin;
    fn div(self, _rhs: conformal_conjugate) -> Self::Output {
        self.conformal_conjugate()
    }
}
impl std::ops::DivAssign<conformal_conjugate> for VersorEvenOnOrigin {
    fn div_assign(&mut self, _rhs: conformal_conjugate) {
        *self = self.conformal_conjugate()
    }
}
impl ConformalConjugate for VersorEvenOnOrigin {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        2        0
    // no simd        0        8        0
    fn conformal_conjugate(self) -> Self {
        return VersorEvenOnOrigin::from_groups(
            // e423, e431, e412, e12345
            self.group0() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e415, e425, e435, e4
            self.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
        );
    }
}
impl std::ops::Div<conformal_conjugate> for VersorEvenOrthogonalOrigin {
    type Output = VersorEvenOrthogonalOrigin;
    fn div(self, _rhs: conformal_conjugate) -> Self::Output {
        self.conformal_conjugate()
    }
}
impl std::ops::DivAssign<conformal_conjugate> for VersorEvenOrthogonalOrigin {
    fn div_assign(&mut self, _rhs: conformal_conjugate) {
        *self = self.conformal_conjugate()
    }
}
impl ConformalConjugate for VersorEvenOrthogonalOrigin {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn conformal_conjugate(self) -> Self {
        return VersorEvenOrthogonalOrigin::from_groups(
            // e423, e431, e412, e321
            self.group0(),
            // e235, e315, e125, e5
            self.group1() * Simd32x4::from(-1.0),
            // e1, e2, e3, e4
            self.group2(),
        );
    }
}
impl std::ops::Div<conformal_conjugate> for VersorOdd {
    type Output = VersorOdd;
    fn div(self, _rhs: conformal_conjugate) -> Self::Output {
        self.conformal_conjugate()
    }
}
impl std::ops::DivAssign<conformal_conjugate> for VersorOdd {
    fn div_assign(&mut self, _rhs: conformal_conjugate) {
        *self = self.conformal_conjugate()
    }
}
impl ConformalConjugate for VersorOdd {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        3        0
    // no simd        0       12        0
    fn conformal_conjugate(self) -> Self {
        return VersorOdd::from_groups(
            // e41, e42, e43, scalar
            self.group0(),
            // e23, e31, e12, e45
            self.group1() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e15, e25, e35, e1234
            self.group2() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e4235, e4315, e4125, e3215
            self.group3() * Simd32x4::from(-1.0),
        );
    }
}
impl std::ops::Div<conformal_conjugate> for VersorOddAtInfinity {
    type Output = VersorOddAtInfinity;
    fn div(self, _rhs: conformal_conjugate) -> Self::Output {
        self.conformal_conjugate()
    }
}
impl std::ops::DivAssign<conformal_conjugate> for VersorOddAtInfinity {
    fn div_assign(&mut self, _rhs: conformal_conjugate) {
        *self = self.conformal_conjugate()
    }
}
impl ConformalConjugate for VersorOddAtInfinity {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        3        0
    // no simd        0       12        0
    fn conformal_conjugate(self) -> Self {
        return VersorOddAtInfinity::from_groups(
            // scalar, e15, e25, e35
            self.group0() * Simd32x4::from([1.0, -1.0, -1.0, -1.0]),
            // e23, e31, e12, e45
            self.group1() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e4235, e4315, e4125, e3215
            self.group2() * Simd32x4::from(-1.0),
        );
    }
}
impl std::ops::Div<conformal_conjugate> for VersorOddOrthogonalOrigin {
    type Output = VersorOddOrthogonalOrigin;
    fn div(self, _rhs: conformal_conjugate) -> Self::Output {
        self.conformal_conjugate()
    }
}
impl std::ops::DivAssign<conformal_conjugate> for VersorOddOrthogonalOrigin {
    fn div_assign(&mut self, _rhs: conformal_conjugate) {
        *self = self.conformal_conjugate()
    }
}
impl ConformalConjugate for VersorOddOrthogonalOrigin {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        2        0
    // no simd        0        8        0
    fn conformal_conjugate(self) -> Self {
        return VersorOddOrthogonalOrigin::from_groups(
            // e41, e42, e43, scalar
            self.group0(),
            // e23, e31, e12, e3215
            self.group1() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e15, e25, e35, e1234
            self.group2() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
        );
    }
}
