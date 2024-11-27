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
// Total Implementations: 23
//
// Yes SIMD:   add/sub     mul     div
//  Minimum:         0       3       0
//   Median:         9      20       0
//  Average:        12      21       0
//  Maximum:        68      92       0
//
//  No SIMD:   add/sub     mul     div
//  Minimum:         0       9       0
//   Median:        12      23       0
//  Average:        14      29       0
//  Maximum:        80     112       0
impl std::ops::Div<flat_bulk_norm_squared> for AntiCircleRotor {
    type Output = Scalar;
    fn div(self, _rhs: flat_bulk_norm_squared) -> Self::Output {
        self.flat_bulk_norm_squared()
    }
}
impl FlatBulkNormSquared for AntiCircleRotor {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        5        9        0
    //    simd3        1        2        0
    // Totals...
    // yes simd        6       11        0
    //  no simd        8       15        0
    fn flat_bulk_norm_squared(self) -> Scalar {
        let flat_bulk_thing = self.flat_bulk().wedge(RoundPoint::from_groups(/* e1, e2, e3, e4 */ Simd32x4::from([0.0, 0.0, 0.0, 1.0]), /* e5 */ 0.0));
        return flat_bulk_thing.dot_product(flat_bulk_thing);
    }
}
impl std::ops::Div<flat_bulk_norm_squared> for AntiDipoleInversion {
    type Output = Scalar;
    fn div(self, _rhs: flat_bulk_norm_squared) -> Self::Output {
        self.flat_bulk_norm_squared()
    }
}
impl FlatBulkNormSquared for AntiDipoleInversion {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       20       29        0
    //    simd3        1        4        0
    //    simd4        0        4        0
    // Totals...
    // yes simd       21       37        0
    //  no simd       23       57        0
    fn flat_bulk_norm_squared(self) -> Scalar {
        let flat_bulk_thing = self.flat_bulk().wedge(RoundPoint::from_groups(/* e1, e2, e3, e4 */ Simd32x4::from([0.0, 0.0, 0.0, 1.0]), /* e5 */ 0.0));
        return flat_bulk_thing.dot_product(flat_bulk_thing);
    }
}
impl std::ops::Div<flat_bulk_norm_squared> for AntiDualNum {
    type Output = Scalar;
    fn div(self, _rhs: flat_bulk_norm_squared) -> Self::Output {
        self.flat_bulk_norm_squared()
    }
}
impl FlatBulkNormSquared for AntiDualNum {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       15       18        0
    //    simd4        0        3        0
    // Totals...
    // yes simd       15       21        0
    //  no simd       15       30        0
    fn flat_bulk_norm_squared(self) -> Scalar {
        let flat_bulk_thing = self.flat_bulk().wedge(RoundPoint::from_groups(/* e1, e2, e3, e4 */ Simd32x4::from([0.0, 0.0, 0.0, 1.0]), /* e5 */ 0.0));
        return flat_bulk_thing.dot_product(flat_bulk_thing);
    }
}
impl std::ops::Div<flat_bulk_norm_squared> for AntiFlatPoint {
    type Output = Scalar;
    fn div(self, _rhs: flat_bulk_norm_squared) -> Self::Output {
        self.flat_bulk_norm_squared()
    }
}
impl FlatBulkNormSquared for AntiFlatPoint {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        7       14        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        7       15        0
    //  no simd        7       18        0
    fn flat_bulk_norm_squared(self) -> Scalar {
        let flat_bulk_thing = self.flat_bulk().wedge(RoundPoint::from_groups(/* e1, e2, e3, e4 */ Simd32x4::from([0.0, 0.0, 0.0, 1.0]), /* e5 */ 0.0));
        return flat_bulk_thing.dot_product(flat_bulk_thing);
    }
}
impl std::ops::Div<flat_bulk_norm_squared> for AntiFlector {
    type Output = Scalar;
    fn div(self, _rhs: flat_bulk_norm_squared) -> Self::Output {
        self.flat_bulk_norm_squared()
    }
}
impl FlatBulkNormSquared for AntiFlector {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       20       29        0
    //    simd3        1        4        0
    //    simd4        0        4        0
    // Totals...
    // yes simd       21       37        0
    //  no simd       23       57        0
    fn flat_bulk_norm_squared(self) -> Scalar {
        let flat_bulk_thing = self.flat_bulk().wedge(RoundPoint::from_groups(/* e1, e2, e3, e4 */ Simd32x4::from([0.0, 0.0, 0.0, 1.0]), /* e5 */ 0.0));
        return flat_bulk_thing.dot_product(flat_bulk_thing);
    }
}
impl std::ops::Div<flat_bulk_norm_squared> for AntiLine {
    type Output = Scalar;
    fn div(self, _rhs: flat_bulk_norm_squared) -> Self::Output {
        self.flat_bulk_norm_squared()
    }
}
impl FlatBulkNormSquared for AntiLine {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        5        9        0
    //    simd3        1        2        0
    // Totals...
    // yes simd        6       11        0
    //  no simd        8       15        0
    fn flat_bulk_norm_squared(self) -> Scalar {
        let flat_bulk_thing = self.flat_bulk().wedge(RoundPoint::from_groups(/* e1, e2, e3, e4 */ Simd32x4::from([0.0, 0.0, 0.0, 1.0]), /* e5 */ 0.0));
        return flat_bulk_thing.dot_product(flat_bulk_thing);
    }
}
impl std::ops::Div<flat_bulk_norm_squared> for AntiMotor {
    type Output = Scalar;
    fn div(self, _rhs: flat_bulk_norm_squared) -> Self::Output {
        self.flat_bulk_norm_squared()
    }
}
impl FlatBulkNormSquared for AntiMotor {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        8       19        0
    //    simd4        1        1        0
    // Totals...
    // yes simd        9       20        0
    //  no simd       12       23        0
    fn flat_bulk_norm_squared(self) -> Scalar {
        let flat_bulk_thing = self.flat_bulk().wedge(RoundPoint::from_groups(/* e1, e2, e3, e4 */ Simd32x4::from([0.0, 0.0, 0.0, 1.0]), /* e5 */ 0.0));
        return flat_bulk_thing.dot_product(flat_bulk_thing);
    }
}
impl std::ops::Div<flat_bulk_norm_squared> for AntiPlane {
    type Output = Scalar;
    fn div(self, _rhs: flat_bulk_norm_squared) -> Self::Output {
        self.flat_bulk_norm_squared()
    }
}
impl FlatBulkNormSquared for AntiPlane {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        0        3        0
    //  no simd        0        9        0
    fn flat_bulk_norm_squared(self) -> Scalar {
        let flat_bulk_thing = self.flat_bulk().wedge(RoundPoint::from_groups(/* e1, e2, e3, e4 */ Simd32x4::from([0.0, 0.0, 0.0, 1.0]), /* e5 */ 0.0));
        return flat_bulk_thing.dot_product(flat_bulk_thing);
    }
}
impl std::ops::Div<flat_bulk_norm_squared> for Circle {
    type Output = Scalar;
    fn div(self, _rhs: flat_bulk_norm_squared) -> Self::Output {
        self.flat_bulk_norm_squared()
    }
}
impl FlatBulkNormSquared for Circle {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        7       14        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        7       15        0
    //  no simd        7       18        0
    fn flat_bulk_norm_squared(self) -> Scalar {
        let flat_bulk_thing = self.flat_bulk().wedge(RoundPoint::from_groups(/* e1, e2, e3, e4 */ Simd32x4::from([0.0, 0.0, 0.0, 1.0]), /* e5 */ 0.0));
        return flat_bulk_thing.dot_product(flat_bulk_thing);
    }
}
impl std::ops::Div<flat_bulk_norm_squared> for CircleRotor {
    type Output = Scalar;
    fn div(self, _rhs: flat_bulk_norm_squared) -> Self::Output {
        self.flat_bulk_norm_squared()
    }
}
impl FlatBulkNormSquared for CircleRotor {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        7       14        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        7       15        0
    //  no simd        7       18        0
    fn flat_bulk_norm_squared(self) -> Scalar {
        let flat_bulk_thing = self.flat_bulk().wedge(RoundPoint::from_groups(/* e1, e2, e3, e4 */ Simd32x4::from([0.0, 0.0, 0.0, 1.0]), /* e5 */ 0.0));
        return flat_bulk_thing.dot_product(flat_bulk_thing);
    }
}
impl std::ops::Div<flat_bulk_norm_squared> for Dipole {
    type Output = Scalar;
    fn div(self, _rhs: flat_bulk_norm_squared) -> Self::Output {
        self.flat_bulk_norm_squared()
    }
}
impl FlatBulkNormSquared for Dipole {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        5        9        0
    //    simd3        1        2        0
    // Totals...
    // yes simd        6       11        0
    //  no simd        8       15        0
    fn flat_bulk_norm_squared(self) -> Scalar {
        let flat_bulk_thing = self.flat_bulk().wedge(RoundPoint::from_groups(/* e1, e2, e3, e4 */ Simd32x4::from([0.0, 0.0, 0.0, 1.0]), /* e5 */ 0.0));
        return flat_bulk_thing.dot_product(flat_bulk_thing);
    }
}
impl std::ops::Div<flat_bulk_norm_squared> for DipoleInversion {
    type Output = Scalar;
    fn div(self, _rhs: flat_bulk_norm_squared) -> Self::Output {
        self.flat_bulk_norm_squared()
    }
}
impl FlatBulkNormSquared for DipoleInversion {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        8       19        0
    //    simd4        1        1        0
    // Totals...
    // yes simd        9       20        0
    //  no simd       12       23        0
    fn flat_bulk_norm_squared(self) -> Scalar {
        let flat_bulk_thing = self.flat_bulk().wedge(RoundPoint::from_groups(/* e1, e2, e3, e4 */ Simd32x4::from([0.0, 0.0, 0.0, 1.0]), /* e5 */ 0.0));
        return flat_bulk_thing.dot_product(flat_bulk_thing);
    }
}
impl std::ops::Div<flat_bulk_norm_squared> for DualNum {
    type Output = Scalar;
    fn div(self, _rhs: flat_bulk_norm_squared) -> Self::Output {
        self.flat_bulk_norm_squared()
    }
}
impl FlatBulkNormSquared for DualNum {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        0        3        0
    //  no simd        0        9        0
    fn flat_bulk_norm_squared(self) -> Scalar {
        let flat_bulk_thing = self.flat_bulk().wedge(RoundPoint::from_groups(/* e1, e2, e3, e4 */ Simd32x4::from([0.0, 0.0, 0.0, 1.0]), /* e5 */ 0.0));
        return flat_bulk_thing.dot_product(flat_bulk_thing);
    }
}
impl std::ops::Div<flat_bulk_norm_squared> for FlatPoint {
    type Output = Scalar;
    fn div(self, _rhs: flat_bulk_norm_squared) -> Self::Output {
        self.flat_bulk_norm_squared()
    }
}
impl FlatBulkNormSquared for FlatPoint {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        5        9        0
    //    simd3        1        2        0
    // Totals...
    // yes simd        6       11        0
    //  no simd        8       15        0
    fn flat_bulk_norm_squared(self) -> Scalar {
        let flat_bulk_thing = self.flat_bulk().wedge(RoundPoint::from_groups(/* e1, e2, e3, e4 */ Simd32x4::from([0.0, 0.0, 0.0, 1.0]), /* e5 */ 0.0));
        return flat_bulk_thing.dot_product(flat_bulk_thing);
    }
}
impl std::ops::Div<flat_bulk_norm_squared> for Flector {
    type Output = Scalar;
    fn div(self, _rhs: flat_bulk_norm_squared) -> Self::Output {
        self.flat_bulk_norm_squared()
    }
}
impl FlatBulkNormSquared for Flector {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        8       19        0
    //    simd4        1        1        0
    // Totals...
    // yes simd        9       20        0
    //  no simd       12       23        0
    fn flat_bulk_norm_squared(self) -> Scalar {
        let flat_bulk_thing = self.flat_bulk().wedge(RoundPoint::from_groups(/* e1, e2, e3, e4 */ Simd32x4::from([0.0, 0.0, 0.0, 1.0]), /* e5 */ 0.0));
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
    //      f32        7       14        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        7       15        0
    //  no simd        7       18        0
    fn flat_bulk_norm_squared(self) -> Scalar {
        let flat_bulk_thing = self.flat_bulk().wedge(RoundPoint::from_groups(/* e1, e2, e3, e4 */ Simd32x4::from([0.0, 0.0, 0.0, 1.0]), /* e5 */ 0.0));
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
    //      f32       20       29        0
    //    simd3        1        4        0
    //    simd4        0        4        0
    // Totals...
    // yes simd       21       37        0
    //  no simd       23       57        0
    fn flat_bulk_norm_squared(self) -> Scalar {
        let flat_bulk_thing = self.flat_bulk().wedge(RoundPoint::from_groups(/* e1, e2, e3, e4 */ Simd32x4::from([0.0, 0.0, 0.0, 1.0]), /* e5 */ 0.0));
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
    //      f32       63       84        0
    //    simd3        3        4        0
    //    simd4        2        4        0
    // Totals...
    // yes simd       68       92        0
    //  no simd       80      112        0
    fn flat_bulk_norm_squared(self) -> Scalar {
        let flat_bulk_thing = self.flat_bulk().wedge(RoundPoint::from_groups(/* e1, e2, e3, e4 */ Simd32x4::from([0.0, 0.0, 0.0, 1.0]), /* e5 */ 0.0));
        return flat_bulk_thing.dot_product(flat_bulk_thing);
    }
}
impl std::ops::Div<flat_bulk_norm_squared> for Plane {
    type Output = Scalar;
    fn div(self, _rhs: flat_bulk_norm_squared) -> Self::Output {
        self.flat_bulk_norm_squared()
    }
}
impl FlatBulkNormSquared for Plane {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       15       18        0
    //    simd4        0        3        0
    // Totals...
    // yes simd       15       21        0
    //  no simd       15       30        0
    fn flat_bulk_norm_squared(self) -> Scalar {
        let flat_bulk_thing = self.flat_bulk().wedge(RoundPoint::from_groups(/* e1, e2, e3, e4 */ Simd32x4::from([0.0, 0.0, 0.0, 1.0]), /* e5 */ 0.0));
        return flat_bulk_thing.dot_product(flat_bulk_thing);
    }
}
impl std::ops::Div<flat_bulk_norm_squared> for RoundPoint {
    type Output = Scalar;
    fn div(self, _rhs: flat_bulk_norm_squared) -> Self::Output {
        self.flat_bulk_norm_squared()
    }
}
impl FlatBulkNormSquared for RoundPoint {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        0        3        0
    //  no simd        0        9        0
    fn flat_bulk_norm_squared(self) -> Scalar {
        let flat_bulk_thing = self.flat_bulk().wedge(RoundPoint::from_groups(/* e1, e2, e3, e4 */ Simd32x4::from([0.0, 0.0, 0.0, 1.0]), /* e5 */ 0.0));
        return flat_bulk_thing.dot_product(flat_bulk_thing);
    }
}
impl std::ops::Div<flat_bulk_norm_squared> for Sphere {
    type Output = Scalar;
    fn div(self, _rhs: flat_bulk_norm_squared) -> Self::Output {
        self.flat_bulk_norm_squared()
    }
}
impl FlatBulkNormSquared for Sphere {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       15       18        0
    //    simd4        0        3        0
    // Totals...
    // yes simd       15       21        0
    //  no simd       15       30        0
    fn flat_bulk_norm_squared(self) -> Scalar {
        let flat_bulk_thing = self.flat_bulk().wedge(RoundPoint::from_groups(/* e1, e2, e3, e4 */ Simd32x4::from([0.0, 0.0, 0.0, 1.0]), /* e5 */ 0.0));
        return flat_bulk_thing.dot_product(flat_bulk_thing);
    }
}
impl std::ops::Div<flat_bulk_norm_squared> for VersorEven {
    type Output = Scalar;
    fn div(self, _rhs: flat_bulk_norm_squared) -> Self::Output {
        self.flat_bulk_norm_squared()
    }
}
impl FlatBulkNormSquared for VersorEven {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       20       29        0
    //    simd3        1        4        0
    //    simd4        0        4        0
    // Totals...
    // yes simd       21       37        0
    //  no simd       23       57        0
    fn flat_bulk_norm_squared(self) -> Scalar {
        let flat_bulk_thing = self.flat_bulk().wedge(RoundPoint::from_groups(/* e1, e2, e3, e4 */ Simd32x4::from([0.0, 0.0, 0.0, 1.0]), /* e5 */ 0.0));
        return flat_bulk_thing.dot_product(flat_bulk_thing);
    }
}
impl std::ops::Div<flat_bulk_norm_squared> for VersorOdd {
    type Output = Scalar;
    fn div(self, _rhs: flat_bulk_norm_squared) -> Self::Output {
        self.flat_bulk_norm_squared()
    }
}
impl FlatBulkNormSquared for VersorOdd {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        8       19        0
    //    simd4        1        1        0
    // Totals...
    // yes simd        9       20        0
    //  no simd       12       23        0
    fn flat_bulk_norm_squared(self) -> Scalar {
        let flat_bulk_thing = self.flat_bulk().wedge(RoundPoint::from_groups(/* e1, e2, e3, e4 */ Simd32x4::from([0.0, 0.0, 0.0, 1.0]), /* e5 */ 0.0));
        return flat_bulk_thing.dot_product(flat_bulk_thing);
    }
}
