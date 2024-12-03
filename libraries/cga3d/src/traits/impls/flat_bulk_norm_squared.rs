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
//   Median:         7      16       0
//  Average:        10      17       0
//  Maximum:        56      74       0
//
//  No SIMD:   add/sub     mul     div
//  Minimum:         0       9       0
//   Median:        12      22       0
//  Average:        14      27       0
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
    //      f32        2        3        0
    //    simd3        2        4        0
    // Totals...
    // yes simd        4        7        0
    //  no simd        8       15        0
    fn flat_bulk_norm_squared(self) -> Scalar {
        let flat_bulk_thing = self.flat_bulk().wedge(RoundPoint::from_groups(/* e1, e2, e3, e4 */ Simd32x3::from(0.0).extend_to_4(1.0), /* e5 */ 0.0));
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
    //      f32       17       23        0
    //    simd3        2        6        0
    //    simd4        0        2        0
    // Totals...
    // yes simd       19       31        0
    //  no simd       23       49        0
    fn flat_bulk_norm_squared(self) -> Scalar {
        let flat_bulk_thing = self.flat_bulk().wedge(RoundPoint::from_groups(/* e1, e2, e3, e4 */ Simd32x3::from(0.0).extend_to_4(1.0), /* e5 */ 0.0));
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
    //    simd4        0        1        0
    // Totals...
    // yes simd       15       19        0
    //  no simd       15       22        0
    fn flat_bulk_norm_squared(self) -> Scalar {
        let flat_bulk_thing = self.flat_bulk().wedge(RoundPoint::from_groups(/* e1, e2, e3, e4 */ Simd32x3::from(0.0).extend_to_4(1.0), /* e5 */ 0.0));
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
    //      f32        7       11        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        7       13        0
    //  no simd        7       19        0
    fn flat_bulk_norm_squared(self) -> Scalar {
        let flat_bulk_thing = self.flat_bulk().wedge(RoundPoint::from_groups(/* e1, e2, e3, e4 */ Simd32x3::from(0.0).extend_to_4(1.0), /* e5 */ 0.0));
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
    //      f32       17       23        0
    //    simd3        2        6        0
    //    simd4        0        2        0
    // Totals...
    // yes simd       19       31        0
    //  no simd       23       49        0
    fn flat_bulk_norm_squared(self) -> Scalar {
        let flat_bulk_thing = self.flat_bulk().wedge(RoundPoint::from_groups(/* e1, e2, e3, e4 */ Simd32x3::from(0.0).extend_to_4(1.0), /* e5 */ 0.0));
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
    //      f32        2        3        0
    //    simd3        2        4        0
    // Totals...
    // yes simd        4        7        0
    //  no simd        8       15        0
    fn flat_bulk_norm_squared(self) -> Scalar {
        let flat_bulk_thing = self.flat_bulk().wedge(RoundPoint::from_groups(/* e1, e2, e3, e4 */ Simd32x3::from(0.0).extend_to_4(1.0), /* e5 */ 0.0));
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
    //      f32        5       13        0
    //    simd3        1        2        0
    //    simd4        1        1        0
    // Totals...
    // yes simd        7       16        0
    //  no simd       12       23        0
    fn flat_bulk_norm_squared(self) -> Scalar {
        let flat_bulk_thing = self.flat_bulk().wedge(RoundPoint::from_groups(/* e1, e2, e3, e4 */ Simd32x3::from(0.0).extend_to_4(1.0), /* e5 */ 0.0));
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
        let flat_bulk_thing = self.flat_bulk().wedge(RoundPoint::from_groups(/* e1, e2, e3, e4 */ Simd32x3::from(0.0).extend_to_4(1.0), /* e5 */ 0.0));
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
    //      f32        7       11        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        7       13        0
    //  no simd        7       19        0
    fn flat_bulk_norm_squared(self) -> Scalar {
        let flat_bulk_thing = self.flat_bulk().wedge(RoundPoint::from_groups(/* e1, e2, e3, e4 */ Simd32x3::from(0.0).extend_to_4(1.0), /* e5 */ 0.0));
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
    //      f32        7       11        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        7       13        0
    //  no simd        7       19        0
    fn flat_bulk_norm_squared(self) -> Scalar {
        let flat_bulk_thing = self.flat_bulk().wedge(RoundPoint::from_groups(/* e1, e2, e3, e4 */ Simd32x3::from(0.0).extend_to_4(1.0), /* e5 */ 0.0));
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
    //      f32        2        3        0
    //    simd3        2        4        0
    // Totals...
    // yes simd        4        7        0
    //  no simd        8       15        0
    fn flat_bulk_norm_squared(self) -> Scalar {
        let flat_bulk_thing = self.flat_bulk().wedge(RoundPoint::from_groups(/* e1, e2, e3, e4 */ Simd32x3::from(0.0).extend_to_4(1.0), /* e5 */ 0.0));
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
    //      f32        5       13        0
    //    simd3        1        2        0
    //    simd4        1        1        0
    // Totals...
    // yes simd        7       16        0
    //  no simd       12       23        0
    fn flat_bulk_norm_squared(self) -> Scalar {
        let flat_bulk_thing = self.flat_bulk().wedge(RoundPoint::from_groups(/* e1, e2, e3, e4 */ Simd32x3::from(0.0).extend_to_4(1.0), /* e5 */ 0.0));
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
        let flat_bulk_thing = self.flat_bulk().wedge(RoundPoint::from_groups(/* e1, e2, e3, e4 */ Simd32x3::from(0.0).extend_to_4(1.0), /* e5 */ 0.0));
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
    //      f32        2        3        0
    //    simd3        2        4        0
    // Totals...
    // yes simd        4        7        0
    //  no simd        8       15        0
    fn flat_bulk_norm_squared(self) -> Scalar {
        let flat_bulk_thing = self.flat_bulk().wedge(RoundPoint::from_groups(/* e1, e2, e3, e4 */ Simd32x3::from(0.0).extend_to_4(1.0), /* e5 */ 0.0));
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
    //      f32        5       13        0
    //    simd3        1        2        0
    //    simd4        1        1        0
    // Totals...
    // yes simd        7       16        0
    //  no simd       12       23        0
    fn flat_bulk_norm_squared(self) -> Scalar {
        let flat_bulk_thing = self.flat_bulk().wedge(RoundPoint::from_groups(/* e1, e2, e3, e4 */ Simd32x3::from(0.0).extend_to_4(1.0), /* e5 */ 0.0));
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
    //      f32        7       11        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        7       13        0
    //  no simd        7       19        0
    fn flat_bulk_norm_squared(self) -> Scalar {
        let flat_bulk_thing = self.flat_bulk().wedge(RoundPoint::from_groups(/* e1, e2, e3, e4 */ Simd32x3::from(0.0).extend_to_4(1.0), /* e5 */ 0.0));
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
    //      f32       17       23        0
    //    simd3        2        6        0
    //    simd4        0        2        0
    // Totals...
    // yes simd       19       31        0
    //  no simd       23       49        0
    fn flat_bulk_norm_squared(self) -> Scalar {
        let flat_bulk_thing = self.flat_bulk().wedge(RoundPoint::from_groups(/* e1, e2, e3, e4 */ Simd32x3::from(0.0).extend_to_4(1.0), /* e5 */ 0.0));
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
    //      f32       46       58        0
    //    simd3        6       10        0
    //    simd4        4        6        0
    // Totals...
    // yes simd       56       74        0
    //  no simd       80      112        0
    fn flat_bulk_norm_squared(self) -> Scalar {
        let flat_bulk_thing = self.flat_bulk().wedge(RoundPoint::from_groups(/* e1, e2, e3, e4 */ Simd32x3::from(0.0).extend_to_4(1.0), /* e5 */ 0.0));
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
    //    simd4        0        1        0
    // Totals...
    // yes simd       15       19        0
    //  no simd       15       22        0
    fn flat_bulk_norm_squared(self) -> Scalar {
        let flat_bulk_thing = self.flat_bulk().wedge(RoundPoint::from_groups(/* e1, e2, e3, e4 */ Simd32x3::from(0.0).extend_to_4(1.0), /* e5 */ 0.0));
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
        let flat_bulk_thing = self.flat_bulk().wedge(RoundPoint::from_groups(/* e1, e2, e3, e4 */ Simd32x3::from(0.0).extend_to_4(1.0), /* e5 */ 0.0));
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
    //    simd4        0        1        0
    // Totals...
    // yes simd       15       19        0
    //  no simd       15       22        0
    fn flat_bulk_norm_squared(self) -> Scalar {
        let flat_bulk_thing = self.flat_bulk().wedge(RoundPoint::from_groups(/* e1, e2, e3, e4 */ Simd32x3::from(0.0).extend_to_4(1.0), /* e5 */ 0.0));
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
    //      f32       17       23        0
    //    simd3        2        6        0
    //    simd4        0        2        0
    // Totals...
    // yes simd       19       31        0
    //  no simd       23       49        0
    fn flat_bulk_norm_squared(self) -> Scalar {
        let flat_bulk_thing = self.flat_bulk().wedge(RoundPoint::from_groups(/* e1, e2, e3, e4 */ Simd32x3::from(0.0).extend_to_4(1.0), /* e5 */ 0.0));
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
    //      f32        5       13        0
    //    simd3        1        2        0
    //    simd4        1        1        0
    // Totals...
    // yes simd        7       16        0
    //  no simd       12       23        0
    fn flat_bulk_norm_squared(self) -> Scalar {
        let flat_bulk_thing = self.flat_bulk().wedge(RoundPoint::from_groups(/* e1, e2, e3, e4 */ Simd32x3::from(0.0).extend_to_4(1.0), /* e5 */ 0.0));
        return flat_bulk_thing.dot_product(flat_bulk_thing);
    }
}
