impl std::ops::Div<flat_weight> for AntiScalar {
    type Output = AntiScalar;
    fn div(self, _rhs: flat_weight) -> Self::Output {
        self.flat_weight()
    }
}
impl std::ops::DivAssign<flat_weight> for AntiScalar {
    fn div_assign(&mut self, _rhs: flat_weight) {
        *self = self.flat_weight()
    }
}
impl FlatWeight for AntiScalar {
    type Output = AntiScalar;
    fn flat_weight(self) -> Self::Output {
        return self;
    }
}
impl std::ops::Div<flat_weight> for DualNum {
    type Output = AntiScalar;
    fn div(self, _rhs: flat_weight) -> Self::Output {
        self.flat_weight()
    }
}
impl FlatWeight for DualNum {
    type Output = AntiScalar;
    fn flat_weight(self) -> Self::Output {
        use crate::elements::*;
        return AntiScalar::from_groups(/* e1234 */self[e1234]);
    }
}
impl std::ops::Div<flat_weight> for Flector {
    type Output = Flector;
    fn div(self, _rhs: flat_weight) -> Self::Output {
        self.flat_weight()
    }
}
impl std::ops::DivAssign<flat_weight> for Flector {
    fn div_assign(&mut self, _rhs: flat_weight) {
        *self = self.flat_weight()
    }
}
impl FlatWeight for Flector {
    type Output = Flector;
    fn flat_weight(self) -> Self::Output {
        use crate::elements::*;
        return Flector::from_groups(/* e1, e2, e3, e4 */float4(float3(0.0), self[e4]), /* e423, e431, e412, e321 */float4(self[e423], self[e431], self[e412], 0.0));
    }
}
impl std::ops::Div<flat_weight> for Line {
    type Output = Line;
    fn div(self, _rhs: flat_weight) -> Self::Output {
        self.flat_weight()
    }
}
impl std::ops::DivAssign<flat_weight> for Line {
    fn div_assign(&mut self, _rhs: flat_weight) {
        *self = self.flat_weight()
    }
}
impl FlatWeight for Line {
    type Output = Line;
    fn flat_weight(self) -> Self::Output {
        return Line::from_groups(/* e41, e42, e43 */self.group0(), /* e23, e31, e12 */float3(0.0));
    }
}
impl std::ops::Div<flat_weight> for Motor {
    type Output = Motor;
    fn div(self, _rhs: flat_weight) -> Self::Output {
        self.flat_weight()
    }
}
impl std::ops::DivAssign<flat_weight> for Motor {
    fn div_assign(&mut self, _rhs: flat_weight) {
        *self = self.flat_weight()
    }
}
impl FlatWeight for Motor {
    type Output = Motor;
    fn flat_weight(self) -> Self::Output {
        return Motor::from_groups(/* e41, e42, e43, e1234 */self.group0(), /* e23, e31, e12, scalar */float4(0.0));
    }
}
impl std::ops::Div<flat_weight> for MultiVector {
    type Output = MultiVector;
    fn div(self, _rhs: flat_weight) -> Self::Output {
        self.flat_weight()
    }
}
impl std::ops::DivAssign<flat_weight> for MultiVector {
    fn div_assign(&mut self, _rhs: flat_weight) {
        *self = self.flat_weight()
    }
}
impl FlatWeight for MultiVector {
    type Output = MultiVector;
    fn flat_weight(self) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(/* scalar, e1234 */float2(0.0, self[e1234]), /* e1, e2, e3, e4 */float4(float3(0.0), self[e4]), /* e41, e42, e43 */self.group2(), /* e23, e31, e12 */float3(0.0), /* e423, e431, e412, e321 */float4(self[e423], self[e431], self[e412], 0.0));
    }
}
impl std::ops::Div<flat_weight> for Origin {
    type Output = Origin;
    fn div(self, _rhs: flat_weight) -> Self::Output {
        self.flat_weight()
    }
}
impl std::ops::DivAssign<flat_weight> for Origin {
    fn div_assign(&mut self, _rhs: flat_weight) {
        *self = self.flat_weight()
    }
}
impl FlatWeight for Origin {
    type Output = Origin;
    fn flat_weight(self) -> Self::Output {
        return self;
    }
}
impl std::ops::Div<flat_weight> for Plane {
    type Output = Plane;
    fn div(self, _rhs: flat_weight) -> Self::Output {
        self.flat_weight()
    }
}
impl std::ops::DivAssign<flat_weight> for Plane {
    fn div_assign(&mut self, _rhs: flat_weight) {
        *self = self.flat_weight()
    }
}
impl FlatWeight for Plane {
    type Output = Plane;
    fn flat_weight(self) -> Self::Output {
        use crate::elements::*;
        return Plane::from_groups(/* e423, e431, e412, e321 */float4(self[e423], self[e431], self[e412], 0.0));
    }
}
impl std::ops::Div<flat_weight> for Point {
    type Output = Origin;
    fn div(self, _rhs: flat_weight) -> Self::Output {
        self.flat_weight()
    }
}
impl FlatWeight for Point {
    type Output = Origin;
    fn flat_weight(self) -> Self::Output {
        use crate::elements::*;
        return Origin::from_groups(/* e4 */self[e4]);
    }
}
