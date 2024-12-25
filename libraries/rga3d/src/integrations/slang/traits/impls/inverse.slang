impl std::ops::Div<inverse> for DualNum {
    type Output = DualNum;
    fn div(self, _rhs: inverse) -> Self::Output {
        self.inverse()
    }
}
impl std::ops::DivAssign<inverse> for DualNum {
    fn div_assign(&mut self, _rhs: inverse) {
        *self = self.inverse()
    }
}
impl Inverse for DualNum {
// Operative Statistics for this implementation:
//          add/sub      mul      div
//   simd2        0        1        0
// no simd        0        2        0
    fn inverse(self) -> Self {
        use crate::elements::*;
        return DualNum::from_groups(/* scalar, e1234 */float2(powi(self[scalar], -2)) * self.group0());
    }
}
impl std::ops::Div<inverse> for Flector {
    type Output = Flector;
    fn div(self, _rhs: inverse) -> Self::Output {
        self.inverse()
    }
}
impl std::ops::DivAssign<inverse> for Flector {
    fn div_assign(&mut self, _rhs: inverse) {
        *self = self.inverse()
    }
}
impl Inverse for Flector {
// Operative Statistics for this implementation:
//           add/sub      mul      div
//      f32        3        0        0
//    simd4        0        3        0
// Totals...
// yes simd        3        3        0
//  no simd        3       12        0
    fn inverse(self) -> Self {
        use crate::elements::*;
let reverse = Flector::from_groups(/* e1, e2, e3, e4 */self.group0(), /* e423, e431, e412, e321 */self.group1() * float4(-1.0));
let other = Scalar::from_groups(/* scalar */powi(self[e1], 2) + powi(self[e2], 2) + powi(self[e3], 2) + powi(self[e321], 2));
        return Flector::from_groups(/* e1, e2, e3, e4 */float4(other[scalar]) * reverse.group0(), /* e423, e431, e412, e321 */float4(other[scalar]) * reverse.group1());
    }
}
impl std::ops::Div<inverse> for Horizon {
    type Output = Horizon;
    fn div(self, _rhs: inverse) -> Self::Output {
        self.inverse()
    }
}
impl std::ops::DivAssign<inverse> for Horizon {
    fn div_assign(&mut self, _rhs: inverse) {
        *self = self.inverse()
    }
}
impl Inverse for Horizon {
// Operative Statistics for this implementation:
//      add/sub      mul      div
// f32        0        1        1
    fn inverse(self) -> Self {
        use crate::elements::*;
        return Horizon::from_groups(/* e321 */1.0/self[e321] * -1.0);
    }
}
impl std::ops::Div<inverse> for Line {
    type Output = Line;
    fn div(self, _rhs: inverse) -> Self::Output {
        self.inverse()
    }
}
impl std::ops::DivAssign<inverse> for Line {
    fn div_assign(&mut self, _rhs: inverse) {
        *self = self.inverse()
    }
}
impl Inverse for Line {
// Operative Statistics for this implementation:
//           add/sub      mul      div
//      f32        2        0        0
//    simd3        0        4        0
// Totals...
// yes simd        2        4        0
//  no simd        2       12        0
    fn inverse(self) -> Self {
        use crate::elements::*;
let reverse = Line::from_groups(/* e41, e42, e43 */self.group0() * float3(-1.0), /* e23, e31, e12 */self.group1() * float3(-1.0));
let other = Scalar::from_groups(/* scalar */powi(self[e23], 2) + powi(self[e31], 2) + powi(self[e12], 2));
        return Line::from_groups(/* e41, e42, e43 */float3(other[scalar]) * reverse.group0(), /* e23, e31, e12 */float3(other[scalar]) * reverse.group1());
    }
}
impl std::ops::Div<inverse> for Motor {
    type Output = Motor;
    fn div(self, _rhs: inverse) -> Self::Output {
        self.inverse()
    }
}
impl std::ops::DivAssign<inverse> for Motor {
    fn div_assign(&mut self, _rhs: inverse) {
        *self = self.inverse()
    }
}
impl Inverse for Motor {
// Operative Statistics for this implementation:
//           add/sub      mul      div
//      f32        3        0        0
//    simd4        0        4        0
// Totals...
// yes simd        3        4        0
//  no simd        3       16        0
    fn inverse(self) -> Self {
        use crate::elements::*;
let reverse = Motor::from_groups(/* e41, e42, e43, e1234 */self.group0() * float4(-1.0, -1.0, -1.0, 1.0), /* e23, e31, e12, scalar */self.group1() * float4(-1.0, -1.0, -1.0, 1.0));
let other = Scalar::from_groups(/* scalar */powi(self[e23], 2) + powi(self[e31], 2) + powi(self[e12], 2) + powi(self[scalar], 2));
        return Motor::from_groups(/* e41, e42, e43, e1234 */float4(other[scalar]) * reverse.group0(), /* e23, e31, e12, scalar */float4(other[scalar]) * reverse.group1());
    }
}
impl std::ops::Div<inverse> for MultiVector {
    type Output = MultiVector;
    fn div(self, _rhs: inverse) -> Self::Output {
        self.inverse()
    }
}
impl std::ops::DivAssign<inverse> for MultiVector {
    fn div_assign(&mut self, _rhs: inverse) {
        *self = self.inverse()
    }
}
impl Inverse for MultiVector {
// Operative Statistics for this implementation:
//           add/sub      mul      div
//      f32        7        0        0
//    simd2        0        1        0
//    simd3        0        4        0
//    simd4        0        3        0
// Totals...
// yes simd        7        8        0
//  no simd        7       26        0
    fn inverse(self) -> Self {
        use crate::elements::*;
let reverse = MultiVector::from_groups(/* scalar, e1234 */self.group0(), /* e1, e2, e3, e4 */self.group1(), /* e41, e42, e43 */self.group2() * float3(-1.0), /* e23, e31, e12 */self.group3() * float3(-1.0), /* e423, e431, e412, e321 */self.group4() * float4(-1.0));
let other = Scalar::from_groups(/* scalar */powi(self[scalar], 2) + powi(self[e1], 2) + powi(self[e2], 2) + powi(self[e3], 2) + powi(self[e23], 2) + powi(self[e31], 2) + powi(self[e12], 2) + powi(self[e321], 2));
        return MultiVector::from_groups(/* scalar, e1234 */float2(other[scalar]) * reverse.group0(), /* e1, e2, e3, e4 */float4(other[scalar]) * reverse.group1(), /* e41, e42, e43 */float3(other[scalar]) * reverse.group2(), /* e23, e31, e12 */float3(other[scalar]) * reverse.group3(), /* e423, e431, e412, e321 */float4(other[scalar]) * reverse.group4());
    }
}
impl std::ops::Div<inverse> for Plane {
    type Output = Plane;
    fn div(self, _rhs: inverse) -> Self::Output {
        self.inverse()
    }
}
impl std::ops::DivAssign<inverse> for Plane {
    fn div_assign(&mut self, _rhs: inverse) {
        *self = self.inverse()
    }
}
impl Inverse for Plane {
// Operative Statistics for this implementation:
//           add/sub      mul      div
//      f32        0        4        0
//    simd4        0        1        0
// Totals...
// yes simd        0        5        0
//  no simd        0        8        0
    fn inverse(self) -> Self {
        use crate::elements::*;
        return Plane::from_groups(/* e423, e431, e412, e321 */float4(powi(self[e321], -2)) * float4(self[e423] * -1.0, self[e431] * -1.0, self[e412] * -1.0, self[e321] * -1.0));
    }
}
impl std::ops::Div<inverse> for Point {
    type Output = Point;
    fn div(self, _rhs: inverse) -> Self::Output {
        self.inverse()
    }
}
impl std::ops::DivAssign<inverse> for Point {
    fn div_assign(&mut self, _rhs: inverse) {
        *self = self.inverse()
    }
}
impl Inverse for Point {
// Operative Statistics for this implementation:
//           add/sub      mul      div
//      f32        2        0        0
//    simd4        0        1        0
// Totals...
// yes simd        2        1        0
//  no simd        2        4        0
    fn inverse(self) -> Self {
        use crate::elements::*;
        return Point::from_groups(/* e1, e2, e3, e4 */float4(powi(self[e1], 2) + powi(self[e2], 2) + powi(self[e3], 2)) * self.group0());
    }
}
impl std::ops::Div<inverse> for Scalar {
    type Output = Scalar;
    fn div(self, _rhs: inverse) -> Self::Output {
        self.inverse()
    }
}
impl std::ops::DivAssign<inverse> for Scalar {
    fn div_assign(&mut self, _rhs: inverse) {
        *self = self.inverse()
    }
}
impl Inverse for Scalar {
// Operative Statistics for this implementation:
//      add/sub      mul      div
// f32        0        0        1
    fn inverse(self) -> Self {
        use crate::elements::*;
        return Scalar::from_groups(/* scalar */1.0/self[scalar]);
    }
}
