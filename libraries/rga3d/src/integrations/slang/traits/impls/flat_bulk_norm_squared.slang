using traits::DotProduct;
using traits::FlatBulk;
using traits::Wedge;
impl std::ops::Div<flat_bulk_norm_squared> for Flector {
    type Output = Scalar;
    fn div(self, _rhs: flat_bulk_norm_squared) -> Self::Output {
        self.flat_bulk_norm_squared()
    }
}
impl FlatBulkNormSquared for Flector {
// Operative Statistics for this implementation:
//           add/sub      mul      div
//      f32        3        4        0
//    simd4        0        2        0
// Totals...
// yes simd        3        6        0
//  no simd        3       12        0
    fn flat_bulk_norm_squared(self) -> Scalar {
let flat_bulk_thing = self.flat_bulk().wedge(Origin::from_groups(/* e4 */1.0));
        return flat_bulk_thing.dot_product(flat_bulk_thing);
    }
}
impl std::ops::Div<flat_bulk_norm_squared> for Line {
    type Output = Scalar;
    fn div(self, _rhs: flat_bulk_norm_squared) -> Self::Output {
        self.flat_bulk_norm_squared()
    }
}
impl FlatBulkNormSquared for Line {
// Operative Statistics for this implementation:
//           add/sub      mul      div
//      f32        0        1        0
//    simd4        0        2        0
// Totals...
// yes simd        0        3        0
//  no simd        0        9        0
    fn flat_bulk_norm_squared(self) -> Scalar {
let flat_bulk_thing = self.flat_bulk().wedge(Origin::from_groups(/* e4 */1.0));
        return flat_bulk_thing.dot_product(flat_bulk_thing);
    }
}
impl std::ops::Div<flat_bulk_norm_squared> for Motor {
    type Output = Scalar;
    fn div(self, _rhs: flat_bulk_norm_squared) -> Self::Output {
        self.flat_bulk_norm_squared()
    }
}
impl FlatBulkNormSquared for Motor {
// Operative Statistics for this implementation:
//           add/sub      mul      div
//      f32        3        5        0
//    simd4        0        2        0
// Totals...
// yes simd        3        7        0
//  no simd        3       13        0
    fn flat_bulk_norm_squared(self) -> Scalar {
let flat_bulk_thing = self.flat_bulk().wedge(Origin::from_groups(/* e4 */1.0));
        return flat_bulk_thing.dot_product(flat_bulk_thing);
    }
}
impl std::ops::Div<flat_bulk_norm_squared> for MultiVector {
    type Output = Scalar;
    fn div(self, _rhs: flat_bulk_norm_squared) -> Self::Output {
        self.flat_bulk_norm_squared()
    }
}
impl FlatBulkNormSquared for MultiVector {
// Operative Statistics for this implementation:
//           add/sub      mul      div
//      f32        7       10        0
//    simd2        0        1        0
//    simd3        0        2        0
//    simd4        0        2        0
// Totals...
// yes simd        7       15        0
//  no simd        7       26        0
    fn flat_bulk_norm_squared(self) -> Scalar {
let flat_bulk_thing = self.flat_bulk().wedge(Origin::from_groups(/* e4 */1.0));
        return flat_bulk_thing.dot_product(flat_bulk_thing);
    }
}
impl std::ops::Div<flat_bulk_norm_squared> for Point {
    type Output = Scalar;
    fn div(self, _rhs: flat_bulk_norm_squared) -> Self::Output {
        self.flat_bulk_norm_squared()
    }
}
impl FlatBulkNormSquared for Point {
// Operative Statistics for this implementation:
//           add/sub      mul      div
//      f32        2        3        0
//    simd3        0        2        0
// Totals...
// yes simd        2        5        0
//  no simd        2        9        0
    fn flat_bulk_norm_squared(self) -> Scalar {
let flat_bulk_thing = self.flat_bulk().wedge(Origin::from_groups(/* e4 */1.0));
        return flat_bulk_thing.dot_product(flat_bulk_thing);
    }
}
