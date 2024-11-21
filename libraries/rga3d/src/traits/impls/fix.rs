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
impl std::ops::Div<fix> for Horizon {
    type Output = Horizon;
    fn div(self, _rhs: fix) -> Self::Output {
        self.fix()
    }
}
impl std::ops::DivAssign<fix> for Horizon {
    fn div_assign(&mut self, _rhs: fix) {
        *self = self.fix()
    }
}
impl Fix for Horizon {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        4        1
    fn fix(self) -> Self {
        use crate::elements::*;
        let reverse = Horizon::from_groups(/* e321 */ self[e321] * -1.0);
        let geometric_product = Scalar::from_groups(/* scalar */ reverse[e321] * self[e321] * -1.0);
        let square_root = Scalar::from_groups(/* scalar */ f32::powf(geometric_product[scalar], 0.5));
        let scalar_product = Scalar::from_groups(/* scalar */ f32::powi(square_root[scalar], 2));
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / scalar_product[scalar]);
        return Horizon::from_groups(/* e321 */ self[e321] * inverse[scalar]);
    }
}
impl std::ops::Div<fix> for Plane {
    type Output = Plane;
    fn div(self, _rhs: fix) -> Self::Output {
        self.fix()
    }
}
impl std::ops::DivAssign<fix> for Plane {
    fn div_assign(&mut self, _rhs: fix) {
        *self = self.fix()
    }
}
impl Fix for Plane {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        2        1
    //    simd4        0        2        0
    // Totals...
    // yes simd        0        4        1
    //  no simd        0       10        1
    fn fix(self) -> Self {
        use crate::elements::*;
        let reverse = Plane::from_groups(
            // e423, e431, e412, e321
            Simd32x4::from([self[e423], self[e431], self[e412], self[e321]]) * Simd32x4::from(-1.0),
        );
        let geometric_product = Scalar::from_groups(/* scalar */ reverse[e321] * self[e321] * -1.0);
        let square_root = Scalar::from_groups(/* scalar */ f32::powf(geometric_product[scalar], 0.5));
        let scalar_product = Scalar::from_groups(/* scalar */ f32::powi(square_root[scalar], 2));
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / scalar_product[scalar]);
        return Plane::from_groups(
            // e423, e431, e412, e321
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e423], self[e431], self[e412], self[e321]]),
        );
    }
}
impl std::ops::Div<fix> for Point {
    type Output = Point;
    fn div(self, _rhs: fix) -> Self::Output {
        self.fix()
    }
}
impl std::ops::DivAssign<fix> for Point {
    fn div_assign(&mut self, _rhs: fix) {
        *self = self.fix()
    }
}
impl Fix for Point {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        0        1
    //    simd4        0        1        0
    // Totals...
    // yes simd        2        1        1
    //  no simd        2        4        1
    fn fix(self) -> Self {
        use crate::elements::*;
        let geometric_product = Scalar::from_groups(/* scalar */ f32::powi(self[e1], 2) + f32::powi(self[e2], 2) + f32::powi(self[e3], 2));
        let square_root = Scalar::from_groups(/* scalar */ f32::powf(geometric_product[scalar], 0.5));
        let scalar_product = Scalar::from_groups(/* scalar */ f32::powi(square_root[scalar], 2));
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / scalar_product[scalar]);
        return Point::from_groups(/* e1, e2, e3, e4 */ Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e1], self[e2], self[e3], self[e4]]));
    }
}
impl std::ops::Div<fix> for Scalar {
    type Output = Scalar;
    fn div(self, _rhs: fix) -> Self::Output {
        self.fix()
    }
}
impl std::ops::DivAssign<fix> for Scalar {
    fn div_assign(&mut self, _rhs: fix) {
        *self = self.fix()
    }
}
impl Fix for Scalar {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        1
    fn fix(self) -> Self {
        use crate::elements::*;
        let geometric_product = Scalar::from_groups(/* scalar */ f32::powi(self[scalar], 2));
        let square_root = Scalar::from_groups(/* scalar */ f32::powf(geometric_product[scalar], 0.5));
        let scalar_product = Scalar::from_groups(/* scalar */ f32::powi(square_root[scalar], 2));
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / scalar_product[scalar]);
        return Scalar::from_groups(/* scalar */ inverse[scalar] * self[scalar]);
    }
}
