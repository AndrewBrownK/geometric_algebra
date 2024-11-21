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
//  Minimum:         9      14       1
//   Median:        30      43       1
//  Average:        31      44       1
//  Maximum:        91     122       1
//
//  No SIMD:   add/sub     mul     div
//  Minimum:         9      16       1
//   Median:        32      45       1
//  Average:        33      46       1
//  Maximum:        96     129       1
impl std::ops::Div<unitized_radius_norm> for AntiCircleRotor {
    type Output = f32;
    fn div(self, _rhs: unitized_radius_norm) -> Self::Output {
        self.unitized_radius_norm()
    }
}
impl UnitizedRadiusNorm for AntiCircleRotor {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       30       43        1
    //    simd3        1        1        0
    // Totals...
    // yes simd       31       44        1
    //  no simd       33       46        1
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
    //      f32       41       56        1
    //    simd3        0        1        0
    //    simd4        1        1        0
    // Totals...
    // yes simd       42       58        1
    //  no simd       45       63        1
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
    // f32       15       24        1
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
    //      f32       20       28        1
    //    simd4        1        1        0
    // Totals...
    // yes simd       21       29        1
    //  no simd       24       32        1
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
    //      f32       21       29        1
    //    simd4        1        1        0
    // Totals...
    // yes simd       22       30        1
    //  no simd       25       33        1
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
    //      f32       29       42        1
    //    simd3        1        1        0
    // Totals...
    // yes simd       30       43        1
    //  no simd       32       45        1
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
    //      f32       37       51        1
    //    simd4        1        1        0
    // Totals...
    // yes simd       38       52        1
    //  no simd       41       55        1
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
    //      f32        9       13        1
    //    simd3        0        1        0
    // Totals...
    // yes simd        9       14        1
    //  no simd        9       16        1
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
    //      f32       89      119        1
    //    simd3        1        2        0
    //    simd4        1        1        0
    // Totals...
    // yes simd       91      122        1
    //  no simd       96      129        1
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
    //      f32       13       17        1
    //    simd3        0        1        0
    // Totals...
    // yes simd       13       18        1
    //  no simd       13       20        1
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
    // f32       19       27        1
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
    //      f32       42       57        1
    //    simd3        0        1        0
    //    simd4        1        1        0
    // Totals...
    // yes simd       43       59        1
    //  no simd       46       64        1
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
    //      f32       38       52        1
    //    simd4        1        1        0
    // Totals...
    // yes simd       39       53        1
    //  no simd       42       56        1
    fn unitized_radius_norm(self) -> f32 {
        return f32::powf(self.unitized_radius_norm_squared(), 0.5);
    }
}
