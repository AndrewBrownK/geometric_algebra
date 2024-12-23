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
//  Minimum:         0       2       1
//   Median:         2       4       1
//  Average:         1       3       1
//  Maximum:         3       5       1
//
//  No SIMD:   add/sub     mul     div
//  Minimum:         0       2       1
//   Median:         2       8       1
//  Average:         1       6       1
//  Maximum:         3      11       1
impl std::ops::Div<anti_fix> for AntiFlatPoint {
    type Output = AntiFlatPoint;
    fn div(self, _rhs: anti_fix) -> Self::Output {
        self.anti_fix()
    }
}
impl std::ops::DivAssign<anti_fix> for AntiFlatPoint {
    fn div_assign(&mut self, _rhs: anti_fix) {
        *self = self.anti_fix()
    }
}
impl AntiFix for AntiFlatPoint {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        2        1
    //    simd4        0        2        0
    // Totals...
    // yes simd        0        4        1
    //  no simd        0       10        1
    fn anti_fix(self) -> Self {
        use crate::elements::*;
        let anti_reverse = AntiFlatPoint::from_groups(/* e235, e315, e125, e321 */ self.group0() * Simd32x4::from(-1.0));
        let geometric_anti_product = AntiScalar::from_groups(/* e12345 */ anti_reverse[e321] * self[e321]);
        let anti_square_root = AntiScalar::from_groups(/* e12345 */ f32::powf(geometric_anti_product[e12345], 0.5));
        let anti_dot_product = AntiScalar::from_groups(/* e12345 */ f32::powi(anti_square_root[e12345], 2));
        let other = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        let geometric_anti_product_2 = AntiScalar::from_groups(/* e12345 */ anti_square_root[e12345] * other[e12345]);
        return AntiFlatPoint::from_groups(/* e235, e315, e125, e321 */ Simd32x4::from(geometric_anti_product_2[e12345]) * self.group0());
    }
}
impl std::ops::Div<anti_fix> for AntiPlane {
    type Output = AntiPlane;
    fn div(self, _rhs: anti_fix) -> Self::Output {
        self.anti_fix()
    }
}
impl std::ops::DivAssign<anti_fix> for AntiPlane {
    fn div_assign(&mut self, _rhs: anti_fix) {
        *self = self.anti_fix()
    }
}
impl AntiFix for AntiPlane {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        1        1
    //    simd4        0        1        0
    // Totals...
    // yes simd        2        2        1
    //  no simd        2        5        1
    fn anti_fix(self) -> Self {
        use crate::elements::*;
        let geometric_anti_product = AntiScalar::from_groups(/* e12345 */ -f32::powi(self[e1], 2) - f32::powi(self[e2], 2) - f32::powi(self[e3], 2));
        let anti_square_root = AntiScalar::from_groups(/* e12345 */ f32::powf(geometric_anti_product[e12345], 0.5));
        let anti_dot_product = AntiScalar::from_groups(/* e12345 */ f32::powi(anti_square_root[e12345], 2));
        let other = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        let geometric_anti_product_2 = AntiScalar::from_groups(/* e12345 */ anti_square_root[e12345] * other[e12345]);
        return AntiPlane::from_groups(/* e1, e2, e3, e5 */ Simd32x4::from(geometric_anti_product_2[e12345]) * self.group0());
    }
}
impl std::ops::Div<anti_fix> for AntiScalar {
    type Output = AntiScalar;
    fn div(self, _rhs: anti_fix) -> Self::Output {
        self.anti_fix()
    }
}
impl std::ops::DivAssign<anti_fix> for AntiScalar {
    fn div_assign(&mut self, _rhs: anti_fix) {
        *self = self.anti_fix()
    }
}
impl AntiFix for AntiScalar {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        2        1
    fn anti_fix(self) -> Self {
        use crate::elements::*;
        let geometric_anti_product = AntiScalar::from_groups(/* e12345 */ f32::powi(self[e12345], 2));
        let anti_square_root = AntiScalar::from_groups(/* e12345 */ f32::powf(geometric_anti_product[e12345], 0.5));
        let anti_dot_product = AntiScalar::from_groups(/* e12345 */ f32::powi(anti_square_root[e12345], 2));
        let other = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        let geometric_anti_product_2 = AntiScalar::from_groups(/* e12345 */ anti_square_root[e12345] * other[e12345]);
        return AntiScalar::from_groups(/* e12345 */ geometric_anti_product_2[e12345] * self[e12345]);
    }
}
impl std::ops::Div<anti_fix> for FlatPoint {
    type Output = FlatPoint;
    fn div(self, _rhs: anti_fix) -> Self::Output {
        self.anti_fix()
    }
}
impl std::ops::DivAssign<anti_fix> for FlatPoint {
    fn div_assign(&mut self, _rhs: anti_fix) {
        *self = self.anti_fix()
    }
}
impl AntiFix for FlatPoint {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        3        1
    //    simd4        0        2        0
    // Totals...
    // yes simd        0        5        1
    //  no simd        0       11        1
    fn anti_fix(self) -> Self {
        use crate::elements::*;
        let anti_reverse = FlatPoint::from_groups(/* e15, e25, e35, e45 */ self.group0() * Simd32x4::from(-1.0));
        let geometric_anti_product = AntiScalar::from_groups(/* e12345 */ anti_reverse[e45] * self[e45] * -1.0);
        let anti_square_root = AntiScalar::from_groups(/* e12345 */ f32::powf(geometric_anti_product[e12345], 0.5));
        let anti_dot_product = AntiScalar::from_groups(/* e12345 */ f32::powi(anti_square_root[e12345], 2));
        let other = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        let geometric_anti_product_2 = AntiScalar::from_groups(/* e12345 */ anti_square_root[e12345] * other[e12345]);
        return FlatPoint::from_groups(/* e15, e25, e35, e45 */ Simd32x4::from(geometric_anti_product_2[e12345]) * self.group0());
    }
}
impl std::ops::Div<anti_fix> for Plane {
    type Output = Plane;
    fn div(self, _rhs: anti_fix) -> Self::Output {
        self.anti_fix()
    }
}
impl std::ops::DivAssign<anti_fix> for Plane {
    fn div_assign(&mut self, _rhs: anti_fix) {
        *self = self.anti_fix()
    }
}
impl AntiFix for Plane {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        1        1
    //    simd4        0        1        0
    // Totals...
    // yes simd        2        2        1
    //  no simd        2        5        1
    fn anti_fix(self) -> Self {
        use crate::elements::*;
        let geometric_anti_product = AntiScalar::from_groups(/* e12345 */ f32::powi(self[e4235], 2) + f32::powi(self[e4315], 2) + f32::powi(self[e4125], 2));
        let anti_square_root = AntiScalar::from_groups(/* e12345 */ f32::powf(geometric_anti_product[e12345], 0.5));
        let anti_dot_product = AntiScalar::from_groups(/* e12345 */ f32::powi(anti_square_root[e12345], 2));
        let other = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        let geometric_anti_product_2 = AntiScalar::from_groups(/* e12345 */ anti_square_root[e12345] * other[e12345]);
        return Plane::from_groups(/* e4235, e4315, e4125, e3215 */ Simd32x4::from(geometric_anti_product_2[e12345]) * self.group0());
    }
}
impl std::ops::Div<anti_fix> for RoundPoint {
    type Output = RoundPoint;
    fn div(self, _rhs: anti_fix) -> Self::Output {
        self.anti_fix()
    }
}
impl std::ops::DivAssign<anti_fix> for RoundPoint {
    fn div_assign(&mut self, _rhs: anti_fix) {
        *self = self.anti_fix()
    }
}
impl AntiFix for RoundPoint {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        4        1
    //    simd4        0        1        0
    // Totals...
    // yes simd        3        5        1
    //  no simd        3        8        1
    fn anti_fix(self) -> Self {
        use crate::elements::*;
        let geometric_anti_product =
            AntiScalar::from_groups(/* e12345 */ 2.0 * (self[e4] * self[e5]) - f32::powi(self[e1], 2) - f32::powi(self[e2], 2) - f32::powi(self[e3], 2));
        let anti_square_root = AntiScalar::from_groups(/* e12345 */ f32::powf(geometric_anti_product[e12345], 0.5));
        let anti_dot_product = AntiScalar::from_groups(/* e12345 */ f32::powi(anti_square_root[e12345], 2));
        let other = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        let geometric_anti_product_2 = AntiScalar::from_groups(/* e12345 */ anti_square_root[e12345] * other[e12345]);
        return RoundPoint::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from(geometric_anti_product_2[e12345]) * self.group0(),
            // e5
            geometric_anti_product_2[e12345] * self[e5],
        );
    }
}
impl std::ops::Div<anti_fix> for Scalar {
    type Output = Scalar;
    fn div(self, _rhs: anti_fix) -> Self::Output {
        self.anti_fix()
    }
}
impl std::ops::DivAssign<anti_fix> for Scalar {
    fn div_assign(&mut self, _rhs: anti_fix) {
        *self = self.anti_fix()
    }
}
impl AntiFix for Scalar {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        3        1
    fn anti_fix(self) -> Self {
        use crate::elements::*;
        let geometric_anti_product = AntiScalar::from_groups(/* e12345 */ f32::powi(self[scalar], 2) * -1.0);
        let anti_square_root = AntiScalar::from_groups(/* e12345 */ f32::powf(geometric_anti_product[e12345], 0.5));
        let anti_dot_product = AntiScalar::from_groups(/* e12345 */ f32::powi(anti_square_root[e12345], 2));
        let other = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        let geometric_anti_product_2 = AntiScalar::from_groups(/* e12345 */ anti_square_root[e12345] * other[e12345]);
        return Scalar::from_groups(/* scalar */ geometric_anti_product_2[e12345] * self[scalar]);
    }
}
impl std::ops::Div<anti_fix> for Sphere {
    type Output = Sphere;
    fn div(self, _rhs: anti_fix) -> Self::Output {
        self.anti_fix()
    }
}
impl std::ops::DivAssign<anti_fix> for Sphere {
    fn div_assign(&mut self, _rhs: anti_fix) {
        *self = self.anti_fix()
    }
}
impl AntiFix for Sphere {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        4        1
    //    simd4        0        1        0
    // Totals...
    // yes simd        3        5        1
    //  no simd        3        8        1
    fn anti_fix(self) -> Self {
        use crate::elements::*;
        let geometric_anti_product = AntiScalar::from_groups(
            // e12345
            f32::powi(self[e4235], 2) + f32::powi(self[e4315], 2) + f32::powi(self[e4125], 2) - 2.0 * (self[e3215] * self[e1234]),
        );
        let anti_square_root = AntiScalar::from_groups(/* e12345 */ f32::powf(geometric_anti_product[e12345], 0.5));
        let anti_dot_product = AntiScalar::from_groups(/* e12345 */ f32::powi(anti_square_root[e12345], 2));
        let other = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        let geometric_anti_product_2 = AntiScalar::from_groups(/* e12345 */ anti_square_root[e12345] * other[e12345]);
        return Sphere::from_groups(
            // e4235, e4315, e4125, e3215
            Simd32x4::from(geometric_anti_product_2[e12345]) * self.group0(),
            // e1234
            geometric_anti_product_2[e12345] * self[e1234],
        );
    }
}
