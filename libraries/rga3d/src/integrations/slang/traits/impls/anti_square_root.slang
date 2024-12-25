impl std::ops::Div<anti_square_root> for AntiScalar {
    type Output = AntiScalar;
    fn div(self, _rhs: anti_square_root) -> Self::Output {
        self.anti_square_root()
    }
}
impl std::ops::DivAssign<anti_square_root> for AntiScalar {
    fn div_assign(&mut self, _rhs: anti_square_root) {
        *self = self.anti_square_root()
    }
}
impl AntiSquareRoot for AntiScalar {
    fn anti_square_root(self) -> AntiScalar {
        use crate::elements::*;
        return AntiScalar::from_groups(/* e1234 */powf(self[e1234], 0.5));
    }
}
