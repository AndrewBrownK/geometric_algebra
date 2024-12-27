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
//   Median:         0       1       0
//  Average:         0       1       0
//  Maximum:         2       3       0
//
//  No SIMD:   add/sub     mul     div
//  Minimum:         0       0       0
//   Median:         0       4       0
//  Average:         0       3       0
//  Maximum:         2       9       0
impl std::ops::Div<FixPrefixOrPostfix> for Horizon {
    type Output = Horizon;
    fn div(self, _rhs: FixPrefixOrPostfix) -> Self::Output {
        self.fix()
    }
}
impl std::ops::DivAssign<FixPrefixOrPostfix> for Horizon {
    fn div_assign(&mut self, _rhs: FixPrefixOrPostfix) {
        *self = self.fix()
    }
}
impl Fix for Horizon {
    fn fix(self) -> Self {
        return Horizon::from_groups(/* e321 */ 1.0);
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
    //      f32        0        1        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        0        3        0
    //  no simd        0        9        0
    fn fix(self) -> Self {
        use crate::elements::*;
        return Plane::from_groups(
            // e423, e431, e412, e321
            Simd32x4::from(f32::powf((self.group0() * Simd32x4::from(-1.0))[3], -0.5) * f32::powf(self[e321], -0.5) * -1.0) * self.group0(),
        );
    }
}
impl std::ops::Div<FixPrefixOrPostfix> for Point {
    type Output = Point;
    fn div(self, _rhs: FixPrefixOrPostfix) -> Self::Output {
        self.fix()
    }
}
impl std::ops::DivAssign<FixPrefixOrPostfix> for Point {
    fn div_assign(&mut self, _rhs: FixPrefixOrPostfix) {
        *self = self.fix()
    }
}
impl Fix for Point {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        0        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        2        1        0
    //  no simd        2        4        0
    fn fix(self) -> Self {
        use crate::elements::*;
        return Point::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from(f32::powi(self[e1], 2) + f32::powi(self[e2], 2) + f32::powi(self[e3], 2)) * self.group0(),
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
