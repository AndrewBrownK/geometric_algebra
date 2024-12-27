use crate::traits::AntiDotProduct;
use crate::traits::FlatWeight;
// Note on Operative Statistics:
// Operative Statistics are not a precise predictor of performance or performance comparisons.
// This is due to varying hardware capabilities and compiler optimizations.
// As always, where performance is a concern, there is no substitute for
// real measurements on real work-loads on real hardware.
// Disclaimer aside, enjoy the fun information =)
//
// Total Implementations: 9
//
// Yes SIMD:   add/sub     mul     div
//  Minimum:         0       1       0
//   Median:         2       3       0
//  Average:         1       2       0
//  Maximum:         7       8       0
//
//  No SIMD:   add/sub     mul     div
//  Minimum:         0       1       0
//   Median:         2       3       0
//  Average:         1       2       0
//  Maximum:         7       8       0
impl std::ops::Div<WeightNormSquaredPrefixOrPostfix> for AntiScalar {
    type Output = AntiScalar;
    fn div(self, _rhs: WeightNormSquaredPrefixOrPostfix) -> Self::Output {
        self.weight_norm_squared()
    }
}
impl std::ops::DivAssign<WeightNormSquaredPrefixOrPostfix> for AntiScalar {
    fn div_assign(&mut self, _rhs: WeightNormSquaredPrefixOrPostfix) {
        *self = self.weight_norm_squared()
    }
}
impl WeightNormSquared for AntiScalar {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn weight_norm_squared(self) -> AntiScalar {
        let flat_weight = self.flat_weight();
        return flat_weight.anti_dot_product(flat_weight);
    }
}
impl std::ops::Div<WeightNormSquaredPrefixOrPostfix> for DualNum {
    type Output = AntiScalar;
    fn div(self, _rhs: WeightNormSquaredPrefixOrPostfix) -> Self::Output {
        self.weight_norm_squared()
    }
}
impl WeightNormSquared for DualNum {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn weight_norm_squared(self) -> AntiScalar {
        let flat_weight = self.flat_weight();
        return flat_weight.anti_dot_product(flat_weight);
    }
}
impl std::ops::Div<WeightNormSquaredPrefixOrPostfix> for Flector {
    type Output = AntiScalar;
    fn div(self, _rhs: WeightNormSquaredPrefixOrPostfix) -> Self::Output {
        self.weight_norm_squared()
    }
}
impl WeightNormSquared for Flector {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        4        0
    fn weight_norm_squared(self) -> AntiScalar {
        let flat_weight = self.flat_weight();
        return flat_weight.anti_dot_product(flat_weight);
    }
}
impl std::ops::Div<WeightNormSquaredPrefixOrPostfix> for Line {
    type Output = AntiScalar;
    fn div(self, _rhs: WeightNormSquaredPrefixOrPostfix) -> Self::Output {
        self.weight_norm_squared()
    }
}
impl WeightNormSquared for Line {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        3        0
    fn weight_norm_squared(self) -> AntiScalar {
        let flat_weight = self.flat_weight();
        return flat_weight.anti_dot_product(flat_weight);
    }
}
impl std::ops::Div<WeightNormSquaredPrefixOrPostfix> for Motor {
    type Output = AntiScalar;
    fn div(self, _rhs: WeightNormSquaredPrefixOrPostfix) -> Self::Output {
        self.weight_norm_squared()
    }
}
impl WeightNormSquared for Motor {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        4        0
    fn weight_norm_squared(self) -> AntiScalar {
        let flat_weight = self.flat_weight();
        return flat_weight.anti_dot_product(flat_weight);
    }
}
impl std::ops::Div<WeightNormSquaredPrefixOrPostfix> for MultiVector {
    type Output = AntiScalar;
    fn div(self, _rhs: WeightNormSquaredPrefixOrPostfix) -> Self::Output {
        self.weight_norm_squared()
    }
}
impl WeightNormSquared for MultiVector {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        7        8        0
    fn weight_norm_squared(self) -> AntiScalar {
        let flat_weight = self.flat_weight();
        return flat_weight.anti_dot_product(flat_weight);
    }
}
impl std::ops::Div<WeightNormSquaredPrefixOrPostfix> for Origin {
    type Output = AntiScalar;
    fn div(self, _rhs: WeightNormSquaredPrefixOrPostfix) -> Self::Output {
        self.weight_norm_squared()
    }
}
impl WeightNormSquared for Origin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn weight_norm_squared(self) -> AntiScalar {
        let flat_weight = self.flat_weight();
        return flat_weight.anti_dot_product(flat_weight);
    }
}
impl std::ops::Div<WeightNormSquaredPrefixOrPostfix> for Plane {
    type Output = AntiScalar;
    fn div(self, _rhs: WeightNormSquaredPrefixOrPostfix) -> Self::Output {
        self.weight_norm_squared()
    }
}
impl WeightNormSquared for Plane {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        3        0
    fn weight_norm_squared(self) -> AntiScalar {
        let flat_weight = self.flat_weight();
        return flat_weight.anti_dot_product(flat_weight);
    }
}
impl std::ops::Div<WeightNormSquaredPrefixOrPostfix> for Point {
    type Output = AntiScalar;
    fn div(self, _rhs: WeightNormSquaredPrefixOrPostfix) -> Self::Output {
        self.weight_norm_squared()
    }
}
impl WeightNormSquared for Point {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn weight_norm_squared(self) -> AntiScalar {
        let flat_weight = self.flat_weight();
        return flat_weight.anti_dot_product(flat_weight);
    }
}
