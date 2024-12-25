impl std::ops::Div<conjugation> for AntiScalar {
    type Output = AntiScalar;
    fn div(self, _rhs: conjugation) -> Self::Output {
        self.conjugation()
    }
}
impl std::ops::DivAssign<conjugation> for AntiScalar {
    fn div_assign(&mut self, _rhs: conjugation) {
        *self = self.conjugation()
    }
}
impl Conjugation for AntiScalar {
    fn conjugation(self) -> Self {
        return self;
    }
}
impl std::ops::Div<conjugation> for DualNum {
    type Output = DualNum;
    fn div(self, _rhs: conjugation) -> Self::Output {
        self.conjugation()
    }
}
impl std::ops::DivAssign<conjugation> for DualNum {
    fn div_assign(&mut self, _rhs: conjugation) {
        *self = self.conjugation()
    }
}
impl Conjugation for DualNum {
    fn conjugation(self) -> Self {
        return self;
    }
}
impl std::ops::Div<conjugation> for Flector {
    type Output = Flector;
    fn div(self, _rhs: conjugation) -> Self::Output {
        self.conjugation()
    }
}
impl std::ops::DivAssign<conjugation> for Flector {
    fn div_assign(&mut self, _rhs: conjugation) {
        *self = self.conjugation()
    }
}
impl Conjugation for Flector {
// Operative Statistics for this implementation:
//          add/sub      mul      div
//   simd4        0        1        0
// no simd        0        4        0
    fn conjugation(self) -> Self {
        return Flector::from_groups(/* e1, e2, e3, e4 */self.group0() * float4(-1.0), /* e423, e431, e412, e321 */self.group1());
    }
}
impl std::ops::Div<conjugation> for Horizon {
    type Output = Horizon;
    fn div(self, _rhs: conjugation) -> Self::Output {
        self.conjugation()
    }
}
impl std::ops::DivAssign<conjugation> for Horizon {
    fn div_assign(&mut self, _rhs: conjugation) {
        *self = self.conjugation()
    }
}
impl Conjugation for Horizon {
    fn conjugation(self) -> Self {
        return self;
    }
}
impl std::ops::Div<conjugation> for Line {
    type Output = Line;
    fn div(self, _rhs: conjugation) -> Self::Output {
        self.conjugation()
    }
}
impl std::ops::DivAssign<conjugation> for Line {
    fn div_assign(&mut self, _rhs: conjugation) {
        *self = self.conjugation()
    }
}
impl Conjugation for Line {
// Operative Statistics for this implementation:
//          add/sub      mul      div
//   simd3        0        2        0
// no simd        0        6        0
    fn conjugation(self) -> Self {
        return Line::from_groups(/* e41, e42, e43 */self.group0() * float3(-1.0), /* e23, e31, e12 */self.group1() * float3(-1.0));
    }
}
impl std::ops::Div<conjugation> for Motor {
    type Output = Motor;
    fn div(self, _rhs: conjugation) -> Self::Output {
        self.conjugation()
    }
}
impl std::ops::DivAssign<conjugation> for Motor {
    fn div_assign(&mut self, _rhs: conjugation) {
        *self = self.conjugation()
    }
}
impl Conjugation for Motor {
// Operative Statistics for this implementation:
//          add/sub      mul      div
//   simd4        0        2        0
// no simd        0        8        0
    fn conjugation(self) -> Self {
        return Motor::from_groups(/* e41, e42, e43, e1234 */self.group0() * float4(-1.0, -1.0, -1.0, 1.0), /* e23, e31, e12, scalar */self.group1() * float4(-1.0, -1.0, -1.0, 1.0));
    }
}
impl std::ops::Div<conjugation> for MultiVector {
    type Output = MultiVector;
    fn div(self, _rhs: conjugation) -> Self::Output {
        self.conjugation()
    }
}
impl std::ops::DivAssign<conjugation> for MultiVector {
    fn div_assign(&mut self, _rhs: conjugation) {
        *self = self.conjugation()
    }
}
impl Conjugation for MultiVector {
// Operative Statistics for this implementation:
//           add/sub      mul      div
//    simd3        0        2        0
//    simd4        0        1        0
// Totals...
// yes simd        0        3        0
//  no simd        0       10        0
    fn conjugation(self) -> Self {
        return MultiVector::from_groups(/* scalar, e1234 */self.group0(), /* e1, e2, e3, e4 */self.group1() * float4(-1.0), /* e41, e42, e43 */self.group2() * float3(-1.0), /* e23, e31, e12 */self.group3() * float3(-1.0), /* e423, e431, e412, e321 */self.group4());
    }
}
impl std::ops::Div<conjugation> for Origin {
    type Output = Origin;
    fn div(self, _rhs: conjugation) -> Self::Output {
        self.conjugation()
    }
}
impl std::ops::DivAssign<conjugation> for Origin {
    fn div_assign(&mut self, _rhs: conjugation) {
        *self = self.conjugation()
    }
}
impl Conjugation for Origin {
// Operative Statistics for this implementation:
//      add/sub      mul      div
// f32        0        1        0
    fn conjugation(self) -> Self {
        use crate::elements::*;
        return Origin::from_groups(/* e4 */self[e4] * -1.0);
    }
}
impl std::ops::Div<conjugation> for Plane {
    type Output = Plane;
    fn div(self, _rhs: conjugation) -> Self::Output {
        self.conjugation()
    }
}
impl std::ops::DivAssign<conjugation> for Plane {
    fn div_assign(&mut self, _rhs: conjugation) {
        *self = self.conjugation()
    }
}
impl Conjugation for Plane {
    fn conjugation(self) -> Self {
        return self;
    }
}
impl std::ops::Div<conjugation> for Point {
    type Output = Point;
    fn div(self, _rhs: conjugation) -> Self::Output {
        self.conjugation()
    }
}
impl std::ops::DivAssign<conjugation> for Point {
    fn div_assign(&mut self, _rhs: conjugation) {
        *self = self.conjugation()
    }
}
impl Conjugation for Point {
// Operative Statistics for this implementation:
//          add/sub      mul      div
//   simd4        0        1        0
// no simd        0        4        0
    fn conjugation(self) -> Self {
        return Point::from_groups(/* e1, e2, e3, e4 */self.group0() * float4(-1.0));
    }
}
impl std::ops::Div<conjugation> for Scalar {
    type Output = Scalar;
    fn div(self, _rhs: conjugation) -> Self::Output {
        self.conjugation()
    }
}
impl std::ops::DivAssign<conjugation> for Scalar {
    fn div_assign(&mut self, _rhs: conjugation) {
        *self = self.conjugation()
    }
}
impl Conjugation for Scalar {
    fn conjugation(self) -> Self {
        return self;
    }
}
