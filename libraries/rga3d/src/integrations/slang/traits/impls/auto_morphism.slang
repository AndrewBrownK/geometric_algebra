impl std::ops::Div<auto_morphism> for AntiScalar {
    type Output = AntiScalar;
    fn div(self, _rhs: auto_morphism) -> Self::Output {
        self.auto_morphism()
    }
}
impl std::ops::DivAssign<auto_morphism> for AntiScalar {
    fn div_assign(&mut self, _rhs: auto_morphism) {
        *self = self.auto_morphism()
    }
}
impl AutoMorphism for AntiScalar {
    fn auto_morphism(self) -> Self {
        return self;
    }
}
impl std::ops::Div<auto_morphism> for DualNum {
    type Output = DualNum;
    fn div(self, _rhs: auto_morphism) -> Self::Output {
        self.auto_morphism()
    }
}
impl std::ops::DivAssign<auto_morphism> for DualNum {
    fn div_assign(&mut self, _rhs: auto_morphism) {
        *self = self.auto_morphism()
    }
}
impl AutoMorphism for DualNum {
    fn auto_morphism(self) -> Self {
        return self;
    }
}
impl std::ops::Div<auto_morphism> for Flector {
    type Output = Flector;
    fn div(self, _rhs: auto_morphism) -> Self::Output {
        self.auto_morphism()
    }
}
impl std::ops::DivAssign<auto_morphism> for Flector {
    fn div_assign(&mut self, _rhs: auto_morphism) {
        *self = self.auto_morphism()
    }
}
impl AutoMorphism for Flector {
// Operative Statistics for this implementation:
//          add/sub      mul      div
//   simd4        0        2        0
// no simd        0        8        0
    fn auto_morphism(self) -> Self {
        return Flector::from_groups(/* e1, e2, e3, e4 */self.group0() * float4(-1.0), /* e423, e431, e412, e321 */self.group1() * float4(-1.0));
    }
}
impl std::ops::Div<auto_morphism> for Horizon {
    type Output = Horizon;
    fn div(self, _rhs: auto_morphism) -> Self::Output {
        self.auto_morphism()
    }
}
impl std::ops::DivAssign<auto_morphism> for Horizon {
    fn div_assign(&mut self, _rhs: auto_morphism) {
        *self = self.auto_morphism()
    }
}
impl AutoMorphism for Horizon {
// Operative Statistics for this implementation:
//      add/sub      mul      div
// f32        0        1        0
    fn auto_morphism(self) -> Self {
        use crate::elements::*;
        return Horizon::from_groups(/* e321 */self[e321] * -1.0);
    }
}
impl std::ops::Div<auto_morphism> for Line {
    type Output = Line;
    fn div(self, _rhs: auto_morphism) -> Self::Output {
        self.auto_morphism()
    }
}
impl std::ops::DivAssign<auto_morphism> for Line {
    fn div_assign(&mut self, _rhs: auto_morphism) {
        *self = self.auto_morphism()
    }
}
impl AutoMorphism for Line {
    fn auto_morphism(self) -> Self {
        return self;
    }
}
impl std::ops::Div<auto_morphism> for Motor {
    type Output = Motor;
    fn div(self, _rhs: auto_morphism) -> Self::Output {
        self.auto_morphism()
    }
}
impl std::ops::DivAssign<auto_morphism> for Motor {
    fn div_assign(&mut self, _rhs: auto_morphism) {
        *self = self.auto_morphism()
    }
}
impl AutoMorphism for Motor {
    fn auto_morphism(self) -> Self {
        return self;
    }
}
impl std::ops::Div<auto_morphism> for MultiVector {
    type Output = MultiVector;
    fn div(self, _rhs: auto_morphism) -> Self::Output {
        self.auto_morphism()
    }
}
impl std::ops::DivAssign<auto_morphism> for MultiVector {
    fn div_assign(&mut self, _rhs: auto_morphism) {
        *self = self.auto_morphism()
    }
}
impl AutoMorphism for MultiVector {
// Operative Statistics for this implementation:
//          add/sub      mul      div
//   simd4        0        2        0
// no simd        0        8        0
    fn auto_morphism(self) -> Self {
        return MultiVector::from_groups(/* scalar, e1234 */self.group0(), /* e1, e2, e3, e4 */self.group1() * float4(-1.0), /* e41, e42, e43 */self.group2(), /* e23, e31, e12 */self.group3(), /* e423, e431, e412, e321 */self.group4() * float4(-1.0));
    }
}
impl std::ops::Div<auto_morphism> for Origin {
    type Output = Origin;
    fn div(self, _rhs: auto_morphism) -> Self::Output {
        self.auto_morphism()
    }
}
impl std::ops::DivAssign<auto_morphism> for Origin {
    fn div_assign(&mut self, _rhs: auto_morphism) {
        *self = self.auto_morphism()
    }
}
impl AutoMorphism for Origin {
// Operative Statistics for this implementation:
//      add/sub      mul      div
// f32        0        1        0
    fn auto_morphism(self) -> Self {
        use crate::elements::*;
        return Origin::from_groups(/* e4 */self[e4] * -1.0);
    }
}
impl std::ops::Div<auto_morphism> for Plane {
    type Output = Plane;
    fn div(self, _rhs: auto_morphism) -> Self::Output {
        self.auto_morphism()
    }
}
impl std::ops::DivAssign<auto_morphism> for Plane {
    fn div_assign(&mut self, _rhs: auto_morphism) {
        *self = self.auto_morphism()
    }
}
impl AutoMorphism for Plane {
// Operative Statistics for this implementation:
//          add/sub      mul      div
//   simd4        0        1        0
// no simd        0        4        0
    fn auto_morphism(self) -> Self {
        return Plane::from_groups(/* e423, e431, e412, e321 */self.group0() * float4(-1.0));
    }
}
impl std::ops::Div<auto_morphism> for Point {
    type Output = Point;
    fn div(self, _rhs: auto_morphism) -> Self::Output {
        self.auto_morphism()
    }
}
impl std::ops::DivAssign<auto_morphism> for Point {
    fn div_assign(&mut self, _rhs: auto_morphism) {
        *self = self.auto_morphism()
    }
}
impl AutoMorphism for Point {
// Operative Statistics for this implementation:
//          add/sub      mul      div
//   simd4        0        1        0
// no simd        0        4        0
    fn auto_morphism(self) -> Self {
        return Point::from_groups(/* e1, e2, e3, e4 */self.group0() * float4(-1.0));
    }
}
impl std::ops::Div<auto_morphism> for Scalar {
    type Output = Scalar;
    fn div(self, _rhs: auto_morphism) -> Self::Output {
        self.auto_morphism()
    }
}
impl std::ops::DivAssign<auto_morphism> for Scalar {
    fn div_assign(&mut self, _rhs: auto_morphism) {
        *self = self.auto_morphism()
    }
}
impl AutoMorphism for Scalar {
    fn auto_morphism(self) -> Self {
        return self;
    }
}
