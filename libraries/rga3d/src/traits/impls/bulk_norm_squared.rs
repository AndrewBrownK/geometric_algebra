use crate::traits::DotProduct;
use crate::traits::FlatBulk;
use crate::traits::Wedge;
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
//  Minimum:         0       4       0
//   Median:         3       6       0
//  Average:         3       7       0
//  Maximum:         7      15       0
//
//  No SIMD:   add/sub     mul     div
//  Minimum:         0       9       0
//   Median:         3      12       0
//  Average:         3      14       0
//  Maximum:         7      26       0
impl std::ops::Div<bulk_norm_squared> for Flector {
    type Output = Scalar;
    fn div(self, _rhs: bulk_norm_squared) -> Self::Output {
        self.bulk_norm_squared()
    }
}
impl BulkNormSquared for Flector {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        4        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        3        6        0
    //  no simd        3       12        0
    fn bulk_norm_squared(self) -> Scalar {
        let flat_bulk_thing = self.flat_bulk().wedge(Origin::from_groups(/* e4 */ 1.0));
        return flat_bulk_thing.dot_product(flat_bulk_thing);
    }
}
impl std::ops::Div<bulk_norm_squared> for Line {
    type Output = Scalar;
    fn div(self, _rhs: bulk_norm_squared) -> Self::Output {
        self.bulk_norm_squared()
    }
}
impl BulkNormSquared for Line {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        2        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        0        4        0
    //  no simd        0       10        0
    fn bulk_norm_squared(self) -> Scalar {
        let flat_bulk_thing = self.flat_bulk().wedge(Origin::from_groups(/* e4 */ 1.0));
        return flat_bulk_thing.dot_product(flat_bulk_thing);
    }
}
impl std::ops::Div<bulk_norm_squared> for Motor {
    type Output = Scalar;
    fn div(self, _rhs: bulk_norm_squared) -> Self::Output {
        self.bulk_norm_squared()
    }
}
impl BulkNormSquared for Motor {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        5        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        3        7        0
    //  no simd        3       13        0
    fn bulk_norm_squared(self) -> Scalar {
        let flat_bulk_thing = self.flat_bulk().wedge(Origin::from_groups(/* e4 */ 1.0));
        return flat_bulk_thing.dot_product(flat_bulk_thing);
    }
}
impl std::ops::Div<bulk_norm_squared> for MultiVector {
    type Output = Scalar;
    fn div(self, _rhs: bulk_norm_squared) -> Self::Output {
        self.bulk_norm_squared()
    }
}
impl BulkNormSquared for MultiVector {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        7       10        0
    //    simd2        0        1        0
    //    simd3        0        2        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        7       15        0
    //  no simd        7       26        0
    fn bulk_norm_squared(self) -> Scalar {
        let flat_bulk_thing = self.flat_bulk().wedge(Origin::from_groups(/* e4 */ 1.0));
        return flat_bulk_thing.dot_product(flat_bulk_thing);
    }
}
impl std::ops::Div<bulk_norm_squared> for Point {
    type Output = Scalar;
    fn div(self, _rhs: bulk_norm_squared) -> Self::Output {
        self.bulk_norm_squared()
    }
}
impl BulkNormSquared for Point {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        3        0
    //    simd3        0        2        0
    // Totals...
    // yes simd        2        5        0
    //  no simd        2        9        0
    fn bulk_norm_squared(self) -> Scalar {
        let flat_bulk_thing = self.flat_bulk().wedge(Origin::from_groups(/* e4 */ 1.0));
        return flat_bulk_thing.dot_product(flat_bulk_thing);
    }
}
