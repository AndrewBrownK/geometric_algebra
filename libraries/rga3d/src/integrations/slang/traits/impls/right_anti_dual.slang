impl std::ops::Div<right_anti_dual> for DualNum {
    type Output = AntiScalar;
    fn div(self, _rhs: right_anti_dual) -> Self::Output {
        self.right_anti_dual()
    }
}
impl RightAntiDual for DualNum {
    type Output = AntiScalar;
    fn right_anti_dual(self) -> Self::Output {
        use crate::elements::*;
        return AntiScalar::from_groups(/* e1234 */self[scalar]);
    }
}
impl std::ops::Div<right_anti_dual> for Flector {
    type Output = Flector;
    fn div(self, _rhs: right_anti_dual) -> Self::Output {
        self.right_anti_dual()
    }
}
impl std::ops::DivAssign<right_anti_dual> for Flector {
    fn div_assign(&mut self, _rhs: right_anti_dual) {
        *self = self.right_anti_dual()
    }
}
impl RightAntiDual for Flector {
    type Output = Flector;
// Operative Statistics for this implementation:
//      add/sub      mul      div
// f32        0        1        0
    fn right_anti_dual(self) -> Self::Output {
        use crate::elements::*;
        return Flector::from_groups(/* e1, e2, e3, e4 */float4(float3(0.0), self[e321] * -1.0), /* e423, e431, e412, e321 */float4(self[e1], self[e2], self[e3], 0.0));
    }
}
impl std::ops::Div<right_anti_dual> for Horizon {
    type Output = Origin;
    fn div(self, _rhs: right_anti_dual) -> Self::Output {
        self.right_anti_dual()
    }
}
impl RightAntiDual for Horizon {
    type Output = Origin;
// Operative Statistics for this implementation:
//      add/sub      mul      div
// f32        0        1        0
    fn right_anti_dual(self) -> Self::Output {
        use crate::elements::*;
        return Origin::from_groups(/* e4 */self[e321] * -1.0);
    }
}
impl std::ops::Div<right_anti_dual> for Line {
    type Output = Line;
    fn div(self, _rhs: right_anti_dual) -> Self::Output {
        self.right_anti_dual()
    }
}
impl std::ops::DivAssign<right_anti_dual> for Line {
    fn div_assign(&mut self, _rhs: right_anti_dual) {
        *self = self.right_anti_dual()
    }
}
impl RightAntiDual for Line {
    type Output = Line;
// Operative Statistics for this implementation:
//          add/sub      mul      div
//   simd3        0        1        0
// no simd        0        3        0
    fn right_anti_dual(self) -> Self::Output {
        return Line::from_groups(/* e41, e42, e43 */self.group1() * float3(-1.0), /* e23, e31, e12 */float3(0.0));
    }
}
impl std::ops::Div<right_anti_dual> for Motor {
    type Output = Motor;
    fn div(self, _rhs: right_anti_dual) -> Self::Output {
        self.right_anti_dual()
    }
}
impl std::ops::DivAssign<right_anti_dual> for Motor {
    fn div_assign(&mut self, _rhs: right_anti_dual) {
        *self = self.right_anti_dual()
    }
}
impl RightAntiDual for Motor {
    type Output = Motor;
// Operative Statistics for this implementation:
//          add/sub      mul      div
//   simd4        0        1        0
// no simd        0        4        0
    fn right_anti_dual(self) -> Self::Output {
        return Motor::from_groups(/* e41, e42, e43, e1234 */self.group1() * float4(-1.0, -1.0, -1.0, 1.0), /* e23, e31, e12, scalar */float4(0.0));
    }
}
impl std::ops::Div<right_anti_dual> for MultiVector {
    type Output = MultiVector;
    fn div(self, _rhs: right_anti_dual) -> Self::Output {
        self.right_anti_dual()
    }
}
impl std::ops::DivAssign<right_anti_dual> for MultiVector {
    fn div_assign(&mut self, _rhs: right_anti_dual) {
        *self = self.right_anti_dual()
    }
}
impl RightAntiDual for MultiVector {
    type Output = MultiVector;
// Operative Statistics for this implementation:
//           add/sub      mul      div
//      f32        0        1        0
//    simd3        0        1        0
// Totals...
// yes simd        0        2        0
//  no simd        0        4        0
    fn right_anti_dual(self) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(/* scalar, e1234 */float2(0.0, self[scalar]), /* e1, e2, e3, e4 */float4(float3(0.0), self[e321] * -1.0), /* e41, e42, e43 */self.group3() * float3(-1.0), /* e23, e31, e12 */float3(0.0), /* e423, e431, e412, e321 */float4(self[e1], self[e2], self[e3], 0.0));
    }
}
impl std::ops::Div<right_anti_dual> for Plane {
    type Output = Origin;
    fn div(self, _rhs: right_anti_dual) -> Self::Output {
        self.right_anti_dual()
    }
}
impl RightAntiDual for Plane {
    type Output = Origin;
// Operative Statistics for this implementation:
//      add/sub      mul      div
// f32        0        1        0
    fn right_anti_dual(self) -> Self::Output {
        use crate::elements::*;
        return Origin::from_groups(/* e4 */self[e321] * -1.0);
    }
}
impl std::ops::Div<right_anti_dual> for Point {
    type Output = Plane;
    fn div(self, _rhs: right_anti_dual) -> Self::Output {
        self.right_anti_dual()
    }
}
impl RightAntiDual for Point {
    type Output = Plane;
    fn right_anti_dual(self) -> Self::Output {
        use crate::elements::*;
        return Plane::from_groups(/* e423, e431, e412, e321 */float4(self[e1], self[e2], self[e3], 0.0));
    }
}
impl std::ops::Div<right_anti_dual> for Scalar {
    type Output = AntiScalar;
    fn div(self, _rhs: right_anti_dual) -> Self::Output {
        self.right_anti_dual()
    }
}
impl RightAntiDual for Scalar {
    type Output = AntiScalar;
    fn right_anti_dual(self) -> Self::Output {
        use crate::elements::*;
        return AntiScalar::from_groups(/* e1234 */self[scalar]);
    }
}
