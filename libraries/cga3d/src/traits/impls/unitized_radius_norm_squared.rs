use crate::traits::RadiusNormSquared;
use crate::traits::RoundWeightNormSquared;
// Note on Operative Statistics:
// Operative Statistics are not a precise predictor of performance or performance comparisons.
// This is due to varying hardware capabilities and compiler optimizations.
// As always, where performance is a concern, there is no substitute for
// real measurements on real work-loads on real hardware.
// Disclaimer aside, enjoy the fun information =)
//
// Total Implementations: 11
//
// Yes SIMD:   add/sub     mul     div
//  Minimum:         4       8       1
//   Median:        12      17       1
//  Average:        17      22       1
//  Maximum:        63      74       1
//
//  No SIMD:   add/sub     mul     div
//  Minimum:         4       8       1
//   Median:        12      21       1
//  Average:        17      29       1
//  Maximum:        63      91       1
impl std::ops::Div<unitized_radius_norm_squared> for AntiCircleRotor {
    type Output = f32;
    fn div(self, _rhs: unitized_radius_norm_squared) -> Self::Output {
        self.unitized_radius_norm_squared()
    }
}
impl UnitizedRadiusNormSquared for AntiCircleRotor {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       12       15        1
    //    simd3        0        2        0
    // Totals...
    // yes simd       12       17        1
    //  no simd       12       21        1
    fn unitized_radius_norm_squared(self) -> f32 {
        use crate::elements::*;
        return self.radius_norm_squared()[scalar] / (self.round_weight_norm_squared()[e12345]);
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
    //      f32       17       20        1
    //    simd4        0        2        0
    // Totals...
    // yes simd       17       22        1
    //  no simd       17       28        1
    fn unitized_radius_norm_squared(self) -> f32 {
        use crate::elements::*;
        return self.radius_norm_squared()[scalar] / (self.round_weight_norm_squared()[e12345]);
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
    //      f32       11       14        1
    //    simd4        0        1        0
    // Totals...
    // yes simd       11       15        1
    //  no simd       11       18        1
    fn unitized_radius_norm_squared(self) -> f32 {
        use crate::elements::*;
        return self.radius_norm_squared()[scalar] / (self.round_weight_norm_squared()[e12345]);
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
    //      f32       12       15        1
    //    simd4        0        1        0
    // Totals...
    // yes simd       12       16        1
    //  no simd       12       19        1
    fn unitized_radius_norm_squared(self) -> f32 {
        use crate::elements::*;
        return self.radius_norm_squared()[scalar] / (self.round_weight_norm_squared()[e12345]);
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
    //      f32       11       14        1
    //    simd3        0        2        0
    // Totals...
    // yes simd       11       16        1
    //  no simd       11       20        1
    fn unitized_radius_norm_squared(self) -> f32 {
        use crate::elements::*;
        return self.radius_norm_squared()[scalar] / (self.round_weight_norm_squared()[e12345]);
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
    //      f32       17       20        1
    //    simd4        0        4        0
    // Totals...
    // yes simd       17       24        1
    //  no simd       17       36        1
    fn unitized_radius_norm_squared(self) -> f32 {
        use crate::elements::*;
        return self.radius_norm_squared()[scalar] / (self.round_weight_norm_squared()[e12345]);
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
    //      f32       63       68        1
    //    simd3        0        1        0
    //    simd4        0        5        0
    // Totals...
    // yes simd       63       74        1
    //  no simd       63       91        1
    fn unitized_radius_norm_squared(self) -> f32 {
        use crate::elements::*;
        return self.radius_norm_squared()[scalar] / (self.round_weight_norm_squared()[e12345]);
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
    //      f32        4        8        1
    //    simd4        0        1        0
    // Totals...
    // yes simd        4        9        1
    //  no simd        4       12        1
    fn unitized_radius_norm_squared(self) -> f32 {
        use crate::elements::*;
        return self.radius_norm_squared()[scalar] / (self.round_weight_norm_squared()[e12345]);
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
    // f32        4        8        1
    fn unitized_radius_norm_squared(self) -> f32 {
        use crate::elements::*;
        return self.radius_norm_squared()[scalar] / (self.round_weight_norm_squared()[e12345]);
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
    //      f32       18       21        1
    //    simd4        0        2        0
    // Totals...
    // yes simd       18       23        1
    //  no simd       18       29        1
    fn unitized_radius_norm_squared(self) -> f32 {
        use crate::elements::*;
        return self.radius_norm_squared()[scalar] / (self.round_weight_norm_squared()[e12345]);
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
    //      f32       18       21        1
    //    simd4        0        4        0
    // Totals...
    // yes simd       18       25        1
    //  no simd       18       37        1
    fn unitized_radius_norm_squared(self) -> f32 {
        use crate::elements::*;
        return self.radius_norm_squared()[scalar] / (self.round_weight_norm_squared()[e12345]);
    }
}
