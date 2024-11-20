use crate::traits::RadiusNormSquared;
use crate::traits::RoundWeightNormSquared;
// Note on Operative Statistics:
// Operative Statistics are not a precise predictor of performance or performance comparisons.
// This is due to varying hardware capabilities and compiler optimizations.
// As always, where performance is a concern, there is no substitute for
// real measurements on real work-loads on real hardware.
// Disclaimer aside, enjoy the fun information =)
//
// Total Implementations: 13
//
// Yes SIMD:   add/sub     mul     div
//  Minimum:         9      15       1
//   Median:        23      33       1
//  Average:        27      41       1
//  Maximum:        83     116       1
//
//  No SIMD:   add/sub     mul     div
//  Minimum:         9      17       1
//   Median:        32      46       1
//  Average:        33      49       1
//  Maximum:        96     133       1
impl std::ops::Div<unitized_radius_norm_squared> for AntiCircleRotor {
    type Output = f32;
    fn div(self, _rhs: unitized_radius_norm_squared) -> Self::Output {
        self.unitized_radius_norm_squared()
    }
}
impl UnitizedRadiusNormSquared for AntiCircleRotor {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       20       28        1
    //    simd3        3        5        0
    //    simd4        1        1        0
    // Totals...
    // yes simd       24       34        1
    //  no simd       33       47        1
    fn unitized_radius_norm_squared(self) -> f32 {
        use crate::elements::*;
        return ((1.0 / self.round_weight_norm_squared()[e12345]) * self.radius_norm_squared()[scalar]);
    }
}
impl std::ops::Div<unitized_radius_norm_squared> for AntiDipoleInversion {
    type Output = f32;
    fn div(self, _rhs: unitized_radius_norm_squared) -> Self::Output {
        self.unitized_radius_norm_squared()
    }
}
impl UnitizedRadiusNormSquared for AntiDipoleInversion {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       33       52        1
    //    simd3        0        1        0
    //    simd4        3        3        0
    // Totals...
    // yes simd       36       56        1
    //  no simd       45       67        1
    fn unitized_radius_norm_squared(self) -> f32 {
        use crate::elements::*;
        return ((1.0 / self.round_weight_norm_squared()[e12345]) * self.radius_norm_squared()[scalar]);
    }
}
impl std::ops::Div<unitized_radius_norm_squared> for AntiDualNum {
    type Output = f32;
    fn div(self, _rhs: unitized_radius_norm_squared) -> Self::Output {
        self.unitized_radius_norm_squared()
    }
}
impl UnitizedRadiusNormSquared for AntiDualNum {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       15       25        1
    fn unitized_radius_norm_squared(self) -> f32 {
        use crate::elements::*;
        return ((1.0 / self.round_weight_norm_squared()[e12345]) * self.radius_norm_squared()[scalar]);
    }
}
impl std::ops::Div<unitized_radius_norm_squared> for Circle {
    type Output = f32;
    fn div(self, _rhs: unitized_radius_norm_squared) -> Self::Output {
        self.unitized_radius_norm_squared()
    }
}
impl UnitizedRadiusNormSquared for Circle {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       16       28        1
    //    simd4        2        2        0
    // Totals...
    // yes simd       18       30        1
    //  no simd       24       36        1
    fn unitized_radius_norm_squared(self) -> f32 {
        use crate::elements::*;
        return ((1.0 / self.round_weight_norm_squared()[e12345]) * self.radius_norm_squared()[scalar]);
    }
}
impl std::ops::Div<unitized_radius_norm_squared> for CircleRotor {
    type Output = f32;
    fn div(self, _rhs: unitized_radius_norm_squared) -> Self::Output {
        self.unitized_radius_norm_squared()
    }
}
impl UnitizedRadiusNormSquared for CircleRotor {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       17       29        1
    //    simd4        2        2        0
    // Totals...
    // yes simd       19       31        1
    //  no simd       25       37        1
    fn unitized_radius_norm_squared(self) -> f32 {
        use crate::elements::*;
        return ((1.0 / self.round_weight_norm_squared()[e12345]) * self.radius_norm_squared()[scalar]);
    }
}
impl std::ops::Div<unitized_radius_norm_squared> for Dipole {
    type Output = f32;
    fn div(self, _rhs: unitized_radius_norm_squared) -> Self::Output {
        self.unitized_radius_norm_squared()
    }
}
impl UnitizedRadiusNormSquared for Dipole {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       19       27        1
    //    simd3        3        5        0
    //    simd4        1        1        0
    // Totals...
    // yes simd       23       33        1
    //  no simd       32       46        1
    fn unitized_radius_norm_squared(self) -> f32 {
        use crate::elements::*;
        return ((1.0 / self.round_weight_norm_squared()[e12345]) * self.radius_norm_squared()[scalar]);
    }
}
impl std::ops::Div<unitized_radius_norm_squared> for DipoleInversion {
    type Output = f32;
    fn div(self, _rhs: unitized_radius_norm_squared) -> Self::Output {
        self.unitized_radius_norm_squared()
    }
}
impl UnitizedRadiusNormSquared for DipoleInversion {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       26       41        1
    //    simd3        1        2        0
    //    simd4        3        3        0
    // Totals...
    // yes simd       30       46        1
    //  no simd       41       59        1
    fn unitized_radius_norm_squared(self) -> f32 {
        use crate::elements::*;
        return ((1.0 / self.round_weight_norm_squared()[e12345]) * self.radius_norm_squared()[scalar]);
    }
}
impl std::ops::Div<unitized_radius_norm_squared> for DualNum {
    type Output = f32;
    fn div(self, _rhs: unitized_radius_norm_squared) -> Self::Output {
        self.unitized_radius_norm_squared()
    }
}
impl UnitizedRadiusNormSquared for DualNum {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        9       14        1
    //    simd3        0        1        0
    // Totals...
    // yes simd        9       15        1
    //  no simd        9       17        1
    fn unitized_radius_norm_squared(self) -> f32 {
        use crate::elements::*;
        return ((1.0 / self.round_weight_norm_squared()[e12345]) * self.radius_norm_squared()[scalar]);
    }
}
impl std::ops::Div<unitized_radius_norm_squared> for MultiVector {
    type Output = f32;
    fn div(self, _rhs: unitized_radius_norm_squared) -> Self::Output {
        self.unitized_radius_norm_squared()
    }
}
impl UnitizedRadiusNormSquared for MultiVector {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       78      109        1
    //    simd3        2        4        0
    //    simd4        3        3        0
    // Totals...
    // yes simd       83      116        1
    //  no simd       96      133        1
    fn unitized_radius_norm_squared(self) -> f32 {
        use crate::elements::*;
        return ((1.0 / self.round_weight_norm_squared()[e12345]) * self.radius_norm_squared()[scalar]);
    }
}
impl std::ops::Div<unitized_radius_norm_squared> for RoundPoint {
    type Output = f32;
    fn div(self, _rhs: unitized_radius_norm_squared) -> Self::Output {
        self.unitized_radius_norm_squared()
    }
}
impl UnitizedRadiusNormSquared for RoundPoint {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       13       18        1
    //    simd3        0        1        0
    // Totals...
    // yes simd       13       19        1
    //  no simd       13       21        1
    fn unitized_radius_norm_squared(self) -> f32 {
        use crate::elements::*;
        return ((1.0 / self.round_weight_norm_squared()[e12345]) * self.radius_norm_squared()[scalar]);
    }
}
impl std::ops::Div<unitized_radius_norm_squared> for Sphere {
    type Output = f32;
    fn div(self, _rhs: unitized_radius_norm_squared) -> Self::Output {
        self.unitized_radius_norm_squared()
    }
}
impl UnitizedRadiusNormSquared for Sphere {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       19       28        1
    fn unitized_radius_norm_squared(self) -> f32 {
        use crate::elements::*;
        return ((1.0 / self.round_weight_norm_squared()[e12345]) * self.radius_norm_squared()[scalar]);
    }
}
impl std::ops::Div<unitized_radius_norm_squared> for VersorEven {
    type Output = f32;
    fn div(self, _rhs: unitized_radius_norm_squared) -> Self::Output {
        self.unitized_radius_norm_squared()
    }
}
impl UnitizedRadiusNormSquared for VersorEven {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       34       53        1
    //    simd3        0        1        0
    //    simd4        3        3        0
    // Totals...
    // yes simd       37       57        1
    //  no simd       46       68        1
    fn unitized_radius_norm_squared(self) -> f32 {
        use crate::elements::*;
        return ((1.0 / self.round_weight_norm_squared()[e12345]) * self.radius_norm_squared()[scalar]);
    }
}
impl std::ops::Div<unitized_radius_norm_squared> for VersorOdd {
    type Output = f32;
    fn div(self, _rhs: unitized_radius_norm_squared) -> Self::Output {
        self.unitized_radius_norm_squared()
    }
}
impl UnitizedRadiusNormSquared for VersorOdd {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       27       42        1
    //    simd3        1        2        0
    //    simd4        3        3        0
    // Totals...
    // yes simd       31       47        1
    //  no simd       42       60        1
    fn unitized_radius_norm_squared(self) -> f32 {
        use crate::elements::*;
        return ((1.0 / self.round_weight_norm_squared()[e12345]) * self.radius_norm_squared()[scalar]);
    }
}
