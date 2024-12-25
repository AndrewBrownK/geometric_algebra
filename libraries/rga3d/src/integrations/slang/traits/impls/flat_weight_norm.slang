using traits::AntiSquareRoot;
using traits::FlatWeightNormSquared;
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
