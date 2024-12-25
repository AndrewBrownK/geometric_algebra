impl std::ops::Div<flat_bulk> for DualNum {
    type Output = Scalar;
    fn div(self, _rhs: flat_bulk) -> Self::Output {
        self.flat_bulk()
    }
}
impl FlatBulk for DualNum {
    type Output = Scalar;
    fn flat_bulk(self) -> Self::Output {
        use crate::elements::*;
        return Scalar::from_groups(/* scalar */self[scalar]);
    }
}
impl std::ops::Div<flat_bulk> for Flector {
    type Output = Flector;
    fn div(self, _rhs: flat_bulk) -> Self::Output {
        self.flat_bulk()
    }
}
impl std::ops::DivAssign<flat_bulk> for Flector {
    fn div_assign(&mut self, _rhs: flat_bulk) {
        *self = self.flat_bulk()
    }
}
impl FlatBulk for Flector {
    type Output = Flector;
    fn flat_bulk(self) -> Self::Output {
        use crate::elements::*;
        return Flector::from_groups(/* e1, e2, e3, e4 */float4(self[e1], self[e2], self[e3], 0.0), /* e423, e431, e412, e321 */float4(float3(0.0), self[e321]));
    }
}
impl std::ops::Div<flat_bulk> for Horizon {
    type Output = Horizon;
    fn div(self, _rhs: flat_bulk) -> Self::Output {
        self.flat_bulk()
    }
}
impl std::ops::DivAssign<flat_bulk> for Horizon {
    fn div_assign(&mut self, _rhs: flat_bulk) {
        *self = self.flat_bulk()
    }
}
impl FlatBulk for Horizon {
    type Output = Horizon;
    fn flat_bulk(self) -> Self::Output {
        return self;
    }
}
impl std::ops::Div<flat_bulk> for Line {
    type Output = Line;
    fn div(self, _rhs: flat_bulk) -> Self::Output {
        self.flat_bulk()
    }
}
impl std::ops::DivAssign<flat_bulk> for Line {
    fn div_assign(&mut self, _rhs: flat_bulk) {
        *self = self.flat_bulk()
    }
}
impl FlatBulk for Line {
    type Output = Line;
    fn flat_bulk(self) -> Self::Output {
        return Line::from_groups(/* e41, e42, e43 */float3(0.0), /* e23, e31, e12 */self.group1());
    }
}
impl std::ops::Div<flat_bulk> for Motor {
    type Output = Motor;
    fn div(self, _rhs: flat_bulk) -> Self::Output {
        self.flat_bulk()
    }
}
impl std::ops::DivAssign<flat_bulk> for Motor {
    fn div_assign(&mut self, _rhs: flat_bulk) {
        *self = self.flat_bulk()
    }
}
impl FlatBulk for Motor {
    type Output = Motor;
    fn flat_bulk(self) -> Self::Output {
        return Motor::from_groups(/* e41, e42, e43, e1234 */float4(0.0), /* e23, e31, e12, scalar */self.group1());
    }
}
impl std::ops::Div<flat_bulk> for MultiVector {
    type Output = MultiVector;
    fn div(self, _rhs: flat_bulk) -> Self::Output {
        self.flat_bulk()
    }
}
impl std::ops::DivAssign<flat_bulk> for MultiVector {
    fn div_assign(&mut self, _rhs: flat_bulk) {
        *self = self.flat_bulk()
    }
}
impl FlatBulk for MultiVector {
    type Output = MultiVector;
    fn flat_bulk(self) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(/* scalar, e1234 */float2(self[scalar], 0.0), /* e1, e2, e3, e4 */float4(self[e1], self[e2], self[e3], 0.0), /* e41, e42, e43 */float3(0.0), /* e23, e31, e12 */self.group3(), /* e423, e431, e412, e321 */float4(float3(0.0), self[e321]));
    }
}
impl std::ops::Div<flat_bulk> for Plane {
    type Output = Horizon;
    fn div(self, _rhs: flat_bulk) -> Self::Output {
        self.flat_bulk()
    }
}
impl FlatBulk for Plane {
    type Output = Horizon;
    fn flat_bulk(self) -> Self::Output {
        use crate::elements::*;
        return Horizon::from_groups(/* e321 */self[e321]);
    }
}
impl std::ops::Div<flat_bulk> for Point {
    type Output = Point;
    fn div(self, _rhs: flat_bulk) -> Self::Output {
        self.flat_bulk()
    }
}
impl std::ops::DivAssign<flat_bulk> for Point {
    fn div_assign(&mut self, _rhs: flat_bulk) {
        *self = self.flat_bulk()
    }
}
impl FlatBulk for Point {
    type Output = Point;
    fn flat_bulk(self) -> Self::Output {
        use crate::elements::*;
        return Point::from_groups(/* e1, e2, e3, e4 */float4(self[e1], self[e2], self[e3], 0.0));
    }
}
impl std::ops::Div<flat_bulk> for Scalar {
    type Output = Scalar;
    fn div(self, _rhs: flat_bulk) -> Self::Output {
        self.flat_bulk()
    }
}
impl std::ops::DivAssign<flat_bulk> for Scalar {
    fn div_assign(&mut self, _rhs: flat_bulk) {
        *self = self.flat_bulk()
    }
}
impl FlatBulk for Scalar {
    type Output = Scalar;
    fn flat_bulk(self) -> Self::Output {
        return self;
    }
}
