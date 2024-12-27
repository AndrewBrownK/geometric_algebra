// Note on Operative Statistics:
// Operative Statistics are not a precise predictor of performance or performance comparisons.
// This is due to varying hardware capabilities and compiler optimizations.
// As always, where performance is a concern, there is no substitute for
// real measurements on real work-loads on real hardware.
// Disclaimer aside, enjoy the fun information =)
//
// Total Implementations: 83
//
// Yes SIMD:   add/sub     mul     div
//  Minimum:         0       0       0
//   Median:         3       4       0
//  Average:         3       6       0
//  Maximum:        23      33       2
//
//  No SIMD:   add/sub     mul     div
//  Minimum:         0       0       0
//   Median:         3      12       0
//  Average:         3      15       0
//  Maximum:        23      68       2
impl std::ops::Div<InversePrefixOrPostfix> for AntiCircleOnOrigin {
    type Output = AntiCircleOnOrigin;
    fn div(self, _rhs: InversePrefixOrPostfix) -> Self::Output {
        self.inverse()
    }
}
impl std::ops::DivAssign<InversePrefixOrPostfix> for AntiCircleOnOrigin {
    fn div_assign(&mut self, _rhs: InversePrefixOrPostfix) {
        *self = self.inverse()
    }
}
impl Inverse for AntiCircleOnOrigin {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        0        0
    //    simd3        0        4        0
    // Totals...
    // yes simd        2        4        0
    //  no simd        2       12        0
    fn inverse(self) -> Self {
        use crate::elements::*;
        let reverse = AntiCircleOnOrigin::from_groups(
            // e41, e42, e43
            self.group0() * Simd32x3::from(-1.0),
            // e23, e31, e12
            self.group1() * Simd32x3::from(-1.0),
        );
        let other = Scalar::from_groups(/* scalar */ f32::powi(self[e23], 2) + f32::powi(self[e31], 2) + f32::powi(self[e12], 2));
        return AntiCircleOnOrigin::from_groups(
            // e41, e42, e43
            Simd32x3::from(other[scalar]) * reverse.group0(),
            // e23, e31, e12
            Simd32x3::from(other[scalar]) * reverse.group1(),
        );
    }
}
impl std::ops::Div<InversePrefixOrPostfix> for AntiCircleRotor {
    type Output = AntiCircleRotor;
    fn div(self, _rhs: InversePrefixOrPostfix) -> Self::Output {
        self.inverse()
    }
}
impl std::ops::DivAssign<InversePrefixOrPostfix> for AntiCircleRotor {
    fn div_assign(&mut self, _rhs: InversePrefixOrPostfix) {
        *self = self.inverse()
    }
}
impl Inverse for AntiCircleRotor {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        7        6        0
    //    simd3        0        2        0
    //    simd4        0        4        0
    // Totals...
    // yes simd        7       12        0
    //  no simd        7       28        0
    fn inverse(self) -> Self {
        use crate::elements::*;
        let reverse = AntiCircleRotor::from_groups(
            // e41, e42, e43
            self.group0() * Simd32x3::from(-1.0),
            // e23, e31, e12, e45
            self.group1() * Simd32x4::from(-1.0),
            // e15, e25, e35, scalar
            self.group2() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
        );
        let other = Scalar::from_groups(
            // scalar
            2.0 * (self[e41] * self[e15])
                + 2.0 * (self[e42] * self[e25])
                + 2.0 * (self[e43] * self[e35])
                + f32::powi(self[e23], 2)
                + f32::powi(self[e31], 2)
                + f32::powi(self[e12], 2)
                + f32::powi(self[scalar], 2)
                - f32::powi(self[e45], 2),
        );
        return AntiCircleRotor::from_groups(
            // e41, e42, e43
            Simd32x3::from(other[scalar]) * reverse.group0(),
            // e23, e31, e12, e45
            Simd32x4::from(other[scalar]) * reverse.group1(),
            // e15, e25, e35, scalar
            Simd32x4::from(other[scalar]) * reverse.group2(),
        );
    }
}
impl std::ops::Div<InversePrefixOrPostfix> for AntiCircleRotorAligningOrigin {
    type Output = AntiCircleRotorAligningOrigin;
    fn div(self, _rhs: InversePrefixOrPostfix) -> Self::Output {
        self.inverse()
    }
}
impl std::ops::DivAssign<InversePrefixOrPostfix> for AntiCircleRotorAligningOrigin {
    fn div_assign(&mut self, _rhs: InversePrefixOrPostfix) {
        *self = self.inverse()
    }
}
impl Inverse for AntiCircleRotorAligningOrigin {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        6        6        0
    //    simd3        0        4        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        6       12        0
    //  no simd        6       26        0
    fn inverse(self) -> Self {
        use crate::elements::*;
        let reverse = AntiCircleRotorAligningOrigin::from_groups(
            // e41, e42, e43
            self.group0() * Simd32x3::from(-1.0),
            // e23, e31, e12
            self.group1() * Simd32x3::from(-1.0),
            // e15, e25, e35, scalar
            self.group2() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
        );
        let other = Scalar::from_groups(
            // scalar
            2.0 * (self[e41] * self[e15])
                + 2.0 * (self[e42] * self[e25])
                + 2.0 * (self[e43] * self[e35])
                + f32::powi(self[e23], 2)
                + f32::powi(self[e31], 2)
                + f32::powi(self[e12], 2)
                + f32::powi(self[scalar], 2),
        );
        return AntiCircleRotorAligningOrigin::from_groups(
            // e41, e42, e43
            Simd32x3::from(other[scalar]) * reverse.group0(),
            // e23, e31, e12
            Simd32x3::from(other[scalar]) * reverse.group1(),
            // e15, e25, e35, scalar
            Simd32x4::from(other[scalar]) * reverse.group2(),
        );
    }
}
impl std::ops::Div<InversePrefixOrPostfix> for AntiCircleRotorAligningOriginAtInfinity {
    type Output = AntiCircleRotorAligningOriginAtInfinity;
    fn div(self, _rhs: InversePrefixOrPostfix) -> Self::Output {
        self.inverse()
    }
}
impl std::ops::DivAssign<InversePrefixOrPostfix> for AntiCircleRotorAligningOriginAtInfinity {
    fn div_assign(&mut self, _rhs: InversePrefixOrPostfix) {
        *self = self.inverse()
    }
}
impl Inverse for AntiCircleRotorAligningOriginAtInfinity {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        0        0
    //    simd3        0        2        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        3        4        0
    //  no simd        3       14        0
    fn inverse(self) -> Self {
        use crate::elements::*;
        let reverse = AntiCircleRotorAligningOriginAtInfinity::from_groups(
            // e23, e31, e12
            self.group0() * Simd32x3::from(-1.0),
            // e15, e25, e35, scalar
            self.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
        );
        let other = Scalar::from_groups(
            // scalar
            f32::powi(self[e23], 2) + f32::powi(self[e31], 2) + f32::powi(self[e12], 2) + f32::powi(self[scalar], 2),
        );
        return AntiCircleRotorAligningOriginAtInfinity::from_groups(
            // e23, e31, e12
            Simd32x3::from(other[scalar]) * reverse.group0(),
            // e15, e25, e35, scalar
            Simd32x4::from(other[scalar]) * reverse.group1(),
        );
    }
}
impl std::ops::Div<InversePrefixOrPostfix> for AntiCircleRotorAtInfinity {
    type Output = AntiCircleRotorAtInfinity;
    fn div(self, _rhs: InversePrefixOrPostfix) -> Self::Output {
        self.inverse()
    }
}
impl std::ops::DivAssign<InversePrefixOrPostfix> for AntiCircleRotorAtInfinity {
    fn div_assign(&mut self, _rhs: InversePrefixOrPostfix) {
        *self = self.inverse()
    }
}
impl Inverse for AntiCircleRotorAtInfinity {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        0        0
    //    simd4        0        4        0
    // Totals...
    // yes simd        4        4        0
    //  no simd        4       16        0
    fn inverse(self) -> Self {
        use crate::elements::*;
        let reverse = AntiCircleRotorAtInfinity::from_groups(
            // e23, e31, e12, e45
            self.group0() * Simd32x4::from(-1.0),
            // e15, e25, e35, scalar
            self.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
        );
        let other = Scalar::from_groups(
            // scalar
            f32::powi(self[e23], 2) + f32::powi(self[e31], 2) + f32::powi(self[e12], 2) + f32::powi(self[scalar], 2) - f32::powi(self[e45], 2),
        );
        return AntiCircleRotorAtInfinity::from_groups(
            // e23, e31, e12, e45
            Simd32x4::from(other[scalar]) * reverse.group0(),
            // e15, e25, e35, scalar
            Simd32x4::from(other[scalar]) * reverse.group1(),
        );
    }
}
impl std::ops::Div<InversePrefixOrPostfix> for AntiCircleRotorOnOrigin {
    type Output = AntiCircleRotorOnOrigin;
    fn div(self, _rhs: InversePrefixOrPostfix) -> Self::Output {
        self.inverse()
    }
}
impl std::ops::DivAssign<InversePrefixOrPostfix> for AntiCircleRotorOnOrigin {
    fn div_assign(&mut self, _rhs: InversePrefixOrPostfix) {
        *self = self.inverse()
    }
}
impl Inverse for AntiCircleRotorOnOrigin {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        0        0
    //    simd3        0        2        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        3        4        0
    //  no simd        3       14        0
    fn inverse(self) -> Self {
        use crate::elements::*;
        let reverse = AntiCircleRotorOnOrigin::from_groups(
            // e41, e42, e43, scalar
            self.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e23, e31, e12
            self.group1() * Simd32x3::from(-1.0),
        );
        let other = Scalar::from_groups(
            // scalar
            f32::powi(self[scalar], 2) + f32::powi(self[e23], 2) + f32::powi(self[e31], 2) + f32::powi(self[e12], 2),
        );
        return AntiCircleRotorOnOrigin::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from(other[scalar]) * reverse.group0(),
            // e23, e31, e12
            Simd32x3::from(other[scalar]) * reverse.group1(),
        );
    }
}
impl std::ops::Div<InversePrefixOrPostfix> for AntiDipoleInversion {
    type Output = AntiDipoleInversion;
    fn div(self, _rhs: InversePrefixOrPostfix) -> Self::Output {
        self.inverse()
    }
}
impl std::ops::DivAssign<InversePrefixOrPostfix> for AntiDipoleInversion {
    fn div_assign(&mut self, _rhs: InversePrefixOrPostfix) {
        *self = self.inverse()
    }
}
impl Inverse for AntiDipoleInversion {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       10        8        0
    //    simd3        0        2        0
    //    simd4        0        5        0
    // Totals...
    // yes simd       10       15        0
    //  no simd       10       34        0
    fn inverse(self) -> Self {
        use crate::elements::*;
        let reverse = AntiDipoleInversion::from_groups(
            // e423, e431, e412
            self.group0() * Simd32x3::from(-1.0),
            // e415, e425, e435, e321
            self.group1() * Simd32x4::from(-1.0),
            // e235, e315, e125, e4
            self.group2() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e1, e2, e3, e5
            self.group3(),
        );
        let other = Scalar::from_groups(
            // scalar
            f32::powi(self[e321], 2) + f32::powi(self[e1], 2) + f32::powi(self[e2], 2) + f32::powi(self[e3], 2)
                - f32::powi(self[e415], 2)
                - f32::powi(self[e425], 2)
                - f32::powi(self[e435], 2)
                - 2.0 * (self[e423] * self[e235])
                - 2.0 * (self[e431] * self[e315])
                - 2.0 * (self[e412] * self[e125])
                - 2.0 * (self[e4] * self[e5]),
        );
        return AntiDipoleInversion::from_groups(
            // e423, e431, e412
            Simd32x3::from(other[scalar]) * reverse.group0(),
            // e415, e425, e435, e321
            Simd32x4::from(other[scalar]) * reverse.group1(),
            // e235, e315, e125, e4
            Simd32x4::from(other[scalar]) * reverse.group2(),
            // e1, e2, e3, e5
            Simd32x4::from(other[scalar]) * reverse.group3(),
        );
    }
}
impl std::ops::Div<InversePrefixOrPostfix> for AntiDipoleInversionAtInfinity {
    type Output = AntiDipoleInversionAtInfinity;
    fn div(self, _rhs: InversePrefixOrPostfix) -> Self::Output {
        self.inverse()
    }
}
impl std::ops::DivAssign<InversePrefixOrPostfix> for AntiDipoleInversionAtInfinity {
    fn div_assign(&mut self, _rhs: InversePrefixOrPostfix) {
        *self = self.inverse()
    }
}
impl Inverse for AntiDipoleInversionAtInfinity {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        6        0        0
    //    simd3        0        2        0
    //    simd4        0        3        0
    // Totals...
    // yes simd        6        5        0
    //  no simd        6       18        0
    fn inverse(self) -> Self {
        use crate::elements::*;
        let reverse = AntiDipoleInversionAtInfinity::from_groups(
            // e415, e425, e435, e321
            self.group0() * Simd32x4::from(-1.0),
            // e235, e315, e125
            self.group1() * Simd32x3::from(-1.0),
            // e1, e2, e3, e5
            self.group2(),
        );
        let other = Scalar::from_groups(
            // scalar
            f32::powi(self[e321], 2) + f32::powi(self[e1], 2) + f32::powi(self[e2], 2) + f32::powi(self[e3], 2)
                - f32::powi(self[e415], 2)
                - f32::powi(self[e425], 2)
                - f32::powi(self[e435], 2),
        );
        return AntiDipoleInversionAtInfinity::from_groups(
            // e415, e425, e435, e321
            Simd32x4::from(other[scalar]) * reverse.group0(),
            // e235, e315, e125
            Simd32x3::from(other[scalar]) * reverse.group1(),
            // e1, e2, e3, e5
            Simd32x4::from(other[scalar]) * reverse.group2(),
        );
    }
}
impl std::ops::Div<InversePrefixOrPostfix> for AntiDipoleInversionOnOrigin {
    type Output = AntiDipoleInversionOnOrigin;
    fn div(self, _rhs: InversePrefixOrPostfix) -> Self::Output {
        self.inverse()
    }
}
impl std::ops::DivAssign<InversePrefixOrPostfix> for AntiDipoleInversionOnOrigin {
    fn div_assign(&mut self, _rhs: InversePrefixOrPostfix) {
        *self = self.inverse()
    }
}
impl Inverse for AntiDipoleInversionOnOrigin {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        0        0
    //    simd4        0        3        0
    // Totals...
    // yes simd        3        3        0
    //  no simd        3       12        0
    fn inverse(self) -> Self {
        use crate::elements::*;
        let reverse = AntiDipoleInversionOnOrigin::from_groups(/* e423, e431, e412, e321 */ self.group0() * Simd32x4::from(-1.0), /* e4, e1, e2, e3 */ self.group1());
        let other = Scalar::from_groups(/* scalar */ f32::powi(self[e321], 2) + f32::powi(self[e1], 2) + f32::powi(self[e2], 2) + f32::powi(self[e3], 2));
        return AntiDipoleInversionOnOrigin::from_groups(
            // e423, e431, e412, e321
            Simd32x4::from(other[scalar]) * reverse.group0(),
            // e4, e1, e2, e3
            Simd32x4::from(other[scalar]) * reverse.group1(),
        );
    }
}
impl std::ops::Div<InversePrefixOrPostfix> for AntiDipoleInversionOrthogonalOrigin {
    type Output = AntiDipoleInversionOrthogonalOrigin;
    fn div(self, _rhs: InversePrefixOrPostfix) -> Self::Output {
        self.inverse()
    }
}
impl std::ops::DivAssign<InversePrefixOrPostfix> for AntiDipoleInversionOrthogonalOrigin {
    fn div_assign(&mut self, _rhs: InversePrefixOrPostfix) {
        *self = self.inverse()
    }
}
impl Inverse for AntiDipoleInversionOrthogonalOrigin {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        6        8        0
    //    simd3        0        2        0
    //    simd4        0        4        0
    // Totals...
    // yes simd        6       14        0
    //  no simd        6       30        0
    fn inverse(self) -> Self {
        use crate::elements::*;
        let reverse = AntiDipoleInversionOrthogonalOrigin::from_groups(
            // e423, e431, e412, e5
            self.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e415, e425, e435
            self.group1() * Simd32x3::from(-1.0),
            // e235, e315, e125, e4
            self.group2() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
        );
        let other = Scalar::from_groups(
            // scalar
            -f32::powi(self[e415], 2)
                - f32::powi(self[e425], 2)
                - f32::powi(self[e435], 2)
                - 2.0 * (self[e423] * self[e235])
                - 2.0 * (self[e431] * self[e315])
                - 2.0 * (self[e412] * self[e125])
                - 2.0 * (self[e5] * self[e4]),
        );
        return AntiDipoleInversionOrthogonalOrigin::from_groups(
            // e423, e431, e412, e5
            Simd32x4::from(other[scalar]) * reverse.group0(),
            // e415, e425, e435
            Simd32x3::from(other[scalar]) * reverse.group1(),
            // e235, e315, e125, e4
            Simd32x4::from(other[scalar]) * reverse.group2(),
        );
    }
}
impl std::ops::Div<InversePrefixOrPostfix> for AntiDipoleOnOrigin {
    type Output = AntiDipoleOnOrigin;
    fn div(self, _rhs: InversePrefixOrPostfix) -> Self::Output {
        self.inverse()
    }
}
impl std::ops::DivAssign<InversePrefixOrPostfix> for AntiDipoleOnOrigin {
    fn div_assign(&mut self, _rhs: InversePrefixOrPostfix) {
        *self = self.inverse()
    }
}
impl Inverse for AntiDipoleOnOrigin {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        4        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        5        0
    //  no simd        0        8        0
    fn inverse(self) -> Self {
        use crate::elements::*;
        return AntiDipoleOnOrigin::from_groups(
            // e423, e431, e412, e321
            Simd32x4::from(f32::powi(self[e321], -2)) * Simd32x4::from([self[e423] * -1.0, self[e431] * -1.0, self[e412] * -1.0, self[e321] * -1.0]),
        );
    }
}
impl std::ops::Div<InversePrefixOrPostfix> for AntiDualNum {
    type Output = AntiDualNum;
    fn div(self, _rhs: InversePrefixOrPostfix) -> Self::Output {
        self.inverse()
    }
}
impl std::ops::DivAssign<InversePrefixOrPostfix> for AntiDualNum {
    fn div_assign(&mut self, _rhs: InversePrefixOrPostfix) {
        *self = self.inverse()
    }
}
impl Inverse for AntiDualNum {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd2        0        1        0
    // no simd        0        2        0
    fn inverse(self) -> Self {
        use crate::elements::*;
        return AntiDualNum::from_groups(/* e1234, scalar */ Simd32x2::from(f32::powi(self[scalar], -2)) * self.group0());
    }
}
impl std::ops::Div<InversePrefixOrPostfix> for AntiFlatOrigin {
    type Output = AntiFlatOrigin;
    fn div(self, _rhs: InversePrefixOrPostfix) -> Self::Output {
        self.inverse()
    }
}
impl std::ops::DivAssign<InversePrefixOrPostfix> for AntiFlatOrigin {
    fn div_assign(&mut self, _rhs: InversePrefixOrPostfix) {
        *self = self.inverse()
    }
}
impl Inverse for AntiFlatOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        1
    fn inverse(self) -> Self {
        use crate::elements::*;
        return AntiFlatOrigin::from_groups(/* e321 */ 1.0 / self[e321] * -1.0);
    }
}
impl std::ops::Div<InversePrefixOrPostfix> for AntiFlatPoint {
    type Output = AntiFlatPoint;
    fn div(self, _rhs: InversePrefixOrPostfix) -> Self::Output {
        self.inverse()
    }
}
impl std::ops::DivAssign<InversePrefixOrPostfix> for AntiFlatPoint {
    fn div_assign(&mut self, _rhs: InversePrefixOrPostfix) {
        *self = self.inverse()
    }
}
impl Inverse for AntiFlatPoint {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        4        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        5        0
    //  no simd        0        8        0
    fn inverse(self) -> Self {
        use crate::elements::*;
        return AntiFlatPoint::from_groups(
            // e235, e315, e125, e321
            Simd32x4::from(f32::powi(self[e321], -2)) * Simd32x4::from([self[e235] * -1.0, self[e315] * -1.0, self[e125] * -1.0, self[e321] * -1.0]),
        );
    }
}
impl std::ops::Div<InversePrefixOrPostfix> for AntiFlector {
    type Output = AntiFlector;
    fn div(self, _rhs: InversePrefixOrPostfix) -> Self::Output {
        self.inverse()
    }
}
impl std::ops::DivAssign<InversePrefixOrPostfix> for AntiFlector {
    fn div_assign(&mut self, _rhs: InversePrefixOrPostfix) {
        *self = self.inverse()
    }
}
impl Inverse for AntiFlector {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        0        0
    //    simd4        0        3        0
    // Totals...
    // yes simd        3        3        0
    //  no simd        3       12        0
    fn inverse(self) -> Self {
        use crate::elements::*;
        let reverse = AntiFlector::from_groups(/* e235, e315, e125, e321 */ self.group0() * Simd32x4::from(-1.0), /* e1, e2, e3, e5 */ self.group1());
        let other = Scalar::from_groups(/* scalar */ f32::powi(self[e321], 2) + f32::powi(self[e1], 2) + f32::powi(self[e2], 2) + f32::powi(self[e3], 2));
        return AntiFlector::from_groups(
            // e235, e315, e125, e321
            Simd32x4::from(other[scalar]) * reverse.group0(),
            // e1, e2, e3, e5
            Simd32x4::from(other[scalar]) * reverse.group1(),
        );
    }
}
impl std::ops::Div<InversePrefixOrPostfix> for AntiFlectorOnOrigin {
    type Output = AntiFlectorOnOrigin;
    fn div(self, _rhs: InversePrefixOrPostfix) -> Self::Output {
        self.inverse()
    }
}
impl std::ops::DivAssign<InversePrefixOrPostfix> for AntiFlectorOnOrigin {
    fn div_assign(&mut self, _rhs: InversePrefixOrPostfix) {
        *self = self.inverse()
    }
}
impl Inverse for AntiFlectorOnOrigin {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        3        2        0
    //  no simd        3        5        0
    fn inverse(self) -> Self {
        use crate::elements::*;
        return AntiFlectorOnOrigin::from_groups(
            // e321, e1, e2, e3
            Simd32x4::from(f32::powi(self[e321], 2) + f32::powi(self[e1], 2) + f32::powi(self[e2], 2) + f32::powi(self[e3], 2))
                * Simd32x4::from([self[e321] * -1.0, self[e1], self[e2], self[e3]]),
        );
    }
}
impl std::ops::Div<InversePrefixOrPostfix> for AntiLine {
    type Output = AntiLine;
    fn div(self, _rhs: InversePrefixOrPostfix) -> Self::Output {
        self.inverse()
    }
}
impl std::ops::DivAssign<InversePrefixOrPostfix> for AntiLine {
    fn div_assign(&mut self, _rhs: InversePrefixOrPostfix) {
        *self = self.inverse()
    }
}
impl Inverse for AntiLine {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        0        0
    //    simd3        0        4        0
    // Totals...
    // yes simd        2        4        0
    //  no simd        2       12        0
    fn inverse(self) -> Self {
        use crate::elements::*;
        let reverse = AntiLine::from_groups(
            // e23, e31, e12
            self.group0() * Simd32x3::from(-1.0),
            // e15, e25, e35
            self.group1() * Simd32x3::from(-1.0),
        );
        let other = Scalar::from_groups(/* scalar */ f32::powi(self[e23], 2) + f32::powi(self[e31], 2) + f32::powi(self[e12], 2));
        return AntiLine::from_groups(
            // e23, e31, e12
            Simd32x3::from(other[scalar]) * reverse.group0(),
            // e15, e25, e35
            Simd32x3::from(other[scalar]) * reverse.group1(),
        );
    }
}
impl std::ops::Div<InversePrefixOrPostfix> for AntiLineOnOrigin {
    type Output = AntiLineOnOrigin;
    fn div(self, _rhs: InversePrefixOrPostfix) -> Self::Output {
        self.inverse()
    }
}
impl std::ops::DivAssign<InversePrefixOrPostfix> for AntiLineOnOrigin {
    fn div_assign(&mut self, _rhs: InversePrefixOrPostfix) {
        *self = self.inverse()
    }
}
impl Inverse for AntiLineOnOrigin {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        3        0
    //    simd3        0        1        0
    // Totals...
    // yes simd        2        4        0
    //  no simd        2        6        0
    fn inverse(self) -> Self {
        use crate::elements::*;
        return AntiLineOnOrigin::from_groups(
            // e23, e31, e12
            Simd32x3::from(f32::powi(self[e23], 2) + f32::powi(self[e31], 2) + f32::powi(self[e12], 2)) * Simd32x3::from([self[e23] * -1.0, self[e31] * -1.0, self[e12] * -1.0]),
        );
    }
}
impl std::ops::Div<InversePrefixOrPostfix> for AntiMotor {
    type Output = AntiMotor;
    fn div(self, _rhs: InversePrefixOrPostfix) -> Self::Output {
        self.inverse()
    }
}
impl std::ops::DivAssign<InversePrefixOrPostfix> for AntiMotor {
    fn div_assign(&mut self, _rhs: InversePrefixOrPostfix) {
        *self = self.inverse()
    }
}
impl Inverse for AntiMotor {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        0        0
    //    simd4        0        4        0
    // Totals...
    // yes simd        3        4        0
    //  no simd        3       16        0
    fn inverse(self) -> Self {
        use crate::elements::*;
        let reverse = AntiMotor::from_groups(
            // e23, e31, e12, scalar
            self.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e15, e25, e35, e3215
            self.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
        );
        let other = Scalar::from_groups(
            // scalar
            f32::powi(self[e23], 2) + f32::powi(self[e31], 2) + f32::powi(self[e12], 2) + f32::powi(self[scalar], 2),
        );
        return AntiMotor::from_groups(
            // e23, e31, e12, scalar
            Simd32x4::from(other[scalar]) * reverse.group0(),
            // e15, e25, e35, e3215
            Simd32x4::from(other[scalar]) * reverse.group1(),
        );
    }
}
impl std::ops::Div<InversePrefixOrPostfix> for AntiMotorOnOrigin {
    type Output = AntiMotorOnOrigin;
    fn div(self, _rhs: InversePrefixOrPostfix) -> Self::Output {
        self.inverse()
    }
}
impl std::ops::DivAssign<InversePrefixOrPostfix> for AntiMotorOnOrigin {
    fn div_assign(&mut self, _rhs: InversePrefixOrPostfix) {
        *self = self.inverse()
    }
}
impl Inverse for AntiMotorOnOrigin {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        3        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        3        4        0
    //  no simd        3        7        0
    fn inverse(self) -> Self {
        use crate::elements::*;
        return AntiMotorOnOrigin::from_groups(
            // e23, e31, e12, scalar
            Simd32x4::from(f32::powi(self[e23], 2) + f32::powi(self[e31], 2) + f32::powi(self[e12], 2) + f32::powi(self[scalar], 2))
                * Simd32x4::from([self[e23] * -1.0, self[e31] * -1.0, self[e12] * -1.0, self[scalar]]),
        );
    }
}
impl std::ops::Div<InversePrefixOrPostfix> for AntiMysteryCircleRotor {
    type Output = AntiMysteryCircleRotor;
    fn div(self, _rhs: InversePrefixOrPostfix) -> Self::Output {
        self.inverse()
    }
}
impl std::ops::DivAssign<InversePrefixOrPostfix> for AntiMysteryCircleRotor {
    fn div_assign(&mut self, _rhs: InversePrefixOrPostfix) {
        *self = self.inverse()
    }
}
impl Inverse for AntiMysteryCircleRotor {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        1        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        4        3        0
    //  no simd        4        9        0
    fn inverse(self) -> Self {
        use crate::elements::*;
        let reverse = AntiMysteryCircleRotor::from_groups(/* e23, e31, e12, e45 */ self.group0() * Simd32x4::from(-1.0), /* scalar */ self[scalar]);
        let other = Scalar::from_groups(
            // scalar
            f32::powi(self[e23], 2) + f32::powi(self[e31], 2) + f32::powi(self[e12], 2) + f32::powi(self[scalar], 2) - f32::powi(self[e45], 2),
        );
        return AntiMysteryCircleRotor::from_groups(
            // e23, e31, e12, e45
            Simd32x4::from(other[scalar]) * reverse.group0(),
            // scalar
            reverse[scalar] * other[scalar],
        );
    }
}
impl std::ops::Div<InversePrefixOrPostfix> for AntiMysteryDipoleInversion {
    type Output = AntiMysteryDipoleInversion;
    fn div(self, _rhs: InversePrefixOrPostfix) -> Self::Output {
        self.inverse()
    }
}
impl std::ops::DivAssign<InversePrefixOrPostfix> for AntiMysteryDipoleInversion {
    fn div_assign(&mut self, _rhs: InversePrefixOrPostfix) {
        *self = self.inverse()
    }
}
impl Inverse for AntiMysteryDipoleInversion {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        6        0        0
    //    simd3        0        1        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        6        3        0
    //  no simd        6       11        0
    fn inverse(self) -> Self {
        use crate::elements::*;
        let reverse = AntiMysteryDipoleInversion::from_groups(/* e415, e425, e435, e321 */ self.group0() * Simd32x4::from(-1.0), /* e1, e2, e3 */ self.group1());
        let other = Scalar::from_groups(
            // scalar
            f32::powi(self[e321], 2) + f32::powi(self[e1], 2) + f32::powi(self[e2], 2) + f32::powi(self[e3], 2)
                - f32::powi(self[e415], 2)
                - f32::powi(self[e425], 2)
                - f32::powi(self[e435], 2),
        );
        return AntiMysteryDipoleInversion::from_groups(
            // e415, e425, e435, e321
            Simd32x4::from(other[scalar]) * reverse.group0(),
            // e1, e2, e3
            Simd32x3::from(other[scalar]) * reverse.group1(),
        );
    }
}
impl std::ops::Div<InversePrefixOrPostfix> for AntiPlane {
    type Output = AntiPlane;
    fn div(self, _rhs: InversePrefixOrPostfix) -> Self::Output {
        self.inverse()
    }
}
impl std::ops::DivAssign<InversePrefixOrPostfix> for AntiPlane {
    fn div_assign(&mut self, _rhs: InversePrefixOrPostfix) {
        *self = self.inverse()
    }
}
impl Inverse for AntiPlane {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        0        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        2        1        0
    //  no simd        2        4        0
    fn inverse(self) -> Self {
        use crate::elements::*;
        return AntiPlane::from_groups(
            // e1, e2, e3, e5
            Simd32x4::from(f32::powi(self[e1], 2) + f32::powi(self[e2], 2) + f32::powi(self[e3], 2)) * self.group0(),
        );
    }
}
impl std::ops::Div<InversePrefixOrPostfix> for AntiPlaneOnOrigin {
    type Output = AntiPlaneOnOrigin;
    fn div(self, _rhs: InversePrefixOrPostfix) -> Self::Output {
        self.inverse()
    }
}
impl std::ops::DivAssign<InversePrefixOrPostfix> for AntiPlaneOnOrigin {
    fn div_assign(&mut self, _rhs: InversePrefixOrPostfix) {
        *self = self.inverse()
    }
}
impl Inverse for AntiPlaneOnOrigin {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        0        0
    //    simd3        0        1        0
    // Totals...
    // yes simd        2        1        0
    //  no simd        2        3        0
    fn inverse(self) -> Self {
        use crate::elements::*;
        return AntiPlaneOnOrigin::from_groups(
            // e1, e2, e3
            Simd32x3::from(f32::powi(self[e1], 2) + f32::powi(self[e2], 2) + f32::powi(self[e3], 2)) * self.group0(),
        );
    }
}
impl std::ops::Div<InversePrefixOrPostfix> for AntiScalar {
    type Output = AntiScalar;
    fn div(self, _rhs: InversePrefixOrPostfix) -> Self::Output {
        self.inverse()
    }
}
impl std::ops::DivAssign<InversePrefixOrPostfix> for AntiScalar {
    fn div_assign(&mut self, _rhs: InversePrefixOrPostfix) {
        *self = self.inverse()
    }
}
impl Inverse for AntiScalar {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        1
    fn inverse(self) -> Self {
        use crate::elements::*;
        return AntiScalar::from_groups(/* e12345 */ 1.0 / self[e12345] * -1.0);
    }
}
impl std::ops::Div<InversePrefixOrPostfix> for AntiSphereOnOrigin {
    type Output = AntiSphereOnOrigin;
    fn div(self, _rhs: InversePrefixOrPostfix) -> Self::Output {
        self.inverse()
    }
}
impl std::ops::DivAssign<InversePrefixOrPostfix> for AntiSphereOnOrigin {
    fn div_assign(&mut self, _rhs: InversePrefixOrPostfix) {
        *self = self.inverse()
    }
}
impl Inverse for AntiSphereOnOrigin {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        0        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        2        1        0
    //  no simd        2        4        0
    fn inverse(self) -> Self {
        use crate::elements::*;
        return AntiSphereOnOrigin::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from(f32::powi(self[e1], 2) + f32::powi(self[e2], 2) + f32::powi(self[e3], 2)) * self.group0(),
        );
    }
}
impl std::ops::Div<InversePrefixOrPostfix> for AntiVersorEvenOnOrigin {
    type Output = AntiVersorEvenOnOrigin;
    fn div(self, _rhs: InversePrefixOrPostfix) -> Self::Output {
        self.inverse()
    }
}
impl std::ops::DivAssign<InversePrefixOrPostfix> for AntiVersorEvenOnOrigin {
    fn div_assign(&mut self, _rhs: InversePrefixOrPostfix) {
        *self = self.inverse()
    }
}
impl Inverse for AntiVersorEvenOnOrigin {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        0        0
    //    simd4        0        4        0
    // Totals...
    // yes simd        3        4        0
    //  no simd        3       16        0
    fn inverse(self) -> Self {
        use crate::elements::*;
        let reverse = AntiVersorEvenOnOrigin::from_groups(
            // e41, e42, e43, scalar
            self.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e23, e31, e12, e1234
            self.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
        );
        let other = Scalar::from_groups(
            // scalar
            f32::powi(self[scalar], 2) + f32::powi(self[e23], 2) + f32::powi(self[e31], 2) + f32::powi(self[e12], 2),
        );
        return AntiVersorEvenOnOrigin::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from(other[scalar]) * reverse.group0(),
            // e23, e31, e12, e1234
            Simd32x4::from(other[scalar]) * reverse.group1(),
        );
    }
}
impl std::ops::Div<InversePrefixOrPostfix> for Circle {
    type Output = Circle;
    fn div(self, _rhs: InversePrefixOrPostfix) -> Self::Output {
        self.inverse()
    }
}
impl std::ops::DivAssign<InversePrefixOrPostfix> for Circle {
    fn div_assign(&mut self, _rhs: InversePrefixOrPostfix) {
        *self = self.inverse()
    }
}
impl Inverse for Circle {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        6        6        0
    //    simd3        0        4        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        6       12        0
    //  no simd        6       26        0
    fn inverse(self) -> Self {
        use crate::elements::*;
        let reverse = Circle::from_groups(
            // e423, e431, e412
            self.group0() * Simd32x3::from(-1.0),
            // e415, e425, e435, e321
            self.group1() * Simd32x4::from(-1.0),
            // e235, e315, e125
            self.group2() * Simd32x3::from(-1.0),
        );
        let other = Scalar::from_groups(
            // scalar
            f32::powi(self[e321], 2)
                - f32::powi(self[e415], 2)
                - f32::powi(self[e425], 2)
                - f32::powi(self[e435], 2)
                - 2.0 * (self[e423] * self[e235])
                - 2.0 * (self[e431] * self[e315])
                - 2.0 * (self[e412] * self[e125]),
        );
        return Circle::from_groups(
            // e423, e431, e412
            Simd32x3::from(other[scalar]) * reverse.group0(),
            // e415, e425, e435, e321
            Simd32x4::from(other[scalar]) * reverse.group1(),
            // e235, e315, e125
            Simd32x3::from(other[scalar]) * reverse.group2(),
        );
    }
}
impl std::ops::Div<InversePrefixOrPostfix> for CircleAligningOrigin {
    type Output = CircleAligningOrigin;
    fn div(self, _rhs: InversePrefixOrPostfix) -> Self::Output {
        self.inverse()
    }
}
impl std::ops::DivAssign<InversePrefixOrPostfix> for CircleAligningOrigin {
    fn div_assign(&mut self, _rhs: InversePrefixOrPostfix) {
        *self = self.inverse()
    }
}
impl Inverse for CircleAligningOrigin {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        5        6        0
    //    simd3        0        6        0
    // Totals...
    // yes simd        5       12        0
    //  no simd        5       24        0
    fn inverse(self) -> Self {
        use crate::elements::*;
        let reverse = CircleAligningOrigin::from_groups(
            // e423, e431, e412
            self.group0() * Simd32x3::from(-1.0),
            // e415, e425, e435
            self.group1() * Simd32x3::from(-1.0),
            // e235, e315, e125
            self.group2() * Simd32x3::from(-1.0),
        );
        let other = Scalar::from_groups(
            // scalar
            -f32::powi(self[e415], 2)
                - f32::powi(self[e425], 2)
                - f32::powi(self[e435], 2)
                - 2.0 * (self[e423] * self[e235])
                - 2.0 * (self[e431] * self[e315])
                - 2.0 * (self[e412] * self[e125]),
        );
        return CircleAligningOrigin::from_groups(
            // e423, e431, e412
            Simd32x3::from(other[scalar]) * reverse.group0(),
            // e415, e425, e435
            Simd32x3::from(other[scalar]) * reverse.group1(),
            // e235, e315, e125
            Simd32x3::from(other[scalar]) * reverse.group2(),
        );
    }
}
impl std::ops::Div<InversePrefixOrPostfix> for CircleAtInfinity {
    type Output = CircleAtInfinity;
    fn div(self, _rhs: InversePrefixOrPostfix) -> Self::Output {
        self.inverse()
    }
}
impl std::ops::DivAssign<InversePrefixOrPostfix> for CircleAtInfinity {
    fn div_assign(&mut self, _rhs: InversePrefixOrPostfix) {
        *self = self.inverse()
    }
}
impl Inverse for CircleAtInfinity {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        0        0
    //    simd3        0        2        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        3        4        0
    //  no simd        3       14        0
    fn inverse(self) -> Self {
        use crate::elements::*;
        let reverse = CircleAtInfinity::from_groups(
            // e415, e425, e435, e321
            self.group0() * Simd32x4::from(-1.0),
            // e235, e315, e125
            self.group1() * Simd32x3::from(-1.0),
        );
        let other = Scalar::from_groups(
            // scalar
            f32::powi(self[e321], 2) - f32::powi(self[e415], 2) - f32::powi(self[e425], 2) - f32::powi(self[e435], 2),
        );
        return CircleAtInfinity::from_groups(
            // e415, e425, e435, e321
            Simd32x4::from(other[scalar]) * reverse.group0(),
            // e235, e315, e125
            Simd32x3::from(other[scalar]) * reverse.group1(),
        );
    }
}
impl std::ops::Div<InversePrefixOrPostfix> for CircleAtOrigin {
    type Output = CircleAtOrigin;
    fn div(self, _rhs: InversePrefixOrPostfix) -> Self::Output {
        self.inverse()
    }
}
impl std::ops::DivAssign<InversePrefixOrPostfix> for CircleAtOrigin {
    fn div_assign(&mut self, _rhs: InversePrefixOrPostfix) {
        *self = self.inverse()
    }
}
impl Inverse for CircleAtOrigin {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        6        0
    //    simd3        0        4        0
    // Totals...
    // yes simd        2       10        0
    //  no simd        2       18        0
    fn inverse(self) -> Self {
        use crate::elements::*;
        let reverse = CircleAtOrigin::from_groups(
            // e423, e431, e412
            self.group0() * Simd32x3::from(-1.0),
            // e235, e315, e125
            self.group1() * Simd32x3::from(-1.0),
        );
        let other = Scalar::from_groups(/* scalar */ -2.0 * (self[e423] * self[e235]) - 2.0 * (self[e431] * self[e315]) - 2.0 * (self[e412] * self[e125]));
        return CircleAtOrigin::from_groups(
            // e423, e431, e412
            Simd32x3::from(other[scalar]) * reverse.group0(),
            // e235, e315, e125
            Simd32x3::from(other[scalar]) * reverse.group1(),
        );
    }
}
impl std::ops::Div<InversePrefixOrPostfix> for CircleOnOrigin {
    type Output = CircleOnOrigin;
    fn div(self, _rhs: InversePrefixOrPostfix) -> Self::Output {
        self.inverse()
    }
}
impl std::ops::DivAssign<InversePrefixOrPostfix> for CircleOnOrigin {
    fn div_assign(&mut self, _rhs: InversePrefixOrPostfix) {
        *self = self.inverse()
    }
}
impl Inverse for CircleOnOrigin {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        0        0
    //    simd3        0        4        0
    // Totals...
    // yes simd        2        4        0
    //  no simd        2       12        0
    fn inverse(self) -> Self {
        use crate::elements::*;
        let reverse = CircleOnOrigin::from_groups(
            // e423, e431, e412
            self.group0() * Simd32x3::from(-1.0),
            // e415, e425, e435
            self.group1() * Simd32x3::from(-1.0),
        );
        let other = Scalar::from_groups(/* scalar */ -f32::powi(self[e415], 2) - f32::powi(self[e425], 2) - f32::powi(self[e435], 2));
        return CircleOnOrigin::from_groups(
            // e423, e431, e412
            Simd32x3::from(other[scalar]) * reverse.group0(),
            // e415, e425, e435
            Simd32x3::from(other[scalar]) * reverse.group1(),
        );
    }
}
impl std::ops::Div<InversePrefixOrPostfix> for CircleOrthogonalOrigin {
    type Output = CircleOrthogonalOrigin;
    fn div(self, _rhs: InversePrefixOrPostfix) -> Self::Output {
        self.inverse()
    }
}
impl std::ops::DivAssign<InversePrefixOrPostfix> for CircleOrthogonalOrigin {
    fn div_assign(&mut self, _rhs: InversePrefixOrPostfix) {
        *self = self.inverse()
    }
}
impl Inverse for CircleOrthogonalOrigin {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        6        0
    //    simd3        0        2        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        3       10        0
    //  no simd        3       20        0
    fn inverse(self) -> Self {
        use crate::elements::*;
        let reverse = CircleOrthogonalOrigin::from_groups(
            // e423, e431, e412, e321
            self.group0() * Simd32x4::from(-1.0),
            // e235, e315, e125
            self.group1() * Simd32x3::from(-1.0),
        );
        let other = Scalar::from_groups(
            // scalar
            f32::powi(self[e321], 2) - 2.0 * (self[e423] * self[e235]) - 2.0 * (self[e431] * self[e315]) - 2.0 * (self[e412] * self[e125]),
        );
        return CircleOrthogonalOrigin::from_groups(
            // e423, e431, e412, e321
            Simd32x4::from(other[scalar]) * reverse.group0(),
            // e235, e315, e125
            Simd32x3::from(other[scalar]) * reverse.group1(),
        );
    }
}
impl std::ops::Div<InversePrefixOrPostfix> for CircleRotor {
    type Output = CircleRotor;
    fn div(self, _rhs: InversePrefixOrPostfix) -> Self::Output {
        self.inverse()
    }
}
impl std::ops::DivAssign<InversePrefixOrPostfix> for CircleRotor {
    fn div_assign(&mut self, _rhs: InversePrefixOrPostfix) {
        *self = self.inverse()
    }
}
impl Inverse for CircleRotor {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        7        6        0
    //    simd3        0        2        0
    //    simd4        0        4        0
    // Totals...
    // yes simd        7       12        0
    //  no simd        7       28        0
    fn inverse(self) -> Self {
        use crate::elements::*;
        let reverse = CircleRotor::from_groups(
            // e423, e431, e412
            self.group0() * Simd32x3::from(-1.0),
            // e415, e425, e435, e321
            self.group1() * Simd32x4::from(-1.0),
            // e235, e315, e125, e12345
            self.group2() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
        );
        let other = Scalar::from_groups(
            // scalar
            f32::powi(self[e321], 2)
                - f32::powi(self[e415], 2)
                - f32::powi(self[e425], 2)
                - f32::powi(self[e435], 2)
                - f32::powi(self[e12345], 2)
                - 2.0 * (self[e423] * self[e235])
                - 2.0 * (self[e431] * self[e315])
                - 2.0 * (self[e412] * self[e125]),
        );
        return CircleRotor::from_groups(
            // e423, e431, e412
            Simd32x3::from(other[scalar]) * reverse.group0(),
            // e415, e425, e435, e321
            Simd32x4::from(other[scalar]) * reverse.group1(),
            // e235, e315, e125, e12345
            Simd32x4::from(other[scalar]) * reverse.group2(),
        );
    }
}
impl std::ops::Div<InversePrefixOrPostfix> for CircleRotorAligningOrigin {
    type Output = CircleRotorAligningOrigin;
    fn div(self, _rhs: InversePrefixOrPostfix) -> Self::Output {
        self.inverse()
    }
}
impl std::ops::DivAssign<InversePrefixOrPostfix> for CircleRotorAligningOrigin {
    fn div_assign(&mut self, _rhs: InversePrefixOrPostfix) {
        *self = self.inverse()
    }
}
impl Inverse for CircleRotorAligningOrigin {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        6        6        0
    //    simd3        0        4        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        6       12        0
    //  no simd        6       26        0
    fn inverse(self) -> Self {
        use crate::elements::*;
        let reverse = CircleRotorAligningOrigin::from_groups(
            // e423, e431, e412
            self.group0() * Simd32x3::from(-1.0),
            // e415, e425, e435
            self.group1() * Simd32x3::from(-1.0),
            // e235, e315, e125, e12345
            self.group2() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
        );
        let other = Scalar::from_groups(
            // scalar
            -f32::powi(self[e415], 2)
                - f32::powi(self[e425], 2)
                - f32::powi(self[e435], 2)
                - f32::powi(self[e12345], 2)
                - 2.0 * (self[e423] * self[e235])
                - 2.0 * (self[e431] * self[e315])
                - 2.0 * (self[e412] * self[e125]),
        );
        return CircleRotorAligningOrigin::from_groups(
            // e423, e431, e412
            Simd32x3::from(other[scalar]) * reverse.group0(),
            // e415, e425, e435
            Simd32x3::from(other[scalar]) * reverse.group1(),
            // e235, e315, e125, e12345
            Simd32x4::from(other[scalar]) * reverse.group2(),
        );
    }
}
impl std::ops::Div<InversePrefixOrPostfix> for CircleRotorAligningOriginAtInfinity {
    type Output = CircleRotorAligningOriginAtInfinity;
    fn div(self, _rhs: InversePrefixOrPostfix) -> Self::Output {
        self.inverse()
    }
}
impl std::ops::DivAssign<InversePrefixOrPostfix> for CircleRotorAligningOriginAtInfinity {
    fn div_assign(&mut self, _rhs: InversePrefixOrPostfix) {
        *self = self.inverse()
    }
}
impl Inverse for CircleRotorAligningOriginAtInfinity {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        0        0
    //    simd3        0        2        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        3        4        0
    //  no simd        3       14        0
    fn inverse(self) -> Self {
        use crate::elements::*;
        let reverse = CircleRotorAligningOriginAtInfinity::from_groups(
            // e415, e425, e435
            self.group0() * Simd32x3::from(-1.0),
            // e235, e315, e125, e12345
            self.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
        );
        let other = Scalar::from_groups(
            // scalar
            -f32::powi(self[e415], 2) - f32::powi(self[e425], 2) - f32::powi(self[e435], 2) - f32::powi(self[e12345], 2),
        );
        return CircleRotorAligningOriginAtInfinity::from_groups(
            // e415, e425, e435
            Simd32x3::from(other[scalar]) * reverse.group0(),
            // e235, e315, e125, e12345
            Simd32x4::from(other[scalar]) * reverse.group1(),
        );
    }
}
impl std::ops::Div<InversePrefixOrPostfix> for CircleRotorAtInfinity {
    type Output = CircleRotorAtInfinity;
    fn div(self, _rhs: InversePrefixOrPostfix) -> Self::Output {
        self.inverse()
    }
}
impl std::ops::DivAssign<InversePrefixOrPostfix> for CircleRotorAtInfinity {
    fn div_assign(&mut self, _rhs: InversePrefixOrPostfix) {
        *self = self.inverse()
    }
}
impl Inverse for CircleRotorAtInfinity {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        0        0
    //    simd4        0        4        0
    // Totals...
    // yes simd        4        4        0
    //  no simd        4       16        0
    fn inverse(self) -> Self {
        use crate::elements::*;
        let reverse = CircleRotorAtInfinity::from_groups(
            // e415, e425, e435, e321
            self.group0() * Simd32x4::from(-1.0),
            // e235, e315, e125, e12345
            self.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
        );
        let other = Scalar::from_groups(
            // scalar
            f32::powi(self[e321], 2) - f32::powi(self[e415], 2) - f32::powi(self[e425], 2) - f32::powi(self[e435], 2) - f32::powi(self[e12345], 2),
        );
        return CircleRotorAtInfinity::from_groups(
            // e415, e425, e435, e321
            Simd32x4::from(other[scalar]) * reverse.group0(),
            // e235, e315, e125, e12345
            Simd32x4::from(other[scalar]) * reverse.group1(),
        );
    }
}
impl std::ops::Div<InversePrefixOrPostfix> for CircleRotorOnOrigin {
    type Output = CircleRotorOnOrigin;
    fn div(self, _rhs: InversePrefixOrPostfix) -> Self::Output {
        self.inverse()
    }
}
impl std::ops::DivAssign<InversePrefixOrPostfix> for CircleRotorOnOrigin {
    fn div_assign(&mut self, _rhs: InversePrefixOrPostfix) {
        *self = self.inverse()
    }
}
impl Inverse for CircleRotorOnOrigin {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        0        0
    //    simd3        0        2        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        3        4        0
    //  no simd        3       14        0
    fn inverse(self) -> Self {
        use crate::elements::*;
        let reverse = CircleRotorOnOrigin::from_groups(
            // e423, e431, e412, e12345
            self.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e415, e425, e435
            self.group1() * Simd32x3::from(-1.0),
        );
        let other = Scalar::from_groups(
            // scalar
            -f32::powi(self[e12345], 2) - f32::powi(self[e415], 2) - f32::powi(self[e425], 2) - f32::powi(self[e435], 2),
        );
        return CircleRotorOnOrigin::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from(other[scalar]) * reverse.group0(),
            // e415, e425, e435
            Simd32x3::from(other[scalar]) * reverse.group1(),
        );
    }
}
impl std::ops::Div<InversePrefixOrPostfix> for Dipole {
    type Output = Dipole;
    fn div(self, _rhs: InversePrefixOrPostfix) -> Self::Output {
        self.inverse()
    }
}
impl std::ops::DivAssign<InversePrefixOrPostfix> for Dipole {
    fn div_assign(&mut self, _rhs: InversePrefixOrPostfix) {
        *self = self.inverse()
    }
}
impl Inverse for Dipole {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        6        6        0
    //    simd3        0        4        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        6       12        0
    //  no simd        6       26        0
    fn inverse(self) -> Self {
        use crate::elements::*;
        let reverse = Dipole::from_groups(
            // e41, e42, e43
            self.group0() * Simd32x3::from(-1.0),
            // e23, e31, e12, e45
            self.group1() * Simd32x4::from(-1.0),
            // e15, e25, e35
            self.group2() * Simd32x3::from(-1.0),
        );
        let other = Scalar::from_groups(
            // scalar
            2.0 * (self[e41] * self[e15])
                + 2.0 * (self[e42] * self[e25])
                + 2.0 * (self[e43] * self[e35])
                + f32::powi(self[e23], 2)
                + f32::powi(self[e31], 2)
                + f32::powi(self[e12], 2)
                - f32::powi(self[e45], 2),
        );
        return Dipole::from_groups(
            // e41, e42, e43
            Simd32x3::from(other[scalar]) * reverse.group0(),
            // e23, e31, e12, e45
            Simd32x4::from(other[scalar]) * reverse.group1(),
            // e15, e25, e35
            Simd32x3::from(other[scalar]) * reverse.group2(),
        );
    }
}
impl std::ops::Div<InversePrefixOrPostfix> for DipoleAligningOrigin {
    type Output = DipoleAligningOrigin;
    fn div(self, _rhs: InversePrefixOrPostfix) -> Self::Output {
        self.inverse()
    }
}
impl std::ops::DivAssign<InversePrefixOrPostfix> for DipoleAligningOrigin {
    fn div_assign(&mut self, _rhs: InversePrefixOrPostfix) {
        *self = self.inverse()
    }
}
impl Inverse for DipoleAligningOrigin {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        6        0
    //    simd3        0        2        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        3       10        0
    //  no simd        3       20        0
    fn inverse(self) -> Self {
        use crate::elements::*;
        let reverse = DipoleAligningOrigin::from_groups(
            // e41, e42, e43, e45
            self.group0() * Simd32x4::from(-1.0),
            // e15, e25, e35
            self.group1() * Simd32x3::from(-1.0),
        );
        let other = Scalar::from_groups(
            // scalar
            2.0 * (self[e41] * self[e15]) + 2.0 * (self[e42] * self[e25]) + 2.0 * (self[e43] * self[e35]) - f32::powi(self[e45], 2),
        );
        return DipoleAligningOrigin::from_groups(
            // e41, e42, e43, e45
            Simd32x4::from(other[scalar]) * reverse.group0(),
            // e15, e25, e35
            Simd32x3::from(other[scalar]) * reverse.group1(),
        );
    }
}
impl std::ops::Div<InversePrefixOrPostfix> for DipoleAtInfinity {
    type Output = DipoleAtInfinity;
    fn div(self, _rhs: InversePrefixOrPostfix) -> Self::Output {
        self.inverse()
    }
}
impl std::ops::DivAssign<InversePrefixOrPostfix> for DipoleAtInfinity {
    fn div_assign(&mut self, _rhs: InversePrefixOrPostfix) {
        *self = self.inverse()
    }
}
impl Inverse for DipoleAtInfinity {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        0        0
    //    simd3        0        2        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        3        4        0
    //  no simd        3       14        0
    fn inverse(self) -> Self {
        use crate::elements::*;
        let reverse = DipoleAtInfinity::from_groups(
            // e23, e31, e12, e45
            self.group0() * Simd32x4::from(-1.0),
            // e15, e25, e35
            self.group1() * Simd32x3::from(-1.0),
        );
        let other = Scalar::from_groups(/* scalar */ f32::powi(self[e23], 2) + f32::powi(self[e31], 2) + f32::powi(self[e12], 2) - f32::powi(self[e45], 2));
        return DipoleAtInfinity::from_groups(
            // e23, e31, e12, e45
            Simd32x4::from(other[scalar]) * reverse.group0(),
            // e15, e25, e35
            Simd32x3::from(other[scalar]) * reverse.group1(),
        );
    }
}
impl std::ops::Div<InversePrefixOrPostfix> for DipoleAtOrigin {
    type Output = DipoleAtOrigin;
    fn div(self, _rhs: InversePrefixOrPostfix) -> Self::Output {
        self.inverse()
    }
}
impl std::ops::DivAssign<InversePrefixOrPostfix> for DipoleAtOrigin {
    fn div_assign(&mut self, _rhs: InversePrefixOrPostfix) {
        *self = self.inverse()
    }
}
impl Inverse for DipoleAtOrigin {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        6        0
    //    simd3        0        4        0
    // Totals...
    // yes simd        2       10        0
    //  no simd        2       18        0
    fn inverse(self) -> Self {
        use crate::elements::*;
        let reverse = DipoleAtOrigin::from_groups(
            // e41, e42, e43
            self.group0() * Simd32x3::from(-1.0),
            // e15, e25, e35
            self.group1() * Simd32x3::from(-1.0),
        );
        let other = Scalar::from_groups(/* scalar */ 2.0 * (self[e41] * self[e15]) + 2.0 * (self[e42] * self[e25]) + 2.0 * (self[e43] * self[e35]));
        return DipoleAtOrigin::from_groups(
            // e41, e42, e43
            Simd32x3::from(other[scalar]) * reverse.group0(),
            // e15, e25, e35
            Simd32x3::from(other[scalar]) * reverse.group1(),
        );
    }
}
impl std::ops::Div<InversePrefixOrPostfix> for DipoleInversion {
    type Output = DipoleInversion;
    fn div(self, _rhs: InversePrefixOrPostfix) -> Self::Output {
        self.inverse()
    }
}
impl std::ops::DivAssign<InversePrefixOrPostfix> for DipoleInversion {
    fn div_assign(&mut self, _rhs: InversePrefixOrPostfix) {
        *self = self.inverse()
    }
}
impl Inverse for DipoleInversion {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       10        8        0
    //    simd3        0        2        0
    //    simd4        0        5        0
    // Totals...
    // yes simd       10       15        0
    //  no simd       10       34        0
    fn inverse(self) -> Self {
        use crate::elements::*;
        let reverse = DipoleInversion::from_groups(
            // e41, e42, e43
            self.group0() * Simd32x3::from(-1.0),
            // e23, e31, e12, e45
            self.group1() * Simd32x4::from(-1.0),
            // e15, e25, e35, e1234
            self.group2() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e4235, e4315, e4125, e3215
            self.group3(),
        );
        let other = Scalar::from_groups(
            // scalar
            2.0 * (self[e41] * self[e15])
                + 2.0 * (self[e42] * self[e25])
                + 2.0 * (self[e43] * self[e35])
                + 2.0 * (self[e1234] * self[e3215])
                + f32::powi(self[e23], 2)
                + f32::powi(self[e31], 2)
                + f32::powi(self[e12], 2)
                - f32::powi(self[e45], 2)
                - f32::powi(self[e4235], 2)
                - f32::powi(self[e4315], 2)
                - f32::powi(self[e4125], 2),
        );
        return DipoleInversion::from_groups(
            // e41, e42, e43
            Simd32x3::from(other[scalar]) * reverse.group0(),
            // e23, e31, e12, e45
            Simd32x4::from(other[scalar]) * reverse.group1(),
            // e15, e25, e35, e1234
            Simd32x4::from(other[scalar]) * reverse.group2(),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(other[scalar]) * reverse.group3(),
        );
    }
}
impl std::ops::Div<InversePrefixOrPostfix> for DipoleInversionAligningOrigin {
    type Output = DipoleInversionAligningOrigin;
    fn div(self, _rhs: InversePrefixOrPostfix) -> Self::Output {
        self.inverse()
    }
}
impl std::ops::DivAssign<InversePrefixOrPostfix> for DipoleInversionAligningOrigin {
    fn div_assign(&mut self, _rhs: InversePrefixOrPostfix) {
        *self = self.inverse()
    }
}
impl Inverse for DipoleInversionAligningOrigin {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        7        8        0
    //    simd4        0        5        0
    // Totals...
    // yes simd        7       13        0
    //  no simd        7       28        0
    fn inverse(self) -> Self {
        use crate::elements::*;
        let reverse = DipoleInversionAligningOrigin::from_groups(
            // e41, e42, e43, e45
            self.group0() * Simd32x4::from(-1.0),
            // e15, e25, e35, e1234
            self.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e4235, e4315, e4125, e3215
            self.group2(),
        );
        let other = Scalar::from_groups(
            // scalar
            2.0 * (self[e41] * self[e15]) + 2.0 * (self[e42] * self[e25]) + 2.0 * (self[e43] * self[e35]) + 2.0 * (self[e1234] * self[e3215])
                - f32::powi(self[e45], 2)
                - f32::powi(self[e4235], 2)
                - f32::powi(self[e4315], 2)
                - f32::powi(self[e4125], 2),
        );
        return DipoleInversionAligningOrigin::from_groups(
            // e41, e42, e43, e45
            Simd32x4::from(other[scalar]) * reverse.group0(),
            // e15, e25, e35, e1234
            Simd32x4::from(other[scalar]) * reverse.group1(),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(other[scalar]) * reverse.group2(),
        );
    }
}
impl std::ops::Div<InversePrefixOrPostfix> for DipoleInversionAtInfinity {
    type Output = DipoleInversionAtInfinity;
    fn div(self, _rhs: InversePrefixOrPostfix) -> Self::Output {
        self.inverse()
    }
}
impl std::ops::DivAssign<InversePrefixOrPostfix> for DipoleInversionAtInfinity {
    fn div_assign(&mut self, _rhs: InversePrefixOrPostfix) {
        *self = self.inverse()
    }
}
impl Inverse for DipoleInversionAtInfinity {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        6        0        0
    //    simd3        0        2        0
    //    simd4        0        3        0
    // Totals...
    // yes simd        6        5        0
    //  no simd        6       18        0
    fn inverse(self) -> Self {
        use crate::elements::*;
        let reverse = DipoleInversionAtInfinity::from_groups(
            // e23, e31, e12, e45
            self.group0() * Simd32x4::from(-1.0),
            // e15, e25, e35
            self.group1() * Simd32x3::from(-1.0),
            // e4235, e4315, e4125, e3215
            self.group2(),
        );
        let other = Scalar::from_groups(
            // scalar
            f32::powi(self[e23], 2) + f32::powi(self[e31], 2) + f32::powi(self[e12], 2)
                - f32::powi(self[e45], 2)
                - f32::powi(self[e4235], 2)
                - f32::powi(self[e4315], 2)
                - f32::powi(self[e4125], 2),
        );
        return DipoleInversionAtInfinity::from_groups(
            // e23, e31, e12, e45
            Simd32x4::from(other[scalar]) * reverse.group0(),
            // e15, e25, e35
            Simd32x3::from(other[scalar]) * reverse.group1(),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(other[scalar]) * reverse.group2(),
        );
    }
}
impl std::ops::Div<InversePrefixOrPostfix> for DipoleInversionAtOrigin {
    type Output = DipoleInversionAtOrigin;
    fn div(self, _rhs: InversePrefixOrPostfix) -> Self::Output {
        self.inverse()
    }
}
impl std::ops::DivAssign<InversePrefixOrPostfix> for DipoleInversionAtOrigin {
    fn div_assign(&mut self, _rhs: InversePrefixOrPostfix) {
        *self = self.inverse()
    }
}
impl Inverse for DipoleInversionAtOrigin {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        8        0
    //    simd4        0        4        0
    // Totals...
    // yes simd        3       12        0
    //  no simd        3       24        0
    fn inverse(self) -> Self {
        use crate::elements::*;
        let reverse = DipoleInversionAtOrigin::from_groups(
            // e41, e42, e43, e3215
            self.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e15, e25, e35, e1234
            self.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
        );
        let other = Scalar::from_groups(
            // scalar
            2.0 * (self[e41] * self[e15]) + 2.0 * (self[e42] * self[e25]) + 2.0 * (self[e43] * self[e35]) + 2.0 * (self[e3215] * self[e1234]),
        );
        return DipoleInversionAtOrigin::from_groups(
            // e41, e42, e43, e3215
            Simd32x4::from(other[scalar]) * reverse.group0(),
            // e15, e25, e35, e1234
            Simd32x4::from(other[scalar]) * reverse.group1(),
        );
    }
}
impl std::ops::Div<InversePrefixOrPostfix> for DipoleInversionOnOrigin {
    type Output = DipoleInversionOnOrigin;
    fn div(self, _rhs: InversePrefixOrPostfix) -> Self::Output {
        self.inverse()
    }
}
impl std::ops::DivAssign<InversePrefixOrPostfix> for DipoleInversionOnOrigin {
    fn div_assign(&mut self, _rhs: InversePrefixOrPostfix) {
        *self = self.inverse()
    }
}
impl Inverse for DipoleInversionOnOrigin {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        0        0
    //    simd4        0        3        0
    // Totals...
    // yes simd        3        3        0
    //  no simd        3       12        0
    fn inverse(self) -> Self {
        use crate::elements::*;
        let reverse = DipoleInversionOnOrigin::from_groups(/* e41, e42, e43, e45 */ self.group0() * Simd32x4::from(-1.0), /* e1234, e4235, e4315, e4125 */ self.group1());
        let other = Scalar::from_groups(
            // scalar
            -f32::powi(self[e45], 2) - f32::powi(self[e4235], 2) - f32::powi(self[e4315], 2) - f32::powi(self[e4125], 2),
        );
        return DipoleInversionOnOrigin::from_groups(
            // e41, e42, e43, e45
            Simd32x4::from(other[scalar]) * reverse.group0(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(other[scalar]) * reverse.group1(),
        );
    }
}
impl std::ops::Div<InversePrefixOrPostfix> for DipoleInversionOrthogonalOrigin {
    type Output = DipoleInversionOrthogonalOrigin;
    fn div(self, _rhs: InversePrefixOrPostfix) -> Self::Output {
        self.inverse()
    }
}
impl std::ops::DivAssign<InversePrefixOrPostfix> for DipoleInversionOrthogonalOrigin {
    fn div_assign(&mut self, _rhs: InversePrefixOrPostfix) {
        *self = self.inverse()
    }
}
impl Inverse for DipoleInversionOrthogonalOrigin {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        6        8        0
    //    simd3        0        2        0
    //    simd4        0        4        0
    // Totals...
    // yes simd        6       14        0
    //  no simd        6       30        0
    fn inverse(self) -> Self {
        use crate::elements::*;
        let reverse = DipoleInversionOrthogonalOrigin::from_groups(
            // e41, e42, e43, e3215
            self.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e23, e31, e12
            self.group1() * Simd32x3::from(-1.0),
            // e15, e25, e35, e1234
            self.group2() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
        );
        let other = Scalar::from_groups(
            // scalar
            2.0 * (self[e41] * self[e15])
                + 2.0 * (self[e42] * self[e25])
                + 2.0 * (self[e43] * self[e35])
                + 2.0 * (self[e3215] * self[e1234])
                + f32::powi(self[e23], 2)
                + f32::powi(self[e31], 2)
                + f32::powi(self[e12], 2),
        );
        return DipoleInversionOrthogonalOrigin::from_groups(
            // e41, e42, e43, e3215
            Simd32x4::from(other[scalar]) * reverse.group0(),
            // e23, e31, e12
            Simd32x3::from(other[scalar]) * reverse.group1(),
            // e15, e25, e35, e1234
            Simd32x4::from(other[scalar]) * reverse.group2(),
        );
    }
}
impl std::ops::Div<InversePrefixOrPostfix> for DipoleOnOrigin {
    type Output = DipoleOnOrigin;
    fn div(self, _rhs: InversePrefixOrPostfix) -> Self::Output {
        self.inverse()
    }
}
impl std::ops::DivAssign<InversePrefixOrPostfix> for DipoleOnOrigin {
    fn div_assign(&mut self, _rhs: InversePrefixOrPostfix) {
        *self = self.inverse()
    }
}
impl Inverse for DipoleOnOrigin {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        5        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        6        0
    //  no simd        0        9        0
    fn inverse(self) -> Self {
        use crate::elements::*;
        return DipoleOnOrigin::from_groups(
            // e41, e42, e43, e45
            Simd32x4::from(f32::powi(self[e45], -2) * -1.0) * Simd32x4::from([self[e41] * -1.0, self[e42] * -1.0, self[e43] * -1.0, self[e45] * -1.0]),
        );
    }
}
impl std::ops::Div<InversePrefixOrPostfix> for DipoleOrthogonalOrigin {
    type Output = DipoleOrthogonalOrigin;
    fn div(self, _rhs: InversePrefixOrPostfix) -> Self::Output {
        self.inverse()
    }
}
impl std::ops::DivAssign<InversePrefixOrPostfix> for DipoleOrthogonalOrigin {
    fn div_assign(&mut self, _rhs: InversePrefixOrPostfix) {
        *self = self.inverse()
    }
}
impl Inverse for DipoleOrthogonalOrigin {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        5        6        0
    //    simd3        0        6        0
    // Totals...
    // yes simd        5       12        0
    //  no simd        5       24        0
    fn inverse(self) -> Self {
        use crate::elements::*;
        let reverse = DipoleOrthogonalOrigin::from_groups(
            // e41, e42, e43
            self.group0() * Simd32x3::from(-1.0),
            // e23, e31, e12
            self.group1() * Simd32x3::from(-1.0),
            // e15, e25, e35
            self.group2() * Simd32x3::from(-1.0),
        );
        let other = Scalar::from_groups(
            // scalar
            2.0 * (self[e41] * self[e15])
                + 2.0 * (self[e42] * self[e25])
                + 2.0 * (self[e43] * self[e35])
                + f32::powi(self[e23], 2)
                + f32::powi(self[e31], 2)
                + f32::powi(self[e12], 2),
        );
        return DipoleOrthogonalOrigin::from_groups(
            // e41, e42, e43
            Simd32x3::from(other[scalar]) * reverse.group0(),
            // e23, e31, e12
            Simd32x3::from(other[scalar]) * reverse.group1(),
            // e15, e25, e35
            Simd32x3::from(other[scalar]) * reverse.group2(),
        );
    }
}
impl std::ops::Div<InversePrefixOrPostfix> for DualNum {
    type Output = DualNum;
    fn div(self, _rhs: InversePrefixOrPostfix) -> Self::Output {
        self.inverse()
    }
}
impl std::ops::DivAssign<InversePrefixOrPostfix> for DualNum {
    fn div_assign(&mut self, _rhs: InversePrefixOrPostfix) {
        *self = self.inverse()
    }
}
impl Inverse for DualNum {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd2        0        1        0
    // Totals...
    // yes simd        0        2        0
    //  no simd        0        3        0
    fn inverse(self) -> Self {
        use crate::elements::*;
        return DualNum::from_groups(/* e4, e12345 */ Simd32x2::from(f32::powi(self[e12345], -2) * -1.0) * self.group0());
    }
}
impl std::ops::Div<InversePrefixOrPostfix> for FlatOrigin {
    type Output = FlatOrigin;
    fn div(self, _rhs: InversePrefixOrPostfix) -> Self::Output {
        self.inverse()
    }
}
impl std::ops::DivAssign<InversePrefixOrPostfix> for FlatOrigin {
    fn div_assign(&mut self, _rhs: InversePrefixOrPostfix) {
        *self = self.inverse()
    }
}
impl Inverse for FlatOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        0        1
    fn inverse(self) -> Self {
        use crate::elements::*;
        return FlatOrigin::from_groups(/* e45 */ 1.0 / self[e45]);
    }
}
impl std::ops::Div<InversePrefixOrPostfix> for FlatPoint {
    type Output = FlatPoint;
    fn div(self, _rhs: InversePrefixOrPostfix) -> Self::Output {
        self.inverse()
    }
}
impl std::ops::DivAssign<InversePrefixOrPostfix> for FlatPoint {
    fn div_assign(&mut self, _rhs: InversePrefixOrPostfix) {
        *self = self.inverse()
    }
}
impl Inverse for FlatPoint {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        5        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        6        0
    //  no simd        0        9        0
    fn inverse(self) -> Self {
        use crate::elements::*;
        return FlatPoint::from_groups(
            // e15, e25, e35, e45
            Simd32x4::from(f32::powi(self[e45], -2) * -1.0) * Simd32x4::from([self[e15] * -1.0, self[e25] * -1.0, self[e35] * -1.0, self[e45] * -1.0]),
        );
    }
}
impl std::ops::Div<InversePrefixOrPostfix> for Flector {
    type Output = Flector;
    fn div(self, _rhs: InversePrefixOrPostfix) -> Self::Output {
        self.inverse()
    }
}
impl std::ops::DivAssign<InversePrefixOrPostfix> for Flector {
    fn div_assign(&mut self, _rhs: InversePrefixOrPostfix) {
        *self = self.inverse()
    }
}
impl Inverse for Flector {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        0        0
    //    simd4        0        3        0
    // Totals...
    // yes simd        3        3        0
    //  no simd        3       12        0
    fn inverse(self) -> Self {
        use crate::elements::*;
        let reverse = Flector::from_groups(/* e15, e25, e35, e45 */ self.group0() * Simd32x4::from(-1.0), /* e4235, e4315, e4125, e3215 */ self.group1());
        let other = Scalar::from_groups(
            // scalar
            -f32::powi(self[e45], 2) - f32::powi(self[e4235], 2) - f32::powi(self[e4315], 2) - f32::powi(self[e4125], 2),
        );
        return Flector::from_groups(
            // e15, e25, e35, e45
            Simd32x4::from(other[scalar]) * reverse.group0(),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(other[scalar]) * reverse.group1(),
        );
    }
}
impl std::ops::Div<InversePrefixOrPostfix> for FlectorOnOrigin {
    type Output = FlectorOnOrigin;
    fn div(self, _rhs: InversePrefixOrPostfix) -> Self::Output {
        self.inverse()
    }
}
impl std::ops::DivAssign<InversePrefixOrPostfix> for FlectorOnOrigin {
    fn div_assign(&mut self, _rhs: InversePrefixOrPostfix) {
        *self = self.inverse()
    }
}
impl Inverse for FlectorOnOrigin {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        3        2        0
    //  no simd        3        5        0
    fn inverse(self) -> Self {
        use crate::elements::*;
        return FlectorOnOrigin::from_groups(
            // e45, e4235, e4315, e4125
            Simd32x4::from(-f32::powi(self[e45], 2) - f32::powi(self[e4235], 2) - f32::powi(self[e4315], 2) - f32::powi(self[e4125], 2))
                * Simd32x4::from([self[e45] * -1.0, self[e4235], self[e4315], self[e4125]]),
        );
    }
}
impl std::ops::Div<InversePrefixOrPostfix> for Line {
    type Output = Line;
    fn div(self, _rhs: InversePrefixOrPostfix) -> Self::Output {
        self.inverse()
    }
}
impl std::ops::DivAssign<InversePrefixOrPostfix> for Line {
    fn div_assign(&mut self, _rhs: InversePrefixOrPostfix) {
        *self = self.inverse()
    }
}
impl Inverse for Line {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        0        0
    //    simd3        0        4        0
    // Totals...
    // yes simd        2        4        0
    //  no simd        2       12        0
    fn inverse(self) -> Self {
        use crate::elements::*;
        let reverse = Line::from_groups(
            // e415, e425, e435
            self.group0() * Simd32x3::from(-1.0),
            // e235, e315, e125
            self.group1() * Simd32x3::from(-1.0),
        );
        let other = Scalar::from_groups(/* scalar */ -f32::powi(self[e415], 2) - f32::powi(self[e425], 2) - f32::powi(self[e435], 2));
        return Line::from_groups(
            // e415, e425, e435
            Simd32x3::from(other[scalar]) * reverse.group0(),
            // e235, e315, e125
            Simd32x3::from(other[scalar]) * reverse.group1(),
        );
    }
}
impl std::ops::Div<InversePrefixOrPostfix> for LineOnOrigin {
    type Output = LineOnOrigin;
    fn div(self, _rhs: InversePrefixOrPostfix) -> Self::Output {
        self.inverse()
    }
}
impl std::ops::DivAssign<InversePrefixOrPostfix> for LineOnOrigin {
    fn div_assign(&mut self, _rhs: InversePrefixOrPostfix) {
        *self = self.inverse()
    }
}
impl Inverse for LineOnOrigin {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        3        0
    //    simd3        0        1        0
    // Totals...
    // yes simd        2        4        0
    //  no simd        2        6        0
    fn inverse(self) -> Self {
        use crate::elements::*;
        return LineOnOrigin::from_groups(
            // e415, e425, e435
            Simd32x3::from(-f32::powi(self[e415], 2) - f32::powi(self[e425], 2) - f32::powi(self[e435], 2))
                * Simd32x3::from([self[e415] * -1.0, self[e425] * -1.0, self[e435] * -1.0]),
        );
    }
}
impl std::ops::Div<InversePrefixOrPostfix> for Motor {
    type Output = Motor;
    fn div(self, _rhs: InversePrefixOrPostfix) -> Self::Output {
        self.inverse()
    }
}
impl std::ops::DivAssign<InversePrefixOrPostfix> for Motor {
    fn div_assign(&mut self, _rhs: InversePrefixOrPostfix) {
        *self = self.inverse()
    }
}
impl Inverse for Motor {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        0        0
    //    simd4        0        4        0
    // Totals...
    // yes simd        3        4        0
    //  no simd        3       16        0
    fn inverse(self) -> Self {
        use crate::elements::*;
        let reverse = Motor::from_groups(
            // e415, e425, e435, e12345
            self.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e235, e315, e125, e5
            self.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
        );
        let other = Scalar::from_groups(
            // scalar
            -f32::powi(self[e415], 2) - f32::powi(self[e425], 2) - f32::powi(self[e435], 2) - f32::powi(self[e12345], 2),
        );
        return Motor::from_groups(
            // e415, e425, e435, e12345
            Simd32x4::from(other[scalar]) * reverse.group0(),
            // e235, e315, e125, e5
            Simd32x4::from(other[scalar]) * reverse.group1(),
        );
    }
}
impl std::ops::Div<InversePrefixOrPostfix> for MotorOnOrigin {
    type Output = MotorOnOrigin;
    fn div(self, _rhs: InversePrefixOrPostfix) -> Self::Output {
        self.inverse()
    }
}
impl std::ops::DivAssign<InversePrefixOrPostfix> for MotorOnOrigin {
    fn div_assign(&mut self, _rhs: InversePrefixOrPostfix) {
        *self = self.inverse()
    }
}
impl Inverse for MotorOnOrigin {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        3        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        3        4        0
    //  no simd        3        7        0
    fn inverse(self) -> Self {
        use crate::elements::*;
        return MotorOnOrigin::from_groups(
            // e415, e425, e435, e12345
            Simd32x4::from(-f32::powi(self[e415], 2) - f32::powi(self[e425], 2) - f32::powi(self[e435], 2) - f32::powi(self[e12345], 2))
                * Simd32x4::from([self[e415] * -1.0, self[e425] * -1.0, self[e435] * -1.0, self[e12345]]),
        );
    }
}
impl std::ops::Div<InversePrefixOrPostfix> for MultiVector {
    type Output = MultiVector;
    fn div(self, _rhs: InversePrefixOrPostfix) -> Self::Output {
        self.inverse()
    }
}
impl std::ops::DivAssign<InversePrefixOrPostfix> for MultiVector {
    fn div_assign(&mut self, _rhs: InversePrefixOrPostfix) {
        *self = self.inverse()
    }
}
impl Inverse for MultiVector {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       23       18        0
    //    simd2        0        1        0
    //    simd3        0        8        0
    //    simd4        0        6        0
    // Totals...
    // yes simd       23       33        0
    //  no simd       23       68        0
    fn inverse(self) -> Self {
        use crate::elements::*;
        let reverse = MultiVector::from_groups(
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
        let other = Scalar::from_groups(
            // scalar
            2.0 * (self[e41] * self[e15])
                + 2.0 * (self[e42] * self[e25])
                + 2.0 * (self[e43] * self[e35])
                + 2.0 * (self[e1234] * self[e3215])
                + f32::powi(self[scalar], 2)
                + f32::powi(self[e1], 2)
                + f32::powi(self[e2], 2)
                + f32::powi(self[e3], 2)
                + f32::powi(self[e23], 2)
                + f32::powi(self[e31], 2)
                + f32::powi(self[e12], 2)
                + f32::powi(self[e321], 2)
                - f32::powi(self[e12345], 2)
                - f32::powi(self[e45], 2)
                - f32::powi(self[e415], 2)
                - f32::powi(self[e425], 2)
                - f32::powi(self[e435], 2)
                - f32::powi(self[e4235], 2)
                - f32::powi(self[e4315], 2)
                - f32::powi(self[e4125], 2)
                - 2.0 * (self[e4] * self[e5])
                - 2.0 * (self[e423] * self[e235])
                - 2.0 * (self[e431] * self[e315])
                - 2.0 * (self[e412] * self[e125]),
        );
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(other[scalar]) * reverse.group0(),
            // e1, e2, e3, e4
            Simd32x4::from(other[scalar]) * reverse.group1(),
            // e5
            reverse[e5] * other[scalar],
            // e41, e42, e43, e45
            Simd32x4::from(other[scalar]) * reverse.group3(),
            // e15, e25, e35
            Simd32x3::from(other[scalar]) * reverse.group4(),
            // e23, e31, e12
            Simd32x3::from(other[scalar]) * reverse.group5(),
            // e415, e425, e435, e321
            Simd32x4::from(other[scalar]) * reverse.group6(),
            // e423, e431, e412
            Simd32x3::from(other[scalar]) * reverse.group7(),
            // e235, e315, e125
            Simd32x3::from(other[scalar]) * reverse.group8(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(other[scalar]) * reverse.group9(),
            // e3215
            reverse[e3215] * other[scalar],
        );
    }
}
impl std::ops::Div<InversePrefixOrPostfix> for MysteryCircle {
    type Output = MysteryCircle;
    fn div(self, _rhs: InversePrefixOrPostfix) -> Self::Output {
        self.inverse()
    }
}
impl std::ops::DivAssign<InversePrefixOrPostfix> for MysteryCircle {
    fn div_assign(&mut self, _rhs: InversePrefixOrPostfix) {
        *self = self.inverse()
    }
}
impl Inverse for MysteryCircle {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        4        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        3        5        0
    //  no simd        3        8        0
    fn inverse(self) -> Self {
        use crate::elements::*;
        return MysteryCircle::from_groups(
            // e415, e425, e435, e321
            Simd32x4::from(f32::powi(self[e321], 2) - f32::powi(self[e415], 2) - f32::powi(self[e425], 2) - f32::powi(self[e435], 2))
                * Simd32x4::from([self[e415] * -1.0, self[e425] * -1.0, self[e435] * -1.0, self[e321] * -1.0]),
        );
    }
}
impl std::ops::Div<InversePrefixOrPostfix> for MysteryCircleRotor {
    type Output = MysteryCircleRotor;
    fn div(self, _rhs: InversePrefixOrPostfix) -> Self::Output {
        self.inverse()
    }
}
impl std::ops::DivAssign<InversePrefixOrPostfix> for MysteryCircleRotor {
    fn div_assign(&mut self, _rhs: InversePrefixOrPostfix) {
        *self = self.inverse()
    }
}
impl Inverse for MysteryCircleRotor {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        1        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        4        3        0
    //  no simd        4        9        0
    fn inverse(self) -> Self {
        use crate::elements::*;
        let reverse = MysteryCircleRotor::from_groups(/* e415, e425, e435, e321 */ self.group0() * Simd32x4::from(-1.0), /* e12345 */ self[e12345]);
        let other = Scalar::from_groups(
            // scalar
            f32::powi(self[e321], 2) - f32::powi(self[e415], 2) - f32::powi(self[e425], 2) - f32::powi(self[e435], 2) - f32::powi(self[e12345], 2),
        );
        return MysteryCircleRotor::from_groups(
            // e415, e425, e435, e321
            Simd32x4::from(other[scalar]) * reverse.group0(),
            // e12345
            reverse[e12345] * other[scalar],
        );
    }
}
impl std::ops::Div<InversePrefixOrPostfix> for MysteryDipole {
    type Output = MysteryDipole;
    fn div(self, _rhs: InversePrefixOrPostfix) -> Self::Output {
        self.inverse()
    }
}
impl std::ops::DivAssign<InversePrefixOrPostfix> for MysteryDipole {
    fn div_assign(&mut self, _rhs: InversePrefixOrPostfix) {
        *self = self.inverse()
    }
}
impl Inverse for MysteryDipole {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        4        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        3        5        0
    //  no simd        3        8        0
    fn inverse(self) -> Self {
        use crate::elements::*;
        return MysteryDipole::from_groups(
            // e23, e31, e12, e45
            Simd32x4::from(f32::powi(self[e23], 2) + f32::powi(self[e31], 2) + f32::powi(self[e12], 2) - f32::powi(self[e45], 2))
                * Simd32x4::from([self[e23] * -1.0, self[e31] * -1.0, self[e12] * -1.0, self[e45] * -1.0]),
        );
    }
}
impl std::ops::Div<InversePrefixOrPostfix> for MysteryDipoleInversion {
    type Output = MysteryDipoleInversion;
    fn div(self, _rhs: InversePrefixOrPostfix) -> Self::Output {
        self.inverse()
    }
}
impl std::ops::DivAssign<InversePrefixOrPostfix> for MysteryDipoleInversion {
    fn div_assign(&mut self, _rhs: InversePrefixOrPostfix) {
        *self = self.inverse()
    }
}
impl Inverse for MysteryDipoleInversion {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        6        0        0
    //    simd3        0        1        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        6        3        0
    //  no simd        6       11        0
    fn inverse(self) -> Self {
        use crate::elements::*;
        let reverse = MysteryDipoleInversion::from_groups(/* e23, e31, e12, e45 */ self.group0() * Simd32x4::from(-1.0), /* e4235, e4315, e4125 */ self.group1());
        let other = Scalar::from_groups(
            // scalar
            f32::powi(self[e23], 2) + f32::powi(self[e31], 2) + f32::powi(self[e12], 2)
                - f32::powi(self[e45], 2)
                - f32::powi(self[e4235], 2)
                - f32::powi(self[e4315], 2)
                - f32::powi(self[e4125], 2),
        );
        return MysteryDipoleInversion::from_groups(
            // e23, e31, e12, e45
            Simd32x4::from(other[scalar]) * reverse.group0(),
            // e4235, e4315, e4125
            Simd32x3::from(other[scalar]) * reverse.group1(),
        );
    }
}
impl std::ops::Div<InversePrefixOrPostfix> for MysteryVersorEven {
    type Output = MysteryVersorEven;
    fn div(self, _rhs: InversePrefixOrPostfix) -> Self::Output {
        self.inverse()
    }
}
impl std::ops::DivAssign<InversePrefixOrPostfix> for MysteryVersorEven {
    fn div_assign(&mut self, _rhs: InversePrefixOrPostfix) {
        *self = self.inverse()
    }
}
impl Inverse for MysteryVersorEven {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        7        0        0
    //    simd4        0        3        0
    // Totals...
    // yes simd        7        3        0
    //  no simd        7       12        0
    fn inverse(self) -> Self {
        use crate::elements::*;
        let reverse = MysteryVersorEven::from_groups(/* e12345, e1, e2, e3 */ self.group0(), /* e415, e425, e435, e321 */ self.group1() * Simd32x4::from(-1.0));
        let other = Scalar::from_groups(
            // scalar
            f32::powi(self[e1], 2) + f32::powi(self[e2], 2) + f32::powi(self[e3], 2) + f32::powi(self[e321], 2)
                - f32::powi(self[e12345], 2)
                - f32::powi(self[e415], 2)
                - f32::powi(self[e425], 2)
                - f32::powi(self[e435], 2),
        );
        return MysteryVersorEven::from_groups(
            // e12345, e1, e2, e3
            Simd32x4::from(other[scalar]) * reverse.group0(),
            // e415, e425, e435, e321
            Simd32x4::from(other[scalar]) * reverse.group1(),
        );
    }
}
impl std::ops::Div<InversePrefixOrPostfix> for MysteryVersorOdd {
    type Output = MysteryVersorOdd;
    fn div(self, _rhs: InversePrefixOrPostfix) -> Self::Output {
        self.inverse()
    }
}
impl std::ops::DivAssign<InversePrefixOrPostfix> for MysteryVersorOdd {
    fn div_assign(&mut self, _rhs: InversePrefixOrPostfix) {
        *self = self.inverse()
    }
}
impl Inverse for MysteryVersorOdd {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        7        0        0
    //    simd4        0        3        0
    // Totals...
    // yes simd        7        3        0
    //  no simd        7       12        0
    fn inverse(self) -> Self {
        use crate::elements::*;
        let reverse = MysteryVersorOdd::from_groups(/* scalar, e4235, e4315, e4125 */ self.group0(), /* e23, e31, e12, e45 */ self.group1() * Simd32x4::from(-1.0));
        let other = Scalar::from_groups(
            // scalar
            f32::powi(self[scalar], 2) + f32::powi(self[e23], 2) + f32::powi(self[e31], 2) + f32::powi(self[e12], 2)
                - f32::powi(self[e4235], 2)
                - f32::powi(self[e4315], 2)
                - f32::powi(self[e4125], 2)
                - f32::powi(self[e45], 2),
        );
        return MysteryVersorOdd::from_groups(
            // scalar, e4235, e4315, e4125
            Simd32x4::from(other[scalar]) * reverse.group0(),
            // e23, e31, e12, e45
            Simd32x4::from(other[scalar]) * reverse.group1(),
        );
    }
}
impl std::ops::Div<InversePrefixOrPostfix> for Plane {
    type Output = Plane;
    fn div(self, _rhs: InversePrefixOrPostfix) -> Self::Output {
        self.inverse()
    }
}
impl std::ops::DivAssign<InversePrefixOrPostfix> for Plane {
    fn div_assign(&mut self, _rhs: InversePrefixOrPostfix) {
        *self = self.inverse()
    }
}
impl Inverse for Plane {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        0        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        2        1        0
    //  no simd        2        4        0
    fn inverse(self) -> Self {
        use crate::elements::*;
        return Plane::from_groups(
            // e4235, e4315, e4125, e3215
            Simd32x4::from(-f32::powi(self[e4235], 2) - f32::powi(self[e4315], 2) - f32::powi(self[e4125], 2)) * self.group0(),
        );
    }
}
impl std::ops::Div<InversePrefixOrPostfix> for PlaneOnOrigin {
    type Output = PlaneOnOrigin;
    fn div(self, _rhs: InversePrefixOrPostfix) -> Self::Output {
        self.inverse()
    }
}
impl std::ops::DivAssign<InversePrefixOrPostfix> for PlaneOnOrigin {
    fn div_assign(&mut self, _rhs: InversePrefixOrPostfix) {
        *self = self.inverse()
    }
}
impl Inverse for PlaneOnOrigin {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        0        0
    //    simd3        0        1        0
    // Totals...
    // yes simd        2        1        0
    //  no simd        2        3        0
    fn inverse(self) -> Self {
        use crate::elements::*;
        return PlaneOnOrigin::from_groups(
            // e4235, e4315, e4125
            Simd32x3::from(-f32::powi(self[e4235], 2) - f32::powi(self[e4315], 2) - f32::powi(self[e4125], 2)) * self.group0(),
        );
    }
}
impl std::ops::Div<InversePrefixOrPostfix> for RoundPoint {
    type Output = RoundPoint;
    fn div(self, _rhs: InversePrefixOrPostfix) -> Self::Output {
        self.inverse()
    }
}
impl std::ops::DivAssign<InversePrefixOrPostfix> for RoundPoint {
    fn div_assign(&mut self, _rhs: InversePrefixOrPostfix) {
        *self = self.inverse()
    }
}
impl Inverse for RoundPoint {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        3        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        3        4        0
    //  no simd        3        7        0
    fn inverse(self) -> Self {
        use crate::elements::*;
        let other = Scalar::from_groups(/* scalar */ f32::powi(self[e1], 2) + f32::powi(self[e2], 2) + f32::powi(self[e3], 2) - 2.0 * (self[e4] * self[e5]));
        return RoundPoint::from_groups(/* e1, e2, e3, e4 */ Simd32x4::from(other[scalar]) * self.group0(), /* e5 */ self[e5] * other[scalar]);
    }
}
impl std::ops::Div<InversePrefixOrPostfix> for RoundPointAtOrigin {
    type Output = RoundPointAtOrigin;
    fn div(self, _rhs: InversePrefixOrPostfix) -> Self::Output {
        self.inverse()
    }
}
impl std::ops::DivAssign<InversePrefixOrPostfix> for RoundPointAtOrigin {
    fn div_assign(&mut self, _rhs: InversePrefixOrPostfix) {
        *self = self.inverse()
    }
}
impl Inverse for RoundPointAtOrigin {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        2
    //    simd2        0        1        0
    // Totals...
    // yes simd        0        2        2
    //  no simd        0        3        2
    fn inverse(self) -> Self {
        use crate::elements::*;
        return RoundPointAtOrigin::from_groups(/* e4, e5 */ Simd32x2::from(1.0 / self[e4] / (self[e5]) * -2.0) * self.group0());
    }
}
impl std::ops::Div<InversePrefixOrPostfix> for Scalar {
    type Output = Scalar;
    fn div(self, _rhs: InversePrefixOrPostfix) -> Self::Output {
        self.inverse()
    }
}
impl std::ops::DivAssign<InversePrefixOrPostfix> for Scalar {
    fn div_assign(&mut self, _rhs: InversePrefixOrPostfix) {
        *self = self.inverse()
    }
}
impl Inverse for Scalar {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        0        1
    fn inverse(self) -> Self {
        use crate::elements::*;
        return Scalar::from_groups(/* scalar */ 1.0 / self[scalar]);
    }
}
impl std::ops::Div<InversePrefixOrPostfix> for Sphere {
    type Output = Sphere;
    fn div(self, _rhs: InversePrefixOrPostfix) -> Self::Output {
        self.inverse()
    }
}
impl std::ops::DivAssign<InversePrefixOrPostfix> for Sphere {
    fn div_assign(&mut self, _rhs: InversePrefixOrPostfix) {
        *self = self.inverse()
    }
}
impl Inverse for Sphere {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        3        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        3        4        0
    //  no simd        3        7        0
    fn inverse(self) -> Self {
        use crate::elements::*;
        let other = Scalar::from_groups(
            // scalar
            2.0 * (self[e3215] * self[e1234]) - f32::powi(self[e4235], 2) - f32::powi(self[e4315], 2) - f32::powi(self[e4125], 2),
        );
        return Sphere::from_groups(
            // e4235, e4315, e4125, e3215
            Simd32x4::from(other[scalar]) * self.group0(),
            // e1234
            other[scalar] * self[e1234],
        );
    }
}
impl std::ops::Div<InversePrefixOrPostfix> for SphereAtOrigin {
    type Output = SphereAtOrigin;
    fn div(self, _rhs: InversePrefixOrPostfix) -> Self::Output {
        self.inverse()
    }
}
impl std::ops::DivAssign<InversePrefixOrPostfix> for SphereAtOrigin {
    fn div_assign(&mut self, _rhs: InversePrefixOrPostfix) {
        *self = self.inverse()
    }
}
impl Inverse for SphereAtOrigin {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        2
    //    simd2        0        1        0
    // Totals...
    // yes simd        0        2        2
    //  no simd        0        3        2
    fn inverse(self) -> Self {
        use crate::elements::*;
        return SphereAtOrigin::from_groups(/* e3215, e1234 */ Simd32x2::from(1.0 / self[e3215] / (self[e1234]) * 2.0) * self.group0());
    }
}
impl std::ops::Div<InversePrefixOrPostfix> for SphereOnOrigin {
    type Output = SphereOnOrigin;
    fn div(self, _rhs: InversePrefixOrPostfix) -> Self::Output {
        self.inverse()
    }
}
impl std::ops::DivAssign<InversePrefixOrPostfix> for SphereOnOrigin {
    fn div_assign(&mut self, _rhs: InversePrefixOrPostfix) {
        *self = self.inverse()
    }
}
impl Inverse for SphereOnOrigin {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        0        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        2        1        0
    //  no simd        2        4        0
    fn inverse(self) -> Self {
        use crate::elements::*;
        return SphereOnOrigin::from_groups(
            // e4235, e4315, e4125, e1234
            Simd32x4::from(-f32::powi(self[e4235], 2) - f32::powi(self[e4315], 2) - f32::powi(self[e4125], 2)) * self.group0(),
        );
    }
}
impl std::ops::Div<InversePrefixOrPostfix> for VersorEven {
    type Output = VersorEven;
    fn div(self, _rhs: InversePrefixOrPostfix) -> Self::Output {
        self.inverse()
    }
}
impl std::ops::DivAssign<InversePrefixOrPostfix> for VersorEven {
    fn div_assign(&mut self, _rhs: InversePrefixOrPostfix) {
        *self = self.inverse()
    }
}
impl Inverse for VersorEven {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       11        8        0
    //    simd4        0        7        0
    // Totals...
    // yes simd       11       15        0
    //  no simd       11       36        0
    fn inverse(self) -> Self {
        use crate::elements::*;
        let reverse = VersorEven::from_groups(
            // e423, e431, e412, e12345
            self.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e415, e425, e435, e321
            self.group1() * Simd32x4::from(-1.0),
            // e235, e315, e125, e5
            self.group2() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e1, e2, e3, e4
            self.group3(),
        );
        let other = Scalar::from_groups(
            // scalar
            f32::powi(self[e321], 2) + f32::powi(self[e1], 2) + f32::powi(self[e2], 2) + f32::powi(self[e3], 2)
                - f32::powi(self[e12345], 2)
                - f32::powi(self[e415], 2)
                - f32::powi(self[e425], 2)
                - f32::powi(self[e435], 2)
                - 2.0 * (self[e423] * self[e235])
                - 2.0 * (self[e431] * self[e315])
                - 2.0 * (self[e412] * self[e125])
                - 2.0 * (self[e5] * self[e4]),
        );
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from(other[scalar]) * reverse.group0(),
            // e415, e425, e435, e321
            Simd32x4::from(other[scalar]) * reverse.group1(),
            // e235, e315, e125, e5
            Simd32x4::from(other[scalar]) * reverse.group2(),
            // e1, e2, e3, e4
            Simd32x4::from(other[scalar]) * reverse.group3(),
        );
    }
}
impl std::ops::Div<InversePrefixOrPostfix> for VersorEvenAligningOrigin {
    type Output = VersorEvenAligningOrigin;
    fn div(self, _rhs: InversePrefixOrPostfix) -> Self::Output {
        self.inverse()
    }
}
impl std::ops::DivAssign<InversePrefixOrPostfix> for VersorEvenAligningOrigin {
    fn div_assign(&mut self, _rhs: InversePrefixOrPostfix) {
        *self = self.inverse()
    }
}
impl Inverse for VersorEvenAligningOrigin {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        7        8        0
    //    simd4        0        6        0
    // Totals...
    // yes simd        7       14        0
    //  no simd        7       32        0
    fn inverse(self) -> Self {
        use crate::elements::*;
        let reverse = VersorEvenAligningOrigin::from_groups(
            // e423, e431, e412, e12345
            self.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e415, e425, e435, e4
            self.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e235, e315, e125, e5
            self.group2() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
        );
        let other = Scalar::from_groups(
            // scalar
            -f32::powi(self[e12345], 2)
                - f32::powi(self[e415], 2)
                - f32::powi(self[e425], 2)
                - f32::powi(self[e435], 2)
                - 2.0 * (self[e423] * self[e235])
                - 2.0 * (self[e431] * self[e315])
                - 2.0 * (self[e412] * self[e125])
                - 2.0 * (self[e4] * self[e5]),
        );
        return VersorEvenAligningOrigin::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from(other[scalar]) * reverse.group0(),
            // e415, e425, e435, e4
            Simd32x4::from(other[scalar]) * reverse.group1(),
            // e235, e315, e125, e5
            Simd32x4::from(other[scalar]) * reverse.group2(),
        );
    }
}
impl std::ops::Div<InversePrefixOrPostfix> for VersorEvenAtInfinity {
    type Output = VersorEvenAtInfinity;
    fn div(self, _rhs: InversePrefixOrPostfix) -> Self::Output {
        self.inverse()
    }
}
impl std::ops::DivAssign<InversePrefixOrPostfix> for VersorEvenAtInfinity {
    fn div_assign(&mut self, _rhs: InversePrefixOrPostfix) {
        *self = self.inverse()
    }
}
impl Inverse for VersorEvenAtInfinity {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        7        0        0
    //    simd4        0        5        0
    // Totals...
    // yes simd        7        5        0
    //  no simd        7       20        0
    fn inverse(self) -> Self {
        use crate::elements::*;
        let reverse = VersorEvenAtInfinity::from_groups(
            // e12345, e1, e2, e3
            self.group0(),
            // e415, e425, e435, e321
            self.group1() * Simd32x4::from(-1.0),
            // e235, e315, e125, e5
            self.group2() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
        );
        let other = Scalar::from_groups(
            // scalar
            f32::powi(self[e1], 2) + f32::powi(self[e2], 2) + f32::powi(self[e3], 2) + f32::powi(self[e321], 2)
                - f32::powi(self[e12345], 2)
                - f32::powi(self[e415], 2)
                - f32::powi(self[e425], 2)
                - f32::powi(self[e435], 2),
        );
        return VersorEvenAtInfinity::from_groups(
            // e12345, e1, e2, e3
            Simd32x4::from(other[scalar]) * reverse.group0(),
            // e415, e425, e435, e321
            Simd32x4::from(other[scalar]) * reverse.group1(),
            // e235, e315, e125, e5
            Simd32x4::from(other[scalar]) * reverse.group2(),
        );
    }
}
impl std::ops::Div<InversePrefixOrPostfix> for VersorEvenAtOrigin {
    type Output = VersorEvenAtOrigin;
    fn div(self, _rhs: InversePrefixOrPostfix) -> Self::Output {
        self.inverse()
    }
}
impl std::ops::DivAssign<InversePrefixOrPostfix> for VersorEvenAtOrigin {
    fn div_assign(&mut self, _rhs: InversePrefixOrPostfix) {
        *self = self.inverse()
    }
}
impl Inverse for VersorEvenAtOrigin {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        8        0
    //    simd4        0        4        0
    // Totals...
    // yes simd        3       12        0
    //  no simd        3       24        0
    fn inverse(self) -> Self {
        use crate::elements::*;
        let reverse = VersorEvenAtOrigin::from_groups(
            // e423, e431, e412, e4
            self.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e235, e315, e125, e5
            self.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
        );
        let other = Scalar::from_groups(
            // scalar
            -2.0 * (self[e423] * self[e235]) - 2.0 * (self[e431] * self[e315]) - 2.0 * (self[e412] * self[e125]) - 2.0 * (self[e4] * self[e5]),
        );
        return VersorEvenAtOrigin::from_groups(
            // e423, e431, e412, e4
            Simd32x4::from(other[scalar]) * reverse.group0(),
            // e235, e315, e125, e5
            Simd32x4::from(other[scalar]) * reverse.group1(),
        );
    }
}
impl std::ops::Div<InversePrefixOrPostfix> for VersorEvenOnOrigin {
    type Output = VersorEvenOnOrigin;
    fn div(self, _rhs: InversePrefixOrPostfix) -> Self::Output {
        self.inverse()
    }
}
impl std::ops::DivAssign<InversePrefixOrPostfix> for VersorEvenOnOrigin {
    fn div_assign(&mut self, _rhs: InversePrefixOrPostfix) {
        *self = self.inverse()
    }
}
impl Inverse for VersorEvenOnOrigin {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        0        0
    //    simd4        0        4        0
    // Totals...
    // yes simd        3        4        0
    //  no simd        3       16        0
    fn inverse(self) -> Self {
        use crate::elements::*;
        let reverse = VersorEvenOnOrigin::from_groups(
            // e423, e431, e412, e12345
            self.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e415, e425, e435, e4
            self.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
        );
        let other = Scalar::from_groups(
            // scalar
            -f32::powi(self[e12345], 2) - f32::powi(self[e415], 2) - f32::powi(self[e425], 2) - f32::powi(self[e435], 2),
        );
        return VersorEvenOnOrigin::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from(other[scalar]) * reverse.group0(),
            // e415, e425, e435, e4
            Simd32x4::from(other[scalar]) * reverse.group1(),
        );
    }
}
impl std::ops::Div<InversePrefixOrPostfix> for VersorEvenOrthogonalOrigin {
    type Output = VersorEvenOrthogonalOrigin;
    fn div(self, _rhs: InversePrefixOrPostfix) -> Self::Output {
        self.inverse()
    }
}
impl std::ops::DivAssign<InversePrefixOrPostfix> for VersorEvenOrthogonalOrigin {
    fn div_assign(&mut self, _rhs: InversePrefixOrPostfix) {
        *self = self.inverse()
    }
}
impl Inverse for VersorEvenOrthogonalOrigin {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        7        8        0
    //    simd4        0        5        0
    // Totals...
    // yes simd        7       13        0
    //  no simd        7       28        0
    fn inverse(self) -> Self {
        use crate::elements::*;
        let reverse = VersorEvenOrthogonalOrigin::from_groups(
            // e423, e431, e412, e321
            self.group0() * Simd32x4::from(-1.0),
            // e235, e315, e125, e5
            self.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e1, e2, e3, e4
            self.group2(),
        );
        let other = Scalar::from_groups(
            // scalar
            f32::powi(self[e321], 2) + f32::powi(self[e1], 2) + f32::powi(self[e2], 2) + f32::powi(self[e3], 2)
                - 2.0 * (self[e423] * self[e235])
                - 2.0 * (self[e431] * self[e315])
                - 2.0 * (self[e412] * self[e125])
                - 2.0 * (self[e5] * self[e4]),
        );
        return VersorEvenOrthogonalOrigin::from_groups(
            // e423, e431, e412, e321
            Simd32x4::from(other[scalar]) * reverse.group0(),
            // e235, e315, e125, e5
            Simd32x4::from(other[scalar]) * reverse.group1(),
            // e1, e2, e3, e4
            Simd32x4::from(other[scalar]) * reverse.group2(),
        );
    }
}
impl std::ops::Div<InversePrefixOrPostfix> for VersorOdd {
    type Output = VersorOdd;
    fn div(self, _rhs: InversePrefixOrPostfix) -> Self::Output {
        self.inverse()
    }
}
impl std::ops::DivAssign<InversePrefixOrPostfix> for VersorOdd {
    fn div_assign(&mut self, _rhs: InversePrefixOrPostfix) {
        *self = self.inverse()
    }
}
impl Inverse for VersorOdd {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       11        8        0
    //    simd4        0        7        0
    // Totals...
    // yes simd       11       15        0
    //  no simd       11       36        0
    fn inverse(self) -> Self {
        use crate::elements::*;
        let reverse = VersorOdd::from_groups(
            // e41, e42, e43, scalar
            self.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e23, e31, e12, e45
            self.group1() * Simd32x4::from(-1.0),
            // e15, e25, e35, e1234
            self.group2() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e4235, e4315, e4125, e3215
            self.group3(),
        );
        let other = Scalar::from_groups(
            // scalar
            2.0 * (self[e41] * self[e15])
                + 2.0 * (self[e42] * self[e25])
                + 2.0 * (self[e43] * self[e35])
                + 2.0 * (self[e1234] * self[e3215])
                + f32::powi(self[scalar], 2)
                + f32::powi(self[e23], 2)
                + f32::powi(self[e31], 2)
                + f32::powi(self[e12], 2)
                - f32::powi(self[e45], 2)
                - f32::powi(self[e4235], 2)
                - f32::powi(self[e4315], 2)
                - f32::powi(self[e4125], 2),
        );
        return VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from(other[scalar]) * reverse.group0(),
            // e23, e31, e12, e45
            Simd32x4::from(other[scalar]) * reverse.group1(),
            // e15, e25, e35, e1234
            Simd32x4::from(other[scalar]) * reverse.group2(),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(other[scalar]) * reverse.group3(),
        );
    }
}
impl std::ops::Div<InversePrefixOrPostfix> for VersorOddAtInfinity {
    type Output = VersorOddAtInfinity;
    fn div(self, _rhs: InversePrefixOrPostfix) -> Self::Output {
        self.inverse()
    }
}
impl std::ops::DivAssign<InversePrefixOrPostfix> for VersorOddAtInfinity {
    fn div_assign(&mut self, _rhs: InversePrefixOrPostfix) {
        *self = self.inverse()
    }
}
impl Inverse for VersorOddAtInfinity {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        7        0        0
    //    simd4        0        5        0
    // Totals...
    // yes simd        7        5        0
    //  no simd        7       20        0
    fn inverse(self) -> Self {
        use crate::elements::*;
        let reverse = VersorOddAtInfinity::from_groups(
            // scalar, e15, e25, e35
            self.group0() * Simd32x4::from([1.0, -1.0, -1.0, -1.0]),
            // e23, e31, e12, e45
            self.group1() * Simd32x4::from(-1.0),
            // e4235, e4315, e4125, e3215
            self.group2(),
        );
        let other = Scalar::from_groups(
            // scalar
            f32::powi(self[scalar], 2) + f32::powi(self[e23], 2) + f32::powi(self[e31], 2) + f32::powi(self[e12], 2)
                - f32::powi(self[e45], 2)
                - f32::powi(self[e4235], 2)
                - f32::powi(self[e4315], 2)
                - f32::powi(self[e4125], 2),
        );
        return VersorOddAtInfinity::from_groups(
            // scalar, e15, e25, e35
            Simd32x4::from(other[scalar]) * reverse.group0(),
            // e23, e31, e12, e45
            Simd32x4::from(other[scalar]) * reverse.group1(),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(other[scalar]) * reverse.group2(),
        );
    }
}
impl std::ops::Div<InversePrefixOrPostfix> for VersorOddOrthogonalOrigin {
    type Output = VersorOddOrthogonalOrigin;
    fn div(self, _rhs: InversePrefixOrPostfix) -> Self::Output {
        self.inverse()
    }
}
impl std::ops::DivAssign<InversePrefixOrPostfix> for VersorOddOrthogonalOrigin {
    fn div_assign(&mut self, _rhs: InversePrefixOrPostfix) {
        *self = self.inverse()
    }
}
impl Inverse for VersorOddOrthogonalOrigin {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        7        8        0
    //    simd4        0        6        0
    // Totals...
    // yes simd        7       14        0
    //  no simd        7       32        0
    fn inverse(self) -> Self {
        use crate::elements::*;
        let reverse = VersorOddOrthogonalOrigin::from_groups(
            // e41, e42, e43, scalar
            self.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e23, e31, e12, e3215
            self.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e15, e25, e35, e1234
            self.group2() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
        );
        let other = Scalar::from_groups(
            // scalar
            2.0 * (self[e41] * self[e15])
                + 2.0 * (self[e42] * self[e25])
                + 2.0 * (self[e43] * self[e35])
                + 2.0 * (self[e3215] * self[e1234])
                + f32::powi(self[scalar], 2)
                + f32::powi(self[e23], 2)
                + f32::powi(self[e31], 2)
                + f32::powi(self[e12], 2),
        );
        return VersorOddOrthogonalOrigin::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from(other[scalar]) * reverse.group0(),
            // e23, e31, e12, e3215
            Simd32x4::from(other[scalar]) * reverse.group1(),
            // e15, e25, e35, e1234
            Simd32x4::from(other[scalar]) * reverse.group2(),
        );
    }
}
