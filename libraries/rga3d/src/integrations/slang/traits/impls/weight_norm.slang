using traits::AntiSquareRoot;
using traits::FlatWeightNormSquared;
impl std::ops::Div<weight_norm> for AntiScalar {
    type Output = AntiScalar;
    fn div(self, _rhs: weight_norm) -> Self::Output {
        self.weight_norm()
    }
}
impl std::ops::DivAssign<weight_norm> for AntiScalar {
    fn div_assign(&mut self, _rhs: weight_norm) {
        *self = self.weight_norm()
    }
}
impl WeightNorm for AntiScalar {
// Operative Statistics for this implementation:
//      add/sub      mul      div
// f32        0        1        0
    fn weight_norm(self) -> AntiScalar {
        return self.flat_weight_norm_squared().anti_square_root();
    }
}
impl std::ops::Div<weight_norm> for DualNum {
    type Output = AntiScalar;
    fn div(self, _rhs: weight_norm) -> Self::Output {
        self.weight_norm()
    }
}
impl WeightNorm for DualNum {
// Operative Statistics for this implementation:
//      add/sub      mul      div
// f32        0        1        0
    fn weight_norm(self) -> AntiScalar {
        return self.flat_weight_norm_squared().anti_square_root();
    }
}
impl std::ops::Div<weight_norm> for Flector {
    type Output = AntiScalar;
    fn div(self, _rhs: weight_norm) -> Self::Output {
        self.weight_norm()
    }
}
impl WeightNorm for Flector {
// Operative Statistics for this implementation:
//      add/sub      mul      div
// f32        3        4        0
    fn weight_norm(self) -> AntiScalar {
        return self.flat_weight_norm_squared().anti_square_root();
    }
}
impl std::ops::Div<weight_norm> for Line {
    type Output = AntiScalar;
    fn div(self, _rhs: weight_norm) -> Self::Output {
        self.weight_norm()
    }
}
impl WeightNorm for Line {
// Operative Statistics for this implementation:
//      add/sub      mul      div
// f32        2        3        0
    fn weight_norm(self) -> AntiScalar {
        return self.flat_weight_norm_squared().anti_square_root();
    }
}
impl std::ops::Div<weight_norm> for Motor {
    type Output = AntiScalar;
    fn div(self, _rhs: weight_norm) -> Self::Output {
        self.weight_norm()
    }
}
impl WeightNorm for Motor {
// Operative Statistics for this implementation:
//      add/sub      mul      div
// f32        3        4        0
    fn weight_norm(self) -> AntiScalar {
        return self.flat_weight_norm_squared().anti_square_root();
    }
}
impl std::ops::Div<weight_norm> for MultiVector {
    type Output = AntiScalar;
    fn div(self, _rhs: weight_norm) -> Self::Output {
        self.weight_norm()
    }
}
impl WeightNorm for MultiVector {
// Operative Statistics for this implementation:
//      add/sub      mul      div
// f32        7        8        0
    fn weight_norm(self) -> AntiScalar {
        return self.flat_weight_norm_squared().anti_square_root();
    }
}
impl std::ops::Div<weight_norm> for Origin {
    type Output = AntiScalar;
    fn div(self, _rhs: weight_norm) -> Self::Output {
        self.weight_norm()
    }
}
impl WeightNorm for Origin {
// Operative Statistics for this implementation:
//      add/sub      mul      div
// f32        0        1        0
    fn weight_norm(self) -> AntiScalar {
        return self.flat_weight_norm_squared().anti_square_root();
    }
}
impl std::ops::Div<weight_norm> for Plane {
    type Output = AntiScalar;
    fn div(self, _rhs: weight_norm) -> Self::Output {
        self.weight_norm()
    }
}
impl WeightNorm for Plane {
// Operative Statistics for this implementation:
//      add/sub      mul      div
// f32        2        3        0
    fn weight_norm(self) -> AntiScalar {
        return self.flat_weight_norm_squared().anti_square_root();
    }
}
impl std::ops::Div<weight_norm> for Point {
    type Output = AntiScalar;
    fn div(self, _rhs: weight_norm) -> Self::Output {
        self.weight_norm()
    }
}
impl WeightNorm for Point {
// Operative Statistics for this implementation:
//      add/sub      mul      div
// f32        0        1        0
    fn weight_norm(self) -> AntiScalar {
        return self.flat_weight_norm_squared().anti_square_root();
    }
}
