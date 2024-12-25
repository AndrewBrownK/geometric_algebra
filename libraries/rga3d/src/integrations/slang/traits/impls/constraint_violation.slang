impl std::ops::Div<constraint_violation> for DualNum {
    type Output = AntiScalar;
    fn div(self, _rhs: constraint_violation) -> Self::Output {
        self.constraint_violation()
    }
}
impl ConstraintViolation for DualNum {
    type Output = AntiScalar;
// Operative Statistics for this implementation:
//      add/sub      mul      div
// f32        0        2        0
    fn constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        return AntiScalar::from_groups(/* e1234 */self[scalar] * self[e1234] * 2.0);
    }
}
impl std::ops::Div<constraint_violation> for Flector {
    type Output = AntiScalar;
    fn div(self, _rhs: constraint_violation) -> Self::Output {
        self.constraint_violation()
    }
}
impl ConstraintViolation for Flector {
    type Output = AntiScalar;
// Operative Statistics for this implementation:
//           add/sub      mul      div
//      f32        7        8        0
//    simd4        0        1        0
// Totals...
// yes simd        7        9        0
//  no simd        7       12        0
    fn constraint_violation(self) -> Self::Output {
        use crate::elements::*;
let reverse = Flector::from_groups(/* e1, e2, e3, e4 */self.group0(), /* e423, e431, e412, e321 */self.group1() * float4(-1.0));
        return AntiScalar::from_groups(/* e1234 */(reverse[e423] * self[e1]) + (reverse[e431] * self[e2]) + (reverse[e412] * self[e3]) + (reverse[e321] * self[e4]) - (reverse[e1] * self[e423]) - (reverse[e2] * self[e431]) - (reverse[e3] * self[e412]) - (reverse[e4] * self[e321]));
    }
}
impl std::ops::Div<constraint_violation> for Line {
    type Output = AntiScalar;
    fn div(self, _rhs: constraint_violation) -> Self::Output {
        self.constraint_violation()
    }
}
impl ConstraintViolation for Line {
    type Output = AntiScalar;
// Operative Statistics for this implementation:
//           add/sub      mul      div
//      f32        5        6        0
//    simd3        0        2        0
// Totals...
// yes simd        5        8        0
//  no simd        5       12        0
    fn constraint_violation(self) -> Self::Output {
        use crate::elements::*;
let reverse = Line::from_groups(/* e41, e42, e43 */self.group0() * float3(-1.0), /* e23, e31, e12 */self.group1() * float3(-1.0));
        return AntiScalar::from_groups(/* e1234 */-(reverse[e41] * self[e23]) - (reverse[e42] * self[e31]) - (reverse[e43] * self[e12]) - (reverse[e23] * self[e41]) - (reverse[e31] * self[e42]) - (reverse[e12] * self[e43]));
    }
}
impl std::ops::Div<constraint_violation> for Motor {
    type Output = AntiScalar;
    fn div(self, _rhs: constraint_violation) -> Self::Output {
        self.constraint_violation()
    }
}
impl ConstraintViolation for Motor {
    type Output = AntiScalar;
// Operative Statistics for this implementation:
//           add/sub      mul      div
//      f32        7        8        0
//    simd4        0        2        0
// Totals...
// yes simd        7       10        0
//  no simd        7       16        0
    fn constraint_violation(self) -> Self::Output {
        use crate::elements::*;
let reverse = Motor::from_groups(/* e41, e42, e43, e1234 */self.group0() * float4(-1.0, -1.0, -1.0, 1.0), /* e23, e31, e12, scalar */self.group1() * float4(-1.0, -1.0, -1.0, 1.0));
        return AntiScalar::from_groups(/* e1234 */(reverse[e1234] * self[scalar]) + (reverse[scalar] * self[e1234]) - (reverse[e41] * self[e23]) - (reverse[e42] * self[e31]) - (reverse[e43] * self[e12]) - (reverse[e23] * self[e41]) - (reverse[e31] * self[e42]) - (reverse[e12] * self[e43]));
    }
}
impl std::ops::Div<constraint_violation> for MultiVector {
    type Output = MultiVector;
    fn div(self, _rhs: constraint_violation) -> Self::Output {
        self.constraint_violation()
    }
}
impl std::ops::DivAssign<constraint_violation> for MultiVector {
    fn div_assign(&mut self, _rhs: constraint_violation) {
        *self = self.constraint_violation()
    }
}
impl ConstraintViolation for MultiVector {
    type Output = MultiVector;
// Operative Statistics for this implementation:
//           add/sub      mul      div
//      f32       14       18        0
//    simd2        8        8        0
//    simd3        0        4        0
//    simd4        8        7        0
// Totals...
// yes simd       30       37        0
//  no simd       62       74        0
    fn constraint_violation(self) -> Self::Output {
        use crate::elements::*;
let reverse = MultiVector::from_groups(/* scalar, e1234 */self.group0(), /* e1, e2, e3, e4 */self.group1(), /* e41, e42, e43 */self.group2() * float3(-1.0), /* e23, e31, e12 */self.group3() * float3(-1.0), /* e423, e431, e412, e321 */self.group4() * float4(-1.0));
let geometric_product = MultiVector::from_groups(/* scalar, e1234 */float2(0.0, (reverse[e1234] * self[scalar]) + (reverse[e321] * self[e4]) - (reverse[e2] * self[e431]) - (reverse[e3] * self[e412]) - (reverse[e4] * self[e321]) - (reverse[e23] * self[e41]) - (reverse[e31] * self[e42]) - (reverse[e12] * self[e43])) + (float2(reverse[scalar]) * self.group0()) + (float2(self[e1]) * float2(reverse[e1], reverse[e423])) + (float2(self[e2]) * float2(reverse[e2], reverse[e431])) + (float2(self[e3]) * float2(reverse[e3], reverse[e412])) - (float2(self[e23]) * float2(reverse[e23], reverse[e41])) - (float2(self[e31]) * float2(reverse[e31], reverse[e42])) - (float2(self[e12]) * float2(reverse[e12], reverse[e43])) - (float2(reverse[e321], reverse[e1]) * self.group4().wx), /* e1, e2, e3, e4 */(float4(reverse[scalar]) * self.group1()) + (float4(reverse[e2], reverse[e321], reverse[e321], reverse[e3]) * float4(self.group3().zyz, self[e43])) + (float4(reverse[e321], reverse[e3], reverse[e1], reverse[e2]) * float4(self.group3().xxy, self[e42])) + (float4(self.group0().xx, self[scalar], reverse[e1234]) * float4(reverse.group1().xyz, self[e321])) + (float4(self.group1().zx, self[e321], reverse[e1]) * float4(reverse.group3().yzz, self[e41])) + (float4(self.group4().ww, self[e2], reverse[e4]) * float4(reverse.group3().xyx, self[scalar])) + float4(float3(0.0), -(reverse[e42] * self[e2]) - (reverse[e43] * self[e3]) - (reverse[e23] * self[e423]) - (reverse[e31] * self[e431]) - (reverse[e12] * self[e412]) - (reverse[e423] * self[e23]) - (reverse[e431] * self[e31]) - (reverse[e412] * self[e12])) - float4((reverse.group3().zxy * self.group1().yzx), reverse[e321] * self[e1234]) - float4((self.group3().yzx * reverse.group1().zxy), reverse[e41] * self[e1]), /* e41, e42, e43 */float3(0.0), /* e23, e31, e12 */float3(0.0), /* e423, e431, e412, e321 */float4(0.0));
        return MultiVector::from_groups(/* scalar, e1234 */float2(0.0, geometric_product[e1234]), /* e1, e2, e3, e4 */geometric_product.group1(), /* e41, e42, e43 */float3(0.0), /* e23, e31, e12 */float3(0.0), /* e423, e431, e412, e321 */float4(0.0));
    }
}
