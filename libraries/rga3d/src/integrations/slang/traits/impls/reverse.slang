impl std::ops::Div<reverse> for AntiScalar {
    type Output = AntiScalar;
    fn div(self, _rhs: reverse) -> Self::Output {
        self.reverse()
    }
}
impl std::ops::DivAssign<reverse> for AntiScalar {
    fn div_assign(&mut self, _rhs: reverse) {
        *self = self.reverse()
    }
}
impl Reverse for AntiScalar {
    fn reverse(self) -> Self {
        return self;
    }
}
impl std::ops::Div<reverse> for DualNum {
    type Output = DualNum;
    fn div(self, _rhs: reverse) -> Self::Output {
        self.reverse()
    }
}
impl std::ops::DivAssign<reverse> for DualNum {
    fn div_assign(&mut self, _rhs: reverse) {
        *self = self.reverse()
    }
}
impl Reverse for DualNum {
    fn reverse(self) -> Self {
        return self;
    }
}
impl std::ops::Div<reverse> for Flector {
    type Output = Flector;
    fn div(self, _rhs: reverse) -> Self::Output {
        self.reverse()
    }
}
impl std::ops::DivAssign<reverse> for Flector {
    fn div_assign(&mut self, _rhs: reverse) {
        *self = self.reverse()
    }
}
impl Reverse for Flector {
// Operative Statistics for this implementation:
//          add/sub      mul      div
//   simd4        0        1        0
// no simd        0        4        0
    fn reverse(self) -> Self {
        return Flector::from_groups(/* e1, e2, e3, e4 */self.group0(), /* e423, e431, e412, e321 */self.group1() * float4(-1.0));
    }
}
impl std::ops::Div<reverse> for Horizon {
    type Output = Horizon;
    fn div(self, _rhs: reverse) -> Self::Output {
        self.reverse()
    }
}
impl std::ops::DivAssign<reverse> for Horizon {
    fn div_assign(&mut self, _rhs: reverse) {
        *self = self.reverse()
    }
}
impl Reverse for Horizon {
// Operative Statistics for this implementation:
//      add/sub      mul      div
// f32        0        1        0
    fn reverse(self) -> Self {
        use crate::elements::*;
        return Horizon::from_groups(/* e321 */self[e321] * -1.0);
    }
}
impl std::ops::Div<reverse> for Line {
    type Output = Line;
    fn div(self, _rhs: reverse) -> Self::Output {
        self.reverse()
    }
}
impl std::ops::DivAssign<reverse> for Line {
    fn div_assign(&mut self, _rhs: reverse) {
        *self = self.reverse()
    }
}
impl Reverse for Line {
// Operative Statistics for this implementation:
//          add/sub      mul      div
//   simd3        0        2        0
// no simd        0        6        0
    fn reverse(self) -> Self {
        return Line::from_groups(/* e41, e42, e43 */self.group0() * float3(-1.0), /* e23, e31, e12 */self.group1() * float3(-1.0));
    }
}
impl std::ops::Div<reverse> for Motor {
    type Output = Motor;
    fn div(self, _rhs: reverse) -> Self::Output {
        self.reverse()
    }
}
impl std::ops::DivAssign<reverse> for Motor {
    fn div_assign(&mut self, _rhs: reverse) {
        *self = self.reverse()
    }
}
impl Reverse for Motor {
// Operative Statistics for this implementation:
//          add/sub      mul      div
//   simd4        0        2        0
// no simd        0        8        0
    fn reverse(self) -> Self {
        return Motor::from_groups(/* e41, e42, e43, e1234 */self.group0() * float4(-1.0, -1.0, -1.0, 1.0), /* e23, e31, e12, scalar */self.group1() * float4(-1.0, -1.0, -1.0, 1.0));
    }
}
impl std::ops::Div<reverse> for MultiVector {
    type Output = MultiVector;
    fn div(self, _rhs: reverse) -> Self::Output {
        self.reverse()
    }
}
impl std::ops::DivAssign<reverse> for MultiVector {
    fn div_assign(&mut self, _rhs: reverse) {
        *self = self.reverse()
    }
}
impl Reverse for MultiVector {
// Operative Statistics for this implementation:
//           add/sub      mul      div
//    simd3        0        2        0
//    simd4        0        1        0
// Totals...
// yes simd        0        3        0
//  no simd        0       10        0
    fn reverse(self) -> Self {
        return MultiVector::from_groups(/* scalar, e1234 */self.group0(), /* e1, e2, e3, e4 */self.group1(), /* e41, e42, e43 */self.group2() * float3(-1.0), /* e23, e31, e12 */self.group3() * float3(-1.0), /* e423, e431, e412, e321 */self.group4() * float4(-1.0));
    }
}
impl std::ops::Div<reverse> for Origin {
    type Output = Origin;
    fn div(self, _rhs: reverse) -> Self::Output {
        self.reverse()
    }
}
impl std::ops::DivAssign<reverse> for Origin {
    fn div_assign(&mut self, _rhs: reverse) {
        *self = self.reverse()
    }
}
impl Reverse for Origin {
    fn reverse(self) -> Self {
        return self;
    }
}
impl std::ops::Div<reverse> for Plane {
    type Output = Plane;
    fn div(self, _rhs: reverse) -> Self::Output {
        self.reverse()
    }
}
impl std::ops::DivAssign<reverse> for Plane {
    fn div_assign(&mut self, _rhs: reverse) {
        *self = self.reverse()
    }
}
impl Reverse for Plane {
// Operative Statistics for this implementation:
//          add/sub      mul      div
//   simd4        0        1        0
// no simd        0        4        0
    fn reverse(self) -> Self {
        return Plane::from_groups(/* e423, e431, e412, e321 */self.group0() * float4(-1.0));
    }
}
impl std::ops::Div<reverse> for Point {
    type Output = Point;
    fn div(self, _rhs: reverse) -> Self::Output {
        self.reverse()
    }
}
impl std::ops::DivAssign<reverse> for Point {
    fn div_assign(&mut self, _rhs: reverse) {
        *self = self.reverse()
    }
}
impl Reverse for Point {
    fn reverse(self) -> Self {
        return self;
    }
}
impl std::ops::Div<reverse> for Scalar {
    type Output = Scalar;
    fn div(self, _rhs: reverse) -> Self::Output {
        self.reverse()
    }
}
impl std::ops::DivAssign<reverse> for Scalar {
    fn div_assign(&mut self, _rhs: reverse) {
        *self = self.reverse()
    }
}
impl Reverse for Scalar {
    fn reverse(self) -> Self {
        return self;
    }
}
