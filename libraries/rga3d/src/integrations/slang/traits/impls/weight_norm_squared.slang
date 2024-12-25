using traits::AntiDotProduct;
using traits::FlatWeight;
impl std::ops::Div<weight_norm_squared> for AntiScalar {
    type Output = AntiScalar;
    fn div(self, _rhs: weight_norm_squared) -> Self::Output {
        self.weight_norm_squared()
    }
}
impl std::ops::DivAssign<weight_norm_squared> for AntiScalar {
    fn div_assign(&mut self, _rhs: weight_norm_squared) {
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
impl std::ops::Div<weight_norm_squared> for DualNum {
    type Output = AntiScalar;
    fn div(self, _rhs: weight_norm_squared) -> Self::Output {
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
impl std::ops::Div<weight_norm_squared> for Flector {
    type Output = AntiScalar;
    fn div(self, _rhs: weight_norm_squared) -> Self::Output {
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
impl std::ops::Div<weight_norm_squared> for Line {
    type Output = AntiScalar;
    fn div(self, _rhs: weight_norm_squared) -> Self::Output {
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
impl std::ops::Div<weight_norm_squared> for Motor {
    type Output = AntiScalar;
    fn div(self, _rhs: weight_norm_squared) -> Self::Output {
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
impl std::ops::Div<weight_norm_squared> for MultiVector {
    type Output = AntiScalar;
    fn div(self, _rhs: weight_norm_squared) -> Self::Output {
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
impl std::ops::Div<weight_norm_squared> for Origin {
    type Output = AntiScalar;
    fn div(self, _rhs: weight_norm_squared) -> Self::Output {
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
impl std::ops::Div<weight_norm_squared> for Plane {
    type Output = AntiScalar;
    fn div(self, _rhs: weight_norm_squared) -> Self::Output {
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
impl std::ops::Div<weight_norm_squared> for Point {
    type Output = AntiScalar;
    fn div(self, _rhs: weight_norm_squared) -> Self::Output {
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
