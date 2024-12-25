impl std::ops::Div<double_complement> for AntiScalar {
    type Output = AntiScalar;
    fn div(self, _rhs: double_complement) -> Self::Output {
        self.double_complement()
    }
}
impl std::ops::DivAssign<double_complement> for AntiScalar {
    fn div_assign(&mut self, _rhs: double_complement) {
        *self = self.double_complement()
    }
}
impl DoubleComplement for AntiScalar {
    fn double_complement(self) -> Self {
        return self;
    }
}
impl std::ops::Div<double_complement> for DualNum {
    type Output = DualNum;
    fn div(self, _rhs: double_complement) -> Self::Output {
        self.double_complement()
    }
}
impl std::ops::DivAssign<double_complement> for DualNum {
    fn div_assign(&mut self, _rhs: double_complement) {
        *self = self.double_complement()
    }
}
impl DoubleComplement for DualNum {
    fn double_complement(self) -> Self {
        return self;
    }
}
impl std::ops::Div<double_complement> for Flector {
    type Output = Flector;
    fn div(self, _rhs: double_complement) -> Self::Output {
        self.double_complement()
    }
}
impl std::ops::DivAssign<double_complement> for Flector {
    fn div_assign(&mut self, _rhs: double_complement) {
        *self = self.double_complement()
    }
}
impl DoubleComplement for Flector {
// Operative Statistics for this implementation:
//          add/sub      mul      div
//   simd4        0        2        0
// no simd        0        8        0
    fn double_complement(self) -> Self {
        return Flector::from_groups(/* e1, e2, e3, e4 */self.group0() * float4(-1.0), /* e423, e431, e412, e321 */self.group1() * float4(-1.0));
    }
}
impl std::ops::Div<double_complement> for Horizon {
    type Output = Horizon;
    fn div(self, _rhs: double_complement) -> Self::Output {
        self.double_complement()
    }
}
impl std::ops::DivAssign<double_complement> for Horizon {
    fn div_assign(&mut self, _rhs: double_complement) {
        *self = self.double_complement()
    }
}
impl DoubleComplement for Horizon {
// Operative Statistics for this implementation:
//      add/sub      mul      div
// f32        0        1        0
    fn double_complement(self) -> Self {
        use crate::elements::*;
        return Horizon::from_groups(/* e321 */self[e321] * -1.0);
    }
}
impl std::ops::Div<double_complement> for Line {
    type Output = Line;
    fn div(self, _rhs: double_complement) -> Self::Output {
        self.double_complement()
    }
}
impl std::ops::DivAssign<double_complement> for Line {
    fn div_assign(&mut self, _rhs: double_complement) {
        *self = self.double_complement()
    }
}
impl DoubleComplement for Line {
    fn double_complement(self) -> Self {
        return self;
    }
}
impl std::ops::Div<double_complement> for Motor {
    type Output = Motor;
    fn div(self, _rhs: double_complement) -> Self::Output {
        self.double_complement()
    }
}
impl std::ops::DivAssign<double_complement> for Motor {
    fn div_assign(&mut self, _rhs: double_complement) {
        *self = self.double_complement()
    }
}
impl DoubleComplement for Motor {
    fn double_complement(self) -> Self {
        return self;
    }
}
impl std::ops::Div<double_complement> for MultiVector {
    type Output = MultiVector;
    fn div(self, _rhs: double_complement) -> Self::Output {
        self.double_complement()
    }
}
impl std::ops::DivAssign<double_complement> for MultiVector {
    fn div_assign(&mut self, _rhs: double_complement) {
        *self = self.double_complement()
    }
}
impl DoubleComplement for MultiVector {
// Operative Statistics for this implementation:
//          add/sub      mul      div
//   simd4        0        2        0
// no simd        0        8        0
    fn double_complement(self) -> Self {
        return MultiVector::from_groups(/* scalar, e1234 */self.group0(), /* e1, e2, e3, e4 */self.group1() * float4(-1.0), /* e41, e42, e43 */self.group2(), /* e23, e31, e12 */self.group3(), /* e423, e431, e412, e321 */self.group4() * float4(-1.0));
    }
}
impl std::ops::Div<double_complement> for Origin {
    type Output = Origin;
    fn div(self, _rhs: double_complement) -> Self::Output {
        self.double_complement()
    }
}
impl std::ops::DivAssign<double_complement> for Origin {
    fn div_assign(&mut self, _rhs: double_complement) {
        *self = self.double_complement()
    }
}
impl DoubleComplement for Origin {
// Operative Statistics for this implementation:
//      add/sub      mul      div
// f32        0        1        0
    fn double_complement(self) -> Self {
        use crate::elements::*;
        return Origin::from_groups(/* e4 */self[e4] * -1.0);
    }
}
impl std::ops::Div<double_complement> for Plane {
    type Output = Plane;
    fn div(self, _rhs: double_complement) -> Self::Output {
        self.double_complement()
    }
}
impl std::ops::DivAssign<double_complement> for Plane {
    fn div_assign(&mut self, _rhs: double_complement) {
        *self = self.double_complement()
    }
}
impl DoubleComplement for Plane {
// Operative Statistics for this implementation:
//          add/sub      mul      div
//   simd4        0        1        0
// no simd        0        4        0
    fn double_complement(self) -> Self {
        return Plane::from_groups(/* e423, e431, e412, e321 */self.group0() * float4(-1.0));
    }
}
impl std::ops::Div<double_complement> for Point {
    type Output = Point;
    fn div(self, _rhs: double_complement) -> Self::Output {
        self.double_complement()
    }
}
impl std::ops::DivAssign<double_complement> for Point {
    fn div_assign(&mut self, _rhs: double_complement) {
        *self = self.double_complement()
    }
}
impl DoubleComplement for Point {
// Operative Statistics for this implementation:
//          add/sub      mul      div
//   simd4        0        1        0
// no simd        0        4        0
    fn double_complement(self) -> Self {
        return Point::from_groups(/* e1, e2, e3, e4 */self.group0() * float4(-1.0));
    }
}
impl std::ops::Div<double_complement> for Scalar {
    type Output = Scalar;
    fn div(self, _rhs: double_complement) -> Self::Output {
        self.double_complement()
    }
}
impl std::ops::DivAssign<double_complement> for Scalar {
    fn div_assign(&mut self, _rhs: double_complement) {
        *self = self.double_complement()
    }
}
impl DoubleComplement for Scalar {
    fn double_complement(self) -> Self {
        return self;
    }
}
