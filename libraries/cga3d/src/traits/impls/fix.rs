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
impl std::ops::Div<FixPrefixOrPostfix> for AntiFlatPoint {
    type Output = AntiFlatPoint;
    fn div(self, _rhs: FixPrefixOrPostfix) -> Self::Output {
        self.fix()
    }
}
impl std::ops::DivAssign<FixPrefixOrPostfix> for AntiFlatPoint {
    fn div_assign(&mut self, _rhs: FixPrefixOrPostfix) {
        *self = self.fix()
    }
}
impl Fix for AntiFlatPoint {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        0        3        0
    //  no simd        0        9        0
    fn fix(self) -> Self {
        use crate::elements::*;
        return AntiFlatPoint::from_groups(
            // e235, e315, e125, e321
            Simd32x4::from(f32::powf((self.group0() * Simd32x4::from(-1.0))[3], -0.5) * f32::powf(self[e321], -0.5) * -1.0) * self.group0(),
        );
    }
}
impl std::ops::Div<FixPrefixOrPostfix> for AntiPlane {
    type Output = AntiPlane;
    fn div(self, _rhs: FixPrefixOrPostfix) -> Self::Output {
        self.fix()
    }
}
impl std::ops::DivAssign<FixPrefixOrPostfix> for AntiPlane {
    fn div_assign(&mut self, _rhs: FixPrefixOrPostfix) {
        *self = self.fix()
    }
}
impl Fix for AntiPlane {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        0        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        2        1        0
    //  no simd        2        4        0
    fn fix(self) -> Self {
        use crate::elements::*;
        return AntiPlane::from_groups(
            // e1, e2, e3, e5
            Simd32x4::from(f32::powi(self[e1], 2) + f32::powi(self[e2], 2) + f32::powi(self[e3], 2)) * self.group0(),
        );
    }
}
impl std::ops::Div<FixPrefixOrPostfix> for AntiScalar {
    type Output = AntiScalar;
    fn div(self, _rhs: FixPrefixOrPostfix) -> Self::Output {
        self.fix()
    }
}
impl std::ops::DivAssign<FixPrefixOrPostfix> for AntiScalar {
    fn div_assign(&mut self, _rhs: FixPrefixOrPostfix) {
        *self = self.fix()
    }
}
impl Fix for AntiScalar {
    fn fix(self) -> Self {
        return AntiScalar::from_groups(/* e12345 */ -1.0);
    }
}
impl std::ops::Div<FixPrefixOrPostfix> for FlatPoint {
    type Output = FlatPoint;
    fn div(self, _rhs: FixPrefixOrPostfix) -> Self::Output {
        self.fix()
    }
}
impl std::ops::DivAssign<FixPrefixOrPostfix> for FlatPoint {
    fn div_assign(&mut self, _rhs: FixPrefixOrPostfix) {
        *self = self.fix()
    }
}
impl Fix for FlatPoint {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        2        0
    // no simd        0        8        0
    fn fix(self) -> Self {
        use crate::elements::*;
        return FlatPoint::from_groups(
            // e15, e25, e35, e45
            Simd32x4::from(f32::powf((self.group0() * Simd32x4::from(-1.0))[3], -0.5) * f32::powf(self[e45], -0.5)) * self.group0(),
        );
    }
}
impl std::ops::Div<FixPrefixOrPostfix> for Plane {
    type Output = Plane;
    fn div(self, _rhs: FixPrefixOrPostfix) -> Self::Output {
        self.fix()
    }
}
impl std::ops::DivAssign<FixPrefixOrPostfix> for Plane {
    fn div_assign(&mut self, _rhs: FixPrefixOrPostfix) {
        *self = self.fix()
    }
}
impl Fix for Plane {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        0        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        2        1        0
    //  no simd        2        4        0
    fn fix(self) -> Self {
        use crate::elements::*;
        return Plane::from_groups(
            // e4235, e4315, e4125, e3215
            Simd32x4::from(-f32::powi(self[e4235], 2) - f32::powi(self[e4315], 2) - f32::powi(self[e4125], 2)) * self.group0(),
        );
    }
}
impl std::ops::Div<FixPrefixOrPostfix> for RoundPoint {
    type Output = RoundPoint;
    fn div(self, _rhs: FixPrefixOrPostfix) -> Self::Output {
        self.fix()
    }
}
impl std::ops::DivAssign<FixPrefixOrPostfix> for RoundPoint {
    fn div_assign(&mut self, _rhs: FixPrefixOrPostfix) {
        *self = self.fix()
    }
}
impl Fix for RoundPoint {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        3        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        3        4        0
    //  no simd        3        7        0
    fn fix(self) -> Self {
        use crate::elements::*;
        let geometric_product_2 = Scalar::from_groups(/* scalar */ f32::powi(self[e1], 2) + f32::powi(self[e2], 2) + f32::powi(self[e3], 2) - 2.0 * (self[e4] * self[e5]));
        return RoundPoint::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from(geometric_product_2[scalar]) * self.group0(),
            // e5
            self[e5] * geometric_product_2[scalar],
        );
    }
}
impl std::ops::Div<FixPrefixOrPostfix> for Scalar {
    type Output = Scalar;
    fn div(self, _rhs: FixPrefixOrPostfix) -> Self::Output {
        self.fix()
    }
}
impl std::ops::DivAssign<FixPrefixOrPostfix> for Scalar {
    fn div_assign(&mut self, _rhs: FixPrefixOrPostfix) {
        *self = self.fix()
    }
}
impl Fix for Scalar {
    fn fix(self) -> Self {
        return Scalar::from_groups(/* scalar */ 1.0);
    }
}
impl std::ops::Div<FixPrefixOrPostfix> for Sphere {
    type Output = Sphere;
    fn div(self, _rhs: FixPrefixOrPostfix) -> Self::Output {
        self.fix()
    }
}
impl std::ops::DivAssign<FixPrefixOrPostfix> for Sphere {
    fn div_assign(&mut self, _rhs: FixPrefixOrPostfix) {
        *self = self.fix()
    }
}
impl Fix for Sphere {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        3        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        3        4        0
    //  no simd        3        7        0
    fn fix(self) -> Self {
        use crate::elements::*;
        let geometric_product_2 = Scalar::from_groups(
            // scalar
            2.0 * (self[e3215] * self[e1234]) - f32::powi(self[e4235], 2) - f32::powi(self[e4315], 2) - f32::powi(self[e4125], 2),
        );
        return Sphere::from_groups(
            // e4235, e4315, e4125, e3215
            Simd32x4::from(geometric_product_2[scalar]) * self.group0(),
            // e1234
            geometric_product_2[scalar] * self[e1234],
        );
    }
}
