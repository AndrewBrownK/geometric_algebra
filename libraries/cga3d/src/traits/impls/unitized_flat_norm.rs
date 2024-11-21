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
//  Minimum:         0       4       1
//   Median:        12      22       1
//  Average:        18      30       1
//  Maximum:        99     124       1
//
//  No SIMD:   add/sub     mul     div
//  Minimum:         0      10       1
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
    //      f32        5       11        1
    //    simd3        1        2        0
    // Totals...
    // yes simd        6       13        1
    //  no simd        8       17        1
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
    //      f32       25       44        1
    //    simd3        0        2        0
    // Totals...
    // yes simd       25       46        1
    //  no simd       25       50        1
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
    //      add/sub      mul      div
    // f32        9       20        1
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
    //      add/sub      mul      div
    // f32       10       21        1
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
    //      f32        5       11        1
    //    simd3        1        2        0
    // Totals...
    // yes simd        6       13        1
    //  no simd        8       17        1
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
    //      f32       11       23        1
    //    simd4        1        1        0
    // Totals...
    // yes simd       12       24        1
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
    //      f32        0        2        1
    //    simd4        0        2        0
    // Totals...
    // yes simd        0        4        1
    //  no simd        0       10        1
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
    //      f32        5       11        1
    //    simd3        1        2        0
    // Totals...
    // yes simd        6       13        1
    //  no simd        8       17        1
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
    //      f32       11       23        1
    //    simd4        1        1        0
    // Totals...
    // yes simd       12       24        1
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
    //      add/sub      mul      div
    // f32        9       20        1
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
    //      f32       26       45        1
    //    simd3        0        2        0
    // Totals...
    // yes simd       26       47        1
    //  no simd       26       51        1
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
    //      f32       94      116        1
    //    simd3        3        4        0
    //    simd4        2        4        0
    // Totals...
    // yes simd       99      124        1
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
    //      f32       26       45        1
    //    simd3        0        2        0
    // Totals...
    // yes simd       26       47        1
    //  no simd       26       51        1
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
    //      f32       11       23        1
    //    simd4        1        1        0
    // Totals...
    // yes simd       12       24        1
    //  no simd       15       27        1
    fn unitized_flat_norm(self) -> f32 {
        return f32::powf(self.unitized_flat_norm_squared(), 0.5);
    }
}
