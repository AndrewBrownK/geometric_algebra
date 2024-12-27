use crate::traits::AntiDotProduct;
use crate::traits::FlatWeight;
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
impl std::ops::Div<FlatWeightNormSquaredPrefixOrPostfix> for AntiCircleRotor {
    type Output = AntiScalar;
    fn div(self, _rhs: FlatWeightNormSquaredPrefixOrPostfix) -> Self::Output {
        self.flat_weight_norm_squared()
    }
}
impl FlatWeightNormSquared for AntiCircleRotor {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn flat_weight_norm_squared(self) -> AntiScalar {
        let flat_weight = self.flat_weight();
        return flat_weight.anti_dot_product(flat_weight);
    }
}
impl std::ops::Div<FlatWeightNormSquaredPrefixOrPostfix> for AntiDipoleInversion {
    type Output = AntiScalar;
    fn div(self, _rhs: FlatWeightNormSquaredPrefixOrPostfix) -> Self::Output {
        self.flat_weight_norm_squared()
    }
}
impl FlatWeightNormSquared for AntiDipoleInversion {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        3        0
    fn flat_weight_norm_squared(self) -> AntiScalar {
        let flat_weight = self.flat_weight();
        return flat_weight.anti_dot_product(flat_weight);
    }
}
impl std::ops::Div<FlatWeightNormSquaredPrefixOrPostfix> for AntiScalar {
    type Output = AntiScalar;
    fn div(self, _rhs: FlatWeightNormSquaredPrefixOrPostfix) -> Self::Output {
        self.flat_weight_norm_squared()
    }
}
impl std::ops::DivAssign<FlatWeightNormSquaredPrefixOrPostfix> for AntiScalar {
    fn div_assign(&mut self, _rhs: FlatWeightNormSquaredPrefixOrPostfix) {
        *self = self.flat_weight_norm_squared()
    }
}
impl FlatWeightNormSquared for AntiScalar {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn flat_weight_norm_squared(self) -> AntiScalar {
        let flat_weight = self.flat_weight();
        return flat_weight.anti_dot_product(flat_weight);
    }
}
impl std::ops::Div<FlatWeightNormSquaredPrefixOrPostfix> for Circle {
    type Output = AntiScalar;
    fn div(self, _rhs: FlatWeightNormSquaredPrefixOrPostfix) -> Self::Output {
        self.flat_weight_norm_squared()
    }
}
impl FlatWeightNormSquared for Circle {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        3        0
    fn flat_weight_norm_squared(self) -> AntiScalar {
        let flat_weight = self.flat_weight();
        return flat_weight.anti_dot_product(flat_weight);
    }
}
impl std::ops::Div<FlatWeightNormSquaredPrefixOrPostfix> for CircleRotor {
    type Output = AntiScalar;
    fn div(self, _rhs: FlatWeightNormSquaredPrefixOrPostfix) -> Self::Output {
        self.flat_weight_norm_squared()
    }
}
impl FlatWeightNormSquared for CircleRotor {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        4        0
    fn flat_weight_norm_squared(self) -> AntiScalar {
        let flat_weight = self.flat_weight();
        return flat_weight.anti_dot_product(flat_weight);
    }
}
impl std::ops::Div<FlatWeightNormSquaredPrefixOrPostfix> for Dipole {
    type Output = AntiScalar;
    fn div(self, _rhs: FlatWeightNormSquaredPrefixOrPostfix) -> Self::Output {
        self.flat_weight_norm_squared()
    }
}
impl FlatWeightNormSquared for Dipole {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn flat_weight_norm_squared(self) -> AntiScalar {
        let flat_weight = self.flat_weight();
        return flat_weight.anti_dot_product(flat_weight);
    }
}
impl std::ops::Div<FlatWeightNormSquaredPrefixOrPostfix> for DipoleInversion {
    type Output = AntiScalar;
    fn div(self, _rhs: FlatWeightNormSquaredPrefixOrPostfix) -> Self::Output {
        self.flat_weight_norm_squared()
    }
}
impl FlatWeightNormSquared for DipoleInversion {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        4        0
    fn flat_weight_norm_squared(self) -> AntiScalar {
        let flat_weight = self.flat_weight();
        return flat_weight.anti_dot_product(flat_weight);
    }
}
impl std::ops::Div<FlatWeightNormSquaredPrefixOrPostfix> for DualNum {
    type Output = AntiScalar;
    fn div(self, _rhs: FlatWeightNormSquaredPrefixOrPostfix) -> Self::Output {
        self.flat_weight_norm_squared()
    }
}
impl FlatWeightNormSquared for DualNum {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn flat_weight_norm_squared(self) -> AntiScalar {
        let flat_weight = self.flat_weight();
        return flat_weight.anti_dot_product(flat_weight);
    }
}
impl std::ops::Div<FlatWeightNormSquaredPrefixOrPostfix> for FlatPoint {
    type Output = AntiScalar;
    fn div(self, _rhs: FlatWeightNormSquaredPrefixOrPostfix) -> Self::Output {
        self.flat_weight_norm_squared()
    }
}
impl FlatWeightNormSquared for FlatPoint {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn flat_weight_norm_squared(self) -> AntiScalar {
        let flat_weight = self.flat_weight();
        return flat_weight.anti_dot_product(flat_weight);
    }
}
impl std::ops::Div<FlatWeightNormSquaredPrefixOrPostfix> for Flector {
    type Output = AntiScalar;
    fn div(self, _rhs: FlatWeightNormSquaredPrefixOrPostfix) -> Self::Output {
        self.flat_weight_norm_squared()
    }
}
impl FlatWeightNormSquared for Flector {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        4        0
    fn flat_weight_norm_squared(self) -> AntiScalar {
        let flat_weight = self.flat_weight();
        return flat_weight.anti_dot_product(flat_weight);
    }
}
impl std::ops::Div<FlatWeightNormSquaredPrefixOrPostfix> for Line {
    type Output = AntiScalar;
    fn div(self, _rhs: FlatWeightNormSquaredPrefixOrPostfix) -> Self::Output {
        self.flat_weight_norm_squared()
    }
}
impl FlatWeightNormSquared for Line {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        3        0
    fn flat_weight_norm_squared(self) -> AntiScalar {
        let flat_weight = self.flat_weight();
        return flat_weight.anti_dot_product(flat_weight);
    }
}
impl std::ops::Div<FlatWeightNormSquaredPrefixOrPostfix> for Motor {
    type Output = AntiScalar;
    fn div(self, _rhs: FlatWeightNormSquaredPrefixOrPostfix) -> Self::Output {
        self.flat_weight_norm_squared()
    }
}
impl FlatWeightNormSquared for Motor {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        4        0
    fn flat_weight_norm_squared(self) -> AntiScalar {
        let flat_weight = self.flat_weight();
        return flat_weight.anti_dot_product(flat_weight);
    }
}
impl std::ops::Div<FlatWeightNormSquaredPrefixOrPostfix> for MultiVector {
    type Output = AntiScalar;
    fn div(self, _rhs: FlatWeightNormSquaredPrefixOrPostfix) -> Self::Output {
        self.flat_weight_norm_squared()
    }
}
impl FlatWeightNormSquared for MultiVector {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       31       32        0
    fn flat_weight_norm_squared(self) -> AntiScalar {
        let flat_weight = self.flat_weight();
        return flat_weight.anti_dot_product(flat_weight);
    }
}
impl std::ops::Div<FlatWeightNormSquaredPrefixOrPostfix> for Plane {
    type Output = AntiScalar;
    fn div(self, _rhs: FlatWeightNormSquaredPrefixOrPostfix) -> Self::Output {
        self.flat_weight_norm_squared()
    }
}
impl FlatWeightNormSquared for Plane {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        3        0
    fn flat_weight_norm_squared(self) -> AntiScalar {
        let flat_weight = self.flat_weight();
        return flat_weight.anti_dot_product(flat_weight);
    }
}
impl std::ops::Div<FlatWeightNormSquaredPrefixOrPostfix> for Sphere {
    type Output = AntiScalar;
    fn div(self, _rhs: FlatWeightNormSquaredPrefixOrPostfix) -> Self::Output {
        self.flat_weight_norm_squared()
    }
}
impl FlatWeightNormSquared for Sphere {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        3        0
    fn flat_weight_norm_squared(self) -> AntiScalar {
        let flat_weight = self.flat_weight();
        return flat_weight.anti_dot_product(flat_weight);
    }
}
impl std::ops::Div<FlatWeightNormSquaredPrefixOrPostfix> for VersorEven {
    type Output = AntiScalar;
    fn div(self, _rhs: FlatWeightNormSquaredPrefixOrPostfix) -> Self::Output {
        self.flat_weight_norm_squared()
    }
}
impl FlatWeightNormSquared for VersorEven {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        4        0
    fn flat_weight_norm_squared(self) -> AntiScalar {
        let flat_weight = self.flat_weight();
        return flat_weight.anti_dot_product(flat_weight);
    }
}
impl std::ops::Div<FlatWeightNormSquaredPrefixOrPostfix> for VersorOdd {
    type Output = AntiScalar;
    fn div(self, _rhs: FlatWeightNormSquaredPrefixOrPostfix) -> Self::Output {
        self.flat_weight_norm_squared()
    }
}
impl FlatWeightNormSquared for VersorOdd {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        4        0
    fn flat_weight_norm_squared(self) -> AntiScalar {
        let flat_weight = self.flat_weight();
        return flat_weight.anti_dot_product(flat_weight);
    }
}
