// Note on Operative Statistics:
// Operative Statistics are not a precise predictor of performance or performance comparisons.
// This is due to varying hardware capabilities and compiler optimizations.
// As always, where performance is a concern, there is no substitute for
// real measurements on real work-loads on real hardware.
// Disclaimer aside, enjoy the fun information =)
//
// Total Implementations: 4
//
// Yes SIMD:   add/sub     mul     div
//  Minimum:         0       1       1
//   Median:         0       4       1
//  Average:         0       2       1
//  Maximum:         2       4       1
//
//  No SIMD:   add/sub     mul     div
//  Minimum:         0       1       1
//   Median:         0       4       1
//  Average:         0       4       1
//  Maximum:         2      10       1
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
    // f32        0        1        1
    fn anti_fix(self) -> Self {
        use crate::elements::*;
        let geometric_anti_product = AntiScalar::from_groups(/* e1234 */ f32::powi(self[e1234], 2));
        let anti_square_root = AntiScalar::from_groups(/* e1234 */ f32::powf(geometric_anti_product[e1234], 0.5));
        let anti_scalar_product = AntiScalar::from_groups(/* e1234 */ f32::powi(anti_square_root[e1234], 2));
        let anti_inverse = AntiScalar::from_groups(/* e1234 */ 1.0 / anti_scalar_product[e1234]);
        return AntiScalar::from_groups(/* e1234 */ anti_inverse[e1234] * self[e1234]);
    }
}
impl std::ops::Div<anti_fix> for Origin {
    type Output = Origin;
    fn div(self, _rhs: anti_fix) -> Self::Output {
        self.anti_fix()
    }
}
impl std::ops::DivAssign<anti_fix> for Origin {
    fn div_assign(&mut self, _rhs: anti_fix) {
        *self = self.anti_fix()
    }
}
impl AntiFix for Origin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        4        1
    fn anti_fix(self) -> Self {
        use crate::elements::*;
        let anti_reverse = Origin::from_groups(/* e4 */ self[e4] * -1.0);
        let geometric_anti_product = AntiScalar::from_groups(/* e1234 */ anti_reverse[e4] * self[e4] * -1.0);
        let anti_square_root = AntiScalar::from_groups(/* e1234 */ f32::powf(geometric_anti_product[e1234], 0.5));
        let anti_scalar_product = AntiScalar::from_groups(/* e1234 */ f32::powi(anti_square_root[e1234], 2));
        let anti_inverse = AntiScalar::from_groups(/* e1234 */ 1.0 / anti_scalar_product[e1234]);
        return Origin::from_groups(/* e4 */ anti_inverse[e1234] * self[e4]);
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
    //      f32        2        0        1
    //    simd4        0        1        0
    // Totals...
    // yes simd        2        1        1
    //  no simd        2        4        1
    fn anti_fix(self) -> Self {
        use crate::elements::*;
        let geometric_anti_product = AntiScalar::from_groups(/* e1234 */ f32::powi(self[e423], 2) + f32::powi(self[e431], 2) + f32::powi(self[e412], 2));
        let anti_square_root = AntiScalar::from_groups(/* e1234 */ f32::powf(geometric_anti_product[e1234], 0.5));
        let anti_scalar_product = AntiScalar::from_groups(/* e1234 */ f32::powi(anti_square_root[e1234], 2));
        let anti_inverse = AntiScalar::from_groups(/* e1234 */ 1.0 / anti_scalar_product[e1234]);
        return Plane::from_groups(
            // e423, e431, e412, e321
            Simd32x4::from(anti_inverse[e1234]) * Simd32x4::from([self[e423], self[e431], self[e412], self[e321]]),
        );
    }
}
impl std::ops::Div<anti_fix> for Point {
    type Output = Point;
    fn div(self, _rhs: anti_fix) -> Self::Output {
        self.anti_fix()
    }
}
impl std::ops::DivAssign<anti_fix> for Point {
    fn div_assign(&mut self, _rhs: anti_fix) {
        *self = self.anti_fix()
    }
}
impl AntiFix for Point {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        2        1
    //    simd4        0        2        0
    // Totals...
    // yes simd        0        4        1
    //  no simd        0       10        1
    fn anti_fix(self) -> Self {
        use crate::elements::*;
        let anti_reverse = Point::from_groups(/* e1, e2, e3, e4 */ Simd32x4::from([self[e1], self[e2], self[e3], self[e4]]) * Simd32x4::from(-1.0));
        let geometric_anti_product = AntiScalar::from_groups(/* e1234 */ anti_reverse[e4] * self[e4] * -1.0);
        let anti_square_root = AntiScalar::from_groups(/* e1234 */ f32::powf(geometric_anti_product[e1234], 0.5));
        let anti_scalar_product = AntiScalar::from_groups(/* e1234 */ f32::powi(anti_square_root[e1234], 2));
        let anti_inverse = AntiScalar::from_groups(/* e1234 */ 1.0 / anti_scalar_product[e1234]);
        return Point::from_groups(/* e1, e2, e3, e4 */ Simd32x4::from(anti_inverse[e1234]) * Simd32x4::from([self[e1], self[e2], self[e3], self[e4]]));
    }
}
