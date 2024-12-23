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
//  Minimum:         0       1       1
//   Median:         3       2       1
//  Average:         4       5       1
//  Maximum:        23      27       1
//
//  No SIMD:   add/sub     mul     div
//  Minimum:         0       1       1
//   Median:         3       8       1
//  Average:         4      11       1
//  Maximum:        23      48       1
impl std::ops::Div<anti_inverse> for AntiCircleRotor {
    type Output = AntiCircleRotor;
    fn div(self, _rhs: anti_inverse) -> Self::Output {
        self.anti_inverse()
    }
}
impl std::ops::DivAssign<anti_inverse> for AntiCircleRotor {
    fn div_assign(&mut self, _rhs: anti_inverse) {
        *self = self.anti_inverse()
    }
}
impl AntiInverse for AntiCircleRotor {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        7        6        1
    //    simd3        0        1        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        7        9        1
    //  no simd        7       17        1
    fn anti_inverse(self) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(
            // e12345
            2.0 * (self[e41] * self[e15])
                + 2.0 * (self[e42] * self[e25])
                + 2.0 * (self[e43] * self[e35])
                + f32::powi(self[e23], 2)
                + f32::powi(self[e31], 2)
                + f32::powi(self[e12], 2)
                - f32::powi(self[e45], 2)
                - f32::powi(self[scalar], 2),
        );
        let other = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return AntiCircleRotor::from_groups(
            // e41, e42, e43
            Simd32x3::from(other[e12345]) * self.group0(),
            // e23, e31, e12, e45
            Simd32x4::from(other[e12345]) * self.group1(),
            // e15, e25, e35, scalar
            Simd32x4::from(other[e12345]) * self.group2(),
        );
    }
}
impl std::ops::Div<anti_inverse> for AntiDipoleInversion {
    type Output = AntiDipoleInversion;
    fn div(self, _rhs: anti_inverse) -> Self::Output {
        self.anti_inverse()
    }
}
impl std::ops::DivAssign<anti_inverse> for AntiDipoleInversion {
    fn div_assign(&mut self, _rhs: anti_inverse) {
        *self = self.anti_inverse()
    }
}
impl AntiInverse for AntiDipoleInversion {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       10        8        1
    //    simd3        0        1        0
    //    simd4        0        3        0
    // Totals...
    // yes simd       10       12        1
    //  no simd       10       23        1
    fn anti_inverse(self) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(
            // e12345
            2.0 * (self[e4] * self[e5]) + f32::powi(self[e321], 2)
                - f32::powi(self[e415], 2)
                - f32::powi(self[e425], 2)
                - f32::powi(self[e435], 2)
                - f32::powi(self[e1], 2)
                - f32::powi(self[e2], 2)
                - f32::powi(self[e3], 2)
                - 2.0 * (self[e423] * self[e235])
                - 2.0 * (self[e431] * self[e315])
                - 2.0 * (self[e412] * self[e125]),
        );
        let other = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return AntiDipoleInversion::from_groups(
            // e423, e431, e412
            Simd32x3::from(other[e12345]) * self.group0(),
            // e415, e425, e435, e321
            Simd32x4::from(other[e12345]) * self.group1(),
            // e235, e315, e125, e4
            Simd32x4::from(other[e12345]) * self.group2(),
            // e1, e2, e3, e5
            Simd32x4::from(other[e12345]) * self.group3(),
        );
    }
}
impl std::ops::Div<anti_inverse> for AntiDualNum {
    type Output = AntiDualNum;
    fn div(self, _rhs: anti_inverse) -> Self::Output {
        self.anti_inverse()
    }
}
impl std::ops::DivAssign<anti_inverse> for AntiDualNum {
    fn div_assign(&mut self, _rhs: anti_inverse) {
        *self = self.anti_inverse()
    }
}
impl AntiInverse for AntiDualNum {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        1
    //    simd2        0        1        0
    // Totals...
    // yes simd        0        2        1
    //  no simd        0        3        1
    fn anti_inverse(self) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(/* e12345 */ f32::powi(self[scalar], 2) * -1.0);
        let other = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return AntiDualNum::from_groups(/* e3215, scalar */ Simd32x2::from(other[e12345]) * self.group0());
    }
}
impl std::ops::Div<anti_inverse> for AntiFlatPoint {
    type Output = AntiFlatPoint;
    fn div(self, _rhs: anti_inverse) -> Self::Output {
        self.anti_inverse()
    }
}
impl std::ops::DivAssign<anti_inverse> for AntiFlatPoint {
    fn div_assign(&mut self, _rhs: anti_inverse) {
        *self = self.anti_inverse()
    }
}
impl AntiInverse for AntiFlatPoint {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        0        1
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        1        1
    //  no simd        0        4        1
    fn anti_inverse(self) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(/* e12345 */ f32::powi(self[e321], 2));
        let other = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return AntiFlatPoint::from_groups(/* e235, e315, e125, e321 */ Simd32x4::from(other[e12345]) * self.group0());
    }
}
impl std::ops::Div<anti_inverse> for AntiFlector {
    type Output = AntiFlector;
    fn div(self, _rhs: anti_inverse) -> Self::Output {
        self.anti_inverse()
    }
}
impl std::ops::DivAssign<anti_inverse> for AntiFlector {
    fn div_assign(&mut self, _rhs: anti_inverse) {
        *self = self.anti_inverse()
    }
}
impl AntiInverse for AntiFlector {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        0        1
    //    simd4        0        2        0
    // Totals...
    // yes simd        3        2        1
    //  no simd        3        8        1
    fn anti_inverse(self) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(/* e12345 */ f32::powi(self[e321], 2) - f32::powi(self[e1], 2) - f32::powi(self[e2], 2) - f32::powi(self[e3], 2));
        let other = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return AntiFlector::from_groups(
            // e235, e315, e125, e321
            Simd32x4::from(other[e12345]) * self.group0(),
            // e1, e2, e3, e5
            Simd32x4::from(other[e12345]) * self.group1(),
        );
    }
}
impl std::ops::Div<anti_inverse> for AntiLine {
    type Output = AntiLine;
    fn div(self, _rhs: anti_inverse) -> Self::Output {
        self.anti_inverse()
    }
}
impl std::ops::DivAssign<anti_inverse> for AntiLine {
    fn div_assign(&mut self, _rhs: anti_inverse) {
        *self = self.anti_inverse()
    }
}
impl AntiInverse for AntiLine {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        0        1
    //    simd3        0        2        0
    // Totals...
    // yes simd        2        2        1
    //  no simd        2        6        1
    fn anti_inverse(self) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(/* e12345 */ f32::powi(self[e23], 2) + f32::powi(self[e31], 2) + f32::powi(self[e12], 2));
        let other = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return AntiLine::from_groups(
            // e23, e31, e12
            Simd32x3::from(other[e12345]) * self.group0(),
            // e15, e25, e35
            Simd32x3::from(other[e12345]) * self.group1(),
        );
    }
}
impl std::ops::Div<anti_inverse> for AntiMotor {
    type Output = AntiMotor;
    fn div(self, _rhs: anti_inverse) -> Self::Output {
        self.anti_inverse()
    }
}
impl std::ops::DivAssign<anti_inverse> for AntiMotor {
    fn div_assign(&mut self, _rhs: anti_inverse) {
        *self = self.anti_inverse()
    }
}
impl AntiInverse for AntiMotor {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        0        1
    //    simd4        0        2        0
    // Totals...
    // yes simd        3        2        1
    //  no simd        3        8        1
    fn anti_inverse(self) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(
            // e12345
            f32::powi(self[e23], 2) + f32::powi(self[e31], 2) + f32::powi(self[e12], 2) - f32::powi(self[scalar], 2),
        );
        let other = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return AntiMotor::from_groups(
            // e23, e31, e12, scalar
            Simd32x4::from(other[e12345]) * self.group0(),
            // e15, e25, e35, e3215
            Simd32x4::from(other[e12345]) * self.group1(),
        );
    }
}
impl std::ops::Div<anti_inverse> for AntiPlane {
    type Output = AntiPlane;
    fn div(self, _rhs: anti_inverse) -> Self::Output {
        self.anti_inverse()
    }
}
impl std::ops::DivAssign<anti_inverse> for AntiPlane {
    fn div_assign(&mut self, _rhs: anti_inverse) {
        *self = self.anti_inverse()
    }
}
impl AntiInverse for AntiPlane {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        0        1
    //    simd4        0        1        0
    // Totals...
    // yes simd        2        1        1
    //  no simd        2        4        1
    fn anti_inverse(self) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(/* e12345 */ -f32::powi(self[e1], 2) - f32::powi(self[e2], 2) - f32::powi(self[e3], 2));
        let other = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return AntiPlane::from_groups(/* e1, e2, e3, e5 */ Simd32x4::from(other[e12345]) * self.group0());
    }
}
impl std::ops::Div<anti_inverse> for AntiScalar {
    type Output = AntiScalar;
    fn div(self, _rhs: anti_inverse) -> Self::Output {
        self.anti_inverse()
    }
}
impl std::ops::DivAssign<anti_inverse> for AntiScalar {
    fn div_assign(&mut self, _rhs: anti_inverse) {
        *self = self.anti_inverse()
    }
}
impl AntiInverse for AntiScalar {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        1
    fn anti_inverse(self) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(/* e12345 */ f32::powi(self[e12345], 2));
        let other = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return AntiScalar::from_groups(/* e12345 */ other[e12345] * self[e12345]);
    }
}
impl std::ops::Div<anti_inverse> for Circle {
    type Output = Circle;
    fn div(self, _rhs: anti_inverse) -> Self::Output {
        self.anti_inverse()
    }
}
impl std::ops::DivAssign<anti_inverse> for Circle {
    fn div_assign(&mut self, _rhs: anti_inverse) {
        *self = self.anti_inverse()
    }
}
impl AntiInverse for Circle {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        6        6        1
    //    simd3        0        2        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        6        9        1
    //  no simd        6       16        1
    fn anti_inverse(self) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(
            // e12345
            f32::powi(self[e321], 2)
                - f32::powi(self[e415], 2)
                - f32::powi(self[e425], 2)
                - f32::powi(self[e435], 2)
                - 2.0 * (self[e423] * self[e235])
                - 2.0 * (self[e431] * self[e315])
                - 2.0 * (self[e412] * self[e125]),
        );
        let other = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return Circle::from_groups(
            // e423, e431, e412
            Simd32x3::from(other[e12345]) * self.group0(),
            // e415, e425, e435, e321
            Simd32x4::from(other[e12345]) * self.group1(),
            // e235, e315, e125
            Simd32x3::from(other[e12345]) * self.group2(),
        );
    }
}
impl std::ops::Div<anti_inverse> for CircleRotor {
    type Output = CircleRotor;
    fn div(self, _rhs: anti_inverse) -> Self::Output {
        self.anti_inverse()
    }
}
impl std::ops::DivAssign<anti_inverse> for CircleRotor {
    fn div_assign(&mut self, _rhs: anti_inverse) {
        *self = self.anti_inverse()
    }
}
impl AntiInverse for CircleRotor {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        7        6        1
    //    simd3        0        1        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        7        9        1
    //  no simd        7       17        1
    fn anti_inverse(self) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(
            // e12345
            f32::powi(self[e321], 2) + f32::powi(self[e12345], 2)
                - f32::powi(self[e415], 2)
                - f32::powi(self[e425], 2)
                - f32::powi(self[e435], 2)
                - 2.0 * (self[e423] * self[e235])
                - 2.0 * (self[e431] * self[e315])
                - 2.0 * (self[e412] * self[e125]),
        );
        let other = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return CircleRotor::from_groups(
            // e423, e431, e412
            Simd32x3::from(other[e12345]) * self.group0(),
            // e415, e425, e435, e321
            Simd32x4::from(other[e12345]) * self.group1(),
            // e235, e315, e125, e12345
            Simd32x4::from(other[e12345]) * self.group2(),
        );
    }
}
impl std::ops::Div<anti_inverse> for Dipole {
    type Output = Dipole;
    fn div(self, _rhs: anti_inverse) -> Self::Output {
        self.anti_inverse()
    }
}
impl std::ops::DivAssign<anti_inverse> for Dipole {
    fn div_assign(&mut self, _rhs: anti_inverse) {
        *self = self.anti_inverse()
    }
}
impl AntiInverse for Dipole {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        6        6        1
    //    simd3        0        2        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        6        9        1
    //  no simd        6       16        1
    fn anti_inverse(self) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(
            // e12345
            2.0 * (self[e41] * self[e15])
                + 2.0 * (self[e42] * self[e25])
                + 2.0 * (self[e43] * self[e35])
                + f32::powi(self[e23], 2)
                + f32::powi(self[e31], 2)
                + f32::powi(self[e12], 2)
                - f32::powi(self[e45], 2),
        );
        let other = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return Dipole::from_groups(
            // e41, e42, e43
            Simd32x3::from(other[e12345]) * self.group0(),
            // e23, e31, e12, e45
            Simd32x4::from(other[e12345]) * self.group1(),
            // e15, e25, e35
            Simd32x3::from(other[e12345]) * self.group2(),
        );
    }
}
impl std::ops::Div<anti_inverse> for DipoleInversion {
    type Output = DipoleInversion;
    fn div(self, _rhs: anti_inverse) -> Self::Output {
        self.anti_inverse()
    }
}
impl std::ops::DivAssign<anti_inverse> for DipoleInversion {
    fn div_assign(&mut self, _rhs: anti_inverse) {
        *self = self.anti_inverse()
    }
}
impl AntiInverse for DipoleInversion {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       10        8        1
    //    simd3        0        1        0
    //    simd4        0        3        0
    // Totals...
    // yes simd       10       12        1
    //  no simd       10       23        1
    fn anti_inverse(self) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(
            // e12345
            2.0 * (self[e41] * self[e15])
                + 2.0 * (self[e42] * self[e25])
                + 2.0 * (self[e43] * self[e35])
                + f32::powi(self[e23], 2)
                + f32::powi(self[e31], 2)
                + f32::powi(self[e12], 2)
                + f32::powi(self[e4235], 2)
                + f32::powi(self[e4315], 2)
                + f32::powi(self[e4125], 2)
                - f32::powi(self[e45], 2)
                - 2.0 * (self[e1234] * self[e3215]),
        );
        let other = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return DipoleInversion::from_groups(
            // e41, e42, e43
            Simd32x3::from(other[e12345]) * self.group0(),
            // e23, e31, e12, e45
            Simd32x4::from(other[e12345]) * self.group1(),
            // e15, e25, e35, e1234
            Simd32x4::from(other[e12345]) * self.group2(),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(other[e12345]) * self.group3(),
        );
    }
}
impl std::ops::Div<anti_inverse> for DualNum {
    type Output = DualNum;
    fn div(self, _rhs: anti_inverse) -> Self::Output {
        self.anti_inverse()
    }
}
impl std::ops::DivAssign<anti_inverse> for DualNum {
    fn div_assign(&mut self, _rhs: anti_inverse) {
        *self = self.anti_inverse()
    }
}
impl AntiInverse for DualNum {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        0        1
    //    simd2        0        1        0
    // Totals...
    // yes simd        0        1        1
    //  no simd        0        2        1
    fn anti_inverse(self) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(/* e12345 */ f32::powi(self[e12345], 2));
        let other = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return DualNum::from_groups(/* e5, e12345 */ Simd32x2::from(other[e12345]) * self.group0());
    }
}
impl std::ops::Div<anti_inverse> for FlatPoint {
    type Output = FlatPoint;
    fn div(self, _rhs: anti_inverse) -> Self::Output {
        self.anti_inverse()
    }
}
impl std::ops::DivAssign<anti_inverse> for FlatPoint {
    fn div_assign(&mut self, _rhs: anti_inverse) {
        *self = self.anti_inverse()
    }
}
impl AntiInverse for FlatPoint {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        1
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        2        1
    //  no simd        0        5        1
    fn anti_inverse(self) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(/* e12345 */ f32::powi(self[e45], 2) * -1.0);
        let other = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return FlatPoint::from_groups(/* e15, e25, e35, e45 */ Simd32x4::from(other[e12345]) * self.group0());
    }
}
impl std::ops::Div<anti_inverse> for Flector {
    type Output = Flector;
    fn div(self, _rhs: anti_inverse) -> Self::Output {
        self.anti_inverse()
    }
}
impl std::ops::DivAssign<anti_inverse> for Flector {
    fn div_assign(&mut self, _rhs: anti_inverse) {
        *self = self.anti_inverse()
    }
}
impl AntiInverse for Flector {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        0        1
    //    simd4        0        2        0
    // Totals...
    // yes simd        3        2        1
    //  no simd        3        8        1
    fn anti_inverse(self) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(
            // e12345
            f32::powi(self[e4235], 2) + f32::powi(self[e4315], 2) + f32::powi(self[e4125], 2) - f32::powi(self[e45], 2),
        );
        let other = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return Flector::from_groups(
            // e15, e25, e35, e45
            Simd32x4::from(other[e12345]) * self.group0(),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(other[e12345]) * self.group1(),
        );
    }
}
impl std::ops::Div<anti_inverse> for Line {
    type Output = Line;
    fn div(self, _rhs: anti_inverse) -> Self::Output {
        self.anti_inverse()
    }
}
impl std::ops::DivAssign<anti_inverse> for Line {
    fn div_assign(&mut self, _rhs: anti_inverse) {
        *self = self.anti_inverse()
    }
}
impl AntiInverse for Line {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        0        1
    //    simd3        0        2        0
    // Totals...
    // yes simd        2        2        1
    //  no simd        2        6        1
    fn anti_inverse(self) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(/* e12345 */ -f32::powi(self[e415], 2) - f32::powi(self[e425], 2) - f32::powi(self[e435], 2));
        let other = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return Line::from_groups(
            // e415, e425, e435
            Simd32x3::from(other[e12345]) * self.group0(),
            // e235, e315, e125
            Simd32x3::from(other[e12345]) * self.group1(),
        );
    }
}
impl std::ops::Div<anti_inverse> for Motor {
    type Output = Motor;
    fn div(self, _rhs: anti_inverse) -> Self::Output {
        self.anti_inverse()
    }
}
impl std::ops::DivAssign<anti_inverse> for Motor {
    fn div_assign(&mut self, _rhs: anti_inverse) {
        *self = self.anti_inverse()
    }
}
impl AntiInverse for Motor {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        0        1
    //    simd4        0        2        0
    // Totals...
    // yes simd        3        2        1
    //  no simd        3        8        1
    fn anti_inverse(self) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(
            // e12345
            f32::powi(self[e12345], 2) - f32::powi(self[e415], 2) - f32::powi(self[e425], 2) - f32::powi(self[e435], 2),
        );
        let other = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return Motor::from_groups(
            // e415, e425, e435, e12345
            Simd32x4::from(other[e12345]) * self.group0(),
            // e235, e315, e125, e5
            Simd32x4::from(other[e12345]) * self.group1(),
        );
    }
}
impl std::ops::Div<anti_inverse> for MultiVector {
    type Output = MultiVector;
    fn div(self, _rhs: anti_inverse) -> Self::Output {
        self.anti_inverse()
    }
}
impl std::ops::DivAssign<anti_inverse> for MultiVector {
    fn div_assign(&mut self, _rhs: anti_inverse) {
        *self = self.anti_inverse()
    }
}
impl AntiInverse for MultiVector {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       23       18        1
    //    simd2        0        1        0
    //    simd3        0        4        0
    //    simd4        0        4        0
    // Totals...
    // yes simd       23       27        1
    //  no simd       23       48        1
    fn anti_inverse(self) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(
            // e12345
            2.0 * (self[e4] * self[e5])
                + 2.0 * (self[e15] * self[e41])
                + 2.0 * (self[e25] * self[e42])
                + 2.0 * (self[e35] * self[e43])
                + f32::powi(self[e12345], 2)
                + f32::powi(self[e23], 2)
                + f32::powi(self[e31], 2)
                + f32::powi(self[e12], 2)
                + f32::powi(self[e321], 2)
                + f32::powi(self[e4235], 2)
                + f32::powi(self[e4315], 2)
                + f32::powi(self[e4125], 2)
                - f32::powi(self[scalar], 2)
                - f32::powi(self[e1], 2)
                - f32::powi(self[e2], 2)
                - f32::powi(self[e3], 2)
                - f32::powi(self[e45], 2)
                - f32::powi(self[e415], 2)
                - f32::powi(self[e425], 2)
                - f32::powi(self[e435], 2)
                - 2.0 * (self[e423] * self[e235])
                - 2.0 * (self[e431] * self[e315])
                - 2.0 * (self[e412] * self[e125])
                - 2.0 * (self[e3215] * self[e1234]),
        );
        let other = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(other[e12345]) * self.group0(),
            // e1, e2, e3, e4
            Simd32x4::from(other[e12345]) * self.group1(),
            // e5
            other[e12345] * self[e5],
            // e15, e25, e35, e45
            Simd32x4::from(other[e12345]) * self.group3(),
            // e41, e42, e43
            Simd32x3::from(other[e12345]) * self.group4(),
            // e23, e31, e12
            Simd32x3::from(other[e12345]) * self.group5(),
            // e415, e425, e435, e321
            Simd32x4::from(other[e12345]) * self.group6(),
            // e423, e431, e412
            Simd32x3::from(other[e12345]) * self.group7(),
            // e235, e315, e125
            Simd32x3::from(other[e12345]) * self.group8(),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(other[e12345]) * self.group9(),
            // e1234
            other[e12345] * self[e1234],
        );
    }
}
impl std::ops::Div<anti_inverse> for Plane {
    type Output = Plane;
    fn div(self, _rhs: anti_inverse) -> Self::Output {
        self.anti_inverse()
    }
}
impl std::ops::DivAssign<anti_inverse> for Plane {
    fn div_assign(&mut self, _rhs: anti_inverse) {
        *self = self.anti_inverse()
    }
}
impl AntiInverse for Plane {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        0        1
    //    simd4        0        1        0
    // Totals...
    // yes simd        2        1        1
    //  no simd        2        4        1
    fn anti_inverse(self) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(/* e12345 */ f32::powi(self[e4235], 2) + f32::powi(self[e4315], 2) + f32::powi(self[e4125], 2));
        let other = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return Plane::from_groups(/* e4235, e4315, e4125, e3215 */ Simd32x4::from(other[e12345]) * self.group0());
    }
}
impl std::ops::Div<anti_inverse> for RoundPoint {
    type Output = RoundPoint;
    fn div(self, _rhs: anti_inverse) -> Self::Output {
        self.anti_inverse()
    }
}
impl std::ops::DivAssign<anti_inverse> for RoundPoint {
    fn div_assign(&mut self, _rhs: anti_inverse) {
        *self = self.anti_inverse()
    }
}
impl AntiInverse for RoundPoint {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        3        1
    //    simd4        0        1        0
    // Totals...
    // yes simd        3        4        1
    //  no simd        3        7        1
    fn anti_inverse(self) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(/* e12345 */ 2.0 * (self[e4] * self[e5]) - f32::powi(self[e1], 2) - f32::powi(self[e2], 2) - f32::powi(self[e3], 2));
        let other = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return RoundPoint::from_groups(/* e1, e2, e3, e4 */ Simd32x4::from(other[e12345]) * self.group0(), /* e5 */ other[e12345] * self[e5]);
    }
}
impl std::ops::Div<anti_inverse> for Scalar {
    type Output = Scalar;
    fn div(self, _rhs: anti_inverse) -> Self::Output {
        self.anti_inverse()
    }
}
impl std::ops::DivAssign<anti_inverse> for Scalar {
    fn div_assign(&mut self, _rhs: anti_inverse) {
        *self = self.anti_inverse()
    }
}
impl AntiInverse for Scalar {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        2        1
    fn anti_inverse(self) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(/* e12345 */ f32::powi(self[scalar], 2) * -1.0);
        let other = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return Scalar::from_groups(/* scalar */ other[e12345] * self[scalar]);
    }
}
impl std::ops::Div<anti_inverse> for Sphere {
    type Output = Sphere;
    fn div(self, _rhs: anti_inverse) -> Self::Output {
        self.anti_inverse()
    }
}
impl std::ops::DivAssign<anti_inverse> for Sphere {
    fn div_assign(&mut self, _rhs: anti_inverse) {
        *self = self.anti_inverse()
    }
}
impl AntiInverse for Sphere {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        3        1
    //    simd4        0        1        0
    // Totals...
    // yes simd        3        4        1
    //  no simd        3        7        1
    fn anti_inverse(self) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(
            // e12345
            f32::powi(self[e4235], 2) + f32::powi(self[e4315], 2) + f32::powi(self[e4125], 2) - 2.0 * (self[e3215] * self[e1234]),
        );
        let other = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return Sphere::from_groups(
            // e4235, e4315, e4125, e3215
            Simd32x4::from(other[e12345]) * self.group0(),
            // e1234
            other[e12345] * self[e1234],
        );
    }
}
impl std::ops::Div<anti_inverse> for VersorEven {
    type Output = VersorEven;
    fn div(self, _rhs: anti_inverse) -> Self::Output {
        self.anti_inverse()
    }
}
impl std::ops::DivAssign<anti_inverse> for VersorEven {
    fn div_assign(&mut self, _rhs: anti_inverse) {
        *self = self.anti_inverse()
    }
}
impl AntiInverse for VersorEven {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       11        8        1
    //    simd4        0        4        0
    // Totals...
    // yes simd       11       12        1
    //  no simd       11       24        1
    fn anti_inverse(self) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(
            // e12345
            2.0 * (self[e5] * self[e4]) + f32::powi(self[e12345], 2) + f32::powi(self[e321], 2)
                - f32::powi(self[e415], 2)
                - f32::powi(self[e425], 2)
                - f32::powi(self[e435], 2)
                - f32::powi(self[e1], 2)
                - f32::powi(self[e2], 2)
                - f32::powi(self[e3], 2)
                - 2.0 * (self[e423] * self[e235])
                - 2.0 * (self[e431] * self[e315])
                - 2.0 * (self[e412] * self[e125]),
        );
        let other = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from(other[e12345]) * self.group0(),
            // e415, e425, e435, e321
            Simd32x4::from(other[e12345]) * self.group1(),
            // e235, e315, e125, e5
            Simd32x4::from(other[e12345]) * self.group2(),
            // e1, e2, e3, e4
            Simd32x4::from(other[e12345]) * self.group3(),
        );
    }
}
impl std::ops::Div<anti_inverse> for VersorOdd {
    type Output = VersorOdd;
    fn div(self, _rhs: anti_inverse) -> Self::Output {
        self.anti_inverse()
    }
}
impl std::ops::DivAssign<anti_inverse> for VersorOdd {
    fn div_assign(&mut self, _rhs: anti_inverse) {
        *self = self.anti_inverse()
    }
}
impl AntiInverse for VersorOdd {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       11        8        1
    //    simd4        0        4        0
    // Totals...
    // yes simd       11       12        1
    //  no simd       11       24        1
    fn anti_inverse(self) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(
            // e12345
            2.0 * (self[e41] * self[e15])
                + 2.0 * (self[e42] * self[e25])
                + 2.0 * (self[e43] * self[e35])
                + f32::powi(self[e23], 2)
                + f32::powi(self[e31], 2)
                + f32::powi(self[e12], 2)
                + f32::powi(self[e4235], 2)
                + f32::powi(self[e4315], 2)
                + f32::powi(self[e4125], 2)
                - f32::powi(self[scalar], 2)
                - f32::powi(self[e45], 2)
                - 2.0 * (self[e1234] * self[e3215]),
        );
        let other = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from(other[e12345]) * self.group0(),
            // e23, e31, e12, e45
            Simd32x4::from(other[e12345]) * self.group1(),
            // e15, e25, e35, e1234
            Simd32x4::from(other[e12345]) * self.group2(),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(other[e12345]) * self.group3(),
        );
    }
}
