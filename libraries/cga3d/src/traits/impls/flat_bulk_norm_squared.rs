use crate::traits::FlatBulk;
use crate::traits::ScalarProduct;
use crate::traits::Wedge;
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
    //    simd3        0        1        0
    // Totals...
    // yes simd        2        4        0
    //  no simd        2        6        0
    fn flat_bulk_norm_squared(self) -> Scalar {
        let flat_bulk_thing = self.flat_bulk().wedge(DualNum::from_groups(/* e4, e12345 */ Simd32x2::from([1.0, 0.0])));
        return flat_bulk_thing.scalar_product(flat_bulk_thing);
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
    //      f32       14       25        0
    //    simd3        0        2        0
    // Totals...
    // yes simd       14       27        0
    //  no simd       14       31        0
    fn flat_bulk_norm_squared(self) -> Scalar {
        let flat_bulk_thing = self.flat_bulk().wedge(DualNum::from_groups(/* e4, e12345 */ Simd32x2::from([1.0, 0.0])));
        return flat_bulk_thing.scalar_product(flat_bulk_thing);
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
    //      add/sub      mul      div
    // f32        4       13        0
    fn flat_bulk_norm_squared(self) -> Scalar {
        let flat_bulk_thing = self.flat_bulk().wedge(DualNum::from_groups(/* e4, e12345 */ Simd32x2::from([1.0, 0.0])));
        return flat_bulk_thing.scalar_product(flat_bulk_thing);
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
    //      f32       14       25        0
    //    simd3        0        2        0
    // Totals...
    // yes simd       14       27        0
    //  no simd       14       31        0
    fn flat_bulk_norm_squared(self) -> Scalar {
        let flat_bulk_thing = self.flat_bulk().wedge(DualNum::from_groups(/* e4, e12345 */ Simd32x2::from([1.0, 0.0])));
        return flat_bulk_thing.scalar_product(flat_bulk_thing);
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
    //    simd3        0        1        0
    // Totals...
    // yes simd        2        4        0
    //  no simd        2        6        0
    fn flat_bulk_norm_squared(self) -> Scalar {
        let flat_bulk_thing = self.flat_bulk().wedge(DualNum::from_groups(/* e4, e12345 */ Simd32x2::from([1.0, 0.0])));
        return flat_bulk_thing.scalar_product(flat_bulk_thing);
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
    //      f32        3        4        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        3        5        0
    //  no simd        3        8        0
    fn flat_bulk_norm_squared(self) -> Scalar {
        let flat_bulk_thing = self.flat_bulk().wedge(DualNum::from_groups(/* e4, e12345 */ Simd32x2::from([1.0, 0.0])));
        return flat_bulk_thing.scalar_product(flat_bulk_thing);
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
    //      f32        9       12        0
    //    simd3        0        2        0
    // Totals...
    // yes simd        9       14        0
    //  no simd        9       18        0
    fn flat_bulk_norm_squared(self) -> Scalar {
        let flat_bulk_thing = self.flat_bulk().wedge(DualNum::from_groups(/* e4, e12345 */ Simd32x2::from([1.0, 0.0])));
        return flat_bulk_thing.scalar_product(flat_bulk_thing);
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
    //      add/sub      mul      div
    // f32        4       13        0
    fn flat_bulk_norm_squared(self) -> Scalar {
        let flat_bulk_thing = self.flat_bulk().wedge(DualNum::from_groups(/* e4, e12345 */ Simd32x2::from([1.0, 0.0])));
        return flat_bulk_thing.scalar_product(flat_bulk_thing);
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
    //      add/sub      mul      div
    // f32        4       13        0
    fn flat_bulk_norm_squared(self) -> Scalar {
        let flat_bulk_thing = self.flat_bulk().wedge(DualNum::from_groups(/* e4, e12345 */ Simd32x2::from([1.0, 0.0])));
        return flat_bulk_thing.scalar_product(flat_bulk_thing);
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
    //    simd3        0        1        0
    // Totals...
    // yes simd        2        4        0
    //  no simd        2        6        0
    fn flat_bulk_norm_squared(self) -> Scalar {
        let flat_bulk_thing = self.flat_bulk().wedge(DualNum::from_groups(/* e4, e12345 */ Simd32x2::from([1.0, 0.0])));
        return flat_bulk_thing.scalar_product(flat_bulk_thing);
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
    //      f32        3        4        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        3        5        0
    //  no simd        3        8        0
    fn flat_bulk_norm_squared(self) -> Scalar {
        let flat_bulk_thing = self.flat_bulk().wedge(DualNum::from_groups(/* e4, e12345 */ Simd32x2::from([1.0, 0.0])));
        return flat_bulk_thing.scalar_product(flat_bulk_thing);
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
    //    simd3        0        1        0
    // Totals...
    // yes simd        2        4        0
    //  no simd        2        6        0
    fn flat_bulk_norm_squared(self) -> Scalar {
        let flat_bulk_thing = self.flat_bulk().wedge(DualNum::from_groups(/* e4, e12345 */ Simd32x2::from([1.0, 0.0])));
        return flat_bulk_thing.scalar_product(flat_bulk_thing);
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
    //      f32        3        4        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        3        5        0
    //  no simd        3        8        0
    fn flat_bulk_norm_squared(self) -> Scalar {
        let flat_bulk_thing = self.flat_bulk().wedge(DualNum::from_groups(/* e4, e12345 */ Simd32x2::from([1.0, 0.0])));
        return flat_bulk_thing.scalar_product(flat_bulk_thing);
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
    //      add/sub      mul      div
    // f32        4       13        0
    fn flat_bulk_norm_squared(self) -> Scalar {
        let flat_bulk_thing = self.flat_bulk().wedge(DualNum::from_groups(/* e4, e12345 */ Simd32x2::from([1.0, 0.0])));
        return flat_bulk_thing.scalar_product(flat_bulk_thing);
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
    //      f32       14       25        0
    //    simd3        0        2        0
    // Totals...
    // yes simd       14       27        0
    //  no simd       14       31        0
    fn flat_bulk_norm_squared(self) -> Scalar {
        let flat_bulk_thing = self.flat_bulk().wedge(DualNum::from_groups(/* e4, e12345 */ Simd32x2::from([1.0, 0.0])));
        return flat_bulk_thing.scalar_product(flat_bulk_thing);
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
    //      f32       32       48        0
    //    simd3        0        3        0
    // Totals...
    // yes simd       32       51        0
    //  no simd       32       57        0
    fn flat_bulk_norm_squared(self) -> Scalar {
        let flat_bulk_thing = self.flat_bulk().wedge(DualNum::from_groups(/* e4, e12345 */ Simd32x2::from([1.0, 0.0])));
        return flat_bulk_thing.scalar_product(flat_bulk_thing);
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
    //      add/sub      mul      div
    // f32        0        3        0
    fn flat_bulk_norm_squared(self) -> Scalar {
        let flat_bulk_thing = self.flat_bulk().wedge(DualNum::from_groups(/* e4, e12345 */ Simd32x2::from([1.0, 0.0])));
        return flat_bulk_thing.scalar_product(flat_bulk_thing);
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
    //      f32        9       12        0
    //    simd3        0        2        0
    // Totals...
    // yes simd        9       14        0
    //  no simd        9       18        0
    fn flat_bulk_norm_squared(self) -> Scalar {
        let flat_bulk_thing = self.flat_bulk().wedge(DualNum::from_groups(/* e4, e12345 */ Simd32x2::from([1.0, 0.0])));
        return flat_bulk_thing.scalar_product(flat_bulk_thing);
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
    //      add/sub      mul      div
    // f32        0        3        0
    fn flat_bulk_norm_squared(self) -> Scalar {
        let flat_bulk_thing = self.flat_bulk().wedge(DualNum::from_groups(/* e4, e12345 */ Simd32x2::from([1.0, 0.0])));
        return flat_bulk_thing.scalar_product(flat_bulk_thing);
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
    //      f32       14       25        0
    //    simd3        0        2        0
    // Totals...
    // yes simd       14       27        0
    //  no simd       14       31        0
    fn flat_bulk_norm_squared(self) -> Scalar {
        let flat_bulk_thing = self.flat_bulk().wedge(DualNum::from_groups(/* e4, e12345 */ Simd32x2::from([1.0, 0.0])));
        return flat_bulk_thing.scalar_product(flat_bulk_thing);
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
    //      f32        3        4        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        3        5        0
    //  no simd        3        8        0
    fn flat_bulk_norm_squared(self) -> Scalar {
        let flat_bulk_thing = self.flat_bulk().wedge(DualNum::from_groups(/* e4, e12345 */ Simd32x2::from([1.0, 0.0])));
        return flat_bulk_thing.scalar_product(flat_bulk_thing);
    }
}
