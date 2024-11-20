use crate::traits::UnitizedRadiusNormSquared;
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
impl std::ops::Div<unitized_radius_norm> for AntiCircleRotor {
    type Output = f32;
    fn div(self, _rhs: unitized_radius_norm) -> Self::Output {
        self.unitized_radius_norm()
    }
}
impl UnitizedRadiusNorm for AntiCircleRotor {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       20       28        1
    //    simd3        3        5        0
    //    simd4        1        1        0
    // Totals...
    // yes simd       24       34        1
    //  no simd       33       47        1
    fn unitized_radius_norm(self) -> f32 {
        return f32::powf(self.unitized_radius_norm_squared(), 0.5);
    }
}
impl std::ops::Div<unitized_radius_norm> for AntiDipoleInversion {
    type Output = f32;
    fn div(self, _rhs: unitized_radius_norm) -> Self::Output {
        self.unitized_radius_norm()
    }
}
impl UnitizedRadiusNorm for AntiDipoleInversion {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       33       52        1
    //    simd3        0        1        0
    //    simd4        3        3        0
    // Totals...
    // yes simd       36       56        1
    //  no simd       45       67        1
    fn unitized_radius_norm(self) -> f32 {
        return f32::powf(self.unitized_radius_norm_squared(), 0.5);
    }
}
impl std::ops::Div<unitized_radius_norm> for AntiDualNum {
    type Output = f32;
    fn div(self, _rhs: unitized_radius_norm) -> Self::Output {
        self.unitized_radius_norm()
    }
}
impl UnitizedRadiusNorm for AntiDualNum {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       15       25        1
    fn unitized_radius_norm(self) -> f32 {
        return f32::powf(self.unitized_radius_norm_squared(), 0.5);
    }
}
impl std::ops::Div<unitized_radius_norm> for Circle {
    type Output = f32;
    fn div(self, _rhs: unitized_radius_norm) -> Self::Output {
        self.unitized_radius_norm()
    }
}
impl UnitizedRadiusNorm for Circle {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       16       28        1
    //    simd4        2        2        0
    // Totals...
    // yes simd       18       30        1
    //  no simd       24       36        1
    fn unitized_radius_norm(self) -> f32 {
        return f32::powf(self.unitized_radius_norm_squared(), 0.5);
    }
}
impl std::ops::Div<unitized_radius_norm> for CircleRotor {
    type Output = f32;
    fn div(self, _rhs: unitized_radius_norm) -> Self::Output {
        self.unitized_radius_norm()
    }
}
impl UnitizedRadiusNorm for CircleRotor {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       17       29        1
    //    simd4        2        2        0
    // Totals...
    // yes simd       19       31        1
    //  no simd       25       37        1
    fn unitized_radius_norm(self) -> f32 {
        return f32::powf(self.unitized_radius_norm_squared(), 0.5);
    }
}
impl std::ops::Div<unitized_radius_norm> for Dipole {
    type Output = f32;
    fn div(self, _rhs: unitized_radius_norm) -> Self::Output {
        self.unitized_radius_norm()
    }
}
impl UnitizedRadiusNorm for Dipole {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       19       27        1
    //    simd3        3        5        0
    //    simd4        1        1        0
    // Totals...
    // yes simd       23       33        1
    //  no simd       32       46        1
    fn unitized_radius_norm(self) -> f32 {
        return f32::powf(self.unitized_radius_norm_squared(), 0.5);
    }
}
impl std::ops::Div<unitized_radius_norm> for DipoleInversion {
    type Output = f32;
    fn div(self, _rhs: unitized_radius_norm) -> Self::Output {
        self.unitized_radius_norm()
    }
}
impl UnitizedRadiusNorm for DipoleInversion {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       26       41        1
    //    simd3        1        2        0
    //    simd4        3        3        0
    // Totals...
    // yes simd       30       46        1
    //  no simd       41       59        1
    fn unitized_radius_norm(self) -> f32 {
        return f32::powf(self.unitized_radius_norm_squared(), 0.5);
    }
}
impl std::ops::Div<unitized_radius_norm> for DualNum {
    type Output = f32;
    fn div(self, _rhs: unitized_radius_norm) -> Self::Output {
        self.unitized_radius_norm()
    }
}
impl UnitizedRadiusNorm for DualNum {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        9       14        1
    //    simd3        0        1        0
    // Totals...
    // yes simd        9       15        1
    //  no simd        9       17        1
    fn unitized_radius_norm(self) -> f32 {
        return f32::powf(self.unitized_radius_norm_squared(), 0.5);
    }
}
impl std::ops::Div<unitized_radius_norm> for MultiVector {
    type Output = f32;
    fn div(self, _rhs: unitized_radius_norm) -> Self::Output {
        self.unitized_radius_norm()
    }
}
impl UnitizedRadiusNorm for MultiVector {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       78      109        1
    //    simd3        2        4        0
    //    simd4        3        3        0
    // Totals...
    // yes simd       83      116        1
    //  no simd       96      133        1
    fn unitized_radius_norm(self) -> f32 {
        return f32::powf(self.unitized_radius_norm_squared(), 0.5);
    }
}
impl std::ops::Div<unitized_radius_norm> for RoundPoint {
    type Output = f32;
    fn div(self, _rhs: unitized_radius_norm) -> Self::Output {
        self.unitized_radius_norm()
    }
}
impl UnitizedRadiusNorm for RoundPoint {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       13       18        1
    //    simd3        0        1        0
    // Totals...
    // yes simd       13       19        1
    //  no simd       13       21        1
    fn unitized_radius_norm(self) -> f32 {
        return f32::powf(self.unitized_radius_norm_squared(), 0.5);
    }
}
impl std::ops::Div<unitized_radius_norm> for Sphere {
    type Output = f32;
    fn div(self, _rhs: unitized_radius_norm) -> Self::Output {
        self.unitized_radius_norm()
    }
}
impl UnitizedRadiusNorm for Sphere {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       19       28        1
    fn unitized_radius_norm(self) -> f32 {
        return f32::powf(self.unitized_radius_norm_squared(), 0.5);
    }
}
impl std::ops::Div<unitized_radius_norm> for VersorEven {
    type Output = f32;
    fn div(self, _rhs: unitized_radius_norm) -> Self::Output {
        self.unitized_radius_norm()
    }
}
impl UnitizedRadiusNorm for VersorEven {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       34       53        1
    //    simd3        0        1        0
    //    simd4        3        3        0
    // Totals...
    // yes simd       37       57        1
    //  no simd       46       68        1
    fn unitized_radius_norm(self) -> f32 {
        return f32::powf(self.unitized_radius_norm_squared(), 0.5);
    }
}
impl std::ops::Div<unitized_radius_norm> for VersorOdd {
    type Output = f32;
    fn div(self, _rhs: unitized_radius_norm) -> Self::Output {
        self.unitized_radius_norm()
    }
}
impl UnitizedRadiusNorm for VersorOdd {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       27       42        1
    //    simd3        1        2        0
    //    simd4        3        3        0
    // Totals...
    // yes simd       31       47        1
    //  no simd       42       60        1
    fn unitized_radius_norm(self) -> f32 {
        return f32::powf(self.unitized_radius_norm_squared(), 0.5);
    }
}
