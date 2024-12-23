use crate::traits::FlatBulkNormSquared;
use crate::traits::FlatWeightNormSquared;
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
//  Minimum:         2       7       1
//   Median:         6      10       1
//  Average:         6      11       1
//  Maximum:        14      23       1
//
//  No SIMD:   add/sub     mul     div
//  Minimum:         2      11       1
//   Median:         6      16       1
//  Average:         6      18       1
//  Maximum:        14      34       1
impl std::ops::Div<unitized_flat_norm_squared> for Flector {
    type Output = f32;
    fn div(self, _rhs: unitized_flat_norm_squared) -> Self::Output {
        self.unitized_flat_norm_squared()
    }
}
impl UnitizedFlatNormSquared for Flector {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        6        8        1
    //    simd4        0        2        0
    // Totals...
    // yes simd        6       10        1
    //  no simd        6       16        1
    fn unitized_flat_norm_squared(self) -> f32 {
        use crate::elements::*;
        return self.flat_bulk_norm_squared()[scalar] / (self.flat_weight_norm_squared()[e1234]);
    }
}
impl std::ops::Div<unitized_flat_norm_squared> for Line {
    type Output = f32;
    fn div(self, _rhs: unitized_flat_norm_squared) -> Self::Output {
        self.unitized_flat_norm_squared()
    }
}
impl UnitizedFlatNormSquared for Line {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        5        1
    //    simd4        0        2        0
    // Totals...
    // yes simd        2        7        1
    //  no simd        2       13        1
    fn unitized_flat_norm_squared(self) -> f32 {
        use crate::elements::*;
        return self.flat_bulk_norm_squared()[scalar] / (self.flat_weight_norm_squared()[e1234]);
    }
}
impl std::ops::Div<unitized_flat_norm_squared> for Motor {
    type Output = f32;
    fn div(self, _rhs: unitized_flat_norm_squared) -> Self::Output {
        self.unitized_flat_norm_squared()
    }
}
impl UnitizedFlatNormSquared for Motor {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        6        9        1
    //    simd4        0        2        0
    // Totals...
    // yes simd        6       11        1
    //  no simd        6       17        1
    fn unitized_flat_norm_squared(self) -> f32 {
        use crate::elements::*;
        return self.flat_bulk_norm_squared()[scalar] / (self.flat_weight_norm_squared()[e1234]);
    }
}
impl std::ops::Div<unitized_flat_norm_squared> for MultiVector {
    type Output = f32;
    fn div(self, _rhs: unitized_flat_norm_squared) -> Self::Output {
        self.unitized_flat_norm_squared()
    }
}
impl UnitizedFlatNormSquared for MultiVector {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       14       18        1
    //    simd2        0        1        0
    //    simd3        0        2        0
    //    simd4        0        2        0
    // Totals...
    // yes simd       14       23        1
    //  no simd       14       34        1
    fn unitized_flat_norm_squared(self) -> f32 {
        use crate::elements::*;
        return self.flat_bulk_norm_squared()[scalar] / (self.flat_weight_norm_squared()[e1234]);
    }
}
impl std::ops::Div<unitized_flat_norm_squared> for Point {
    type Output = f32;
    fn div(self, _rhs: unitized_flat_norm_squared) -> Self::Output {
        self.unitized_flat_norm_squared()
    }
}
impl UnitizedFlatNormSquared for Point {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        5        1
    //    simd3        0        2        0
    // Totals...
    // yes simd        2        7        1
    //  no simd        2       11        1
    fn unitized_flat_norm_squared(self) -> f32 {
        use crate::elements::*;
        return self.flat_bulk_norm_squared()[scalar] / (self.flat_weight_norm_squared()[e1234]);
    }
}
