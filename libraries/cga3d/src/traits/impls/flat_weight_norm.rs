use crate::traits::AntiSquareRoot;
use crate::traits::FlatWeightNormSquared;
// Note on Operative Statistics:
// Operative Statistics are not a precise predictor of performance or performance comparisons.
// This is due to varying hardware capabilities and compiler optimizations.
// As always, where performance is a concern, there is no substitute for
// real measurements on real work-loads on real hardware.
// Disclaimer aside, enjoy the fun information =)
//
// Total Implementations: 17
//
// Yes SIMD:   add/sub     mul     div
//  Minimum:         0       1       0
//   Median:         2       3       0
//  Average:         3       4       0
//  Maximum:        31      32       0
//
//  No SIMD:   add/sub     mul     div
//  Minimum:         0       1       0
//   Median:         2       3       0
//  Average:         3       4       0
//  Maximum:        31      32       0
impl std::ops::Div<flat_weight_norm> for AntiCircleRotor {
    type Output = AntiScalar;
    fn div(self, _rhs: flat_weight_norm) -> Self::Output {
        self.flat_weight_norm()
    }
}
impl FlatWeightNorm for AntiCircleRotor {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        2        0
    fn flat_weight_norm(self) -> AntiScalar {
        return self.flat_weight_norm_squared().anti_square_root();
    }
}
impl std::ops::Div<flat_weight_norm> for AntiDipoleInversion {
    type Output = AntiScalar;
    fn div(self, _rhs: flat_weight_norm) -> Self::Output {
        self.flat_weight_norm()
    }
}
impl FlatWeightNorm for AntiDipoleInversion {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        3        0
    fn flat_weight_norm(self) -> AntiScalar {
        return self.flat_weight_norm_squared().anti_square_root();
    }
}
impl std::ops::Div<flat_weight_norm> for AntiScalar {
    type Output = AntiScalar;
    fn div(self, _rhs: flat_weight_norm) -> Self::Output {
        self.flat_weight_norm()
    }
}
impl std::ops::DivAssign<flat_weight_norm> for AntiScalar {
    fn div_assign(&mut self, _rhs: flat_weight_norm) {
        *self = self.flat_weight_norm()
    }
}
impl FlatWeightNorm for AntiScalar {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn flat_weight_norm(self) -> AntiScalar {
        return self.flat_weight_norm_squared().anti_square_root();
    }
}
impl std::ops::Div<flat_weight_norm> for Circle {
    type Output = AntiScalar;
    fn div(self, _rhs: flat_weight_norm) -> Self::Output {
        self.flat_weight_norm()
    }
}
impl FlatWeightNorm for Circle {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        3        0
    fn flat_weight_norm(self) -> AntiScalar {
        return self.flat_weight_norm_squared().anti_square_root();
    }
}
impl std::ops::Div<flat_weight_norm> for CircleRotor {
    type Output = AntiScalar;
    fn div(self, _rhs: flat_weight_norm) -> Self::Output {
        self.flat_weight_norm()
    }
}
impl FlatWeightNorm for CircleRotor {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        4        0
    fn flat_weight_norm(self) -> AntiScalar {
        return self.flat_weight_norm_squared().anti_square_root();
    }
}
impl std::ops::Div<flat_weight_norm> for Dipole {
    type Output = AntiScalar;
    fn div(self, _rhs: flat_weight_norm) -> Self::Output {
        self.flat_weight_norm()
    }
}
impl FlatWeightNorm for Dipole {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        2        0
    fn flat_weight_norm(self) -> AntiScalar {
        return self.flat_weight_norm_squared().anti_square_root();
    }
}
impl std::ops::Div<flat_weight_norm> for DipoleInversion {
    type Output = AntiScalar;
    fn div(self, _rhs: flat_weight_norm) -> Self::Output {
        self.flat_weight_norm()
    }
}
impl FlatWeightNorm for DipoleInversion {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        4        0
    fn flat_weight_norm(self) -> AntiScalar {
        return self.flat_weight_norm_squared().anti_square_root();
    }
}
impl std::ops::Div<flat_weight_norm> for DualNum {
    type Output = AntiScalar;
    fn div(self, _rhs: flat_weight_norm) -> Self::Output {
        self.flat_weight_norm()
    }
}
impl FlatWeightNorm for DualNum {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn flat_weight_norm(self) -> AntiScalar {
        return self.flat_weight_norm_squared().anti_square_root();
    }
}
impl std::ops::Div<flat_weight_norm> for FlatPoint {
    type Output = AntiScalar;
    fn div(self, _rhs: flat_weight_norm) -> Self::Output {
        self.flat_weight_norm()
    }
}
impl FlatWeightNorm for FlatPoint {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        2        0
    fn flat_weight_norm(self) -> AntiScalar {
        return self.flat_weight_norm_squared().anti_square_root();
    }
}
impl std::ops::Div<flat_weight_norm> for Flector {
    type Output = AntiScalar;
    fn div(self, _rhs: flat_weight_norm) -> Self::Output {
        self.flat_weight_norm()
    }
}
impl FlatWeightNorm for Flector {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        4        0
    fn flat_weight_norm(self) -> AntiScalar {
        return self.flat_weight_norm_squared().anti_square_root();
    }
}
impl std::ops::Div<flat_weight_norm> for Line {
    type Output = AntiScalar;
    fn div(self, _rhs: flat_weight_norm) -> Self::Output {
        self.flat_weight_norm()
    }
}
impl FlatWeightNorm for Line {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        3        0
    fn flat_weight_norm(self) -> AntiScalar {
        return self.flat_weight_norm_squared().anti_square_root();
    }
}
impl std::ops::Div<flat_weight_norm> for Motor {
    type Output = AntiScalar;
    fn div(self, _rhs: flat_weight_norm) -> Self::Output {
        self.flat_weight_norm()
    }
}
impl FlatWeightNorm for Motor {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        4        0
    fn flat_weight_norm(self) -> AntiScalar {
        return self.flat_weight_norm_squared().anti_square_root();
    }
}
impl std::ops::Div<flat_weight_norm> for MultiVector {
    type Output = AntiScalar;
    fn div(self, _rhs: flat_weight_norm) -> Self::Output {
        self.flat_weight_norm()
    }
}
impl FlatWeightNorm for MultiVector {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       31       32        0
    fn flat_weight_norm(self) -> AntiScalar {
        return self.flat_weight_norm_squared().anti_square_root();
    }
}
impl std::ops::Div<flat_weight_norm> for Plane {
    type Output = AntiScalar;
    fn div(self, _rhs: flat_weight_norm) -> Self::Output {
        self.flat_weight_norm()
    }
}
impl FlatWeightNorm for Plane {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        3        0
    fn flat_weight_norm(self) -> AntiScalar {
        return self.flat_weight_norm_squared().anti_square_root();
    }
}
impl std::ops::Div<flat_weight_norm> for Sphere {
    type Output = AntiScalar;
    fn div(self, _rhs: flat_weight_norm) -> Self::Output {
        self.flat_weight_norm()
    }
}
impl FlatWeightNorm for Sphere {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        3        0
    fn flat_weight_norm(self) -> AntiScalar {
        return self.flat_weight_norm_squared().anti_square_root();
    }
}
impl std::ops::Div<flat_weight_norm> for VersorEven {
    type Output = AntiScalar;
    fn div(self, _rhs: flat_weight_norm) -> Self::Output {
        self.flat_weight_norm()
    }
}
impl FlatWeightNorm for VersorEven {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        4        0
    fn flat_weight_norm(self) -> AntiScalar {
        return self.flat_weight_norm_squared().anti_square_root();
    }
}
impl std::ops::Div<flat_weight_norm> for VersorOdd {
    type Output = AntiScalar;
    fn div(self, _rhs: flat_weight_norm) -> Self::Output {
        self.flat_weight_norm()
    }
}
impl FlatWeightNorm for VersorOdd {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        4        0
    fn flat_weight_norm(self) -> AntiScalar {
        return self.flat_weight_norm_squared().anti_square_root();
    }
}
