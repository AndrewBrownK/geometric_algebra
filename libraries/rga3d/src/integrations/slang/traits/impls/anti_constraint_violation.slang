impl std::ops::Div<anti_constraint_violation> for DualNum {
    type Output = Scalar;
    fn div(self, _rhs: anti_constraint_violation) -> Self::Output {
        self.anti_constraint_violation()
    }
}
impl AntiConstraintViolation for DualNum {
    type Output = Scalar;
// Operative Statistics for this implementation:
//      add/sub      mul      div
// f32        0        2        0
    fn anti_constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        return Scalar::from_groups(/* scalar */self[scalar] * self[e1234] * 2.0);
    }
}
impl std::ops::Div<anti_constraint_violation> for Flector {
    type Output = Scalar;
    fn div(self, _rhs: anti_constraint_violation) -> Self::Output {
        self.anti_constraint_violation()
    }
}
impl AntiConstraintViolation for Flector {
    type Output = Scalar;
// Operative Statistics for this implementation:
//           add/sub      mul      div
//      f32        7        8        0
//    simd4        0        1        0
// Totals...
// yes simd        7        9        0
//  no simd        7       12        0
    fn anti_constraint_violation(self) -> Self::Output {
        use crate::elements::*;
let anti_reverse = Flector::from_groups(/* e1, e2, e3, e4 */self.group0() * float4(-1.0), /* e423, e431, e412, e321 */self.group1());
        return Scalar::from_groups(/* scalar */(anti_reverse[e423] * self[e1]) + (anti_reverse[e431] * self[e2]) + (anti_reverse[e412] * self[e3]) + (anti_reverse[e321] * self[e4]) - (anti_reverse[e1] * self[e423]) - (anti_reverse[e2] * self[e431]) - (anti_reverse[e3] * self[e412]) - (anti_reverse[e4] * self[e321]));
    }
}
impl std::ops::Div<anti_constraint_violation> for Line {
    type Output = Scalar;
    fn div(self, _rhs: anti_constraint_violation) -> Self::Output {
        self.anti_constraint_violation()
    }
}
impl AntiConstraintViolation for Line {
    type Output = Scalar;
// Operative Statistics for this implementation:
//           add/sub      mul      div
//      f32        5        6        0
//    simd3        0        2        0
// Totals...
// yes simd        5        8        0
//  no simd        5       12        0
    fn anti_constraint_violation(self) -> Self::Output {
        use crate::elements::*;
let anti_reverse = Line::from_groups(/* e41, e42, e43 */self.group0() * float3(-1.0), /* e23, e31, e12 */self.group1() * float3(-1.0));
        return Scalar::from_groups(/* scalar */-(anti_reverse[e41] * self[e23]) - (anti_reverse[e42] * self[e31]) - (anti_reverse[e43] * self[e12]) - (anti_reverse[e23] * self[e41]) - (anti_reverse[e31] * self[e42]) - (anti_reverse[e12] * self[e43]));
    }
}
impl std::ops::Div<anti_constraint_violation> for Motor {
    type Output = Scalar;
    fn div(self, _rhs: anti_constraint_violation) -> Self::Output {
        self.anti_constraint_violation()
    }
}
impl AntiConstraintViolation for Motor {
    type Output = Scalar;
// Operative Statistics for this implementation:
//           add/sub      mul      div
//      f32        7        8        0
//    simd4        0        2        0
// Totals...
// yes simd        7       10        0
//  no simd        7       16        0
    fn anti_constraint_violation(self) -> Self::Output {
        use crate::elements::*;
let anti_reverse = Motor::from_groups(/* e41, e42, e43, e1234 */self.group0() * float4(-1.0, -1.0, -1.0, 1.0), /* e23, e31, e12, scalar */self.group1() * float4(-1.0, -1.0, -1.0, 1.0));
        return Scalar::from_groups(/* scalar */(anti_reverse[e1234] * self[scalar]) + (anti_reverse[scalar] * self[e1234]) - (anti_reverse[e41] * self[e23]) - (anti_reverse[e42] * self[e31]) - (anti_reverse[e43] * self[e12]) - (anti_reverse[e23] * self[e41]) - (anti_reverse[e31] * self[e42]) - (anti_reverse[e12] * self[e43]));
    }
}
impl std::ops::Div<anti_constraint_violation> for MultiVector {
    type Output = MultiVector;
    fn div(self, _rhs: anti_constraint_violation) -> Self::Output {
        self.anti_constraint_violation()
    }
}
impl std::ops::DivAssign<anti_constraint_violation> for MultiVector {
    fn div_assign(&mut self, _rhs: anti_constraint_violation) {
        *self = self.anti_constraint_violation()
    }
}
impl AntiConstraintViolation for MultiVector {
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
    fn anti_constraint_violation(self) -> Self::Output {
        use crate::elements::*;
let anti_reverse = MultiVector::from_groups(/* scalar, e1234 */self.group0(), /* e1, e2, e3, e4 */self.group1() * float4(-1.0), /* e41, e42, e43 */self.group2() * float3(-1.0), /* e23, e31, e12 */self.group3() * float3(-1.0), /* e423, e431, e412, e321 */self.group4());
let geometric_anti_product = MultiVector::from_groups(/* scalar, e1234 */float2((anti_reverse[e1234] * self[scalar]) + (anti_reverse[e321] * self[e4]) - (anti_reverse[e2] * self[e431]) - (anti_reverse[e3] * self[e412]) - (anti_reverse[e4] * self[e321]) - (anti_reverse[e23] * self[e41]) - (anti_reverse[e31] * self[e42]) - (anti_reverse[e12] * self[e43]), 0.0) + (float2(anti_reverse[e423]) * float2(self[e1], self[e423])) + (float2(anti_reverse[e431]) * float2(self[e2], self[e431])) + (float2(anti_reverse[e412]) * float2(self[e3], self[e412])) + (float2(self[e1234]) * anti_reverse.group0()) - (float2(anti_reverse[e41]) * float2(self[e23], self[e41])) - (float2(anti_reverse[e42]) * float2(self[e31], self[e42])) - (float2(anti_reverse[e43]) * float2(self[e12], self[e43])) - (float2(self[e423], self[e4]) * anti_reverse.group1().xw), /* e1, e2, e3, e4 */float4(0.0), /* e41, e42, e43 */float3(0.0), /* e23, e31, e12 */float3(0.0), /* e423, e431, e412, e321 */(float4(anti_reverse[e1234]) * self.group4()) + (float4(self[e4], self[e412], self[e423], anti_reverse[e321]) * float4(anti_reverse.group2().xxy, self[e1234])) + (float4(self[e431], self[e4], self[e4], self[e423]) * float4(anti_reverse.group2().zyz, anti_reverse[e23])) + (float4(self.group0().yy, self[e1234], self[scalar]) * float4(anti_reverse.group4().xyz, anti_reverse[e4])) + (float4(anti_reverse.group1().ww, anti_reverse[e431], self[e431]) * float4(self.group2().xyx, anti_reverse[e31])) + (float4(anti_reverse.group4().zx, anti_reverse[e4], self[e412]) * float4(self.group2().yzz, anti_reverse[e12])) + float4(float3(0.0), -(anti_reverse[e1] * self[e41]) - (anti_reverse[e2] * self[e42]) - (anti_reverse[e3] * self[e43]) - (anti_reverse[e42] * self[e2]) - (anti_reverse[e43] * self[e3]) - (anti_reverse[e423] * self[e23]) - (anti_reverse[e431] * self[e31]) - (anti_reverse[e412] * self[e12])) - float4((anti_reverse.group2().yzx * self.group4().zxy), anti_reverse[scalar] * self[e4]) - float4((self.group2().zxy * anti_reverse.group4().yzx), anti_reverse[e41] * self[e1]));
        return MultiVector::from_groups(/* scalar, e1234 */float2(geometric_anti_product[scalar], 0.0), /* e1, e2, e3, e4 */float4(0.0), /* e41, e42, e43 */float3(0.0), /* e23, e31, e12 */float3(0.0), /* e423, e431, e412, e321 */geometric_anti_product.group4());
    }
}
