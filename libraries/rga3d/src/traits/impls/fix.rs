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
//  Minimum:         0       0       0
//   Median:         0       2       0
//  Average:         0       2       0
//  Maximum:         2       4       0
//
//  No SIMD:   add/sub     mul     div
//  Minimum:         0       0       0
//   Median:         0       5       0
//  Average:         0       4       0
//  Maximum:         2      10       0
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
    // f32        0        2        0
    fn fix(self) -> Self {
        use crate::elements::*;
        let square_root = Scalar::from_groups(/* scalar */ self[e321]);
        let other = Scalar::from_groups(/* scalar */ f32::powi(square_root[scalar], -2));
        return Horizon::from_groups(/* e321 */ self[e321] * other[scalar] * square_root[scalar]);
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
    //      f32        0        2        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        0        4        0
    //  no simd        0       10        0
    fn fix(self) -> Self {
        use crate::elements::*;
        let square_root = Scalar::from_groups(/* scalar */ f32::powf((self.group0() * Simd32x4::from(-1.0))[3], 0.5) * f32::powf(self[e321], 0.5) * -1.0);
        let other = Scalar::from_groups(/* scalar */ f32::powi(square_root[scalar], -2));
        return Plane::from_groups(/* e423, e431, e412, e321 */ Simd32x4::from(other[scalar] * square_root[scalar]) * self.group0());
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
    //      f32        2        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        2        2        0
    //  no simd        2        5        0
    fn fix(self) -> Self {
        use crate::elements::*;
        let square_root = Scalar::from_groups(/* scalar */ f32::powi(self[e1], 2) + f32::powi(self[e2], 2) + f32::powi(self[e3], 2));
        let other = Scalar::from_groups(/* scalar */ f32::powi(square_root[scalar], -2));
        return Point::from_groups(/* e1, e2, e3, e4 */ Simd32x4::from(other[scalar] * square_root[scalar]) * self.group0());
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
    fn fix(self) -> Self {
        use crate::elements::*;
        let other = Scalar::from_groups(/* scalar */ f32::powi(self[scalar], -2));
        return Scalar::from_groups(/* scalar */ other[scalar] * f32::powi(self[scalar], 2));
    }
}
