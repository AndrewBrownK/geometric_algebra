use crate::traits::FlatBulkNormSquared;
use crate::traits::SquareRoot;
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
//  Minimum:         0       2       0
//   Median:         3       6       0
//  Average:         3       6       0
//  Maximum:         7      14       0
//
//  No SIMD:   add/sub     mul     div
//  Minimum:         0       4       0
//   Median:         3       9       0
//  Average:         3      10       0
//  Maximum:         7      21       0
impl std::ops::Div<BulkNormPrefixOrPostfix> for Flector {
    type Output = Scalar;
    fn div(self, _rhs: BulkNormPrefixOrPostfix) -> Self::Output {
        self.bulk_norm()
    }
}
impl BulkNorm for Flector {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        4        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        3        6        0
    //  no simd        3       12        0
    fn bulk_norm(self) -> Scalar {
        return self.flat_bulk_norm_squared().square_root();
    }
}
impl std::ops::Div<BulkNormPrefixOrPostfix> for Line {
    type Output = Scalar;
    fn div(self, _rhs: BulkNormPrefixOrPostfix) -> Self::Output {
        self.bulk_norm()
    }
}
impl BulkNorm for Line {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd3        0        1        0
    // Totals...
    // yes simd        0        2        0
    //  no simd        0        4        0
    fn bulk_norm(self) -> Scalar {
        return self.flat_bulk_norm_squared().square_root();
    }
}
impl std::ops::Div<BulkNormPrefixOrPostfix> for Motor {
    type Output = Scalar;
    fn div(self, _rhs: BulkNormPrefixOrPostfix) -> Self::Output {
        self.bulk_norm()
    }
}
impl BulkNorm for Motor {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        5        0
    //    simd3        0        1        0
    // Totals...
    // yes simd        3        6        0
    //  no simd        3        8        0
    fn bulk_norm(self) -> Scalar {
        return self.flat_bulk_norm_squared().square_root();
    }
}
impl std::ops::Div<BulkNormPrefixOrPostfix> for MultiVector {
    type Output = Scalar;
    fn div(self, _rhs: BulkNormPrefixOrPostfix) -> Self::Output {
        self.bulk_norm()
    }
}
impl BulkNorm for MultiVector {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        7       10        0
    //    simd2        0        1        0
    //    simd3        0        3        0
    // Totals...
    // yes simd        7       14        0
    //  no simd        7       21        0
    fn bulk_norm(self) -> Scalar {
        return self.flat_bulk_norm_squared().square_root();
    }
}
impl std::ops::Div<BulkNormPrefixOrPostfix> for Point {
    type Output = Scalar;
    fn div(self, _rhs: BulkNormPrefixOrPostfix) -> Self::Output {
        self.bulk_norm()
    }
}
impl BulkNorm for Point {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        3        0
    //    simd3        0        2        0
    // Totals...
    // yes simd        2        5        0
    //  no simd        2        9        0
    fn bulk_norm(self) -> Scalar {
        return self.flat_bulk_norm_squared().square_root();
    }
}
