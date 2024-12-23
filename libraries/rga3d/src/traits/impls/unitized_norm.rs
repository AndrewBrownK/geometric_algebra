use crate::traits::UnitizedFlatNormSquared;
// Note on Operative Statistics:
// Operative Statistics are not a precise predictor of performance or performance comparisons.
// This is due to varying hardware capabilities and compiler optimizations.
// As always, where performance is a concern, there is no substitute for
// real measurements on real work-loads on real hardware.
// Disclaimer aside, enjoy the fun information =)
//
// Total Implementations: 5
//
// Yes SIMD:   add/sub     mul     div
//  Minimum:         2       6       1
//   Median:         6      10       1
//  Average:         6      11       1
//  Maximum:        14      23       1
//
//  No SIMD:   add/sub     mul     div
//  Minimum:         2      10       1
//   Median:         6      16       1
//  Average:         6      17       1
//  Maximum:        14      34       1
impl std::ops::Div<unitized_norm> for Flector {
    type Output = f32;
    fn div(self, _rhs: unitized_norm) -> Self::Output {
        self.unitized_norm()
    }
}
impl UnitizedNorm for Flector {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        6        8        1
    //    simd4        0        2        0
    // Totals...
    // yes simd        6       10        1
    //  no simd        6       16        1
    fn unitized_norm(self) -> f32 {
        return f32::powf(self.unitized_flat_norm_squared(), 0.5);
    }
}
impl std::ops::Div<unitized_norm> for Line {
    type Output = f32;
    fn div(self, _rhs: unitized_norm) -> Self::Output {
        self.unitized_norm()
    }
}
impl UnitizedNorm for Line {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        4        1
    //    simd4        0        2        0
    // Totals...
    // yes simd        2        6        1
    //  no simd        2       12        1
    fn unitized_norm(self) -> f32 {
        return f32::powf(self.unitized_flat_norm_squared(), 0.5);
    }
}
impl std::ops::Div<unitized_norm> for Motor {
    type Output = f32;
    fn div(self, _rhs: unitized_norm) -> Self::Output {
        self.unitized_norm()
    }
}
impl UnitizedNorm for Motor {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        6        9        1
    //    simd4        0        2        0
    // Totals...
    // yes simd        6       11        1
    //  no simd        6       17        1
    fn unitized_norm(self) -> f32 {
        return f32::powf(self.unitized_flat_norm_squared(), 0.5);
    }
}
impl std::ops::Div<unitized_norm> for MultiVector {
    type Output = f32;
    fn div(self, _rhs: unitized_norm) -> Self::Output {
        self.unitized_norm()
    }
}
impl UnitizedNorm for MultiVector {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       14       18        1
    //    simd2        0        1        0
    //    simd3        0        2        0
    //    simd4        0        2        0
    // Totals...
    // yes simd       14       23        1
    //  no simd       14       34        1
    fn unitized_norm(self) -> f32 {
        return f32::powf(self.unitized_flat_norm_squared(), 0.5);
    }
}
impl std::ops::Div<unitized_norm> for Point {
    type Output = f32;
    fn div(self, _rhs: unitized_norm) -> Self::Output {
        self.unitized_norm()
    }
}
impl UnitizedNorm for Point {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        4        1
    //    simd3        0        2        0
    // Totals...
    // yes simd        2        6        1
    //  no simd        2       10        1
    fn unitized_norm(self) -> f32 {
        return f32::powf(self.unitized_flat_norm_squared(), 0.5);
    }
}
