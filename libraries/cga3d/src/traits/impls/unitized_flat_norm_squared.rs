use crate::traits::FlatBulkNormSquared;
use crate::traits::FlatWeightNormSquared;
// Note on Operative Statistics:
// Operative Statistics are not a precise predictor of performance or performance comparisons.
// This is due to varying hardware capabilities and compiler optimizations.
// As always, where performance is a concern, there is no substitute for
// real measurements on real work-loads on real hardware.
// Disclaimer aside, enjoy the fun information =)
//
// Total Implementations: 16
//
// Yes SIMD:   add/sub     mul     div
//  Minimum:         0       5       1
//   Median:        10      20       1
//  Average:        16      24       1
//  Maximum:        87     107       1
//
//  No SIMD:   add/sub     mul     div
//  Minimum:         0      11       1
//   Median:        15      25       1
//  Average:        19      34       1
//  Maximum:       111     144       1
impl std::ops::Div<UnitizedFlatNormSquaredPrefixOrPostfix> for AntiCircleRotor {
    type Output = f32;
    fn div(self, _rhs: UnitizedFlatNormSquaredPrefixOrPostfix) -> Self::Output {
        self.unitized_flat_norm_squared()
    }
}
impl UnitizedFlatNormSquared for AntiCircleRotor {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        4        1
    //    simd3        2        4        0
    // Totals...
    // yes simd        4        8        1
    //  no simd        8       16        1
    fn unitized_flat_norm_squared(self) -> f32 {
        use crate::elements::*;
        return self.flat_bulk_norm_squared()[scalar] / (self.flat_weight_norm_squared()[e12345]);
    }
}
impl std::ops::Div<UnitizedFlatNormSquaredPrefixOrPostfix> for AntiDipoleInversion {
    type Output = f32;
    fn div(self, _rhs: UnitizedFlatNormSquaredPrefixOrPostfix) -> Self::Output {
        self.unitized_flat_norm_squared()
    }
}
impl UnitizedFlatNormSquared for AntiDipoleInversion {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       19       26        1
    //    simd3        2        8        0
    // Totals...
    // yes simd       21       34        1
    //  no simd       25       50        1
    fn unitized_flat_norm_squared(self) -> f32 {
        use crate::elements::*;
        return self.flat_bulk_norm_squared()[scalar] / (self.flat_weight_norm_squared()[e12345]);
    }
}
impl std::ops::Div<UnitizedFlatNormSquaredPrefixOrPostfix> for Circle {
    type Output = f32;
    fn div(self, _rhs: UnitizedFlatNormSquaredPrefixOrPostfix) -> Self::Output {
        self.unitized_flat_norm_squared()
    }
}
impl UnitizedFlatNormSquared for Circle {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        9       14        1
    //    simd3        0        2        0
    // Totals...
    // yes simd        9       16        1
    //  no simd        9       20        1
    fn unitized_flat_norm_squared(self) -> f32 {
        use crate::elements::*;
        return self.flat_bulk_norm_squared()[scalar] / (self.flat_weight_norm_squared()[e12345]);
    }
}
impl std::ops::Div<UnitizedFlatNormSquaredPrefixOrPostfix> for CircleRotor {
    type Output = f32;
    fn div(self, _rhs: UnitizedFlatNormSquaredPrefixOrPostfix) -> Self::Output {
        self.unitized_flat_norm_squared()
    }
}
impl UnitizedFlatNormSquared for CircleRotor {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       10       15        1
    //    simd3        0        2        0
    // Totals...
    // yes simd       10       17        1
    //  no simd       10       21        1
    fn unitized_flat_norm_squared(self) -> f32 {
        use crate::elements::*;
        return self.flat_bulk_norm_squared()[scalar] / (self.flat_weight_norm_squared()[e12345]);
    }
}
impl std::ops::Div<UnitizedFlatNormSquaredPrefixOrPostfix> for Dipole {
    type Output = f32;
    fn div(self, _rhs: UnitizedFlatNormSquaredPrefixOrPostfix) -> Self::Output {
        self.unitized_flat_norm_squared()
    }
}
impl UnitizedFlatNormSquared for Dipole {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        4        1
    //    simd3        2        4        0
    // Totals...
    // yes simd        4        8        1
    //  no simd        8       16        1
    fn unitized_flat_norm_squared(self) -> f32 {
        use crate::elements::*;
        return self.flat_bulk_norm_squared()[scalar] / (self.flat_weight_norm_squared()[e12345]);
    }
}
impl std::ops::Div<UnitizedFlatNormSquaredPrefixOrPostfix> for DipoleInversion {
    type Output = f32;
    fn div(self, _rhs: UnitizedFlatNormSquaredPrefixOrPostfix) -> Self::Output {
        self.unitized_flat_norm_squared()
    }
}
impl UnitizedFlatNormSquared for DipoleInversion {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        8       17        1
    //    simd3        1        2        0
    //    simd4        1        1        0
    // Totals...
    // yes simd       10       20        1
    //  no simd       15       27        1
    fn unitized_flat_norm_squared(self) -> f32 {
        use crate::elements::*;
        return self.flat_bulk_norm_squared()[scalar] / (self.flat_weight_norm_squared()[e12345]);
    }
}
impl std::ops::Div<UnitizedFlatNormSquaredPrefixOrPostfix> for DualNum {
    type Output = f32;
    fn div(self, _rhs: UnitizedFlatNormSquaredPrefixOrPostfix) -> Self::Output {
        self.unitized_flat_norm_squared()
    }
}
impl UnitizedFlatNormSquared for DualNum {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        3        1
    //    simd4        0        2        0
    // Totals...
    // yes simd        0        5        1
    //  no simd        0       11        1
    fn unitized_flat_norm_squared(self) -> f32 {
        use crate::elements::*;
        return self.flat_bulk_norm_squared()[scalar] / (self.flat_weight_norm_squared()[e12345]);
    }
}
impl std::ops::Div<UnitizedFlatNormSquaredPrefixOrPostfix> for FlatPoint {
    type Output = f32;
    fn div(self, _rhs: UnitizedFlatNormSquaredPrefixOrPostfix) -> Self::Output {
        self.unitized_flat_norm_squared()
    }
}
impl UnitizedFlatNormSquared for FlatPoint {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        4        1
    //    simd3        2        4        0
    // Totals...
    // yes simd        4        8        1
    //  no simd        8       16        1
    fn unitized_flat_norm_squared(self) -> f32 {
        use crate::elements::*;
        return self.flat_bulk_norm_squared()[scalar] / (self.flat_weight_norm_squared()[e12345]);
    }
}
impl std::ops::Div<UnitizedFlatNormSquaredPrefixOrPostfix> for Flector {
    type Output = f32;
    fn div(self, _rhs: UnitizedFlatNormSquaredPrefixOrPostfix) -> Self::Output {
        self.unitized_flat_norm_squared()
    }
}
impl UnitizedFlatNormSquared for Flector {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        8       17        1
    //    simd3        1        2        0
    //    simd4        1        1        0
    // Totals...
    // yes simd       10       20        1
    //  no simd       15       27        1
    fn unitized_flat_norm_squared(self) -> f32 {
        use crate::elements::*;
        return self.flat_bulk_norm_squared()[scalar] / (self.flat_weight_norm_squared()[e12345]);
    }
}
impl std::ops::Div<UnitizedFlatNormSquaredPrefixOrPostfix> for Line {
    type Output = f32;
    fn div(self, _rhs: UnitizedFlatNormSquaredPrefixOrPostfix) -> Self::Output {
        self.unitized_flat_norm_squared()
    }
}
impl UnitizedFlatNormSquared for Line {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        9       14        1
    //    simd3        0        2        0
    // Totals...
    // yes simd        9       16        1
    //  no simd        9       20        1
    fn unitized_flat_norm_squared(self) -> f32 {
        use crate::elements::*;
        return self.flat_bulk_norm_squared()[scalar] / (self.flat_weight_norm_squared()[e12345]);
    }
}
impl std::ops::Div<UnitizedFlatNormSquaredPrefixOrPostfix> for Motor {
    type Output = f32;
    fn div(self, _rhs: UnitizedFlatNormSquaredPrefixOrPostfix) -> Self::Output {
        self.unitized_flat_norm_squared()
    }
}
impl UnitizedFlatNormSquared for Motor {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       20       27        1
    //    simd3        2        8        0
    // Totals...
    // yes simd       22       35        1
    //  no simd       26       51        1
    fn unitized_flat_norm_squared(self) -> f32 {
        use crate::elements::*;
        return self.flat_bulk_norm_squared()[scalar] / (self.flat_weight_norm_squared()[e12345]);
    }
}
impl std::ops::Div<UnitizedFlatNormSquaredPrefixOrPostfix> for MultiVector {
    type Output = f32;
    fn div(self, _rhs: UnitizedFlatNormSquaredPrefixOrPostfix) -> Self::Output {
        self.unitized_flat_norm_squared()
    }
}
impl UnitizedFlatNormSquared for MultiVector {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       77       91        1
    //    simd3        6       11        0
    //    simd4        4        5        0
    // Totals...
    // yes simd       87      107        1
    //  no simd      111      144        1
    fn unitized_flat_norm_squared(self) -> f32 {
        use crate::elements::*;
        return self.flat_bulk_norm_squared()[scalar] / (self.flat_weight_norm_squared()[e12345]);
    }
}
impl std::ops::Div<UnitizedFlatNormSquaredPrefixOrPostfix> for Plane {
    type Output = f32;
    fn div(self, _rhs: UnitizedFlatNormSquaredPrefixOrPostfix) -> Self::Output {
        self.unitized_flat_norm_squared()
    }
}
impl UnitizedFlatNormSquared for Plane {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       17       21        1
    //    simd4        0        1        0
    // Totals...
    // yes simd       17       22        1
    //  no simd       17       25        1
    fn unitized_flat_norm_squared(self) -> f32 {
        use crate::elements::*;
        return self.flat_bulk_norm_squared()[scalar] / (self.flat_weight_norm_squared()[e12345]);
    }
}
impl std::ops::Div<UnitizedFlatNormSquaredPrefixOrPostfix> for Sphere {
    type Output = f32;
    fn div(self, _rhs: UnitizedFlatNormSquaredPrefixOrPostfix) -> Self::Output {
        self.unitized_flat_norm_squared()
    }
}
impl UnitizedFlatNormSquared for Sphere {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       17       21        1
    //    simd4        0        1        0
    // Totals...
    // yes simd       17       22        1
    //  no simd       17       25        1
    fn unitized_flat_norm_squared(self) -> f32 {
        use crate::elements::*;
        return self.flat_bulk_norm_squared()[scalar] / (self.flat_weight_norm_squared()[e12345]);
    }
}
impl std::ops::Div<UnitizedFlatNormSquaredPrefixOrPostfix> for VersorEven {
    type Output = f32;
    fn div(self, _rhs: UnitizedFlatNormSquaredPrefixOrPostfix) -> Self::Output {
        self.unitized_flat_norm_squared()
    }
}
impl UnitizedFlatNormSquared for VersorEven {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       20       27        1
    //    simd3        2        8        0
    // Totals...
    // yes simd       22       35        1
    //  no simd       26       51        1
    fn unitized_flat_norm_squared(self) -> f32 {
        use crate::elements::*;
        return self.flat_bulk_norm_squared()[scalar] / (self.flat_weight_norm_squared()[e12345]);
    }
}
impl std::ops::Div<UnitizedFlatNormSquaredPrefixOrPostfix> for VersorOdd {
    type Output = f32;
    fn div(self, _rhs: UnitizedFlatNormSquaredPrefixOrPostfix) -> Self::Output {
        self.unitized_flat_norm_squared()
    }
}
impl UnitizedFlatNormSquared for VersorOdd {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        8       17        1
    //    simd3        1        2        0
    //    simd4        1        1        0
    // Totals...
    // yes simd       10       20        1
    //  no simd       15       27        1
    fn unitized_flat_norm_squared(self) -> f32 {
        use crate::elements::*;
        return self.flat_bulk_norm_squared()[scalar] / (self.flat_weight_norm_squared()[e12345]);
    }
}
