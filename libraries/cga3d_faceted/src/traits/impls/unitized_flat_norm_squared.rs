use crate::traits::FlatBulkNormSquared;
use crate::traits::FlatWeightNormSquared;
// Note on Operative Statistics:
// Operative Statistics are not a precise predictor of performance or performance comparisons.
// This is due to varying hardware capabilities and compiler optimizations.
// As always, where performance is a concern, there is no substitute for
// real measurements on real work-loads on real hardware.
// Disclaimer aside, enjoy the fun information =)
//
// Total Implementations: 30
//
// Yes SIMD:   add/sub     mul     div
//  Minimum:         2       5       1
//   Median:         5       9       1
//  Average:         6      10       1
//  Maximum:        62      73       1
//
//  No SIMD:   add/sub     mul     div
//  Minimum:         2       6       1
//   Median:         5      12       1
//  Average:         6      14       1
//  Maximum:        62      90       1
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
    //    simd3        0        1        0
    // Totals...
    // yes simd        2        5        1
    //  no simd        2        7        1
    fn unitized_flat_norm_squared(self) -> f32 {
        use crate::elements::*;
        return self.flat_bulk_norm_squared()[scalar] / (self.flat_weight_norm_squared()[e12345]);
    }
}
impl std::ops::Div<UnitizedFlatNormSquaredPrefixOrPostfix> for AntiCircleRotorAtInfinity {
    type Output = f32;
    fn div(self, _rhs: UnitizedFlatNormSquaredPrefixOrPostfix) -> Self::Output {
        self.unitized_flat_norm_squared()
    }
}
impl UnitizedFlatNormSquared for AntiCircleRotorAtInfinity {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        4        1
    //    simd3        0        1        0
    // Totals...
    // yes simd        2        5        1
    //  no simd        2        7        1
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
    //      f32        5        7        1
    //    simd4        0        2        0
    // Totals...
    // yes simd        5        9        1
    //  no simd        5       15        1
    fn unitized_flat_norm_squared(self) -> f32 {
        use crate::elements::*;
        return self.flat_bulk_norm_squared()[scalar] / (self.flat_weight_norm_squared()[e12345]);
    }
}
impl std::ops::Div<UnitizedFlatNormSquaredPrefixOrPostfix> for AntiDipoleInversionAtInfinity {
    type Output = f32;
    fn div(self, _rhs: UnitizedFlatNormSquaredPrefixOrPostfix) -> Self::Output {
        self.unitized_flat_norm_squared()
    }
}
impl UnitizedFlatNormSquared for AntiDipoleInversionAtInfinity {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        5        7        1
    //    simd4        0        2        0
    // Totals...
    // yes simd        5        9        1
    //  no simd        5       15        1
    fn unitized_flat_norm_squared(self) -> f32 {
        use crate::elements::*;
        return self.flat_bulk_norm_squared()[scalar] / (self.flat_weight_norm_squared()[e12345]);
    }
}
impl std::ops::Div<UnitizedFlatNormSquaredPrefixOrPostfix> for AntiDipoleInversionOrthogonalOrigin {
    type Output = f32;
    fn div(self, _rhs: UnitizedFlatNormSquaredPrefixOrPostfix) -> Self::Output {
        self.unitized_flat_norm_squared()
    }
}
impl UnitizedFlatNormSquared for AntiDipoleInversionOrthogonalOrigin {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        5        7        1
    //    simd4        0        2        0
    // Totals...
    // yes simd        5        9        1
    //  no simd        5       15        1
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
    //      f32        4        6        1
    //    simd3        0        2        0
    // Totals...
    // yes simd        4        8        1
    //  no simd        4       12        1
    fn unitized_flat_norm_squared(self) -> f32 {
        use crate::elements::*;
        return self.flat_bulk_norm_squared()[scalar] / (self.flat_weight_norm_squared()[e12345]);
    }
}
impl std::ops::Div<UnitizedFlatNormSquaredPrefixOrPostfix> for CircleAligningOrigin {
    type Output = f32;
    fn div(self, _rhs: UnitizedFlatNormSquaredPrefixOrPostfix) -> Self::Output {
        self.unitized_flat_norm_squared()
    }
}
impl UnitizedFlatNormSquared for CircleAligningOrigin {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        6        1
    //    simd3        0        2        0
    // Totals...
    // yes simd        4        8        1
    //  no simd        4       12        1
    fn unitized_flat_norm_squared(self) -> f32 {
        use crate::elements::*;
        return self.flat_bulk_norm_squared()[scalar] / (self.flat_weight_norm_squared()[e12345]);
    }
}
impl std::ops::Div<UnitizedFlatNormSquaredPrefixOrPostfix> for CircleAtInfinity {
    type Output = f32;
    fn div(self, _rhs: UnitizedFlatNormSquaredPrefixOrPostfix) -> Self::Output {
        self.unitized_flat_norm_squared()
    }
}
impl UnitizedFlatNormSquared for CircleAtInfinity {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        6        1
    //    simd3        0        2        0
    // Totals...
    // yes simd        4        8        1
    //  no simd        4       12        1
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
    //      f32        5        7        1
    //    simd3        0        2        0
    // Totals...
    // yes simd        5        9        1
    //  no simd        5       13        1
    fn unitized_flat_norm_squared(self) -> f32 {
        use crate::elements::*;
        return self.flat_bulk_norm_squared()[scalar] / (self.flat_weight_norm_squared()[e12345]);
    }
}
impl std::ops::Div<UnitizedFlatNormSquaredPrefixOrPostfix> for CircleRotorAligningOrigin {
    type Output = f32;
    fn div(self, _rhs: UnitizedFlatNormSquaredPrefixOrPostfix) -> Self::Output {
        self.unitized_flat_norm_squared()
    }
}
impl UnitizedFlatNormSquared for CircleRotorAligningOrigin {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        5        7        1
    //    simd3        0        2        0
    // Totals...
    // yes simd        5        9        1
    //  no simd        5       13        1
    fn unitized_flat_norm_squared(self) -> f32 {
        use crate::elements::*;
        return self.flat_bulk_norm_squared()[scalar] / (self.flat_weight_norm_squared()[e12345]);
    }
}
impl std::ops::Div<UnitizedFlatNormSquaredPrefixOrPostfix> for CircleRotorAligningOriginAtInfinity {
    type Output = f32;
    fn div(self, _rhs: UnitizedFlatNormSquaredPrefixOrPostfix) -> Self::Output {
        self.unitized_flat_norm_squared()
    }
}
impl UnitizedFlatNormSquared for CircleRotorAligningOriginAtInfinity {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        5        7        1
    //    simd3        0        2        0
    // Totals...
    // yes simd        5        9        1
    //  no simd        5       13        1
    fn unitized_flat_norm_squared(self) -> f32 {
        use crate::elements::*;
        return self.flat_bulk_norm_squared()[scalar] / (self.flat_weight_norm_squared()[e12345]);
    }
}
impl std::ops::Div<UnitizedFlatNormSquaredPrefixOrPostfix> for CircleRotorAtInfinity {
    type Output = f32;
    fn div(self, _rhs: UnitizedFlatNormSquaredPrefixOrPostfix) -> Self::Output {
        self.unitized_flat_norm_squared()
    }
}
impl UnitizedFlatNormSquared for CircleRotorAtInfinity {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        5        7        1
    //    simd3        0        2        0
    // Totals...
    // yes simd        5        9        1
    //  no simd        5       13        1
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
    //    simd3        0        1        0
    // Totals...
    // yes simd        2        5        1
    //  no simd        2        7        1
    fn unitized_flat_norm_squared(self) -> f32 {
        use crate::elements::*;
        return self.flat_bulk_norm_squared()[scalar] / (self.flat_weight_norm_squared()[e12345]);
    }
}
impl std::ops::Div<UnitizedFlatNormSquaredPrefixOrPostfix> for DipoleAligningOrigin {
    type Output = f32;
    fn div(self, _rhs: UnitizedFlatNormSquaredPrefixOrPostfix) -> Self::Output {
        self.unitized_flat_norm_squared()
    }
}
impl UnitizedFlatNormSquared for DipoleAligningOrigin {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        4        1
    //    simd3        0        1        0
    // Totals...
    // yes simd        2        5        1
    //  no simd        2        7        1
    fn unitized_flat_norm_squared(self) -> f32 {
        use crate::elements::*;
        return self.flat_bulk_norm_squared()[scalar] / (self.flat_weight_norm_squared()[e12345]);
    }
}
impl std::ops::Div<UnitizedFlatNormSquaredPrefixOrPostfix> for DipoleAtInfinity {
    type Output = f32;
    fn div(self, _rhs: UnitizedFlatNormSquaredPrefixOrPostfix) -> Self::Output {
        self.unitized_flat_norm_squared()
    }
}
impl UnitizedFlatNormSquared for DipoleAtInfinity {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        4        1
    //    simd3        0        1        0
    // Totals...
    // yes simd        2        5        1
    //  no simd        2        7        1
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
    //      f32        6        8        1
    //    simd4        0        1        0
    // Totals...
    // yes simd        6        9        1
    //  no simd        6       12        1
    fn unitized_flat_norm_squared(self) -> f32 {
        use crate::elements::*;
        return self.flat_bulk_norm_squared()[scalar] / (self.flat_weight_norm_squared()[e12345]);
    }
}
impl std::ops::Div<UnitizedFlatNormSquaredPrefixOrPostfix> for DipoleInversionAligningOrigin {
    type Output = f32;
    fn div(self, _rhs: UnitizedFlatNormSquaredPrefixOrPostfix) -> Self::Output {
        self.unitized_flat_norm_squared()
    }
}
impl UnitizedFlatNormSquared for DipoleInversionAligningOrigin {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        6        8        1
    //    simd4        0        1        0
    // Totals...
    // yes simd        6        9        1
    //  no simd        6       12        1
    fn unitized_flat_norm_squared(self) -> f32 {
        use crate::elements::*;
        return self.flat_bulk_norm_squared()[scalar] / (self.flat_weight_norm_squared()[e12345]);
    }
}
impl std::ops::Div<UnitizedFlatNormSquaredPrefixOrPostfix> for DipoleInversionAtInfinity {
    type Output = f32;
    fn div(self, _rhs: UnitizedFlatNormSquaredPrefixOrPostfix) -> Self::Output {
        self.unitized_flat_norm_squared()
    }
}
impl UnitizedFlatNormSquared for DipoleInversionAtInfinity {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        6        8        1
    //    simd4        0        1        0
    // Totals...
    // yes simd        6        9        1
    //  no simd        6       12        1
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
    //    simd3        0        1        0
    // Totals...
    // yes simd        2        5        1
    //  no simd        2        7        1
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
    //      f32        6        8        1
    //    simd4        0        1        0
    // Totals...
    // yes simd        6        9        1
    //  no simd        6       12        1
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
    //      f32        4        6        1
    //    simd3        0        2        0
    // Totals...
    // yes simd        4        8        1
    //  no simd        4       12        1
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
    //      f32        6        8        1
    //    simd4        0        2        0
    // Totals...
    // yes simd        6       10        1
    //  no simd        6       16        1
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
    //      f32       62       66        1
    //    simd2        0        1        0
    //    simd3        0        2        0
    //    simd4        0        4        0
    // Totals...
    // yes simd       62       73        1
    //  no simd       62       90        1
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
    //      add/sub      mul      div
    // f32        2        6        1
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
    //      add/sub      mul      div
    // f32        2        6        1
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
    //      f32        6        8        1
    //    simd4        0        2        0
    // Totals...
    // yes simd        6       10        1
    //  no simd        6       16        1
    fn unitized_flat_norm_squared(self) -> f32 {
        use crate::elements::*;
        return self.flat_bulk_norm_squared()[scalar] / (self.flat_weight_norm_squared()[e12345]);
    }
}
impl std::ops::Div<UnitizedFlatNormSquaredPrefixOrPostfix> for VersorEvenAligningOrigin {
    type Output = f32;
    fn div(self, _rhs: UnitizedFlatNormSquaredPrefixOrPostfix) -> Self::Output {
        self.unitized_flat_norm_squared()
    }
}
impl UnitizedFlatNormSquared for VersorEvenAligningOrigin {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        6        8        1
    //    simd4        0        2        0
    // Totals...
    // yes simd        6       10        1
    //  no simd        6       16        1
    fn unitized_flat_norm_squared(self) -> f32 {
        use crate::elements::*;
        return self.flat_bulk_norm_squared()[scalar] / (self.flat_weight_norm_squared()[e12345]);
    }
}
impl std::ops::Div<UnitizedFlatNormSquaredPrefixOrPostfix> for VersorEvenAtInfinity {
    type Output = f32;
    fn div(self, _rhs: UnitizedFlatNormSquaredPrefixOrPostfix) -> Self::Output {
        self.unitized_flat_norm_squared()
    }
}
impl UnitizedFlatNormSquared for VersorEvenAtInfinity {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        6        8        1
    //    simd4        0        2        0
    // Totals...
    // yes simd        6       10        1
    //  no simd        6       16        1
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
    //      f32        6        8        1
    //    simd4        0        1        0
    // Totals...
    // yes simd        6        9        1
    //  no simd        6       12        1
    fn unitized_flat_norm_squared(self) -> f32 {
        use crate::elements::*;
        return self.flat_bulk_norm_squared()[scalar] / (self.flat_weight_norm_squared()[e12345]);
    }
}
impl std::ops::Div<UnitizedFlatNormSquaredPrefixOrPostfix> for VersorOddAtInfinity {
    type Output = f32;
    fn div(self, _rhs: UnitizedFlatNormSquaredPrefixOrPostfix) -> Self::Output {
        self.unitized_flat_norm_squared()
    }
}
impl UnitizedFlatNormSquared for VersorOddAtInfinity {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        6        8        1
    //    simd4        0        1        0
    // Totals...
    // yes simd        6        9        1
    //  no simd        6       12        1
    fn unitized_flat_norm_squared(self) -> f32 {
        use crate::elements::*;
        return self.flat_bulk_norm_squared()[scalar] / (self.flat_weight_norm_squared()[e12345]);
    }
}
