use crate::traits::UnitizedFlatNormSquared;
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
//  Maximum:        87     106       1
//
//  No SIMD:   add/sub     mul     div
//  Minimum:         0      11       1
//   Median:        15      25       1
//  Average:        19      34       1
//  Maximum:       111     144       1
impl std::ops::Div<unitized_flat_norm> for AntiCircleRotor {
    type Output = f32;
    fn div(self, _rhs: unitized_flat_norm) -> Self::Output {
        self.unitized_flat_norm()
    }
}
impl UnitizedFlatNorm for AntiCircleRotor {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        4        1
    //    simd3        2        4        0
    // Totals...
    // yes simd        4        8        1
    //  no simd        8       16        1
    fn unitized_flat_norm(self) -> f32 {
        return f32::powf(self.unitized_flat_norm_squared(), 0.5);
    }
}
impl std::ops::Div<unitized_flat_norm> for AntiDipoleInversion {
    type Output = f32;
    fn div(self, _rhs: unitized_flat_norm) -> Self::Output {
        self.unitized_flat_norm()
    }
}
impl UnitizedFlatNorm for AntiDipoleInversion {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       19       26        1
    //    simd3        2        6        0
    //    simd4        0        2        0
    // Totals...
    // yes simd       21       34        1
    //  no simd       25       52        1
    fn unitized_flat_norm(self) -> f32 {
        return f32::powf(self.unitized_flat_norm_squared(), 0.5);
    }
}
impl std::ops::Div<unitized_flat_norm> for Circle {
    type Output = f32;
    fn div(self, _rhs: unitized_flat_norm) -> Self::Output {
        self.unitized_flat_norm()
    }
}
impl UnitizedFlatNorm for Circle {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        9       14        1
    //    simd4        0        2        0
    // Totals...
    // yes simd        9       16        1
    //  no simd        9       22        1
    fn unitized_flat_norm(self) -> f32 {
        return f32::powf(self.unitized_flat_norm_squared(), 0.5);
    }
}
impl std::ops::Div<unitized_flat_norm> for CircleRotor {
    type Output = f32;
    fn div(self, _rhs: unitized_flat_norm) -> Self::Output {
        self.unitized_flat_norm()
    }
}
impl UnitizedFlatNorm for CircleRotor {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       10       15        1
    //    simd4        0        2        0
    // Totals...
    // yes simd       10       17        1
    //  no simd       10       23        1
    fn unitized_flat_norm(self) -> f32 {
        return f32::powf(self.unitized_flat_norm_squared(), 0.5);
    }
}
impl std::ops::Div<unitized_flat_norm> for Dipole {
    type Output = f32;
    fn div(self, _rhs: unitized_flat_norm) -> Self::Output {
        self.unitized_flat_norm()
    }
}
impl UnitizedFlatNorm for Dipole {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        4        1
    //    simd3        2        4        0
    // Totals...
    // yes simd        4        8        1
    //  no simd        8       16        1
    fn unitized_flat_norm(self) -> f32 {
        return f32::powf(self.unitized_flat_norm_squared(), 0.5);
    }
}
impl std::ops::Div<unitized_flat_norm> for DipoleInversion {
    type Output = f32;
    fn div(self, _rhs: unitized_flat_norm) -> Self::Output {
        self.unitized_flat_norm()
    }
}
impl UnitizedFlatNorm for DipoleInversion {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        8       17        1
    //    simd3        1        2        0
    //    simd4        1        1        0
    // Totals...
    // yes simd       10       20        1
    //  no simd       15       27        1
    fn unitized_flat_norm(self) -> f32 {
        return f32::powf(self.unitized_flat_norm_squared(), 0.5);
    }
}
impl std::ops::Div<unitized_flat_norm> for DualNum {
    type Output = f32;
    fn div(self, _rhs: unitized_flat_norm) -> Self::Output {
        self.unitized_flat_norm()
    }
}
impl UnitizedFlatNorm for DualNum {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        3        1
    //    simd4        0        2        0
    // Totals...
    // yes simd        0        5        1
    //  no simd        0       11        1
    fn unitized_flat_norm(self) -> f32 {
        return f32::powf(self.unitized_flat_norm_squared(), 0.5);
    }
}
impl std::ops::Div<unitized_flat_norm> for FlatPoint {
    type Output = f32;
    fn div(self, _rhs: unitized_flat_norm) -> Self::Output {
        self.unitized_flat_norm()
    }
}
impl UnitizedFlatNorm for FlatPoint {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        4        1
    //    simd3        2        4        0
    // Totals...
    // yes simd        4        8        1
    //  no simd        8       16        1
    fn unitized_flat_norm(self) -> f32 {
        return f32::powf(self.unitized_flat_norm_squared(), 0.5);
    }
}
impl std::ops::Div<unitized_flat_norm> for Flector {
    type Output = f32;
    fn div(self, _rhs: unitized_flat_norm) -> Self::Output {
        self.unitized_flat_norm()
    }
}
impl UnitizedFlatNorm for Flector {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        8       17        1
    //    simd3        1        2        0
    //    simd4        1        1        0
    // Totals...
    // yes simd       10       20        1
    //  no simd       15       27        1
    fn unitized_flat_norm(self) -> f32 {
        return f32::powf(self.unitized_flat_norm_squared(), 0.5);
    }
}
impl std::ops::Div<unitized_flat_norm> for Line {
    type Output = f32;
    fn div(self, _rhs: unitized_flat_norm) -> Self::Output {
        self.unitized_flat_norm()
    }
}
impl UnitizedFlatNorm for Line {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        9       14        1
    //    simd4        0        2        0
    // Totals...
    // yes simd        9       16        1
    //  no simd        9       22        1
    fn unitized_flat_norm(self) -> f32 {
        return f32::powf(self.unitized_flat_norm_squared(), 0.5);
    }
}
impl std::ops::Div<unitized_flat_norm> for Motor {
    type Output = f32;
    fn div(self, _rhs: unitized_flat_norm) -> Self::Output {
        self.unitized_flat_norm()
    }
}
impl UnitizedFlatNorm for Motor {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       20       27        1
    //    simd3        2        6        0
    //    simd4        0        2        0
    // Totals...
    // yes simd       22       35        1
    //  no simd       26       53        1
    fn unitized_flat_norm(self) -> f32 {
        return f32::powf(self.unitized_flat_norm_squared(), 0.5);
    }
}
impl std::ops::Div<unitized_flat_norm> for MultiVector {
    type Output = f32;
    fn div(self, _rhs: unitized_flat_norm) -> Self::Output {
        self.unitized_flat_norm()
    }
}
impl UnitizedFlatNorm for MultiVector {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       77       90        1
    //    simd3        6       10        0
    //    simd4        4        6        0
    // Totals...
    // yes simd       87      106        1
    //  no simd      111      144        1
    fn unitized_flat_norm(self) -> f32 {
        return f32::powf(self.unitized_flat_norm_squared(), 0.5);
    }
}
impl std::ops::Div<unitized_flat_norm> for Plane {
    type Output = f32;
    fn div(self, _rhs: unitized_flat_norm) -> Self::Output {
        self.unitized_flat_norm()
    }
}
impl UnitizedFlatNorm for Plane {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       17       21        1
    //    simd4        0        1        0
    // Totals...
    // yes simd       17       22        1
    //  no simd       17       25        1
    fn unitized_flat_norm(self) -> f32 {
        return f32::powf(self.unitized_flat_norm_squared(), 0.5);
    }
}
impl std::ops::Div<unitized_flat_norm> for Sphere {
    type Output = f32;
    fn div(self, _rhs: unitized_flat_norm) -> Self::Output {
        self.unitized_flat_norm()
    }
}
impl UnitizedFlatNorm for Sphere {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       17       21        1
    //    simd4        0        1        0
    // Totals...
    // yes simd       17       22        1
    //  no simd       17       25        1
    fn unitized_flat_norm(self) -> f32 {
        return f32::powf(self.unitized_flat_norm_squared(), 0.5);
    }
}
impl std::ops::Div<unitized_flat_norm> for VersorEven {
    type Output = f32;
    fn div(self, _rhs: unitized_flat_norm) -> Self::Output {
        self.unitized_flat_norm()
    }
}
impl UnitizedFlatNorm for VersorEven {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       20       27        1
    //    simd3        2        6        0
    //    simd4        0        2        0
    // Totals...
    // yes simd       22       35        1
    //  no simd       26       53        1
    fn unitized_flat_norm(self) -> f32 {
        return f32::powf(self.unitized_flat_norm_squared(), 0.5);
    }
}
impl std::ops::Div<unitized_flat_norm> for VersorOdd {
    type Output = f32;
    fn div(self, _rhs: unitized_flat_norm) -> Self::Output {
        self.unitized_flat_norm()
    }
}
impl UnitizedFlatNorm for VersorOdd {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        8       17        1
    //    simd3        1        2        0
    //    simd4        1        1        0
    // Totals...
    // yes simd       10       20        1
    //  no simd       15       27        1
    fn unitized_flat_norm(self) -> f32 {
        return f32::powf(self.unitized_flat_norm_squared(), 0.5);
    }
}
