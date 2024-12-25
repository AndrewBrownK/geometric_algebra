impl std::ops::Div<anti_support> for DualNum {
    type Output = Horizon;
    fn div(self, _rhs: anti_support) -> Self::Output {
        self.anti_support()
    }
}
impl AntiSupport for DualNum {
    type Output = Horizon;
    fn anti_support(self) -> Self::Output {
        use crate::elements::*;
        return Horizon::from_groups(/* e321 */self[scalar]);
    }
}
impl std::ops::Div<anti_support> for Flector {
    type Output = Motor;
    fn div(self, _rhs: anti_support) -> Self::Output {
        self.anti_support()
    }
}
impl AntiSupport for Flector {
    type Output = Motor;
// Operative Statistics for this implementation:
//           add/sub      mul      div
//      f32        0        1        0
//    simd4        0        1        0
// Totals...
// yes simd        0        2        0
//  no simd        0        5        0
    fn anti_support(self) -> Self::Output {
        use crate::elements::*;
let right_dual = Flector::from_groups(/* e1, e2, e3, e4 */float4(float3(0.0), self[e321] * -1.0), /* e423, e431, e412, e321 */float4(self[e1], self[e2], self[e3], 0.0));
        return Motor::from_groups(/* e41, e42, e43, e1234 */float4(0.0), /* e23, e31, e12, scalar */float4(right_dual.group1().xyz, right_dual[e4]) * float4(-1.0));
    }
}
impl std::ops::Div<anti_support> for Horizon {
    type Output = Scalar;
    fn div(self, _rhs: anti_support) -> Self::Output {
        self.anti_support()
    }
}
impl AntiSupport for Horizon {
    type Output = Scalar;
    fn anti_support(self) -> Self::Output {
        use crate::elements::*;
        return Scalar::from_groups(/* scalar */self[e321]);
    }
}
impl std::ops::Div<anti_support> for Line {
    type Output = Point;
    fn div(self, _rhs: anti_support) -> Self::Output {
        self.anti_support()
    }
}
impl AntiSupport for Line {
    type Output = Point;
// Operative Statistics for this implementation:
//           add/sub      mul      div
//      f32        0        3        0
//    simd4        0        2        0
// Totals...
// yes simd        0        5        0
//  no simd        0       11        0
    fn anti_support(self) -> Self::Output {
        use crate::elements::*;
let right_complement = Horizon::from_groups(/* e321 */1.0);
        return Point::from_groups(/* e1, e2, e3, e4 */float4(right_complement[e321], right_complement[e321], right_complement[e321], 0.0) * float4(float3(self[e23] * -1.0, self[e31] * -1.0, self[e12] * -1.0), 0.0) * float4(1.0, 1.0, 1.0, 0.0));
    }
}
impl std::ops::Div<anti_support> for Motor {
    type Output = Flector;
    fn div(self, _rhs: anti_support) -> Self::Output {
        self.anti_support()
    }
}
impl AntiSupport for Motor {
    type Output = Flector;
// Operative Statistics for this implementation:
//           add/sub      mul      div
//      f32        0        1        0
//    simd4        0        3        0
// Totals...
// yes simd        0        4        0
//  no simd        0       13        0
    fn anti_support(self) -> Self::Output {
        use crate::elements::*;
let right_complement = Horizon::from_groups(/* e321 */1.0);
let right_dual = Motor::from_groups(/* e41, e42, e43, e1234 */self.group1() * float4(-1.0, -1.0, -1.0, 1.0), /* e23, e31, e12, scalar */float4(0.0));
        return Flector::from_groups(/* e1, e2, e3, e4 */float4(right_complement[e321], right_complement[e321], right_complement[e321], 0.0) * float4(right_dual.group0().xyz, 0.0) * float4(1.0, 1.0, 1.0, 0.0), /* e423, e431, e412, e321 */float4(float3(0.0), right_complement[e321] * right_dual[e1234]));
    }
}
impl std::ops::Div<anti_support> for MultiVector {
    type Output = MultiVector;
    fn div(self, _rhs: anti_support) -> Self::Output {
        self.anti_support()
    }
}
impl std::ops::DivAssign<anti_support> for MultiVector {
    fn div_assign(&mut self, _rhs: anti_support) {
        *self = self.anti_support()
    }
}
impl AntiSupport for MultiVector {
    type Output = MultiVector;
// Operative Statistics for this implementation:
//           add/sub      mul      div
//      f32        0        3        0
//    simd2        0        1        0
//    simd3        0        3        0
//    simd4        0        2        0
// Totals...
// yes simd        0        9        0
//  no simd        0       22        0
    fn anti_support(self) -> Self::Output {
        use crate::elements::*;
let right_complement = Horizon::from_groups(/* e321 */1.0);
let right_dual = MultiVector::from_groups(/* scalar, e1234 */float2(0.0, self[scalar]), /* e1, e2, e3, e4 */float4(float3(0.0), self[e321] * -1.0), /* e41, e42, e43 */self.group3() * float3(-1.0), /* e23, e31, e12 */float3(0.0), /* e423, e431, e412, e321 */float4(self[e1], self[e2], self[e3], 0.0));
        return MultiVector::from_groups(/* scalar, e1234 */float2(right_complement[e321] * right_dual[e4], 1.0) * float2(-1.0, 0.0), /* e1, e2, e3, e4 */float4(right_complement[e321], right_complement[e321], right_complement[e321], 0.0) * float4(right_dual.group2(), 0.0) * float4(1.0, 1.0, 1.0, 0.0), /* e41, e42, e43 */float3(0.0), /* e23, e31, e12 */float3(right_complement[e321]) * right_dual.group4().xyz * float3(-1.0), /* e423, e431, e412, e321 */float4(float3(0.0), right_complement[e321] * right_dual[e1234]));
    }
}
impl std::ops::Div<anti_support> for Plane {
    type Output = Scalar;
    fn div(self, _rhs: anti_support) -> Self::Output {
        self.anti_support()
    }
}
impl AntiSupport for Plane {
    type Output = Scalar;
    fn anti_support(self) -> Self::Output {
        use crate::elements::*;
        return Scalar::from_groups(/* scalar */self[e321]);
    }
}
impl std::ops::Div<anti_support> for Point {
    type Output = Line;
    fn div(self, _rhs: anti_support) -> Self::Output {
        self.anti_support()
    }
}
impl AntiSupport for Point {
    type Output = Line;
// Operative Statistics for this implementation:
//          add/sub      mul      div
//   simd3        0        1        0
// no simd        0        3        0
    fn anti_support(self) -> Self::Output {
        use crate::elements::*;
        return Line::from_groups(/* e41, e42, e43 */float3(0.0), /* e23, e31, e12 */float4(self[e1], self[e2], self[e3], 0.0).xyz * float3(-1.0));
    }
}
impl std::ops::Div<anti_support> for Scalar {
    type Output = Horizon;
    fn div(self, _rhs: anti_support) -> Self::Output {
        self.anti_support()
    }
}
impl AntiSupport for Scalar {
    type Output = Horizon;
    fn anti_support(self) -> Self::Output {
        use crate::elements::*;
        return Horizon::from_groups(/* e321 */self[scalar]);
    }
}
