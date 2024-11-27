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
//  Minimum:         0       4       1
//   Median:        12      24       1
//  Average:        17      29       1
//  Maximum:        99     124       1
//
//  No SIMD:   add/sub     mul     div
//  Minimum:         0      10       1
//   Median:        15      27       1
//  Average:        19      37       1
//  Maximum:       111     144       1
impl std::ops::Div<unitized_flat_norm_squared> for AntiCircleRotor {
    type Output = f32;
    fn div(self, _rhs: unitized_flat_norm_squared) -> Self::Output {
        self.unitized_flat_norm_squared()
    }
}
impl UnitizedFlatNormSquared for AntiCircleRotor {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        5       11        1
    //    simd3        1        2        0
    // Totals...
    // yes simd        6       13        1
    //  no simd        8       17        1
    fn unitized_flat_norm_squared(self) -> f32 {
        use crate::elements::*;
        return (self.flat_bulk_norm_squared()[scalar] / (self.flat_weight_norm_squared()[e12345]));
    }
}
impl std::ops::Div<unitized_flat_norm_squared> for AntiDipoleInversion {
    type Output = f32;
    fn div(self, _rhs: unitized_flat_norm_squared) -> Self::Output {
        self.unitized_flat_norm_squared()
    }
}
impl UnitizedFlatNormSquared for AntiDipoleInversion {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       22       32        1
    //    simd3        1        4        0
    //    simd4        0        4        0
    // Totals...
    // yes simd       23       40        1
    //  no simd       25       60        1
    fn unitized_flat_norm_squared(self) -> f32 {
        use crate::elements::*;
        return (self.flat_bulk_norm_squared()[scalar] / (self.flat_weight_norm_squared()[e12345]));
    }
}
impl std::ops::Div<unitized_flat_norm_squared> for Circle {
    type Output = f32;
    fn div(self, _rhs: unitized_flat_norm_squared) -> Self::Output {
        self.unitized_flat_norm_squared()
    }
}
impl UnitizedFlatNormSquared for Circle {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        9       17        1
    //    simd4        0        1        0
    // Totals...
    // yes simd        9       18        1
    //  no simd        9       21        1
    fn unitized_flat_norm_squared(self) -> f32 {
        use crate::elements::*;
        return (self.flat_bulk_norm_squared()[scalar] / (self.flat_weight_norm_squared()[e12345]));
    }
}
impl std::ops::Div<unitized_flat_norm_squared> for CircleRotor {
    type Output = f32;
    fn div(self, _rhs: unitized_flat_norm_squared) -> Self::Output {
        self.unitized_flat_norm_squared()
    }
}
impl UnitizedFlatNormSquared for CircleRotor {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       10       18        1
    //    simd4        0        1        0
    // Totals...
    // yes simd       10       19        1
    //  no simd       10       22        1
    fn unitized_flat_norm_squared(self) -> f32 {
        use crate::elements::*;
        return (self.flat_bulk_norm_squared()[scalar] / (self.flat_weight_norm_squared()[e12345]));
    }
}
impl std::ops::Div<unitized_flat_norm_squared> for Dipole {
    type Output = f32;
    fn div(self, _rhs: unitized_flat_norm_squared) -> Self::Output {
        self.unitized_flat_norm_squared()
    }
}
impl UnitizedFlatNormSquared for Dipole {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        5       11        1
    //    simd3        1        2        0
    // Totals...
    // yes simd        6       13        1
    //  no simd        8       17        1
    fn unitized_flat_norm_squared(self) -> f32 {
        use crate::elements::*;
        return (self.flat_bulk_norm_squared()[scalar] / (self.flat_weight_norm_squared()[e12345]));
    }
}
impl std::ops::Div<unitized_flat_norm_squared> for DipoleInversion {
    type Output = f32;
    fn div(self, _rhs: unitized_flat_norm_squared) -> Self::Output {
        self.unitized_flat_norm_squared()
    }
}
impl UnitizedFlatNormSquared for DipoleInversion {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       11       23        1
    //    simd4        1        1        0
    // Totals...
    // yes simd       12       24        1
    //  no simd       15       27        1
    fn unitized_flat_norm_squared(self) -> f32 {
        use crate::elements::*;
        return (self.flat_bulk_norm_squared()[scalar] / (self.flat_weight_norm_squared()[e12345]));
    }
}
impl std::ops::Div<unitized_flat_norm_squared> for DualNum {
    type Output = f32;
    fn div(self, _rhs: unitized_flat_norm_squared) -> Self::Output {
        self.unitized_flat_norm_squared()
    }
}
impl UnitizedFlatNormSquared for DualNum {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        2        1
    //    simd4        0        2        0
    // Totals...
    // yes simd        0        4        1
    //  no simd        0       10        1
    fn unitized_flat_norm_squared(self) -> f32 {
        use crate::elements::*;
        return (self.flat_bulk_norm_squared()[scalar] / (self.flat_weight_norm_squared()[e12345]));
    }
}
impl std::ops::Div<unitized_flat_norm_squared> for FlatPoint {
    type Output = f32;
    fn div(self, _rhs: unitized_flat_norm_squared) -> Self::Output {
        self.unitized_flat_norm_squared()
    }
}
impl UnitizedFlatNormSquared for FlatPoint {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        5       11        1
    //    simd3        1        2        0
    // Totals...
    // yes simd        6       13        1
    //  no simd        8       17        1
    fn unitized_flat_norm_squared(self) -> f32 {
        use crate::elements::*;
        return (self.flat_bulk_norm_squared()[scalar] / (self.flat_weight_norm_squared()[e12345]));
    }
}
impl std::ops::Div<unitized_flat_norm_squared> for Flector {
    type Output = f32;
    fn div(self, _rhs: unitized_flat_norm_squared) -> Self::Output {
        self.unitized_flat_norm_squared()
    }
}
impl UnitizedFlatNormSquared for Flector {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       11       23        1
    //    simd4        1        1        0
    // Totals...
    // yes simd       12       24        1
    //  no simd       15       27        1
    fn unitized_flat_norm_squared(self) -> f32 {
        use crate::elements::*;
        return (self.flat_bulk_norm_squared()[scalar] / (self.flat_weight_norm_squared()[e12345]));
    }
}
impl std::ops::Div<unitized_flat_norm_squared> for Line {
    type Output = f32;
    fn div(self, _rhs: unitized_flat_norm_squared) -> Self::Output {
        self.unitized_flat_norm_squared()
    }
}
impl UnitizedFlatNormSquared for Line {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        9       17        1
    //    simd4        0        1        0
    // Totals...
    // yes simd        9       18        1
    //  no simd        9       21        1
    fn unitized_flat_norm_squared(self) -> f32 {
        use crate::elements::*;
        return (self.flat_bulk_norm_squared()[scalar] / (self.flat_weight_norm_squared()[e12345]));
    }
}
impl std::ops::Div<unitized_flat_norm_squared> for Motor {
    type Output = f32;
    fn div(self, _rhs: unitized_flat_norm_squared) -> Self::Output {
        self.unitized_flat_norm_squared()
    }
}
impl UnitizedFlatNormSquared for Motor {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       23       33        1
    //    simd3        1        4        0
    //    simd4        0        4        0
    // Totals...
    // yes simd       24       41        1
    //  no simd       26       61        1
    fn unitized_flat_norm_squared(self) -> f32 {
        use crate::elements::*;
        return (self.flat_bulk_norm_squared()[scalar] / (self.flat_weight_norm_squared()[e12345]));
    }
}
impl std::ops::Div<unitized_flat_norm_squared> for MultiVector {
    type Output = f32;
    fn div(self, _rhs: unitized_flat_norm_squared) -> Self::Output {
        self.unitized_flat_norm_squared()
    }
}
impl UnitizedFlatNormSquared for MultiVector {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       94      116        1
    //    simd3        3        4        0
    //    simd4        2        4        0
    // Totals...
    // yes simd       99      124        1
    //  no simd      111      144        1
    fn unitized_flat_norm_squared(self) -> f32 {
        use crate::elements::*;
        return (self.flat_bulk_norm_squared()[scalar] / (self.flat_weight_norm_squared()[e12345]));
    }
}
impl std::ops::Div<unitized_flat_norm_squared> for Plane {
    type Output = f32;
    fn div(self, _rhs: unitized_flat_norm_squared) -> Self::Output {
        self.unitized_flat_norm_squared()
    }
}
impl UnitizedFlatNormSquared for Plane {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       17       21        1
    //    simd4        0        3        0
    // Totals...
    // yes simd       17       24        1
    //  no simd       17       33        1
    fn unitized_flat_norm_squared(self) -> f32 {
        use crate::elements::*;
        return (self.flat_bulk_norm_squared()[scalar] / (self.flat_weight_norm_squared()[e12345]));
    }
}
impl std::ops::Div<unitized_flat_norm_squared> for Sphere {
    type Output = f32;
    fn div(self, _rhs: unitized_flat_norm_squared) -> Self::Output {
        self.unitized_flat_norm_squared()
    }
}
impl UnitizedFlatNormSquared for Sphere {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       17       21        1
    //    simd4        0        3        0
    // Totals...
    // yes simd       17       24        1
    //  no simd       17       33        1
    fn unitized_flat_norm_squared(self) -> f32 {
        use crate::elements::*;
        return (self.flat_bulk_norm_squared()[scalar] / (self.flat_weight_norm_squared()[e12345]));
    }
}
impl std::ops::Div<unitized_flat_norm_squared> for VersorEven {
    type Output = f32;
    fn div(self, _rhs: unitized_flat_norm_squared) -> Self::Output {
        self.unitized_flat_norm_squared()
    }
}
impl UnitizedFlatNormSquared for VersorEven {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       23       33        1
    //    simd3        1        4        0
    //    simd4        0        4        0
    // Totals...
    // yes simd       24       41        1
    //  no simd       26       61        1
    fn unitized_flat_norm_squared(self) -> f32 {
        use crate::elements::*;
        return (self.flat_bulk_norm_squared()[scalar] / (self.flat_weight_norm_squared()[e12345]));
    }
}
impl std::ops::Div<unitized_flat_norm_squared> for VersorOdd {
    type Output = f32;
    fn div(self, _rhs: unitized_flat_norm_squared) -> Self::Output {
        self.unitized_flat_norm_squared()
    }
}
impl UnitizedFlatNormSquared for VersorOdd {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       11       23        1
    //    simd4        1        1        0
    // Totals...
    // yes simd       12       24        1
    //  no simd       15       27        1
    fn unitized_flat_norm_squared(self) -> f32 {
        use crate::elements::*;
        return (self.flat_bulk_norm_squared()[scalar] / (self.flat_weight_norm_squared()[e12345]));
    }
}
