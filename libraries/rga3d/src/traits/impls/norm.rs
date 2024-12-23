use crate::traits::FlatBulkNorm;
use crate::traits::FlatWeightNorm;
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
//  Minimum:         2       7       0
//   Median:         6      10       0
//  Average:         6      11       0
//  Maximum:        14      23       0
//
//  No SIMD:   add/sub     mul     div
//  Minimum:         2      11       0
//   Median:         6      16       0
//  Average:         6      18       0
//  Maximum:        14      34       0
impl std::ops::Div<norm> for Flector {
    type Output = DualNum;
    fn div(self, _rhs: norm) -> Self::Output {
        self.norm()
    }
}
impl Norm for Flector {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        6        8        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        6       10        0
    //  no simd        6       16        0
    fn norm(self) -> DualNum {
        use crate::elements::*;
        return DualNum::from_groups(/* scalar, e1234 */ Simd32x2::from([self.flat_bulk_norm()[scalar], self.flat_weight_norm()[e1234]]));
    }
}
impl std::ops::Div<norm> for Line {
    type Output = DualNum;
    fn div(self, _rhs: norm) -> Self::Output {
        self.norm()
    }
}
impl Norm for Line {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        5        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        2        7        0
    //  no simd        2       13        0
    fn norm(self) -> DualNum {
        use crate::elements::*;
        return DualNum::from_groups(/* scalar, e1234 */ Simd32x2::from([self.flat_bulk_norm()[scalar], self.flat_weight_norm()[e1234]]));
    }
}
impl std::ops::Div<norm> for Motor {
    type Output = DualNum;
    fn div(self, _rhs: norm) -> Self::Output {
        self.norm()
    }
}
impl Norm for Motor {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        6        9        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        6       11        0
    //  no simd        6       17        0
    fn norm(self) -> DualNum {
        use crate::elements::*;
        return DualNum::from_groups(/* scalar, e1234 */ Simd32x2::from([self.flat_bulk_norm()[scalar], self.flat_weight_norm()[e1234]]));
    }
}
impl std::ops::Div<norm> for MultiVector {
    type Output = DualNum;
    fn div(self, _rhs: norm) -> Self::Output {
        self.norm()
    }
}
impl Norm for MultiVector {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       14       18        0
    //    simd2        0        1        0
    //    simd3        0        2        0
    //    simd4        0        2        0
    // Totals...
    // yes simd       14       23        0
    //  no simd       14       34        0
    fn norm(self) -> DualNum {
        use crate::elements::*;
        return DualNum::from_groups(/* scalar, e1234 */ Simd32x2::from([self.flat_bulk_norm()[scalar], self.flat_weight_norm()[e1234]]));
    }
}
impl std::ops::Div<norm> for Point {
    type Output = DualNum;
    fn div(self, _rhs: norm) -> Self::Output {
        self.norm()
    }
}
impl Norm for Point {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        5        0
    //    simd3        0        2        0
    // Totals...
    // yes simd        2        7        0
    //  no simd        2       11        0
    fn norm(self) -> DualNum {
        use crate::elements::*;
        return DualNum::from_groups(/* scalar, e1234 */ Simd32x2::from([self.flat_bulk_norm()[scalar], self.flat_weight_norm()[e1234]]));
    }
}
