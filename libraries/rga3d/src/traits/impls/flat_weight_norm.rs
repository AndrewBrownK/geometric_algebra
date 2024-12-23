use crate::traits::AntiSquareRoot;
use crate::traits::FlatWeightNormSquared;
// Note on Operative Statistics:
// Operative Statistics are not a precise predictor of performance or performance comparisons.
// This is due to varying hardware capabilities and compiler optimizations.
// As always, where performance is a concern, there is no substitute for
// real measurements on real work-loads on real hardware.
// Disclaimer aside, enjoy the fun information =)
//
// Total Implementations: 5
//
// Yes SIMD:   add/sub     mul     div
//  Minimum:         0       1       0
//   Median:         3       4       0
//  Average:         3       4       0
//  Maximum:         7       8       0
//
//  No SIMD:   add/sub     mul     div
//  Minimum:         0       1       0
//   Median:         3       4       0
//  Average:         3       4       0
//  Maximum:         7       8       0
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
    // f32        7        8        0
    fn flat_weight_norm(self) -> AntiScalar {
        return self.flat_weight_norm_squared().anti_square_root();
    }
}
impl std::ops::Div<flat_weight_norm> for Point {
    type Output = AntiScalar;
    fn div(self, _rhs: flat_weight_norm) -> Self::Output {
        self.flat_weight_norm()
    }
}
impl FlatWeightNorm for Point {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn flat_weight_norm(self) -> AntiScalar {
        return self.flat_weight_norm_squared().anti_square_root();
    }
}
