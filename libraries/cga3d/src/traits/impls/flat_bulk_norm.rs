use crate::traits::FlatBulkNormSquared;
use crate::traits::SquareRoot;
// Note on Operative Statistics:
// Operative Statistics are not a precise predictor of performance or performance comparisons.
// This is due to varying hardware capabilities and compiler optimizations.
// As always, where performance is a concern, there is no substitute for
// real measurements on real work-loads on real hardware.
// Disclaimer aside, enjoy the fun information =)
//
// Total Implementations: 23
//
// Yes SIMD:   add/sub     mul     div
//  Minimum:         0       4       0
//   Median:         7      16       0
//  Average:        10      17       0
//  Maximum:        56      75       0
//
//  No SIMD:   add/sub     mul     div
//  Minimum:         0      10       0
//   Median:        12      22       0
//  Average:        14      26       0
//  Maximum:        80     112       0
impl std::ops::Div<FlatBulkNormPrefixOrPostfix> for AntiCircleRotor {
    type Output = Scalar;
    fn div(self, _rhs: FlatBulkNormPrefixOrPostfix) -> Self::Output {
        self.flat_bulk_norm()
    }
}
impl FlatBulkNorm for AntiCircleRotor {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        3        0
    //    simd3        2        4        0
    // Totals...
    // yes simd        4        7        0
    //  no simd        8       15        0
    fn flat_bulk_norm(self) -> Scalar {
        return self.flat_bulk_norm_squared().square_root();
    }
}
impl std::ops::Div<FlatBulkNormPrefixOrPostfix> for AntiDipoleInversion {
    type Output = Scalar;
    fn div(self, _rhs: FlatBulkNormPrefixOrPostfix) -> Self::Output {
        self.flat_bulk_norm()
    }
}
impl FlatBulkNorm for AntiDipoleInversion {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       17       23        0
    //    simd3        2        8        0
    // Totals...
    // yes simd       19       31        0
    //  no simd       23       47        0
    fn flat_bulk_norm(self) -> Scalar {
        return self.flat_bulk_norm_squared().square_root();
    }
}
impl std::ops::Div<FlatBulkNormPrefixOrPostfix> for AntiDualNum {
    type Output = Scalar;
    fn div(self, _rhs: FlatBulkNormPrefixOrPostfix) -> Self::Output {
        self.flat_bulk_norm()
    }
}
impl FlatBulkNorm for AntiDualNum {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       15       18        0
    //    simd4        0        1        0
    // Totals...
    // yes simd       15       19        0
    //  no simd       15       22        0
    fn flat_bulk_norm(self) -> Scalar {
        return self.flat_bulk_norm_squared().square_root();
    }
}
impl std::ops::Div<FlatBulkNormPrefixOrPostfix> for AntiFlatPoint {
    type Output = Scalar;
    fn div(self, _rhs: FlatBulkNormPrefixOrPostfix) -> Self::Output {
        self.flat_bulk_norm()
    }
}
impl FlatBulkNorm for AntiFlatPoint {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        7       11        0
    //    simd3        0        2        0
    // Totals...
    // yes simd        7       13        0
    //  no simd        7       17        0
    fn flat_bulk_norm(self) -> Scalar {
        return self.flat_bulk_norm_squared().square_root();
    }
}
impl std::ops::Div<FlatBulkNormPrefixOrPostfix> for AntiFlector {
    type Output = Scalar;
    fn div(self, _rhs: FlatBulkNormPrefixOrPostfix) -> Self::Output {
        self.flat_bulk_norm()
    }
}
impl FlatBulkNorm for AntiFlector {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       17       23        0
    //    simd3        2        8        0
    // Totals...
    // yes simd       19       31        0
    //  no simd       23       47        0
    fn flat_bulk_norm(self) -> Scalar {
        return self.flat_bulk_norm_squared().square_root();
    }
}
impl std::ops::Div<FlatBulkNormPrefixOrPostfix> for AntiLine {
    type Output = Scalar;
    fn div(self, _rhs: FlatBulkNormPrefixOrPostfix) -> Self::Output {
        self.flat_bulk_norm()
    }
}
impl FlatBulkNorm for AntiLine {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        3        0
    //    simd3        2        4        0
    // Totals...
    // yes simd        4        7        0
    //  no simd        8       15        0
    fn flat_bulk_norm(self) -> Scalar {
        return self.flat_bulk_norm_squared().square_root();
    }
}
impl std::ops::Div<FlatBulkNormPrefixOrPostfix> for AntiMotor {
    type Output = Scalar;
    fn div(self, _rhs: FlatBulkNormPrefixOrPostfix) -> Self::Output {
        self.flat_bulk_norm()
    }
}
impl FlatBulkNorm for AntiMotor {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        5       13        0
    //    simd3        1        2        0
    //    simd4        1        1        0
    // Totals...
    // yes simd        7       16        0
    //  no simd       12       23        0
    fn flat_bulk_norm(self) -> Scalar {
        return self.flat_bulk_norm_squared().square_root();
    }
}
impl std::ops::Div<FlatBulkNormPrefixOrPostfix> for AntiPlane {
    type Output = Scalar;
    fn div(self, _rhs: FlatBulkNormPrefixOrPostfix) -> Self::Output {
        self.flat_bulk_norm()
    }
}
impl FlatBulkNorm for AntiPlane {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        2        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        0        4        0
    //  no simd        0       10        0
    fn flat_bulk_norm(self) -> Scalar {
        return self.flat_bulk_norm_squared().square_root();
    }
}
impl std::ops::Div<FlatBulkNormPrefixOrPostfix> for Circle {
    type Output = Scalar;
    fn div(self, _rhs: FlatBulkNormPrefixOrPostfix) -> Self::Output {
        self.flat_bulk_norm()
    }
}
impl FlatBulkNorm for Circle {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        7       11        0
    //    simd3        0        2        0
    // Totals...
    // yes simd        7       13        0
    //  no simd        7       17        0
    fn flat_bulk_norm(self) -> Scalar {
        return self.flat_bulk_norm_squared().square_root();
    }
}
impl std::ops::Div<FlatBulkNormPrefixOrPostfix> for CircleRotor {
    type Output = Scalar;
    fn div(self, _rhs: FlatBulkNormPrefixOrPostfix) -> Self::Output {
        self.flat_bulk_norm()
    }
}
impl FlatBulkNorm for CircleRotor {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        7       11        0
    //    simd3        0        2        0
    // Totals...
    // yes simd        7       13        0
    //  no simd        7       17        0
    fn flat_bulk_norm(self) -> Scalar {
        return self.flat_bulk_norm_squared().square_root();
    }
}
impl std::ops::Div<FlatBulkNormPrefixOrPostfix> for Dipole {
    type Output = Scalar;
    fn div(self, _rhs: FlatBulkNormPrefixOrPostfix) -> Self::Output {
        self.flat_bulk_norm()
    }
}
impl FlatBulkNorm for Dipole {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        3        0
    //    simd3        2        4        0
    // Totals...
    // yes simd        4        7        0
    //  no simd        8       15        0
    fn flat_bulk_norm(self) -> Scalar {
        return self.flat_bulk_norm_squared().square_root();
    }
}
impl std::ops::Div<FlatBulkNormPrefixOrPostfix> for DipoleInversion {
    type Output = Scalar;
    fn div(self, _rhs: FlatBulkNormPrefixOrPostfix) -> Self::Output {
        self.flat_bulk_norm()
    }
}
impl FlatBulkNorm for DipoleInversion {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        5       13        0
    //    simd3        1        2        0
    //    simd4        1        1        0
    // Totals...
    // yes simd        7       16        0
    //  no simd       12       23        0
    fn flat_bulk_norm(self) -> Scalar {
        return self.flat_bulk_norm_squared().square_root();
    }
}
impl std::ops::Div<FlatBulkNormPrefixOrPostfix> for DualNum {
    type Output = Scalar;
    fn div(self, _rhs: FlatBulkNormPrefixOrPostfix) -> Self::Output {
        self.flat_bulk_norm()
    }
}
impl FlatBulkNorm for DualNum {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        2        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        0        4        0
    //  no simd        0       10        0
    fn flat_bulk_norm(self) -> Scalar {
        return self.flat_bulk_norm_squared().square_root();
    }
}
impl std::ops::Div<FlatBulkNormPrefixOrPostfix> for FlatPoint {
    type Output = Scalar;
    fn div(self, _rhs: FlatBulkNormPrefixOrPostfix) -> Self::Output {
        self.flat_bulk_norm()
    }
}
impl FlatBulkNorm for FlatPoint {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        3        0
    //    simd3        2        4        0
    // Totals...
    // yes simd        4        7        0
    //  no simd        8       15        0
    fn flat_bulk_norm(self) -> Scalar {
        return self.flat_bulk_norm_squared().square_root();
    }
}
impl std::ops::Div<FlatBulkNormPrefixOrPostfix> for Flector {
    type Output = Scalar;
    fn div(self, _rhs: FlatBulkNormPrefixOrPostfix) -> Self::Output {
        self.flat_bulk_norm()
    }
}
impl FlatBulkNorm for Flector {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        5       13        0
    //    simd3        1        2        0
    //    simd4        1        1        0
    // Totals...
    // yes simd        7       16        0
    //  no simd       12       23        0
    fn flat_bulk_norm(self) -> Scalar {
        return self.flat_bulk_norm_squared().square_root();
    }
}
impl std::ops::Div<FlatBulkNormPrefixOrPostfix> for Line {
    type Output = Scalar;
    fn div(self, _rhs: FlatBulkNormPrefixOrPostfix) -> Self::Output {
        self.flat_bulk_norm()
    }
}
impl FlatBulkNorm for Line {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        7       11        0
    //    simd3        0        2        0
    // Totals...
    // yes simd        7       13        0
    //  no simd        7       17        0
    fn flat_bulk_norm(self) -> Scalar {
        return self.flat_bulk_norm_squared().square_root();
    }
}
impl std::ops::Div<FlatBulkNormPrefixOrPostfix> for Motor {
    type Output = Scalar;
    fn div(self, _rhs: FlatBulkNormPrefixOrPostfix) -> Self::Output {
        self.flat_bulk_norm()
    }
}
impl FlatBulkNorm for Motor {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       17       23        0
    //    simd3        2        8        0
    // Totals...
    // yes simd       19       31        0
    //  no simd       23       47        0
    fn flat_bulk_norm(self) -> Scalar {
        return self.flat_bulk_norm_squared().square_root();
    }
}
impl std::ops::Div<FlatBulkNormPrefixOrPostfix> for MultiVector {
    type Output = Scalar;
    fn div(self, _rhs: FlatBulkNormPrefixOrPostfix) -> Self::Output {
        self.flat_bulk_norm()
    }
}
impl FlatBulkNorm for MultiVector {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       46       59        0
    //    simd3        6       11        0
    //    simd4        4        5        0
    // Totals...
    // yes simd       56       75        0
    //  no simd       80      112        0
    fn flat_bulk_norm(self) -> Scalar {
        return self.flat_bulk_norm_squared().square_root();
    }
}
impl std::ops::Div<FlatBulkNormPrefixOrPostfix> for Plane {
    type Output = Scalar;
    fn div(self, _rhs: FlatBulkNormPrefixOrPostfix) -> Self::Output {
        self.flat_bulk_norm()
    }
}
impl FlatBulkNorm for Plane {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       15       18        0
    //    simd4        0        1        0
    // Totals...
    // yes simd       15       19        0
    //  no simd       15       22        0
    fn flat_bulk_norm(self) -> Scalar {
        return self.flat_bulk_norm_squared().square_root();
    }
}
impl std::ops::Div<FlatBulkNormPrefixOrPostfix> for RoundPoint {
    type Output = Scalar;
    fn div(self, _rhs: FlatBulkNormPrefixOrPostfix) -> Self::Output {
        self.flat_bulk_norm()
    }
}
impl FlatBulkNorm for RoundPoint {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        2        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        0        4        0
    //  no simd        0       10        0
    fn flat_bulk_norm(self) -> Scalar {
        return self.flat_bulk_norm_squared().square_root();
    }
}
impl std::ops::Div<FlatBulkNormPrefixOrPostfix> for Sphere {
    type Output = Scalar;
    fn div(self, _rhs: FlatBulkNormPrefixOrPostfix) -> Self::Output {
        self.flat_bulk_norm()
    }
}
impl FlatBulkNorm for Sphere {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       15       18        0
    //    simd4        0        1        0
    // Totals...
    // yes simd       15       19        0
    //  no simd       15       22        0
    fn flat_bulk_norm(self) -> Scalar {
        return self.flat_bulk_norm_squared().square_root();
    }
}
impl std::ops::Div<FlatBulkNormPrefixOrPostfix> for VersorEven {
    type Output = Scalar;
    fn div(self, _rhs: FlatBulkNormPrefixOrPostfix) -> Self::Output {
        self.flat_bulk_norm()
    }
}
impl FlatBulkNorm for VersorEven {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       17       23        0
    //    simd3        2        8        0
    // Totals...
    // yes simd       19       31        0
    //  no simd       23       47        0
    fn flat_bulk_norm(self) -> Scalar {
        return self.flat_bulk_norm_squared().square_root();
    }
}
impl std::ops::Div<FlatBulkNormPrefixOrPostfix> for VersorOdd {
    type Output = Scalar;
    fn div(self, _rhs: FlatBulkNormPrefixOrPostfix) -> Self::Output {
        self.flat_bulk_norm()
    }
}
impl FlatBulkNorm for VersorOdd {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        5       13        0
    //    simd3        1        2        0
    //    simd4        1        1        0
    // Totals...
    // yes simd        7       16        0
    //  no simd       12       23        0
    fn flat_bulk_norm(self) -> Scalar {
        return self.flat_bulk_norm_squared().square_root();
    }
}
