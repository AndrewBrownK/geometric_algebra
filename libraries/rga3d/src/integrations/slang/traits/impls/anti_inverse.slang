impl std::ops::Div<anti_inverse> for AntiScalar {
    type Output = AntiScalar;
    fn div(self, _rhs: anti_inverse) -> Self::Output {
        self.anti_inverse()
    }
}
impl std::ops::DivAssign<anti_inverse> for AntiScalar {
    fn div_assign(&mut self, _rhs: anti_inverse) {
        *self = self.anti_inverse()
    }
}
impl AntiInverse for AntiScalar {
// Operative Statistics for this implementation:
//      add/sub      mul      div
// f32        0        0        1
    fn anti_inverse(self) -> Self {
        use crate::elements::*;
        return AntiScalar::from_groups(/* e1234 */1.0/self[e1234]);
    }
}
impl std::ops::Div<anti_inverse> for DualNum {
    type Output = DualNum;
    fn div(self, _rhs: anti_inverse) -> Self::Output {
        self.anti_inverse()
    }
}
impl std::ops::DivAssign<anti_inverse> for DualNum {
    fn div_assign(&mut self, _rhs: anti_inverse) {
        *self = self.anti_inverse()
    }
}
impl AntiInverse for DualNum {
// Operative Statistics for this implementation:
//          add/sub      mul      div
//   simd2        0        1        0
// no simd        0        2        0
    fn anti_inverse(self) -> Self {
        use crate::elements::*;
        return DualNum::from_groups(/* scalar, e1234 */float2(powi(self[e1234], -2)) * self.group0());
    }
}
impl std::ops::Div<anti_inverse> for Flector {
    type Output = Flector;
    fn div(self, _rhs: anti_inverse) -> Self::Output {
        self.anti_inverse()
    }
}
impl std::ops::DivAssign<anti_inverse> for Flector {
    fn div_assign(&mut self, _rhs: anti_inverse) {
        *self = self.anti_inverse()
    }
}
impl AntiInverse for Flector {
// Operative Statistics for this implementation:
//           add/sub      mul      div
//      f32        3        0        0
//    simd4        0        3        0
// Totals...
// yes simd        3        3        0
//  no simd        3       12        0
    fn anti_inverse(self) -> Self {
        use crate::elements::*;
let anti_reverse = Flector::from_groups(/* e1, e2, e3, e4 */self.group0() * float4(-1.0), /* e423, e431, e412, e321 */self.group1());
let other = AntiScalar::from_groups(/* e1234 */powi(self[e4], 2) + powi(self[e423], 2) + powi(self[e431], 2) + powi(self[e412], 2));
        return Flector::from_groups(/* e1, e2, e3, e4 */float4(other[e1234]) * anti_reverse.group0(), /* e423, e431, e412, e321 */float4(other[e1234]) * anti_reverse.group1());
    }
}
impl std::ops::Div<anti_inverse> for Line {
    type Output = Line;
    fn div(self, _rhs: anti_inverse) -> Self::Output {
        self.anti_inverse()
    }
}
impl std::ops::DivAssign<anti_inverse> for Line {
    fn div_assign(&mut self, _rhs: anti_inverse) {
        *self = self.anti_inverse()
    }
}
impl AntiInverse for Line {
// Operative Statistics for this implementation:
//           add/sub      mul      div
//      f32        2        0        0
//    simd3        0        4        0
// Totals...
// yes simd        2        4        0
//  no simd        2       12        0
    fn anti_inverse(self) -> Self {
        use crate::elements::*;
let anti_reverse = Line::from_groups(/* e41, e42, e43 */self.group0() * float3(-1.0), /* e23, e31, e12 */self.group1() * float3(-1.0));
let other = AntiScalar::from_groups(/* e1234 */powi(self[e41], 2) + powi(self[e42], 2) + powi(self[e43], 2));
        return Line::from_groups(/* e41, e42, e43 */float3(other[e1234]) * anti_reverse.group0(), /* e23, e31, e12 */float3(other[e1234]) * anti_reverse.group1());
    }
}
impl std::ops::Div<anti_inverse> for Motor {
    type Output = Motor;
    fn div(self, _rhs: anti_inverse) -> Self::Output {
        self.anti_inverse()
    }
}
impl std::ops::DivAssign<anti_inverse> for Motor {
    fn div_assign(&mut self, _rhs: anti_inverse) {
        *self = self.anti_inverse()
    }
}
impl AntiInverse for Motor {
// Operative Statistics for this implementation:
//           add/sub      mul      div
//      f32        3        0        0
//    simd4        0        4        0
// Totals...
// yes simd        3        4        0
//  no simd        3       16        0
    fn anti_inverse(self) -> Self {
        use crate::elements::*;
let anti_reverse = Motor::from_groups(/* e41, e42, e43, e1234 */self.group0() * float4(-1.0, -1.0, -1.0, 1.0), /* e23, e31, e12, scalar */self.group1() * float4(-1.0, -1.0, -1.0, 1.0));
let other = AntiScalar::from_groups(/* e1234 */powi(self[e41], 2) + powi(self[e42], 2) + powi(self[e43], 2) + powi(self[e1234], 2));
        return Motor::from_groups(/* e41, e42, e43, e1234 */float4(other[e1234]) * anti_reverse.group0(), /* e23, e31, e12, scalar */float4(other[e1234]) * anti_reverse.group1());
    }
}
impl std::ops::Div<anti_inverse> for MultiVector {
    type Output = MultiVector;
    fn div(self, _rhs: anti_inverse) -> Self::Output {
        self.anti_inverse()
    }
}
impl std::ops::DivAssign<anti_inverse> for MultiVector {
    fn div_assign(&mut self, _rhs: anti_inverse) {
        *self = self.anti_inverse()
    }
}
impl AntiInverse for MultiVector {
// Operative Statistics for this implementation:
//           add/sub      mul      div
//      f32        7        0        0
//    simd2        0        1        0
//    simd3        0        4        0
//    simd4        0        3        0
// Totals...
// yes simd        7        8        0
//  no simd        7       26        0
    fn anti_inverse(self) -> Self {
        use crate::elements::*;
let anti_reverse = MultiVector::from_groups(/* scalar, e1234 */self.group0(), /* e1, e2, e3, e4 */self.group1() * float4(-1.0), /* e41, e42, e43 */self.group2() * float3(-1.0), /* e23, e31, e12 */self.group3() * float3(-1.0), /* e423, e431, e412, e321 */self.group4());
let other = AntiScalar::from_groups(/* e1234 */powi(self[e1234], 2) + powi(self[e4], 2) + powi(self[e41], 2) + powi(self[e42], 2) + powi(self[e43], 2) + powi(self[e423], 2) + powi(self[e431], 2) + powi(self[e412], 2));
        return MultiVector::from_groups(/* scalar, e1234 */float2(other[e1234]) * anti_reverse.group0(), /* e1, e2, e3, e4 */float4(other[e1234]) * anti_reverse.group1(), /* e41, e42, e43 */float3(other[e1234]) * anti_reverse.group2(), /* e23, e31, e12 */float3(other[e1234]) * anti_reverse.group3(), /* e423, e431, e412, e321 */float4(other[e1234]) * anti_reverse.group4());
    }
}
impl std::ops::Div<anti_inverse> for Origin {
    type Output = Origin;
    fn div(self, _rhs: anti_inverse) -> Self::Output {
        self.anti_inverse()
    }
}
impl std::ops::DivAssign<anti_inverse> for Origin {
    fn div_assign(&mut self, _rhs: anti_inverse) {
        *self = self.anti_inverse()
    }
}
impl AntiInverse for Origin {
// Operative Statistics for this implementation:
//      add/sub      mul      div
// f32        0        1        1
    fn anti_inverse(self) -> Self {
        use crate::elements::*;
        return Origin::from_groups(/* e4 */1.0/self[e4] * -1.0);
    }
}
impl std::ops::Div<anti_inverse> for Plane {
    type Output = Plane;
    fn div(self, _rhs: anti_inverse) -> Self::Output {
        self.anti_inverse()
    }
}
impl std::ops::DivAssign<anti_inverse> for Plane {
    fn div_assign(&mut self, _rhs: anti_inverse) {
        *self = self.anti_inverse()
    }
}
impl AntiInverse for Plane {
// Operative Statistics for this implementation:
//           add/sub      mul      div
//      f32        2        0        0
//    simd4        0        1        0
// Totals...
// yes simd        2        1        0
//  no simd        2        4        0
    fn anti_inverse(self) -> Self {
        use crate::elements::*;
        return Plane::from_groups(/* e423, e431, e412, e321 */float4(powi(self[e423], 2) + powi(self[e431], 2) + powi(self[e412], 2)) * self.group0());
    }
}
impl std::ops::Div<anti_inverse> for Point {
    type Output = Point;
    fn div(self, _rhs: anti_inverse) -> Self::Output {
        self.anti_inverse()
    }
}
impl std::ops::DivAssign<anti_inverse> for Point {
    fn div_assign(&mut self, _rhs: anti_inverse) {
        *self = self.anti_inverse()
    }
}
impl AntiInverse for Point {
// Operative Statistics for this implementation:
//           add/sub      mul      div
//      f32        0        4        0
//    simd4        0        1        0
// Totals...
// yes simd        0        5        0
//  no simd        0        8        0
    fn anti_inverse(self) -> Self {
        use crate::elements::*;
        return Point::from_groups(/* e1, e2, e3, e4 */float4(powi(self[e4], -2)) * float4(self[e1] * -1.0, self[e2] * -1.0, self[e3] * -1.0, self[e4] * -1.0));
    }
}
