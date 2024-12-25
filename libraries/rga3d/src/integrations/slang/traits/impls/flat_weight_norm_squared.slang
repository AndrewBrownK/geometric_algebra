using traits::AntiDotProduct;
using traits::FlatWeight;
impl std::ops::Div<flat_weight_norm_squared> for AntiScalar {
    type Output = AntiScalar;
    fn div(self, _rhs: flat_weight_norm_squared) -> Self::Output {
        self.flat_weight_norm_squared()
    }
}
impl std::ops::DivAssign<flat_weight_norm_squared> for AntiScalar {
    fn div_assign(&mut self, _rhs: flat_weight_norm_squared) {
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
impl std::ops::Div<flat_weight_norm_squared> for DualNum {
    type Output = AntiScalar;
    fn div(self, _rhs: flat_weight_norm_squared) -> Self::Output {
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
impl std::ops::Div<flat_weight_norm_squared> for Flector {
    type Output = AntiScalar;
    fn div(self, _rhs: flat_weight_norm_squared) -> Self::Output {
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
impl std::ops::Div<flat_weight_norm_squared> for Line {
    type Output = AntiScalar;
    fn div(self, _rhs: flat_weight_norm_squared) -> Self::Output {
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
impl std::ops::Div<flat_weight_norm_squared> for Motor {
    type Output = AntiScalar;
    fn div(self, _rhs: flat_weight_norm_squared) -> Self::Output {
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
impl std::ops::Div<flat_weight_norm_squared> for MultiVector {
    type Output = AntiScalar;
    fn div(self, _rhs: flat_weight_norm_squared) -> Self::Output {
        self.flat_weight_norm_squared()
    }
}
impl FlatWeightNormSquared for MultiVector {
// Operative Statistics for this implementation:
//      add/sub      mul      div
// f32        7        8        0
    fn flat_weight_norm_squared(self) -> AntiScalar {
let flat_weight = self.flat_weight();
        return flat_weight.anti_dot_product(flat_weight);
    }
}
impl std::ops::Div<flat_weight_norm_squared> for Origin {
    type Output = AntiScalar;
    fn div(self, _rhs: flat_weight_norm_squared) -> Self::Output {
        self.flat_weight_norm_squared()
    }
}
impl FlatWeightNormSquared for Origin {
// Operative Statistics for this implementation:
//      add/sub      mul      div
// f32        0        1        0
    fn flat_weight_norm_squared(self) -> AntiScalar {
let flat_weight = self.flat_weight();
        return flat_weight.anti_dot_product(flat_weight);
    }
}
impl std::ops::Div<flat_weight_norm_squared> for Plane {
    type Output = AntiScalar;
    fn div(self, _rhs: flat_weight_norm_squared) -> Self::Output {
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
impl std::ops::Div<flat_weight_norm_squared> for Point {
    type Output = AntiScalar;
    fn div(self, _rhs: flat_weight_norm_squared) -> Self::Output {
        self.flat_weight_norm_squared()
    }
}
impl FlatWeightNormSquared for Point {
// Operative Statistics for this implementation:
//      add/sub      mul      div
// f32        0        1        0
    fn flat_weight_norm_squared(self) -> AntiScalar {
let flat_weight = self.flat_weight();
        return flat_weight.anti_dot_product(flat_weight);
    }
}
