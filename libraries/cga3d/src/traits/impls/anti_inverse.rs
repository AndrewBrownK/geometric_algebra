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
//   Median:         3       4       0
//  Average:         4       7       0
//  Maximum:        23      33       1
//
//  No SIMD:   add/sub     mul     div
//  Minimum:         0       0       0
//   Median:         3      12       0
//  Average:         4      17       0
//  Maximum:        23      68       1
impl std::ops::Div<AntiInversePrefixOrPostfix> for AntiCircleRotor {
    type Output = AntiCircleRotor;
    fn div(self, _rhs: AntiInversePrefixOrPostfix) -> Self::Output {
        self.anti_inverse()
    }
}
impl std::ops::DivAssign<AntiInversePrefixOrPostfix> for AntiCircleRotor {
    fn div_assign(&mut self, _rhs: AntiInversePrefixOrPostfix) {
        *self = self.anti_inverse()
    }
}
impl AntiInverse for AntiCircleRotor {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        7        6        0
    //    simd3        0        2        0
    //    simd4        0        4        0
    // Totals...
    // yes simd        7       12        0
    //  no simd        7       28        0
    fn anti_inverse(self) -> Self {
        use crate::elements::*;
        let anti_reverse = AntiCircleRotor::from_groups(
            // e41, e42, e43
            self.group0() * Simd32x3::from(-1.0),
            // e23, e31, e12, e45
            self.group1() * Simd32x4::from(-1.0),
            // e15, e25, e35, scalar
            self.group2() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
        );
        let other = AntiScalar::from_groups(
            // e12345
            f32::powi(self[e45], 2)
                - f32::powi(self[e23], 2)
                - f32::powi(self[e31], 2)
                - f32::powi(self[e12], 2)
                - f32::powi(self[scalar], 2)
                - 2.0 * (self[e41] * self[e15])
                - 2.0 * (self[e42] * self[e25])
                - 2.0 * (self[e43] * self[e35]),
        );
        return AntiCircleRotor::from_groups(
            // e41, e42, e43
            Simd32x3::from(other[e12345]) * anti_reverse.group0(),
            // e23, e31, e12, e45
            Simd32x4::from(other[e12345]) * anti_reverse.group1(),
            // e15, e25, e35, scalar
            Simd32x4::from(other[e12345]) * anti_reverse.group2(),
        );
    }
}
impl std::ops::Div<AntiInversePrefixOrPostfix> for AntiDipoleInversion {
    type Output = AntiDipoleInversion;
    fn div(self, _rhs: AntiInversePrefixOrPostfix) -> Self::Output {
        self.anti_inverse()
    }
}
impl std::ops::DivAssign<AntiInversePrefixOrPostfix> for AntiDipoleInversion {
    fn div_assign(&mut self, _rhs: AntiInversePrefixOrPostfix) {
        *self = self.anti_inverse()
    }
}
impl AntiInverse for AntiDipoleInversion {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       10        8        0
    //    simd3        0        2        0
    //    simd4        0        5        0
    // Totals...
    // yes simd       10       15        0
    //  no simd       10       34        0
    fn anti_inverse(self) -> Self {
        use crate::elements::*;
        let anti_reverse = AntiDipoleInversion::from_groups(
            // e423, e431, e412
            self.group0() * Simd32x3::from(-1.0),
            // e415, e425, e435, e321
            self.group1() * Simd32x4::from(-1.0),
            // e235, e315, e125, e4
            self.group2() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e1, e2, e3, e5
            self.group3(),
        );
        let other = AntiScalar::from_groups(
            // e12345
            2.0 * (self[e423] * self[e235])
                + 2.0 * (self[e431] * self[e315])
                + 2.0 * (self[e412] * self[e125])
                + 2.0 * (self[e4] * self[e5])
                + f32::powi(self[e415], 2)
                + f32::powi(self[e425], 2)
                + f32::powi(self[e435], 2)
                - f32::powi(self[e321], 2)
                - f32::powi(self[e1], 2)
                - f32::powi(self[e2], 2)
                - f32::powi(self[e3], 2),
        );
        return AntiDipoleInversion::from_groups(
            // e423, e431, e412
            Simd32x3::from(other[e12345]) * anti_reverse.group0(),
            // e415, e425, e435, e321
            Simd32x4::from(other[e12345]) * anti_reverse.group1(),
            // e235, e315, e125, e4
            Simd32x4::from(other[e12345]) * anti_reverse.group2(),
            // e1, e2, e3, e5
            Simd32x4::from(other[e12345]) * anti_reverse.group3(),
        );
    }
}
impl std::ops::Div<AntiInversePrefixOrPostfix> for AntiDualNum {
    type Output = AntiDualNum;
    fn div(self, _rhs: AntiInversePrefixOrPostfix) -> Self::Output {
        self.anti_inverse()
    }
}
impl std::ops::DivAssign<AntiInversePrefixOrPostfix> for AntiDualNum {
    fn div_assign(&mut self, _rhs: AntiInversePrefixOrPostfix) {
        *self = self.anti_inverse()
    }
}
impl AntiInverse for AntiDualNum {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd2        0        1        0
    // Totals...
    // yes simd        0        2        0
    //  no simd        0        3        0
    fn anti_inverse(self) -> Self {
        use crate::elements::*;
        return AntiDualNum::from_groups(/* e3215, scalar */ Simd32x2::from(f32::powi(self[scalar], -2) * -1.0) * self.group0());
    }
}
impl std::ops::Div<AntiInversePrefixOrPostfix> for AntiFlatPoint {
    type Output = AntiFlatPoint;
    fn div(self, _rhs: AntiInversePrefixOrPostfix) -> Self::Output {
        self.anti_inverse()
    }
}
impl std::ops::DivAssign<AntiInversePrefixOrPostfix> for AntiFlatPoint {
    fn div_assign(&mut self, _rhs: AntiInversePrefixOrPostfix) {
        *self = self.anti_inverse()
    }
}
impl AntiInverse for AntiFlatPoint {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        5        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        6        0
    //  no simd        0        9        0
    fn anti_inverse(self) -> Self {
        use crate::elements::*;
        return AntiFlatPoint::from_groups(
            // e235, e315, e125, e321
            Simd32x4::from(f32::powi(self[e321], -2) * -1.0) * Simd32x4::from([self[e235] * -1.0, self[e315] * -1.0, self[e125] * -1.0, self[e321] * -1.0]),
        );
    }
}
impl std::ops::Div<AntiInversePrefixOrPostfix> for AntiFlector {
    type Output = AntiFlector;
    fn div(self, _rhs: AntiInversePrefixOrPostfix) -> Self::Output {
        self.anti_inverse()
    }
}
impl std::ops::DivAssign<AntiInversePrefixOrPostfix> for AntiFlector {
    fn div_assign(&mut self, _rhs: AntiInversePrefixOrPostfix) {
        *self = self.anti_inverse()
    }
}
impl AntiInverse for AntiFlector {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        0        0
    //    simd4        0        3        0
    // Totals...
    // yes simd        3        3        0
    //  no simd        3       12        0
    fn anti_inverse(self) -> Self {
        use crate::elements::*;
        let anti_reverse = AntiFlector::from_groups(/* e235, e315, e125, e321 */ self.group0() * Simd32x4::from(-1.0), /* e1, e2, e3, e5 */ self.group1());
        let other = AntiScalar::from_groups(/* e12345 */ -f32::powi(self[e321], 2) - f32::powi(self[e1], 2) - f32::powi(self[e2], 2) - f32::powi(self[e3], 2));
        return AntiFlector::from_groups(
            // e235, e315, e125, e321
            Simd32x4::from(other[e12345]) * anti_reverse.group0(),
            // e1, e2, e3, e5
            Simd32x4::from(other[e12345]) * anti_reverse.group1(),
        );
    }
}
impl std::ops::Div<AntiInversePrefixOrPostfix> for AntiLine {
    type Output = AntiLine;
    fn div(self, _rhs: AntiInversePrefixOrPostfix) -> Self::Output {
        self.anti_inverse()
    }
}
impl std::ops::DivAssign<AntiInversePrefixOrPostfix> for AntiLine {
    fn div_assign(&mut self, _rhs: AntiInversePrefixOrPostfix) {
        *self = self.anti_inverse()
    }
}
impl AntiInverse for AntiLine {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        0        0
    //    simd3        0        4        0
    // Totals...
    // yes simd        2        4        0
    //  no simd        2       12        0
    fn anti_inverse(self) -> Self {
        use crate::elements::*;
        let anti_reverse = AntiLine::from_groups(
            // e23, e31, e12
            self.group0() * Simd32x3::from(-1.0),
            // e15, e25, e35
            self.group1() * Simd32x3::from(-1.0),
        );
        let other = AntiScalar::from_groups(/* e12345 */ -f32::powi(self[e23], 2) - f32::powi(self[e31], 2) - f32::powi(self[e12], 2));
        return AntiLine::from_groups(
            // e23, e31, e12
            Simd32x3::from(other[e12345]) * anti_reverse.group0(),
            // e15, e25, e35
            Simd32x3::from(other[e12345]) * anti_reverse.group1(),
        );
    }
}
impl std::ops::Div<AntiInversePrefixOrPostfix> for AntiMotor {
    type Output = AntiMotor;
    fn div(self, _rhs: AntiInversePrefixOrPostfix) -> Self::Output {
        self.anti_inverse()
    }
}
impl std::ops::DivAssign<AntiInversePrefixOrPostfix> for AntiMotor {
    fn div_assign(&mut self, _rhs: AntiInversePrefixOrPostfix) {
        *self = self.anti_inverse()
    }
}
impl AntiInverse for AntiMotor {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        0        0
    //    simd4        0        4        0
    // Totals...
    // yes simd        3        4        0
    //  no simd        3       16        0
    fn anti_inverse(self) -> Self {
        use crate::elements::*;
        let anti_reverse = AntiMotor::from_groups(
            // e23, e31, e12, scalar
            self.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e15, e25, e35, e3215
            self.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
        );
        let other = AntiScalar::from_groups(
            // e12345
            -f32::powi(self[e23], 2) - f32::powi(self[e31], 2) - f32::powi(self[e12], 2) - f32::powi(self[scalar], 2),
        );
        return AntiMotor::from_groups(
            // e23, e31, e12, scalar
            Simd32x4::from(other[e12345]) * anti_reverse.group0(),
            // e15, e25, e35, e3215
            Simd32x4::from(other[e12345]) * anti_reverse.group1(),
        );
    }
}
impl std::ops::Div<AntiInversePrefixOrPostfix> for AntiPlane {
    type Output = AntiPlane;
    fn div(self, _rhs: AntiInversePrefixOrPostfix) -> Self::Output {
        self.anti_inverse()
    }
}
impl std::ops::DivAssign<AntiInversePrefixOrPostfix> for AntiPlane {
    fn div_assign(&mut self, _rhs: AntiInversePrefixOrPostfix) {
        *self = self.anti_inverse()
    }
}
impl AntiInverse for AntiPlane {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        0        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        2        1        0
    //  no simd        2        4        0
    fn anti_inverse(self) -> Self {
        use crate::elements::*;
        return AntiPlane::from_groups(
            // e1, e2, e3, e5
            Simd32x4::from(-f32::powi(self[e1], 2) - f32::powi(self[e2], 2) - f32::powi(self[e3], 2)) * self.group0(),
        );
    }
}
impl std::ops::Div<AntiInversePrefixOrPostfix> for AntiScalar {
    type Output = AntiScalar;
    fn div(self, _rhs: AntiInversePrefixOrPostfix) -> Self::Output {
        self.anti_inverse()
    }
}
impl std::ops::DivAssign<AntiInversePrefixOrPostfix> for AntiScalar {
    fn div_assign(&mut self, _rhs: AntiInversePrefixOrPostfix) {
        *self = self.anti_inverse()
    }
}
impl AntiInverse for AntiScalar {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        0        1
    fn anti_inverse(self) -> Self {
        use crate::elements::*;
        return AntiScalar::from_groups(/* e12345 */ 1.0 / self[e12345]);
    }
}
impl std::ops::Div<AntiInversePrefixOrPostfix> for Circle {
    type Output = Circle;
    fn div(self, _rhs: AntiInversePrefixOrPostfix) -> Self::Output {
        self.anti_inverse()
    }
}
impl std::ops::DivAssign<AntiInversePrefixOrPostfix> for Circle {
    fn div_assign(&mut self, _rhs: AntiInversePrefixOrPostfix) {
        *self = self.anti_inverse()
    }
}
impl AntiInverse for Circle {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        6        6        0
    //    simd3        0        4        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        6       12        0
    //  no simd        6       26        0
    fn anti_inverse(self) -> Self {
        use crate::elements::*;
        let anti_reverse = Circle::from_groups(
            // e423, e431, e412
            self.group0() * Simd32x3::from(-1.0),
            // e415, e425, e435, e321
            self.group1() * Simd32x4::from(-1.0),
            // e235, e315, e125
            self.group2() * Simd32x3::from(-1.0),
        );
        let other = AntiScalar::from_groups(
            // e12345
            2.0 * (self[e423] * self[e235])
                + 2.0 * (self[e431] * self[e315])
                + 2.0 * (self[e412] * self[e125])
                + f32::powi(self[e415], 2)
                + f32::powi(self[e425], 2)
                + f32::powi(self[e435], 2)
                - f32::powi(self[e321], 2),
        );
        return Circle::from_groups(
            // e423, e431, e412
            Simd32x3::from(other[e12345]) * anti_reverse.group0(),
            // e415, e425, e435, e321
            Simd32x4::from(other[e12345]) * anti_reverse.group1(),
            // e235, e315, e125
            Simd32x3::from(other[e12345]) * anti_reverse.group2(),
        );
    }
}
impl std::ops::Div<AntiInversePrefixOrPostfix> for CircleRotor {
    type Output = CircleRotor;
    fn div(self, _rhs: AntiInversePrefixOrPostfix) -> Self::Output {
        self.anti_inverse()
    }
}
impl std::ops::DivAssign<AntiInversePrefixOrPostfix> for CircleRotor {
    fn div_assign(&mut self, _rhs: AntiInversePrefixOrPostfix) {
        *self = self.anti_inverse()
    }
}
impl AntiInverse for CircleRotor {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        7        6        0
    //    simd3        0        2        0
    //    simd4        0        4        0
    // Totals...
    // yes simd        7       12        0
    //  no simd        7       28        0
    fn anti_inverse(self) -> Self {
        use crate::elements::*;
        let anti_reverse = CircleRotor::from_groups(
            // e423, e431, e412
            self.group0() * Simd32x3::from(-1.0),
            // e415, e425, e435, e321
            self.group1() * Simd32x4::from(-1.0),
            // e235, e315, e125, e12345
            self.group2() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
        );
        let other = AntiScalar::from_groups(
            // e12345
            2.0 * (self[e423] * self[e235])
                + 2.0 * (self[e431] * self[e315])
                + 2.0 * (self[e412] * self[e125])
                + f32::powi(self[e415], 2)
                + f32::powi(self[e425], 2)
                + f32::powi(self[e435], 2)
                + f32::powi(self[e12345], 2)
                - f32::powi(self[e321], 2),
        );
        return CircleRotor::from_groups(
            // e423, e431, e412
            Simd32x3::from(other[e12345]) * anti_reverse.group0(),
            // e415, e425, e435, e321
            Simd32x4::from(other[e12345]) * anti_reverse.group1(),
            // e235, e315, e125, e12345
            Simd32x4::from(other[e12345]) * anti_reverse.group2(),
        );
    }
}
impl std::ops::Div<AntiInversePrefixOrPostfix> for Dipole {
    type Output = Dipole;
    fn div(self, _rhs: AntiInversePrefixOrPostfix) -> Self::Output {
        self.anti_inverse()
    }
}
impl std::ops::DivAssign<AntiInversePrefixOrPostfix> for Dipole {
    fn div_assign(&mut self, _rhs: AntiInversePrefixOrPostfix) {
        *self = self.anti_inverse()
    }
}
impl AntiInverse for Dipole {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        6        6        0
    //    simd3        0        4        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        6       12        0
    //  no simd        6       26        0
    fn anti_inverse(self) -> Self {
        use crate::elements::*;
        let anti_reverse = Dipole::from_groups(
            // e41, e42, e43
            self.group0() * Simd32x3::from(-1.0),
            // e23, e31, e12, e45
            self.group1() * Simd32x4::from(-1.0),
            // e15, e25, e35
            self.group2() * Simd32x3::from(-1.0),
        );
        let other = AntiScalar::from_groups(
            // e12345
            f32::powi(self[e45], 2)
                - f32::powi(self[e23], 2)
                - f32::powi(self[e31], 2)
                - f32::powi(self[e12], 2)
                - 2.0 * (self[e41] * self[e15])
                - 2.0 * (self[e42] * self[e25])
                - 2.0 * (self[e43] * self[e35]),
        );
        return Dipole::from_groups(
            // e41, e42, e43
            Simd32x3::from(other[e12345]) * anti_reverse.group0(),
            // e23, e31, e12, e45
            Simd32x4::from(other[e12345]) * anti_reverse.group1(),
            // e15, e25, e35
            Simd32x3::from(other[e12345]) * anti_reverse.group2(),
        );
    }
}
impl std::ops::Div<AntiInversePrefixOrPostfix> for DipoleInversion {
    type Output = DipoleInversion;
    fn div(self, _rhs: AntiInversePrefixOrPostfix) -> Self::Output {
        self.anti_inverse()
    }
}
impl std::ops::DivAssign<AntiInversePrefixOrPostfix> for DipoleInversion {
    fn div_assign(&mut self, _rhs: AntiInversePrefixOrPostfix) {
        *self = self.anti_inverse()
    }
}
impl AntiInverse for DipoleInversion {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       10        8        0
    //    simd3        0        2        0
    //    simd4        0        5        0
    // Totals...
    // yes simd       10       15        0
    //  no simd       10       34        0
    fn anti_inverse(self) -> Self {
        use crate::elements::*;
        let anti_reverse = DipoleInversion::from_groups(
            // e41, e42, e43
            self.group0() * Simd32x3::from(-1.0),
            // e23, e31, e12, e45
            self.group1() * Simd32x4::from(-1.0),
            // e15, e25, e35, e1234
            self.group2() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e4235, e4315, e4125, e3215
            self.group3(),
        );
        let other = AntiScalar::from_groups(
            // e12345
            f32::powi(self[e45], 2) + f32::powi(self[e4235], 2) + f32::powi(self[e4315], 2) + f32::powi(self[e4125], 2)
                - f32::powi(self[e23], 2)
                - f32::powi(self[e31], 2)
                - f32::powi(self[e12], 2)
                - 2.0 * (self[e41] * self[e15])
                - 2.0 * (self[e42] * self[e25])
                - 2.0 * (self[e43] * self[e35])
                - 2.0 * (self[e1234] * self[e3215]),
        );
        return DipoleInversion::from_groups(
            // e41, e42, e43
            Simd32x3::from(other[e12345]) * anti_reverse.group0(),
            // e23, e31, e12, e45
            Simd32x4::from(other[e12345]) * anti_reverse.group1(),
            // e15, e25, e35, e1234
            Simd32x4::from(other[e12345]) * anti_reverse.group2(),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(other[e12345]) * anti_reverse.group3(),
        );
    }
}
impl std::ops::Div<AntiInversePrefixOrPostfix> for DualNum {
    type Output = DualNum;
    fn div(self, _rhs: AntiInversePrefixOrPostfix) -> Self::Output {
        self.anti_inverse()
    }
}
impl std::ops::DivAssign<AntiInversePrefixOrPostfix> for DualNum {
    fn div_assign(&mut self, _rhs: AntiInversePrefixOrPostfix) {
        *self = self.anti_inverse()
    }
}
impl AntiInverse for DualNum {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd2        0        1        0
    // no simd        0        2        0
    fn anti_inverse(self) -> Self {
        use crate::elements::*;
        return DualNum::from_groups(/* e5, e12345 */ Simd32x2::from(f32::powi(self[e12345], -2)) * self.group0());
    }
}
impl std::ops::Div<AntiInversePrefixOrPostfix> for FlatPoint {
    type Output = FlatPoint;
    fn div(self, _rhs: AntiInversePrefixOrPostfix) -> Self::Output {
        self.anti_inverse()
    }
}
impl std::ops::DivAssign<AntiInversePrefixOrPostfix> for FlatPoint {
    fn div_assign(&mut self, _rhs: AntiInversePrefixOrPostfix) {
        *self = self.anti_inverse()
    }
}
impl AntiInverse for FlatPoint {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        4        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        5        0
    //  no simd        0        8        0
    fn anti_inverse(self) -> Self {
        use crate::elements::*;
        return FlatPoint::from_groups(
            // e15, e25, e35, e45
            Simd32x4::from(f32::powi(self[e45], -2)) * Simd32x4::from([self[e15] * -1.0, self[e25] * -1.0, self[e35] * -1.0, self[e45] * -1.0]),
        );
    }
}
impl std::ops::Div<AntiInversePrefixOrPostfix> for Flector {
    type Output = Flector;
    fn div(self, _rhs: AntiInversePrefixOrPostfix) -> Self::Output {
        self.anti_inverse()
    }
}
impl std::ops::DivAssign<AntiInversePrefixOrPostfix> for Flector {
    fn div_assign(&mut self, _rhs: AntiInversePrefixOrPostfix) {
        *self = self.anti_inverse()
    }
}
impl AntiInverse for Flector {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        0        0
    //    simd4        0        3        0
    // Totals...
    // yes simd        3        3        0
    //  no simd        3       12        0
    fn anti_inverse(self) -> Self {
        use crate::elements::*;
        let anti_reverse = Flector::from_groups(/* e15, e25, e35, e45 */ self.group0() * Simd32x4::from(-1.0), /* e4235, e4315, e4125, e3215 */ self.group1());
        let other = AntiScalar::from_groups(
            // e12345
            f32::powi(self[e45], 2) + f32::powi(self[e4235], 2) + f32::powi(self[e4315], 2) + f32::powi(self[e4125], 2),
        );
        return Flector::from_groups(
            // e15, e25, e35, e45
            Simd32x4::from(other[e12345]) * anti_reverse.group0(),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(other[e12345]) * anti_reverse.group1(),
        );
    }
}
impl std::ops::Div<AntiInversePrefixOrPostfix> for Line {
    type Output = Line;
    fn div(self, _rhs: AntiInversePrefixOrPostfix) -> Self::Output {
        self.anti_inverse()
    }
}
impl std::ops::DivAssign<AntiInversePrefixOrPostfix> for Line {
    fn div_assign(&mut self, _rhs: AntiInversePrefixOrPostfix) {
        *self = self.anti_inverse()
    }
}
impl AntiInverse for Line {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        0        0
    //    simd3        0        4        0
    // Totals...
    // yes simd        2        4        0
    //  no simd        2       12        0
    fn anti_inverse(self) -> Self {
        use crate::elements::*;
        let anti_reverse = Line::from_groups(
            // e415, e425, e435
            self.group0() * Simd32x3::from(-1.0),
            // e235, e315, e125
            self.group1() * Simd32x3::from(-1.0),
        );
        let other = AntiScalar::from_groups(/* e12345 */ f32::powi(self[e415], 2) + f32::powi(self[e425], 2) + f32::powi(self[e435], 2));
        return Line::from_groups(
            // e415, e425, e435
            Simd32x3::from(other[e12345]) * anti_reverse.group0(),
            // e235, e315, e125
            Simd32x3::from(other[e12345]) * anti_reverse.group1(),
        );
    }
}
impl std::ops::Div<AntiInversePrefixOrPostfix> for Motor {
    type Output = Motor;
    fn div(self, _rhs: AntiInversePrefixOrPostfix) -> Self::Output {
        self.anti_inverse()
    }
}
impl std::ops::DivAssign<AntiInversePrefixOrPostfix> for Motor {
    fn div_assign(&mut self, _rhs: AntiInversePrefixOrPostfix) {
        *self = self.anti_inverse()
    }
}
impl AntiInverse for Motor {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        0        0
    //    simd4        0        4        0
    // Totals...
    // yes simd        3        4        0
    //  no simd        3       16        0
    fn anti_inverse(self) -> Self {
        use crate::elements::*;
        let anti_reverse = Motor::from_groups(
            // e415, e425, e435, e12345
            self.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e235, e315, e125, e5
            self.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
        );
        let other = AntiScalar::from_groups(
            // e12345
            f32::powi(self[e415], 2) + f32::powi(self[e425], 2) + f32::powi(self[e435], 2) + f32::powi(self[e12345], 2),
        );
        return Motor::from_groups(
            // e415, e425, e435, e12345
            Simd32x4::from(other[e12345]) * anti_reverse.group0(),
            // e235, e315, e125, e5
            Simd32x4::from(other[e12345]) * anti_reverse.group1(),
        );
    }
}
impl std::ops::Div<AntiInversePrefixOrPostfix> for MultiVector {
    type Output = MultiVector;
    fn div(self, _rhs: AntiInversePrefixOrPostfix) -> Self::Output {
        self.anti_inverse()
    }
}
impl std::ops::DivAssign<AntiInversePrefixOrPostfix> for MultiVector {
    fn div_assign(&mut self, _rhs: AntiInversePrefixOrPostfix) {
        *self = self.anti_inverse()
    }
}
impl AntiInverse for MultiVector {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       23       18        0
    //    simd2        0        1        0
    //    simd3        0        8        0
    //    simd4        0        6        0
    // Totals...
    // yes simd       23       33        0
    //  no simd       23       68        0
    fn anti_inverse(self) -> Self {
        use crate::elements::*;
        let anti_reverse = MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e15, e25, e35, e45
            self.group3() * Simd32x4::from(-1.0),
            // e41, e42, e43
            self.group4() * Simd32x3::from(-1.0),
            // e23, e31, e12
            self.group5() * Simd32x3::from(-1.0),
            // e415, e425, e435, e321
            self.group6() * Simd32x4::from(-1.0),
            // e423, e431, e412
            self.group7() * Simd32x3::from(-1.0),
            // e235, e315, e125
            self.group8() * Simd32x3::from(-1.0),
            // e4235, e4315, e4125, e3215
            self.group9(),
            // e1234
            self[e1234],
        );
        let other = AntiScalar::from_groups(
            // e12345
            2.0 * (self[e4] * self[e5])
                + 2.0 * (self[e423] * self[e235])
                + 2.0 * (self[e431] * self[e315])
                + 2.0 * (self[e412] * self[e125])
                + f32::powi(self[e12345], 2)
                + f32::powi(self[e45], 2)
                + f32::powi(self[e415], 2)
                + f32::powi(self[e425], 2)
                + f32::powi(self[e435], 2)
                + f32::powi(self[e4235], 2)
                + f32::powi(self[e4315], 2)
                + f32::powi(self[e4125], 2)
                - f32::powi(self[scalar], 2)
                - f32::powi(self[e1], 2)
                - f32::powi(self[e2], 2)
                - f32::powi(self[e3], 2)
                - f32::powi(self[e23], 2)
                - f32::powi(self[e31], 2)
                - f32::powi(self[e12], 2)
                - f32::powi(self[e321], 2)
                - 2.0 * (self[e15] * self[e41])
                - 2.0 * (self[e25] * self[e42])
                - 2.0 * (self[e35] * self[e43])
                - 2.0 * (self[e3215] * self[e1234]),
        );
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(other[e12345]) * anti_reverse.group0(),
            // e1, e2, e3, e4
            Simd32x4::from(other[e12345]) * anti_reverse.group1(),
            // e5
            other[e12345] * anti_reverse[e5],
            // e15, e25, e35, e45
            Simd32x4::from(other[e12345]) * anti_reverse.group3(),
            // e41, e42, e43
            Simd32x3::from(other[e12345]) * anti_reverse.group4(),
            // e23, e31, e12
            Simd32x3::from(other[e12345]) * anti_reverse.group5(),
            // e415, e425, e435, e321
            Simd32x4::from(other[e12345]) * anti_reverse.group6(),
            // e423, e431, e412
            Simd32x3::from(other[e12345]) * anti_reverse.group7(),
            // e235, e315, e125
            Simd32x3::from(other[e12345]) * anti_reverse.group8(),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(other[e12345]) * anti_reverse.group9(),
            // e1234
            other[e12345] * anti_reverse[e1234],
        );
    }
}
impl std::ops::Div<AntiInversePrefixOrPostfix> for Plane {
    type Output = Plane;
    fn div(self, _rhs: AntiInversePrefixOrPostfix) -> Self::Output {
        self.anti_inverse()
    }
}
impl std::ops::DivAssign<AntiInversePrefixOrPostfix> for Plane {
    fn div_assign(&mut self, _rhs: AntiInversePrefixOrPostfix) {
        *self = self.anti_inverse()
    }
}
impl AntiInverse for Plane {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        0        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        2        1        0
    //  no simd        2        4        0
    fn anti_inverse(self) -> Self {
        use crate::elements::*;
        return Plane::from_groups(
            // e4235, e4315, e4125, e3215
            Simd32x4::from(f32::powi(self[e4235], 2) + f32::powi(self[e4315], 2) + f32::powi(self[e4125], 2)) * self.group0(),
        );
    }
}
impl std::ops::Div<AntiInversePrefixOrPostfix> for RoundPoint {
    type Output = RoundPoint;
    fn div(self, _rhs: AntiInversePrefixOrPostfix) -> Self::Output {
        self.anti_inverse()
    }
}
impl std::ops::DivAssign<AntiInversePrefixOrPostfix> for RoundPoint {
    fn div_assign(&mut self, _rhs: AntiInversePrefixOrPostfix) {
        *self = self.anti_inverse()
    }
}
impl AntiInverse for RoundPoint {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        3        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        3        4        0
    //  no simd        3        7        0
    fn anti_inverse(self) -> Self {
        use crate::elements::*;
        let other = AntiScalar::from_groups(/* e12345 */ 2.0 * (self[e4] * self[e5]) - f32::powi(self[e1], 2) - f32::powi(self[e2], 2) - f32::powi(self[e3], 2));
        return RoundPoint::from_groups(/* e1, e2, e3, e4 */ Simd32x4::from(other[e12345]) * self.group0(), /* e5 */ other[e12345] * self[e5]);
    }
}
impl std::ops::Div<AntiInversePrefixOrPostfix> for Scalar {
    type Output = Scalar;
    fn div(self, _rhs: AntiInversePrefixOrPostfix) -> Self::Output {
        self.anti_inverse()
    }
}
impl std::ops::DivAssign<AntiInversePrefixOrPostfix> for Scalar {
    fn div_assign(&mut self, _rhs: AntiInversePrefixOrPostfix) {
        *self = self.anti_inverse()
    }
}
impl AntiInverse for Scalar {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        1
    fn anti_inverse(self) -> Self {
        use crate::elements::*;
        return Scalar::from_groups(/* scalar */ 1.0 / self[scalar] * -1.0);
    }
}
impl std::ops::Div<AntiInversePrefixOrPostfix> for Sphere {
    type Output = Sphere;
    fn div(self, _rhs: AntiInversePrefixOrPostfix) -> Self::Output {
        self.anti_inverse()
    }
}
impl std::ops::DivAssign<AntiInversePrefixOrPostfix> for Sphere {
    fn div_assign(&mut self, _rhs: AntiInversePrefixOrPostfix) {
        *self = self.anti_inverse()
    }
}
impl AntiInverse for Sphere {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        3        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        3        4        0
    //  no simd        3        7        0
    fn anti_inverse(self) -> Self {
        use crate::elements::*;
        let other = AntiScalar::from_groups(
            // e12345
            f32::powi(self[e4235], 2) + f32::powi(self[e4315], 2) + f32::powi(self[e4125], 2) - 2.0 * (self[e3215] * self[e1234]),
        );
        return Sphere::from_groups(
            // e4235, e4315, e4125, e3215
            Simd32x4::from(other[e12345]) * self.group0(),
            // e1234
            other[e12345] * self[e1234],
        );
    }
}
impl std::ops::Div<AntiInversePrefixOrPostfix> for VersorEven {
    type Output = VersorEven;
    fn div(self, _rhs: AntiInversePrefixOrPostfix) -> Self::Output {
        self.anti_inverse()
    }
}
impl std::ops::DivAssign<AntiInversePrefixOrPostfix> for VersorEven {
    fn div_assign(&mut self, _rhs: AntiInversePrefixOrPostfix) {
        *self = self.anti_inverse()
    }
}
impl AntiInverse for VersorEven {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       11        8        0
    //    simd4        0        7        0
    // Totals...
    // yes simd       11       15        0
    //  no simd       11       36        0
    fn anti_inverse(self) -> Self {
        use crate::elements::*;
        let anti_reverse = VersorEven::from_groups(
            // e423, e431, e412, e12345
            self.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e415, e425, e435, e321
            self.group1() * Simd32x4::from(-1.0),
            // e235, e315, e125, e5
            self.group2() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e1, e2, e3, e4
            self.group3(),
        );
        let other = AntiScalar::from_groups(
            // e12345
            2.0 * (self[e423] * self[e235])
                + 2.0 * (self[e431] * self[e315])
                + 2.0 * (self[e412] * self[e125])
                + 2.0 * (self[e5] * self[e4])
                + f32::powi(self[e12345], 2)
                + f32::powi(self[e415], 2)
                + f32::powi(self[e425], 2)
                + f32::powi(self[e435], 2)
                - f32::powi(self[e321], 2)
                - f32::powi(self[e1], 2)
                - f32::powi(self[e2], 2)
                - f32::powi(self[e3], 2),
        );
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from(other[e12345]) * anti_reverse.group0(),
            // e415, e425, e435, e321
            Simd32x4::from(other[e12345]) * anti_reverse.group1(),
            // e235, e315, e125, e5
            Simd32x4::from(other[e12345]) * anti_reverse.group2(),
            // e1, e2, e3, e4
            Simd32x4::from(other[e12345]) * anti_reverse.group3(),
        );
    }
}
impl std::ops::Div<AntiInversePrefixOrPostfix> for VersorOdd {
    type Output = VersorOdd;
    fn div(self, _rhs: AntiInversePrefixOrPostfix) -> Self::Output {
        self.anti_inverse()
    }
}
impl std::ops::DivAssign<AntiInversePrefixOrPostfix> for VersorOdd {
    fn div_assign(&mut self, _rhs: AntiInversePrefixOrPostfix) {
        *self = self.anti_inverse()
    }
}
impl AntiInverse for VersorOdd {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       11        8        0
    //    simd4        0        7        0
    // Totals...
    // yes simd       11       15        0
    //  no simd       11       36        0
    fn anti_inverse(self) -> Self {
        use crate::elements::*;
        let anti_reverse = VersorOdd::from_groups(
            // e41, e42, e43, scalar
            self.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e23, e31, e12, e45
            self.group1() * Simd32x4::from(-1.0),
            // e15, e25, e35, e1234
            self.group2() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e4235, e4315, e4125, e3215
            self.group3(),
        );
        let other = AntiScalar::from_groups(
            // e12345
            f32::powi(self[e45], 2) + f32::powi(self[e4235], 2) + f32::powi(self[e4315], 2) + f32::powi(self[e4125], 2)
                - f32::powi(self[scalar], 2)
                - f32::powi(self[e23], 2)
                - f32::powi(self[e31], 2)
                - f32::powi(self[e12], 2)
                - 2.0 * (self[e41] * self[e15])
                - 2.0 * (self[e42] * self[e25])
                - 2.0 * (self[e43] * self[e35])
                - 2.0 * (self[e1234] * self[e3215]),
        );
        return VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from(other[e12345]) * anti_reverse.group0(),
            // e23, e31, e12, e45
            Simd32x4::from(other[e12345]) * anti_reverse.group1(),
            // e15, e25, e35, e1234
            Simd32x4::from(other[e12345]) * anti_reverse.group2(),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(other[e12345]) * anti_reverse.group3(),
        );
    }
}
