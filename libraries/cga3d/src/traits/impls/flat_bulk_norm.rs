use crate::traits::FlatBulkNormSquared;
use crate::traits::SquareRoot;
// Note on Operative Statistics:
// Operative Statistics are not a precise predictor of performance or performance comparisons.
// This is due to varying hardware capabilities and compiler optimizations.
// As always, where performance is a concern, there is no substitute for
// real measurements on real work-loads on real hardware.
// Disclaimer aside, enjoy the fun information =)
//
// Total Implementations: 21
//
// Yes SIMD:   add/sub     mul     div
//  Minimum:         0       3       0
//   Median:         4      13       0
//  Average:         6      13       0
//  Maximum:        32      51       0
//
//  No SIMD:   add/sub     mul     div
//  Minimum:         0       3       0
//   Median:         4      13       0
//  Average:         6      15       0
//  Maximum:        32      57       0
impl std::ops::Div<flat_bulk_norm> for AntiCircleRotor {
    type Output = Scalar;
    fn div(self, _rhs: flat_bulk_norm) -> Self::Output {
        self.flat_bulk_norm()
    }
}
impl FlatBulkNorm for AntiCircleRotor {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        3        0
    //    simd3        0        1        0
    // Totals...
    // yes simd        2        4        0
    //  no simd        2        6        0
    fn flat_bulk_norm(self) -> Scalar {
        return self.flat_bulk_norm_squared().square_root();
    }
}
impl std::ops::Div<flat_bulk_norm> for AntiDipoleInversion {
    type Output = Scalar;
    fn div(self, _rhs: flat_bulk_norm) -> Self::Output {
        self.flat_bulk_norm()
    }
}
impl FlatBulkNorm for AntiDipoleInversion {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       14       25        0
    //    simd3        0        2        0
    // Totals...
    // yes simd       14       27        0
    //  no simd       14       31        0
    fn flat_bulk_norm(self) -> Scalar {
        return self.flat_bulk_norm_squared().square_root();
    }
}
impl std::ops::Div<flat_bulk_norm> for AntiFlatPoint {
    type Output = Scalar;
    fn div(self, _rhs: flat_bulk_norm) -> Self::Output {
        self.flat_bulk_norm()
    }
}
impl FlatBulkNorm for AntiFlatPoint {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        4       13        0
    fn flat_bulk_norm(self) -> Scalar {
        return self.flat_bulk_norm_squared().square_root();
    }
}
impl std::ops::Div<flat_bulk_norm> for AntiFlector {
    type Output = Scalar;
    fn div(self, _rhs: flat_bulk_norm) -> Self::Output {
        self.flat_bulk_norm()
    }
}
impl FlatBulkNorm for AntiFlector {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       14       25        0
    //    simd3        0        2        0
    // Totals...
    // yes simd       14       27        0
    //  no simd       14       31        0
    fn flat_bulk_norm(self) -> Scalar {
        return self.flat_bulk_norm_squared().square_root();
    }
}
impl std::ops::Div<flat_bulk_norm> for AntiLine {
    type Output = Scalar;
    fn div(self, _rhs: flat_bulk_norm) -> Self::Output {
        self.flat_bulk_norm()
    }
}
impl FlatBulkNorm for AntiLine {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        3        0
    //    simd3        0        1        0
    // Totals...
    // yes simd        2        4        0
    //  no simd        2        6        0
    fn flat_bulk_norm(self) -> Scalar {
        return self.flat_bulk_norm_squared().square_root();
    }
}
impl std::ops::Div<flat_bulk_norm> for AntiMotor {
    type Output = Scalar;
    fn div(self, _rhs: flat_bulk_norm) -> Self::Output {
        self.flat_bulk_norm()
    }
}
impl FlatBulkNorm for AntiMotor {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        4        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        3        5        0
    //  no simd        3        8        0
    fn flat_bulk_norm(self) -> Scalar {
        return self.flat_bulk_norm_squared().square_root();
    }
}
impl std::ops::Div<flat_bulk_norm> for AntiPlane {
    type Output = Scalar;
    fn div(self, _rhs: flat_bulk_norm) -> Self::Output {
        self.flat_bulk_norm()
    }
}
impl FlatBulkNorm for AntiPlane {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        9       12        0
    //    simd3        0        2        0
    // Totals...
    // yes simd        9       14        0
    //  no simd        9       18        0
    fn flat_bulk_norm(self) -> Scalar {
        return self.flat_bulk_norm_squared().square_root();
    }
}
impl std::ops::Div<flat_bulk_norm> for Circle {
    type Output = Scalar;
    fn div(self, _rhs: flat_bulk_norm) -> Self::Output {
        self.flat_bulk_norm()
    }
}
impl FlatBulkNorm for Circle {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        4       13        0
    fn flat_bulk_norm(self) -> Scalar {
        return self.flat_bulk_norm_squared().square_root();
    }
}
impl std::ops::Div<flat_bulk_norm> for CircleRotor {
    type Output = Scalar;
    fn div(self, _rhs: flat_bulk_norm) -> Self::Output {
        self.flat_bulk_norm()
    }
}
impl FlatBulkNorm for CircleRotor {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        4       13        0
    fn flat_bulk_norm(self) -> Scalar {
        return self.flat_bulk_norm_squared().square_root();
    }
}
impl std::ops::Div<flat_bulk_norm> for Dipole {
    type Output = Scalar;
    fn div(self, _rhs: flat_bulk_norm) -> Self::Output {
        self.flat_bulk_norm()
    }
}
impl FlatBulkNorm for Dipole {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        3        0
    //    simd3        0        1        0
    // Totals...
    // yes simd        2        4        0
    //  no simd        2        6        0
    fn flat_bulk_norm(self) -> Scalar {
        return self.flat_bulk_norm_squared().square_root();
    }
}
impl std::ops::Div<flat_bulk_norm> for DipoleInversion {
    type Output = Scalar;
    fn div(self, _rhs: flat_bulk_norm) -> Self::Output {
        self.flat_bulk_norm()
    }
}
impl FlatBulkNorm for DipoleInversion {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        4        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        3        5        0
    //  no simd        3        8        0
    fn flat_bulk_norm(self) -> Scalar {
        return self.flat_bulk_norm_squared().square_root();
    }
}
impl std::ops::Div<flat_bulk_norm> for FlatPoint {
    type Output = Scalar;
    fn div(self, _rhs: flat_bulk_norm) -> Self::Output {
        self.flat_bulk_norm()
    }
}
impl FlatBulkNorm for FlatPoint {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        3        0
    //    simd3        0        1        0
    // Totals...
    // yes simd        2        4        0
    //  no simd        2        6        0
    fn flat_bulk_norm(self) -> Scalar {
        return self.flat_bulk_norm_squared().square_root();
    }
}
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
    //    simd4        0        1        0
    // Totals...
    // yes simd        3        5        0
    //  no simd        3        8        0
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
    //      add/sub      mul      div
    // f32        4       13        0
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
    //      f32       14       25        0
    //    simd3        0        2        0
    // Totals...
    // yes simd       14       27        0
    //  no simd       14       31        0
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
    //      f32       32       48        0
    //    simd3        0        3        0
    // Totals...
    // yes simd       32       51        0
    //  no simd       32       57        0
    fn flat_bulk_norm(self) -> Scalar {
        return self.flat_bulk_norm_squared().square_root();
    }
}
impl std::ops::Div<flat_bulk_norm> for Plane {
    type Output = Scalar;
    fn div(self, _rhs: flat_bulk_norm) -> Self::Output {
        self.flat_bulk_norm()
    }
}
impl FlatBulkNorm for Plane {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        3        0
    fn flat_bulk_norm(self) -> Scalar {
        return self.flat_bulk_norm_squared().square_root();
    }
}
impl std::ops::Div<flat_bulk_norm> for RoundPoint {
    type Output = Scalar;
    fn div(self, _rhs: flat_bulk_norm) -> Self::Output {
        self.flat_bulk_norm()
    }
}
impl FlatBulkNorm for RoundPoint {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        9       12        0
    //    simd3        0        2        0
    // Totals...
    // yes simd        9       14        0
    //  no simd        9       18        0
    fn flat_bulk_norm(self) -> Scalar {
        return self.flat_bulk_norm_squared().square_root();
    }
}
impl std::ops::Div<flat_bulk_norm> for Sphere {
    type Output = Scalar;
    fn div(self, _rhs: flat_bulk_norm) -> Self::Output {
        self.flat_bulk_norm()
    }
}
impl FlatBulkNorm for Sphere {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        3        0
    fn flat_bulk_norm(self) -> Scalar {
        return self.flat_bulk_norm_squared().square_root();
    }
}
impl std::ops::Div<flat_bulk_norm> for VersorEven {
    type Output = Scalar;
    fn div(self, _rhs: flat_bulk_norm) -> Self::Output {
        self.flat_bulk_norm()
    }
}
impl FlatBulkNorm for VersorEven {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       14       25        0
    //    simd3        0        2        0
    // Totals...
    // yes simd       14       27        0
    //  no simd       14       31        0
    fn flat_bulk_norm(self) -> Scalar {
        return self.flat_bulk_norm_squared().square_root();
    }
}
impl std::ops::Div<flat_bulk_norm> for VersorOdd {
    type Output = Scalar;
    fn div(self, _rhs: flat_bulk_norm) -> Self::Output {
        self.flat_bulk_norm()
    }
}
impl FlatBulkNorm for VersorOdd {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        4        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        3        5        0
    //  no simd        3        8        0
    fn flat_bulk_norm(self) -> Scalar {
        return self.flat_bulk_norm_squared().square_root();
    }
}
