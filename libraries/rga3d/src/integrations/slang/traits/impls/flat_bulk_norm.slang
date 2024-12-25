using traits::FlatBulkNormSquared;
using traits::SquareRoot;
impl std::ops::Div<flat_bulk_norm> for Flector {
    type Output = Scalar;
    fn div(self, _rhs: flat_bulk_norm) -> Self::Output {
        self.flat_bulk_norm()
    }
}
impl FlatBulkNorm for Flector {
// Operative Statistics for this implementation:
//           add/sub      mul      div
//      f32        3        4        0
//    simd4        0        2        0
// Totals...
// yes simd        3        6        0
//  no simd        3       12        0
    fn flat_bulk_norm(self) -> Scalar {
        return self.flat_bulk_norm_squared().square_root();
    }
}
impl std::ops::Div<flat_bulk_norm> for Line {
    type Output = Scalar;
    fn div(self, _rhs: flat_bulk_norm) -> Self::Output {
        self.flat_bulk_norm()
    }
}
impl FlatBulkNorm for Line {
// Operative Statistics for this implementation:
//           add/sub      mul      div
//      f32        0        1        0
//    simd4        0        2        0
// Totals...
// yes simd        0        3        0
//  no simd        0        9        0
    fn flat_bulk_norm(self) -> Scalar {
        return self.flat_bulk_norm_squared().square_root();
    }
}
impl std::ops::Div<flat_bulk_norm> for Motor {
    type Output = Scalar;
    fn div(self, _rhs: flat_bulk_norm) -> Self::Output {
        self.flat_bulk_norm()
    }
}
impl FlatBulkNorm for Motor {
// Operative Statistics for this implementation:
//           add/sub      mul      div
//      f32        3        5        0
//    simd4        0        2        0
// Totals...
// yes simd        3        7        0
//  no simd        3       13        0
    fn flat_bulk_norm(self) -> Scalar {
        return self.flat_bulk_norm_squared().square_root();
    }
}
impl std::ops::Div<flat_bulk_norm> for MultiVector {
    type Output = Scalar;
    fn div(self, _rhs: flat_bulk_norm) -> Self::Output {
        self.flat_bulk_norm()
    }
}
impl FlatBulkNorm for MultiVector {
// Operative Statistics for this implementation:
//           add/sub      mul      div
//      f32        7       10        0
//    simd2        0        1        0
//    simd3        0        2        0
//    simd4        0        2        0
// Totals...
// yes simd        7       15        0
//  no simd        7       26        0
    fn flat_bulk_norm(self) -> Scalar {
        return self.flat_bulk_norm_squared().square_root();
    }
}
impl std::ops::Div<flat_bulk_norm> for Point {
    type Output = Scalar;
    fn div(self, _rhs: flat_bulk_norm) -> Self::Output {
        self.flat_bulk_norm()
    }
}
impl FlatBulkNorm for Point {
// Operative Statistics for this implementation:
//           add/sub      mul      div
//      f32        2        3        0
//    simd3        0        2        0
// Totals...
// yes simd        2        5        0
//  no simd        2        9        0
    fn flat_bulk_norm(self) -> Scalar {
        return self.flat_bulk_norm_squared().square_root();
    }
}
