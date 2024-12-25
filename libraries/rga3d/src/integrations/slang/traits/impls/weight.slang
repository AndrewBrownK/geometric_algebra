impl std::ops::Div<weight> for AntiScalar {
    type Output = AntiScalar;
    fn div(self, _rhs: weight) -> Self::Output {
        self.weight()
    }
}
impl std::ops::DivAssign<weight> for AntiScalar {
    fn div_assign(&mut self, _rhs: weight) {
        *self = self.weight()
    }
}
impl Weight for AntiScalar {
    type Output = AntiScalar;
    fn weight(self) -> Self::Output {
        return self;
    }
}
impl std::ops::Div<weight> for DualNum {
    type Output = AntiScalar;
    fn div(self, _rhs: weight) -> Self::Output {
        self.weight()
    }
}
impl Weight for DualNum {
    type Output = AntiScalar;
    fn weight(self) -> Self::Output {
        use crate::elements::*;
        return AntiScalar::from_groups(/* e1234 */self[e1234]);
    }
}
impl std::ops::Div<weight> for Flector {
    type Output = Flector;
    fn div(self, _rhs: weight) -> Self::Output {
        self.weight()
    }
}
impl std::ops::DivAssign<weight> for Flector {
    fn div_assign(&mut self, _rhs: weight) {
        *self = self.weight()
    }
}
impl Weight for Flector {
    type Output = Flector;
    fn weight(self) -> Self::Output {
        use crate::elements::*;
        return Flector::from_groups(/* e1, e2, e3, e4 */float4(float3(0.0), self[e4]), /* e423, e431, e412, e321 */float4(self[e423], self[e431], self[e412], 0.0));
    }
}
impl std::ops::Div<weight> for Line {
    type Output = Line;
    fn div(self, _rhs: weight) -> Self::Output {
        self.weight()
    }
}
impl std::ops::DivAssign<weight> for Line {
    fn div_assign(&mut self, _rhs: weight) {
        *self = self.weight()
    }
}
impl Weight for Line {
    type Output = Line;
    fn weight(self) -> Self::Output {
        return Line::from_groups(/* e41, e42, e43 */self.group0(), /* e23, e31, e12 */float3(0.0));
    }
}
impl std::ops::Div<weight> for Motor {
    type Output = Motor;
    fn div(self, _rhs: weight) -> Self::Output {
        self.weight()
    }
}
impl std::ops::DivAssign<weight> for Motor {
    fn div_assign(&mut self, _rhs: weight) {
        *self = self.weight()
    }
}
impl Weight for Motor {
    type Output = Motor;
    fn weight(self) -> Self::Output {
        return Motor::from_groups(/* e41, e42, e43, e1234 */self.group0(), /* e23, e31, e12, scalar */float4(0.0));
    }
}
impl std::ops::Div<weight> for MultiVector {
    type Output = MultiVector;
    fn div(self, _rhs: weight) -> Self::Output {
        self.weight()
    }
}
impl std::ops::DivAssign<weight> for MultiVector {
    fn div_assign(&mut self, _rhs: weight) {
        *self = self.weight()
    }
}
impl Weight for MultiVector {
    type Output = MultiVector;
    fn weight(self) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(/* scalar, e1234 */float2(0.0, self[e1234]), /* e1, e2, e3, e4 */float4(float3(0.0), self[e4]), /* e41, e42, e43 */self.group2(), /* e23, e31, e12 */float3(0.0), /* e423, e431, e412, e321 */float4(self[e423], self[e431], self[e412], 0.0));
    }
}
impl std::ops::Div<weight> for Origin {
    type Output = Origin;
    fn div(self, _rhs: weight) -> Self::Output {
        self.weight()
    }
}
impl std::ops::DivAssign<weight> for Origin {
    fn div_assign(&mut self, _rhs: weight) {
        *self = self.weight()
    }
}
impl Weight for Origin {
    type Output = Origin;
    fn weight(self) -> Self::Output {
        return self;
    }
}
impl std::ops::Div<weight> for Plane {
    type Output = Plane;
    fn div(self, _rhs: weight) -> Self::Output {
        self.weight()
    }
}
impl std::ops::DivAssign<weight> for Plane {
    fn div_assign(&mut self, _rhs: weight) {
        *self = self.weight()
    }
}
impl Weight for Plane {
    type Output = Plane;
    fn weight(self) -> Self::Output {
        use crate::elements::*;
        return Plane::from_groups(/* e423, e431, e412, e321 */float4(self[e423], self[e431], self[e412], 0.0));
    }
}
impl std::ops::Div<weight> for Point {
    type Output = Origin;
    fn div(self, _rhs: weight) -> Self::Output {
        self.weight()
    }
}
impl Weight for Point {
    type Output = Origin;
    fn weight(self) -> Self::Output {
        use crate::elements::*;
        return Origin::from_groups(/* e4 */self[e4]);
    }
}
