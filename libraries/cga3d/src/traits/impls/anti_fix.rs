// Note on Operative Statistics:
// Operative Statistics are not a precise predictor of performance or performance comparisons.
// This is due to varying hardware capabilities and compiler optimizations.
// As always, where performance is a concern, there is no substitute for
// real measurements on real work-loads on real hardware.
// Disclaimer aside, enjoy the fun information =)
//
// Total Implementations: 8
//
// Yes SIMD:   add/sub     mul     div
//  Minimum:         0       0       0
//   Median:         2       2       0
//  Average:         1       1       0
//  Maximum:         3       4       0
//
//  No SIMD:   add/sub     mul     div
//  Minimum:         0       0       0
//   Median:         2       7       0
//  Average:         1       4       0
//  Maximum:         3       9       0
impl std::ops::Div<AntiFixPrefixOrPostfix> for AntiFlatPoint {
    type Output = AntiFlatPoint;
    fn div(self, _rhs: AntiFixPrefixOrPostfix) -> Self::Output {
        self.anti_fix()
    }
}
impl std::ops::DivAssign<AntiFixPrefixOrPostfix> for AntiFlatPoint {
    fn div_assign(&mut self, _rhs: AntiFixPrefixOrPostfix) {
        *self = self.anti_fix()
    }
}
impl AntiFix for AntiFlatPoint {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        2        0
    // no simd        0        8        0
    fn anti_fix(self) -> Self {
        use crate::elements::*;
        return AntiFlatPoint::from_groups(
            // e235, e315, e125, e321
            Simd32x4::from(f32::powf((self.group0() * Simd32x4::from(-1.0))[3], -0.5) * f32::powf(self[e321], -0.5)) * self.group0(),
        );
    }
}
impl std::ops::Div<AntiFixPrefixOrPostfix> for AntiPlane {
    type Output = AntiPlane;
    fn div(self, _rhs: AntiFixPrefixOrPostfix) -> Self::Output {
        self.anti_fix()
    }
}
impl std::ops::DivAssign<AntiFixPrefixOrPostfix> for AntiPlane {
    fn div_assign(&mut self, _rhs: AntiFixPrefixOrPostfix) {
        *self = self.anti_fix()
    }
}
impl AntiFix for AntiPlane {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        0        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        2        1        0
    //  no simd        2        4        0
    fn anti_fix(self) -> Self {
        use crate::elements::*;
        return AntiPlane::from_groups(
            // e1, e2, e3, e5
            Simd32x4::from(-f32::powi(self[e1], 2) - f32::powi(self[e2], 2) - f32::powi(self[e3], 2)) * self.group0(),
        );
    }
}
impl std::ops::Div<AntiFixPrefixOrPostfix> for AntiScalar {
    type Output = AntiScalar;
    fn div(self, _rhs: AntiFixPrefixOrPostfix) -> Self::Output {
        self.anti_fix()
    }
}
impl std::ops::DivAssign<AntiFixPrefixOrPostfix> for AntiScalar {
    fn div_assign(&mut self, _rhs: AntiFixPrefixOrPostfix) {
        *self = self.anti_fix()
    }
}
impl AntiFix for AntiScalar {
    fn anti_fix(self) -> Self {
        return AntiScalar::from_groups(/* e12345 */ 1.0);
    }
}
impl std::ops::Div<AntiFixPrefixOrPostfix> for FlatPoint {
    type Output = FlatPoint;
    fn div(self, _rhs: AntiFixPrefixOrPostfix) -> Self::Output {
        self.anti_fix()
    }
}
impl std::ops::DivAssign<AntiFixPrefixOrPostfix> for FlatPoint {
    fn div_assign(&mut self, _rhs: AntiFixPrefixOrPostfix) {
        *self = self.anti_fix()
    }
}
impl AntiFix for FlatPoint {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        0        3        0
    //  no simd        0        9        0
    fn anti_fix(self) -> Self {
        use crate::elements::*;
        return FlatPoint::from_groups(
            // e15, e25, e35, e45
            Simd32x4::from(f32::powf((self.group0() * Simd32x4::from(-1.0))[3], -0.5) * f32::powf(self[e45], -0.5) * -1.0) * self.group0(),
        );
    }
}
impl std::ops::Div<AntiFixPrefixOrPostfix> for Plane {
    type Output = Plane;
    fn div(self, _rhs: AntiFixPrefixOrPostfix) -> Self::Output {
        self.anti_fix()
    }
}
impl std::ops::DivAssign<AntiFixPrefixOrPostfix> for Plane {
    fn div_assign(&mut self, _rhs: AntiFixPrefixOrPostfix) {
        *self = self.anti_fix()
    }
}
impl AntiFix for Plane {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        0        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        2        1        0
    //  no simd        2        4        0
    fn anti_fix(self) -> Self {
        use crate::elements::*;
        return Plane::from_groups(
            // e4235, e4315, e4125, e3215
            Simd32x4::from(f32::powi(self[e4235], 2) + f32::powi(self[e4315], 2) + f32::powi(self[e4125], 2)) * self.group0(),
        );
    }
}
impl std::ops::Div<AntiFixPrefixOrPostfix> for RoundPoint {
    type Output = RoundPoint;
    fn div(self, _rhs: AntiFixPrefixOrPostfix) -> Self::Output {
        self.anti_fix()
    }
}
impl std::ops::DivAssign<AntiFixPrefixOrPostfix> for RoundPoint {
    fn div_assign(&mut self, _rhs: AntiFixPrefixOrPostfix) {
        *self = self.anti_fix()
    }
}
impl AntiFix for RoundPoint {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        3        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        3        4        0
    //  no simd        3        7        0
    fn anti_fix(self) -> Self {
        use crate::elements::*;
        let geometric_anti_product_2 =
            AntiScalar::from_groups(/* e12345 */ 2.0 * (self[e4] * self[e5]) - f32::powi(self[e1], 2) - f32::powi(self[e2], 2) - f32::powi(self[e3], 2));
        return RoundPoint::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from(geometric_anti_product_2[e12345]) * self.group0(),
            // e5
            geometric_anti_product_2[e12345] * self[e5],
        );
    }
}
impl std::ops::Div<AntiFixPrefixOrPostfix> for Scalar {
    type Output = Scalar;
    fn div(self, _rhs: AntiFixPrefixOrPostfix) -> Self::Output {
        self.anti_fix()
    }
}
impl std::ops::DivAssign<AntiFixPrefixOrPostfix> for Scalar {
    fn div_assign(&mut self, _rhs: AntiFixPrefixOrPostfix) {
        *self = self.anti_fix()
    }
}
impl AntiFix for Scalar {
    fn anti_fix(self) -> Self {
        return Scalar::from_groups(/* scalar */ -1.0);
    }
}
impl std::ops::Div<AntiFixPrefixOrPostfix> for Sphere {
    type Output = Sphere;
    fn div(self, _rhs: AntiFixPrefixOrPostfix) -> Self::Output {
        self.anti_fix()
    }
}
impl std::ops::DivAssign<AntiFixPrefixOrPostfix> for Sphere {
    fn div_assign(&mut self, _rhs: AntiFixPrefixOrPostfix) {
        *self = self.anti_fix()
    }
}
impl AntiFix for Sphere {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        3        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        3        4        0
    //  no simd        3        7        0
    fn anti_fix(self) -> Self {
        use crate::elements::*;
        let geometric_anti_product_2 = AntiScalar::from_groups(
            // e12345
            f32::powi(self[e4235], 2) + f32::powi(self[e4315], 2) + f32::powi(self[e4125], 2) - 2.0 * (self[e3215] * self[e1234]),
        );
        return Sphere::from_groups(
            // e4235, e4315, e4125, e3215
            Simd32x4::from(geometric_anti_product_2[e12345]) * self.group0(),
            // e1234
            geometric_anti_product_2[e12345] * self[e1234],
        );
    }
}
