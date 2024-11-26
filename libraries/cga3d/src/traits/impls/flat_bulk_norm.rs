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
//  Minimum:         0       3       0
//   Median:         9      20       0
//  Average:        12      22       0
//  Maximum:        68      92       0
//
//  No SIMD:   add/sub     mul     div
//  Minimum:         0       9       0
//   Median:        12      23       0
//  Average:        14      29       0
//  Maximum:        80     112       0
impl std::ops::Div<flat_bulk_norm> for AntiCircleRotor {
    type Output = Scalar;
    fn div(self, _rhs: flat_bulk_norm) -> Self::Output {
        self.flat_bulk_norm()
    }
}
impl FlatBulkNorm for AntiCircleRotor {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        5        9        0
    //    simd3        1        2        0
    // Totals...
    // yes simd        6       11        0
    //  no simd        8       15        0
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
    //      f32       23       36        0
    //    simd3        0        2        0
    //    simd4        0        3        0
    // Totals...
    // yes simd       23       41        0
    //  no simd       23       54        0
    fn flat_bulk_norm(self) -> Scalar {
        return self.flat_bulk_norm_squared().square_root();
    }
}
impl std::ops::Div<flat_bulk_norm> for AntiDualNum {
    type Output = Scalar;
    fn div(self, _rhs: flat_bulk_norm) -> Self::Output {
        self.flat_bulk_norm()
    }
}
impl FlatBulkNorm for AntiDualNum {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       15       18        0
    //    simd4        0        3        0
    // Totals...
    // yes simd       15       21        0
    //  no simd       15       30        0
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
    //           add/sub      mul      div
    //      f32        7       14        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        7       15        0
    //  no simd        7       18        0
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
    //      f32       23       36        0
    //    simd3        0        2        0
    //    simd4        0        3        0
    // Totals...
    // yes simd       23       41        0
    //  no simd       23       54        0
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
    //      f32        5        9        0
    //    simd3        1        2        0
    // Totals...
    // yes simd        6       11        0
    //  no simd        8       15        0
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
    //      f32        8       19        0
    //    simd4        1        1        0
    // Totals...
    // yes simd        9       20        0
    //  no simd       12       23        0
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
    //      f32        0        1        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        0        3        0
    //  no simd        0        9        0
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
    //           add/sub      mul      div
    //      f32        7       14        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        7       15        0
    //  no simd        7       18        0
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
    //           add/sub      mul      div
    //      f32        7       14        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        7       15        0
    //  no simd        7       18        0
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
    //      f32        5        9        0
    //    simd3        1        2        0
    // Totals...
    // yes simd        6       11        0
    //  no simd        8       15        0
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
    //      f32        8       19        0
    //    simd4        1        1        0
    // Totals...
    // yes simd        9       20        0
    //  no simd       12       23        0
    fn flat_bulk_norm(self) -> Scalar {
        return self.flat_bulk_norm_squared().square_root();
    }
}
impl std::ops::Div<flat_bulk_norm> for DualNum {
    type Output = Scalar;
    fn div(self, _rhs: flat_bulk_norm) -> Self::Output {
        self.flat_bulk_norm()
    }
}
impl FlatBulkNorm for DualNum {
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
impl std::ops::Div<flat_bulk_norm> for FlatPoint {
    type Output = Scalar;
    fn div(self, _rhs: flat_bulk_norm) -> Self::Output {
        self.flat_bulk_norm()
    }
}
impl FlatBulkNorm for FlatPoint {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        5        9        0
    //    simd3        1        2        0
    // Totals...
    // yes simd        6       11        0
    //  no simd        8       15        0
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
    //      f32        8       19        0
    //    simd4        1        1        0
    // Totals...
    // yes simd        9       20        0
    //  no simd       12       23        0
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
    //      f32        7       14        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        7       15        0
    //  no simd        7       18        0
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
    //      f32       23       36        0
    //    simd3        0        2        0
    //    simd4        0        3        0
    // Totals...
    // yes simd       23       41        0
    //  no simd       23       54        0
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
    //      f32       63       84        0
    //    simd3        3        4        0
    //    simd4        2        4        0
    // Totals...
    // yes simd       68       92        0
    //  no simd       80      112        0
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
    //           add/sub      mul      div
    //      f32       15       18        0
    //    simd4        0        3        0
    // Totals...
    // yes simd       15       21        0
    //  no simd       15       30        0
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
    //      f32        0        1        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        0        3        0
    //  no simd        0        9        0
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
    //           add/sub      mul      div
    //      f32       15       18        0
    //    simd4        0        3        0
    // Totals...
    // yes simd       15       21        0
    //  no simd       15       30        0
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
    //      f32       23       36        0
    //    simd3        0        2        0
    //    simd4        0        3        0
    // Totals...
    // yes simd       23       41        0
    //  no simd       23       54        0
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
    //      f32        8       19        0
    //    simd4        1        1        0
    // Totals...
    // yes simd        9       20        0
    //  no simd       12       23        0
    fn flat_bulk_norm(self) -> Scalar {
        return self.flat_bulk_norm_squared().square_root();
    }
}
