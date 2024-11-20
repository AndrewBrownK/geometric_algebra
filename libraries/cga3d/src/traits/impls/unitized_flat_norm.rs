use crate::traits::UnitizedFlatNormSquared;
// Note on Operative Statistics:
// Operative Statistics are not a precise predictor of performance or performance comparisons.
// This is due to varying hardware capabilities and compiler optimizations.
// As always, where performance is a concern, there is no substitute for
// real measurements on real work-loads on real hardware.
// Disclaimer aside, enjoy the fun information =)
//
// Total Implementations: 15
//
// Yes SIMD:   add/sub     mul     div
//  Minimum:         2       7       1
//   Median:         6      10       1
//  Average:        10      19       1
//  Maximum:        63      84       1
//
//  No SIMD:   add/sub     mul     div
//  Minimum:         2       7       1
//   Median:         6      13       1
//  Average:        10      21       1
//  Maximum:        63      90       1
impl std::ops::Div<unitized_flat_norm> for AntiCircleRotor {
    type Output = f32;
    fn div(self, _rhs: unitized_flat_norm) -> Self::Output {
        self.unitized_flat_norm()
    }
}
impl UnitizedFlatNorm for AntiCircleRotor {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        6        1
    //    simd3        0        1        0
    // Totals...
    // yes simd        2        7        1
    //  no simd        2        9        1
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
    //      f32       16       29        1
    //    simd3        0        2        0
    // Totals...
    // yes simd       16       31        1
    //  no simd       16       35        1
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
    // f32        6       17        1
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
    // f32        7       18        1
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
    //      f32        2        6        1
    //    simd3        0        1        0
    // Totals...
    // yes simd        2        7        1
    //  no simd        2        9        1
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
    //      f32        6        9        1
    //    simd4        0        1        0
    // Totals...
    // yes simd        6       10        1
    //  no simd        6       13        1
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
    //      f32        2        6        1
    //    simd3        0        1        0
    // Totals...
    // yes simd        2        7        1
    //  no simd        2        9        1
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
    //      f32        6        9        1
    //    simd4        0        1        0
    // Totals...
    // yes simd        6       10        1
    //  no simd        6       13        1
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
    // f32        6       17        1
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
    //      f32       17       30        1
    //    simd3        0        2        0
    // Totals...
    // yes simd       17       32        1
    //  no simd       17       36        1
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
    //      f32       63       81        1
    //    simd3        0        3        0
    // Totals...
    // yes simd       63       84        1
    //  no simd       63       90        1
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
    //      add/sub      mul      div
    // f32        2        7        1
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
    //      add/sub      mul      div
    // f32        2        7        1
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
    //      f32       17       30        1
    //    simd3        0        2        0
    // Totals...
    // yes simd       17       32        1
    //  no simd       17       36        1
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
    //      f32        6        9        1
    //    simd4        0        1        0
    // Totals...
    // yes simd        6       10        1
    //  no simd        6       13        1
    fn unitized_flat_norm(self) -> f32 {
        return f32::powf(self.unitized_flat_norm_squared(), 0.5);
    }
}
