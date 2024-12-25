impl std::ops::Div<fix> for Horizon {
    type Output = Horizon;
    fn div(self, _rhs: fix) -> Self::Output {
        self.fix()
    }
}
impl std::ops::DivAssign<fix> for Horizon {
    fn div_assign(&mut self, _rhs: fix) {
        *self = self.fix()
    }
}
impl Fix for Horizon {
    fn fix(self) -> Self {
        return Horizon::from_groups(/* e321 */1.0);
    }
}
impl std::ops::Div<fix> for Plane {
    type Output = Plane;
    fn div(self, _rhs: fix) -> Self::Output {
        self.fix()
    }
}
impl std::ops::DivAssign<fix> for Plane {
    fn div_assign(&mut self, _rhs: fix) {
        *self = self.fix()
    }
}
impl Fix for Plane {
// Operative Statistics for this implementation:
//           add/sub      mul      div
//      f32        0        1        0
//    simd4        0        2        0
// Totals...
// yes simd        0        3        0
//  no simd        0        9        0
    fn fix(self) -> Self {
        use crate::elements::*;
        return Plane::from_groups(/* e423, e431, e412, e321 */float4(powf((self.group0() * float4(-1.0))[3], -0.5) * powf(self[e321], -0.5) * -1.0) * self.group0());
    }
}
impl std::ops::Div<fix> for Point {
    type Output = Point;
    fn div(self, _rhs: fix) -> Self::Output {
        self.fix()
    }
}
impl std::ops::DivAssign<fix> for Point {
    fn div_assign(&mut self, _rhs: fix) {
        *self = self.fix()
    }
}
impl Fix for Point {
// Operative Statistics for this implementation:
//           add/sub      mul      div
//      f32        2        0        0
//    simd4        0        1        0
// Totals...
// yes simd        2        1        0
//  no simd        2        4        0
    fn fix(self) -> Self {
        use crate::elements::*;
        return Point::from_groups(/* e1, e2, e3, e4 */float4(powi(self[e1], 2) + powi(self[e2], 2) + powi(self[e3], 2)) * self.group0());
    }
}
impl std::ops::Div<fix> for Scalar {
    type Output = Scalar;
    fn div(self, _rhs: fix) -> Self::Output {
        self.fix()
    }
}
impl std::ops::DivAssign<fix> for Scalar {
    fn div_assign(&mut self, _rhs: fix) {
        *self = self.fix()
    }
}
impl Fix for Scalar {
    fn fix(self) -> Self {
        return Scalar::from_groups(/* scalar */1.0);
    }
}
