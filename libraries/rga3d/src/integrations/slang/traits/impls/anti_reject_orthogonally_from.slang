impl std::ops::Div<anti_reject_orthogonally_from> for AntiScalar {
    type Output = anti_reject_orthogonally_from_partial<AntiScalar>;
    fn div(self, _rhs: anti_reject_orthogonally_from) -> Self::Output {
        anti_reject_orthogonally_from_partial(self)
    }
}
impl AntiRejectOrthogonallyFrom<DualNum> for AntiScalar {
    type Output = AntiScalar;
    fn anti_reject_orthogonally_from(self, other: DualNum) -> Self::Output {
        use crate::elements::*;
        return AntiScalar::from_groups(/* e1234 */self[e1234] * powi(other[scalar], 2));
    }
}
impl AntiRejectOrthogonallyFrom<Motor> for AntiScalar {
    type Output = Motor;
// Operative Statistics for this implementation:
//           add/sub      mul      div
//      f32        0        1        0
//    simd4        0        3        0
// Totals...
// yes simd        0        4        0
//  no simd        0       13        0
    fn anti_reject_orthogonally_from(self, other: Motor) -> Self::Output {
        use crate::elements::*;
let wedge = AntiScalar::from_groups(/* e1234 */self[e1234] * other[scalar]);
let right_anti_dual = Motor::from_groups(/* e41, e42, e43, e1234 */other.group1() * float4(-1.0, -1.0, -1.0, 1.0), /* e23, e31, e12, scalar */float4(0.0));
        return Motor::from_groups(/* e41, e42, e43, e1234 */float4(wedge[e1234]) * right_anti_dual.group0(), /* e23, e31, e12, scalar */float4(wedge[e1234]) * right_anti_dual.group1());
    }
}
impl AntiRejectOrthogonallyFrom<MultiVector> for AntiScalar {
    type Output = MultiVector;
// Operative Statistics for this implementation:
//           add/sub      mul      div
//      f32        0        2        0
//    simd2        0        1        0
//    simd3        0        3        0
//    simd4        0        2        0
// Totals...
// yes simd        0        8        0
//  no simd        0       21        0
    fn anti_reject_orthogonally_from(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
let wedge = AntiScalar::from_groups(/* e1234 */self[e1234] * other[scalar]);
let right_anti_dual = MultiVector::from_groups(/* scalar, e1234 */float2(0.0, other[scalar]), /* e1, e2, e3, e4 */float4(float3(0.0), other[e321] * -1.0), /* e41, e42, e43 */other.group3() * float3(-1.0), /* e23, e31, e12 */float3(0.0), /* e423, e431, e412, e321 */float4(other[e1], other[e2], other[e3], 0.0));
        return MultiVector::from_groups(/* scalar, e1234 */float2(wedge[e1234]) * right_anti_dual.group0(), /* e1, e2, e3, e4 */float4(wedge[e1234]) * right_anti_dual.group1(), /* e41, e42, e43 */float3(wedge[e1234]) * right_anti_dual.group2(), /* e23, e31, e12 */float3(wedge[e1234]) * right_anti_dual.group3(), /* e423, e431, e412, e321 */float4(wedge[e1234]) * right_anti_dual.group4());
    }
}
impl AntiRejectOrthogonallyFrom<Scalar> for AntiScalar {
    type Output = AntiScalar;
    fn anti_reject_orthogonally_from(self, other: Scalar) -> Self::Output {
        use crate::elements::*;
        return AntiScalar::from_groups(/* e1234 */self[e1234] * powi(other[scalar], 2));
    }
}
impl std::ops::Div<anti_reject_orthogonally_from> for DualNum {
    type Output = anti_reject_orthogonally_from_partial<DualNum>;
    fn div(self, _rhs: anti_reject_orthogonally_from) -> Self::Output {
        anti_reject_orthogonally_from_partial(self)
    }
}
impl AntiRejectOrthogonallyFrom<DualNum> for DualNum {
    type Output = DualNum;
// Operative Statistics for this implementation:
//           add/sub      mul      div
//      f32        1        3        0
//    simd2        0        1        0
// Totals...
// yes simd        1        4        0
//  no simd        1        5        0
    fn anti_reject_orthogonally_from(self, other: DualNum) -> Self::Output {
        use crate::elements::*;
        return DualNum::from_groups(/* scalar, e1234 */float2(other[scalar]) * float2(other[scalar] * self[scalar], (other[scalar] * self[e1234]) + (other[e1234] * self[scalar])));
    }
}
impl AntiRejectOrthogonallyFrom<Flector> for DualNum {
    type Output = Motor;
// Operative Statistics for this implementation:
//           add/sub      mul      div
//      f32        5        7        0
//    simd3        1        2        0
//    simd4        2        4        0
// Totals...
// yes simd        8       13        0
//  no simd       16       29        0
    fn anti_reject_orthogonally_from(self, other: Flector) -> Self::Output {
        use crate::elements::*;
let wedge = Flector::from_groups(/* e1, e2, e3, e4 */float4(self[scalar]) * other.group0(), /* e423, e431, e412, e321 */float4(self[scalar]) * other.group1());
let right_anti_dual = Flector::from_groups(/* e1, e2, e3, e4 */float4(float3(0.0), other[e321] * -1.0), /* e423, e431, e412, e321 */float4(other[e1], other[e2], other[e3], 0.0));
        return Motor::from_groups(/* e41, e42, e43, e1234 */float4(((right_anti_dual.group1().yzx * wedge.group1().zxy) - (right_anti_dual.group1().zxy * wedge.group1().yzx)), 0.0), /* e23, e31, e12, scalar */(right_anti_dual.group1().wwwx * float4(wedge.group1().xyz, wedge[e1])) + float4(float3(0.0), (right_anti_dual[e431] * wedge[e2]) + (right_anti_dual[e412] * wedge[e3]) + (right_anti_dual[e321] * wedge[e4]) - (right_anti_dual[e2] * wedge[e431]) - (right_anti_dual[e3] * wedge[e412]) - (right_anti_dual[e4] * wedge[e321])) - (wedge.group1().wwwx * float4(right_anti_dual.group1().xyz, right_anti_dual[e1])));
    }
}
impl AntiRejectOrthogonallyFrom<Horizon> for DualNum {
    type Output = Scalar;
    fn anti_reject_orthogonally_from(self, other: Horizon) -> Self::Output {
        use crate::elements::*;
        return Scalar::from_groups(/* scalar */self[scalar] * powi(other[e321], 2));
    }
}
impl AntiRejectOrthogonallyFrom<Line> for DualNum {
    type Output = Scalar;
// Operative Statistics for this implementation:
//           add/sub      mul      div
//      f32        5        6        0
//    simd3        0        3        0
// Totals...
// yes simd        5        9        0
//  no simd        5       15        0
    fn anti_reject_orthogonally_from(self, other: Line) -> Self::Output {
        use crate::elements::*;
let wedge = Line::from_groups(/* e41, e42, e43 */float3(self[scalar]) * other.group0(), /* e23, e31, e12 */float3(self[scalar]) * other.group1());
let right_anti_dual = Line::from_groups(/* e41, e42, e43 */other.group1() * float3(-1.0), /* e23, e31, e12 */float3(0.0));
        return Scalar::from_groups(/* scalar */-(right_anti_dual[e41] * wedge[e23]) - (right_anti_dual[e42] * wedge[e31]) - (right_anti_dual[e43] * wedge[e12]) - (right_anti_dual[e23] * wedge[e41]) - (right_anti_dual[e31] * wedge[e42]) - (right_anti_dual[e12] * wedge[e43]));
    }
}
impl AntiRejectOrthogonallyFrom<Motor> for DualNum {
    type Output = Motor;
// Operative Statistics for this implementation:
//           add/sub      mul      div
//      f32        6        9        0
//    simd3        1        2        0
//    simd4        2        5        0
// Totals...
// yes simd        9       16        0
//  no simd       17       35        0
    fn anti_reject_orthogonally_from(self, other: Motor) -> Self::Output {
        use crate::elements::*;
let wedge = Motor::from_groups(/* e41, e42, e43, e1234 */float4(other[e41], other[e42], other[e43], 1.0) * float4(self.group0().xx, self[scalar], (self[scalar] * other[e1234]) + (self[e1234] * other[scalar])), /* e23, e31, e12, scalar */float4(self[scalar]) * other.group1());
let right_anti_dual = Motor::from_groups(/* e41, e42, e43, e1234 */other.group1() * float4(-1.0, -1.0, -1.0, 1.0), /* e23, e31, e12, scalar */float4(0.0));
        return Motor::from_groups(/* e41, e42, e43, e1234 */float4(((float3(right_anti_dual[e1234]) * wedge.group0().xyz) + (float3(wedge[e1234]) * right_anti_dual.group0().xyz)), right_anti_dual[e1234] * wedge[e1234]), /* e23, e31, e12, scalar */(float4(right_anti_dual[e1234]) * wedge.group1()) + (float4(wedge[e1234]) * right_anti_dual.group1()) + float4(float3(0.0), -(right_anti_dual[e41] * wedge[e23]) - (right_anti_dual[e42] * wedge[e31]) - (right_anti_dual[e43] * wedge[e12]) - (right_anti_dual[e23] * wedge[e41]) - (right_anti_dual[e31] * wedge[e42]) - (right_anti_dual[e12] * wedge[e43])));
    }
}
impl AntiRejectOrthogonallyFrom<MultiVector> for DualNum {
    type Output = MultiVector;
// Operative Statistics for this implementation:
//           add/sub      mul      div
//      f32       28       38        0
//    simd3        6       12        0
//    simd4        5        7        0
// Totals...
// yes simd       39       57        0
//  no simd       66      102        0
    fn anti_reject_orthogonally_from(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
let wedge = MultiVector::from_groups(/* scalar, e1234 */float2(self[scalar] * other[scalar], (self[scalar] * other[e1234]) + (self[e1234] * other[scalar])), /* e1, e2, e3, e4 */float4(self[scalar]) * other.group1(), /* e41, e42, e43 */float3(self[scalar]) * other.group2(), /* e23, e31, e12 */float3(self[scalar]) * other.group3(), /* e423, e431, e412, e321 */float4(self[scalar]) * other.group4());
let right_anti_dual = MultiVector::from_groups(/* scalar, e1234 */float2(0.0, other[scalar]), /* e1, e2, e3, e4 */float4(float3(0.0), other[e321] * -1.0), /* e41, e42, e43 */other.group3() * float3(-1.0), /* e23, e31, e12 */float3(0.0), /* e423, e431, e412, e321 */float4(other[e1], other[e2], other[e3], 0.0));
        return MultiVector::from_groups(/* scalar, e1234 */float2((right_anti_dual[scalar] * wedge[e1234]) + (right_anti_dual[e1234] * wedge[scalar]) + (right_anti_dual[e423] * wedge[e1]) + (right_anti_dual[e431] * wedge[e2]) + (right_anti_dual[e412] * wedge[e3]) + (right_anti_dual[e321] * wedge[e4]) - (right_anti_dual[e1] * wedge[e423]) - (right_anti_dual[e2] * wedge[e431]) - (right_anti_dual[e3] * wedge[e412]) - (right_anti_dual[e4] * wedge[e321]) - (right_anti_dual[e41] * wedge[e23]) - (right_anti_dual[e42] * wedge[e31]) - (right_anti_dual[e43] * wedge[e12]) - (right_anti_dual[e23] * wedge[e41]) - (right_anti_dual[e31] * wedge[e42]) - (right_anti_dual[e12] * wedge[e43]), right_anti_dual[e1234] * wedge[e1234]), /* e1, e2, e3, e4 */float4((right_anti_dual[e41] * wedge[e321]) + (right_anti_dual[e31] * wedge[e412]) + (right_anti_dual[e412] * wedge[e31]) + (right_anti_dual[e321] * wedge[e41]), (right_anti_dual[e42] * wedge[e321]) + (right_anti_dual[e12] * wedge[e423]) + (right_anti_dual[e423] * wedge[e12]) + (right_anti_dual[e321] * wedge[e42]), (right_anti_dual[e43] * wedge[e321]) + (right_anti_dual[e23] * wedge[e431]) + (right_anti_dual[e431] * wedge[e23]) + (right_anti_dual[e321] * wedge[e43]), -(right_anti_dual[e43] * wedge[e412]) - (right_anti_dual[e423] * wedge[e41]) - (right_anti_dual[e431] * wedge[e42]) - (right_anti_dual[e412] * wedge[e43])) + (float4(right_anti_dual[e1234]) * wedge.group1()) + (float4(wedge[e1234]) * right_anti_dual.group1()) - (wedge.group4().yzxx * float4(right_anti_dual.group3().zxy, right_anti_dual[e41])) - float4((wedge.group3().zxy * right_anti_dual.group4().yzx), right_anti_dual[e42] * wedge[e431]), /* e41, e42, e43 */(float3(right_anti_dual[e1234]) * wedge.group2()) + (float3(wedge[e1234]) * right_anti_dual.group2()) + (right_anti_dual.group4().yzx * wedge.group4().zxy) - (right_anti_dual.group4().zxy * wedge.group4().yzx), /* e23, e31, e12 */(float3(right_anti_dual[e1234]) * wedge.group3()) + (float3(right_anti_dual[e321]) * wedge.group4().xyz) + (float3(wedge[e1234]) * right_anti_dual.group3()) - (float3(wedge[e321]) * right_anti_dual.group4().xyz), /* e423, e431, e412, e321 */(float4(right_anti_dual[e1234]) * wedge.group4()) + (float4(wedge[e1234]) * right_anti_dual.group4()));
    }
}
impl AntiRejectOrthogonallyFrom<Plane> for DualNum {
    type Output = Scalar;
    fn anti_reject_orthogonally_from(self, other: Plane) -> Self::Output {
        use crate::elements::*;
        return Scalar::from_groups(/* scalar */self[scalar] * powi(other[e321], 2));
    }
}
impl AntiRejectOrthogonallyFrom<Point> for DualNum {
    type Output = Scalar;
// Operative Statistics for this implementation:
//           add/sub      mul      div
//      f32        3        4        0
//    simd4        0        1        0
// Totals...
// yes simd        3        5        0
//  no simd        3        8        0
    fn anti_reject_orthogonally_from(self, other: Point) -> Self::Output {
        use crate::elements::*;
let wedge = Point::from_groups(/* e1, e2, e3, e4 */float4(self[scalar]) * other.group0());
let right_anti_dual = Plane::from_groups(/* e423, e431, e412, e321 */float4(other[e1], other[e2], other[e3], 0.0));
        return Scalar::from_groups(/* scalar */(right_anti_dual[e423] * wedge[e1]) + (right_anti_dual[e431] * wedge[e2]) + (right_anti_dual[e412] * wedge[e3]) + (right_anti_dual[e321] * wedge[e4]));
    }
}
impl AntiRejectOrthogonallyFrom<Scalar> for DualNum {
    type Output = DualNum;
// Operative Statistics for this implementation:
//           add/sub      mul      div
//      f32        0        2        0
//    simd2        0        1        0
// Totals...
// yes simd        0        3        0
//  no simd        0        4        0
    fn anti_reject_orthogonally_from(self, other: Scalar) -> Self::Output {
        use crate::elements::*;
        return DualNum::from_groups(/* scalar, e1234 */float2(other[scalar]) * float2(self[scalar] * other[scalar], self[e1234] * other[scalar]));
    }
}
impl std::ops::Div<anti_reject_orthogonally_from> for Flector {
    type Output = anti_reject_orthogonally_from_partial<Flector>;
    fn div(self, _rhs: anti_reject_orthogonally_from) -> Self::Output {
        anti_reject_orthogonally_from_partial(self)
    }
}
impl AntiRejectOrthogonallyFrom<DualNum> for Flector {
    type Output = Flector;
// Operative Statistics for this implementation:
//          add/sub      mul      div
//   simd4        0        4        0
// no simd        0       16        0
    fn anti_reject_orthogonally_from(self, other: DualNum) -> Self::Output {
        use crate::elements::*;
let wedge = Flector::from_groups(/* e1, e2, e3, e4 */float4(other[scalar]) * self.group0(), /* e423, e431, e412, e321 */float4(other[scalar]) * self.group1());
let right_anti_dual = AntiScalar::from_groups(/* e1234 */other[scalar]);
        return Flector::from_groups(/* e1, e2, e3, e4 */float4(right_anti_dual[e1234]) * wedge.group0(), /* e423, e431, e412, e321 */float4(right_anti_dual[e1234]) * wedge.group1());
    }
}
impl AntiRejectOrthogonallyFrom<Flector> for Flector {
    type Output = Flector;
// Operative Statistics for this implementation:
//           add/sub      mul      div
//      f32        9       15        0
//    simd3        1        2        0
//    simd4        4        7        0
// Totals...
// yes simd       14       24        0
//  no simd       28       49        0
    fn anti_reject_orthogonally_from(self, other: Flector) -> Self::Output {
        use crate::elements::*;
let wedge = Motor::from_groups(/* e41, e42, e43, e1234 */(self.group0().wwwx * float4(other.group0().xyz, other[e423])) + float4(float3(0.0), (other[e431] * self[e2]) + (other[e412] * self[e3]) + (other[e321] * self[e4]) - (other[e2] * self[e431]) - (other[e3] * self[e412]) - (other[e4] * self[e321])) - (other.group0().wwwx * float4(self.group0().xyz, self[e423])), /* e23, e31, e12, scalar */float4(((other.group0().zxy * self.group0().yzx) - (other.group0().yzx * self.group0().zxy)), 0.0));
let right_anti_dual = Flector::from_groups(/* e1, e2, e3, e4 */float4(float3(0.0), other[e321] * -1.0), /* e423, e431, e412, e321 */float4(other[e1], other[e2], other[e3], 0.0));
        return Flector::from_groups(/* e1, e2, e3, e4 */float4((right_anti_dual[e412] * wedge[e31]) + (right_anti_dual[e321] * wedge[e41]), (right_anti_dual[e423] * wedge[e12]) + (right_anti_dual[e321] * wedge[e42]), (right_anti_dual[e431] * wedge[e23]) + (right_anti_dual[e321] * wedge[e43]), -(right_anti_dual[e431] * wedge[e42]) - (right_anti_dual[e412] * wedge[e43])) + (float4(wedge[e1234]) * right_anti_dual.group0()) - (right_anti_dual.group1().yzxx * float4(wedge.group1().zxy, wedge[e41])), /* e423, e431, e412, e321 */float4(float3(1.0), 0.0) * float4(wedge.group0().www, 0.0) * float4(right_anti_dual.group1().xyz, 0.0) * float4(1.0, 1.0, 1.0, 0.0));
    }
}
impl AntiRejectOrthogonallyFrom<Horizon> for Flector {
    type Output = Origin;
// Operative Statistics for this implementation:
//      add/sub      mul      div
// f32        0        1        0
    fn anti_reject_orthogonally_from(self, other: Horizon) -> Self::Output {
        use crate::elements::*;
        return Origin::from_groups(/* e4 */self[e4] * powi(other[e321], 2) * -1.0);
    }
}
impl AntiRejectOrthogonallyFrom<Line> for Flector {
    type Output = Point;
// Operative Statistics for this implementation:
//           add/sub      mul      div
//      f32        8       16        0
//    simd3        0        1        0
//    simd4        2        2        0
// Totals...
// yes simd       10       19        0
//  no simd       16       27        0
    fn anti_reject_orthogonally_from(self, other: Line) -> Self::Output {
        use crate::elements::*;
let wedge = Plane::from_groups(/* e423, e431, e412, e321 */float4((self[e3] * other[e42]) + (self[e4] * other[e23]), (self[e1] * other[e43]) + (self[e4] * other[e31]), (self[e2] * other[e41]) + (self[e4] * other[e12]), -(self[e2] * other[e31]) - (self[e3] * other[e12])) - (self.group0().yzxx * float4(other.group0().zxy, other[e23])));
let right_anti_dual = Line::from_groups(/* e41, e42, e43 */other.group1() * float3(-1.0), /* e23, e31, e12 */float3(0.0));
        return Point::from_groups(/* e1, e2, e3, e4 */float4((right_anti_dual[e41] * wedge[e321]) + (right_anti_dual[e31] * wedge[e412]), (right_anti_dual[e42] * wedge[e321]) + (right_anti_dual[e12] * wedge[e423]), (right_anti_dual[e43] * wedge[e321]) + (right_anti_dual[e23] * wedge[e431]), -(right_anti_dual[e42] * wedge[e431]) - (right_anti_dual[e43] * wedge[e412])) - (wedge.group0().yzxx * float4(right_anti_dual.group1().zxy, right_anti_dual[e41])));
    }
}
impl AntiRejectOrthogonallyFrom<Motor> for Flector {
    type Output = Flector;
// Operative Statistics for this implementation:
//           add/sub      mul      div
//      f32        8       17        0
//    simd3        0        1        0
//    simd4        4        6        0
// Totals...
// yes simd       12       24        0
//  no simd       24       44        0
    fn anti_reject_orthogonally_from(self, other: Motor) -> Self::Output {
        use crate::elements::*;
let wedge = Flector::from_groups(/* e1, e2, e3, e4 */float4(other[scalar]) * self.group0(), /* e423, e431, e412, e321 */float4((self[e4] * other[e23]) + (self[e423] * other[scalar]), (self[e4] * other[e31]) + (self[e431] * other[scalar]), (self[e4] * other[e12]) + (self[e412] * other[scalar]), -(self[e2] * other[e31]) - (self[e3] * other[e12])) + float4((self.group0().zxy * other.group0().yzx), self[e321] * other[scalar]) - (self.group0().yzxx * float4(other.group0().zxy, other[e23])));
let right_anti_dual = Motor::from_groups(/* e41, e42, e43, e1234 */other.group1() * float4(-1.0, -1.0, -1.0, 1.0), /* e23, e31, e12, scalar */float4(0.0));
        return Flector::from_groups(/* e1, e2, e3, e4 */float4((wedge[e412] * right_anti_dual[e31]) + (wedge[e321] * right_anti_dual[e41]), (wedge[e423] * right_anti_dual[e12]) + (wedge[e321] * right_anti_dual[e42]), (wedge[e431] * right_anti_dual[e23]) + (wedge[e321] * right_anti_dual[e43]), -(wedge[e431] * right_anti_dual[e42]) - (wedge[e412] * right_anti_dual[e43])) + (float4(right_anti_dual[e1234]) * wedge.group0()) - (wedge.group1().yzxx * float4(right_anti_dual.group1().zxy, right_anti_dual[e41])), /* e423, e431, e412, e321 */float4(right_anti_dual[e1234]) * wedge.group1());
    }
}
impl AntiRejectOrthogonallyFrom<MultiVector> for Flector {
    type Output = MultiVector;
// Operative Statistics for this implementation:
//           add/sub      mul      div
//      f32       38       51        0
//    simd3        8       14        0
//    simd4        7        8        0
// Totals...
// yes simd       53       73        0
//  no simd       90      125        0
    fn anti_reject_orthogonally_from(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
let wedge = MultiVector::from_groups(/* scalar, e1234 */float2(0.0, (self[e1] * other[e423]) + (self[e2] * other[e431]) + (self[e3] * other[e412]) + (self[e4] * other[e321]) - (self[e423] * other[e1]) - (self[e431] * other[e2]) - (self[e412] * other[e3]) - (self[e321] * other[e4])), /* e1, e2, e3, e4 */float4(other[scalar]) * self.group0(), /* e41, e42, e43 */(float3(self[e4]) * other.group1().xyz) - (float3(other[e4]) * self.group0().xyz), /* e23, e31, e12 */(self.group0().yzx * other.group1().zxy) - (self.group0().zxy * other.group1().yzx), /* e423, e431, e412, e321 */float4((self[e3] * other[e42]) + (self[e4] * other[e23]), (self[e1] * other[e43]) + (self[e4] * other[e31]), (self[e2] * other[e41]) + (self[e4] * other[e12]), -(self[e2] * other[e31]) - (self[e3] * other[e12])) + (float4(other[scalar]) * self.group1()) - (self.group0().yzxx * float4(other.group2().zxy, other[e23])));
let right_anti_dual = MultiVector::from_groups(/* scalar, e1234 */float2(0.0, other[scalar]), /* e1, e2, e3, e4 */float4(float3(0.0), other[e321] * -1.0), /* e41, e42, e43 */other.group3() * float3(-1.0), /* e23, e31, e12 */float3(0.0), /* e423, e431, e412, e321 */float4(other[e1], other[e2], other[e3], 0.0));
        return MultiVector::from_groups(/* scalar, e1234 */float2((right_anti_dual[scalar] * wedge[e1234]) + (right_anti_dual[e1234] * wedge[scalar]) + (right_anti_dual[e423] * wedge[e1]) + (right_anti_dual[e431] * wedge[e2]) + (right_anti_dual[e412] * wedge[e3]) + (right_anti_dual[e321] * wedge[e4]) - (right_anti_dual[e1] * wedge[e423]) - (right_anti_dual[e2] * wedge[e431]) - (right_anti_dual[e3] * wedge[e412]) - (right_anti_dual[e4] * wedge[e321]) - (right_anti_dual[e41] * wedge[e23]) - (right_anti_dual[e42] * wedge[e31]) - (right_anti_dual[e43] * wedge[e12]) - (right_anti_dual[e23] * wedge[e41]) - (right_anti_dual[e31] * wedge[e42]) - (right_anti_dual[e12] * wedge[e43]), right_anti_dual[e1234] * wedge[e1234]), /* e1, e2, e3, e4 */float4((right_anti_dual[e41] * wedge[e321]) + (right_anti_dual[e31] * wedge[e412]) + (right_anti_dual[e412] * wedge[e31]) + (right_anti_dual[e321] * wedge[e41]), (right_anti_dual[e42] * wedge[e321]) + (right_anti_dual[e12] * wedge[e423]) + (right_anti_dual[e423] * wedge[e12]) + (right_anti_dual[e321] * wedge[e42]), (right_anti_dual[e43] * wedge[e321]) + (right_anti_dual[e23] * wedge[e431]) + (right_anti_dual[e431] * wedge[e23]) + (right_anti_dual[e321] * wedge[e43]), -(right_anti_dual[e43] * wedge[e412]) - (right_anti_dual[e423] * wedge[e41]) - (right_anti_dual[e431] * wedge[e42]) - (right_anti_dual[e412] * wedge[e43])) + (float4(right_anti_dual[e1234]) * wedge.group1()) + (float4(wedge[e1234]) * right_anti_dual.group1()) - (wedge.group4().yzxx * float4(right_anti_dual.group3().zxy, right_anti_dual[e41])) - float4((wedge.group3().zxy * right_anti_dual.group4().yzx), right_anti_dual[e42] * wedge[e431]), /* e41, e42, e43 */(float3(right_anti_dual[e1234]) * wedge.group2()) + (float3(wedge[e1234]) * right_anti_dual.group2()) + (right_anti_dual.group4().yzx * wedge.group4().zxy) - (right_anti_dual.group4().zxy * wedge.group4().yzx), /* e23, e31, e12 */(float3(right_anti_dual[e1234]) * wedge.group3()) + (float3(right_anti_dual[e321]) * wedge.group4().xyz) + (float3(wedge[e1234]) * right_anti_dual.group3()) - (float3(wedge[e321]) * right_anti_dual.group4().xyz), /* e423, e431, e412, e321 */(float4(right_anti_dual[e1234]) * wedge.group4()) + (float4(wedge[e1234]) * right_anti_dual.group4()));
    }
}
impl AntiRejectOrthogonallyFrom<Plane> for Flector {
    type Output = Origin;
// Operative Statistics for this implementation:
//      add/sub      mul      div
// f32        3        7        0
    fn anti_reject_orthogonally_from(self, other: Plane) -> Self::Output {
        use crate::elements::*;
        return Origin::from_groups(/* e4 */-(powi(other[e321], 2) * self[e4]) - (self[e1] * other[e423] * other[e321]) - (self[e2] * other[e431] * other[e321]) - (self[e3] * other[e412] * other[e321]));
    }
}
impl AntiRejectOrthogonallyFrom<Point> for Flector {
    type Output = Flector;
// Operative Statistics for this implementation:
//           add/sub      mul      div
//      f32        6       14        0
//    simd3        1        2        0
//    simd4        2        5        0
// Totals...
// yes simd        9       21        0
//  no simd       17       40        0
    fn anti_reject_orthogonally_from(self, other: Point) -> Self::Output {
        use crate::elements::*;
let wedge = Motor::from_groups(/* e41, e42, e43, e1234 */float4(self[e4] * other[e1], self[e4] * other[e2], self[e4] * other[e3], -(self[e431] * other[e2]) - (self[e412] * other[e3]) - (self[e321] * other[e4])) - (other.group0().wwwx * float4(self.group0().xyz, self[e423])), /* e23, e31, e12, scalar */float4(((self.group0().yzx * other.group0().zxy) - (self.group0().zxy * other.group0().yzx)), 0.0));
let right_anti_dual = Plane::from_groups(/* e423, e431, e412, e321 */float4(other[e1], other[e2], other[e3], 0.0));
        return Flector::from_groups(/* e1, e2, e3, e4 */float4((wedge[e41] * right_anti_dual[e321]) + (wedge[e31] * right_anti_dual[e412]), (wedge[e42] * right_anti_dual[e321]) + (wedge[e12] * right_anti_dual[e423]), (wedge[e43] * right_anti_dual[e321]) + (wedge[e23] * right_anti_dual[e431]), -(wedge[e42] * right_anti_dual[e431]) - (wedge[e43] * right_anti_dual[e412])) - (right_anti_dual.group0().yzxx * float4(wedge.group1().zxy, wedge[e41])), /* e423, e431, e412, e321 */float4(float3(1.0), 0.0) * float4(wedge.group0().www, 0.0) * float4(right_anti_dual.group0().xyz, 0.0) * float4(1.0, 1.0, 1.0, 0.0));
    }
}
impl AntiRejectOrthogonallyFrom<Scalar> for Flector {
    type Output = Flector;
// Operative Statistics for this implementation:
//          add/sub      mul      div
//   simd4        0        4        0
// no simd        0       16        0
    fn anti_reject_orthogonally_from(self, other: Scalar) -> Self::Output {
        use crate::elements::*;
let wedge = Flector::from_groups(/* e1, e2, e3, e4 */float4(other[scalar]) * self.group0(), /* e423, e431, e412, e321 */float4(other[scalar]) * self.group1());
let right_anti_dual = AntiScalar::from_groups(/* e1234 */other[scalar]);
        return Flector::from_groups(/* e1, e2, e3, e4 */float4(right_anti_dual[e1234]) * wedge.group0(), /* e423, e431, e412, e321 */float4(right_anti_dual[e1234]) * wedge.group1());
    }
}
impl std::ops::Div<anti_reject_orthogonally_from> for Horizon {
    type Output = anti_reject_orthogonally_from_partial<Horizon>;
    fn div(self, _rhs: anti_reject_orthogonally_from) -> Self::Output {
        anti_reject_orthogonally_from_partial(self)
    }
}
impl AntiRejectOrthogonallyFrom<DualNum> for Horizon {
    type Output = Horizon;
// Operative Statistics for this implementation:
//      add/sub      mul      div
// f32        0        1        0
    fn anti_reject_orthogonally_from(self, other: DualNum) -> Self::Output {
        use crate::elements::*;
        return Horizon::from_groups(/* e321 */powi(other[scalar], 2) * self[e321]);
    }
}
impl AntiRejectOrthogonallyFrom<Flector> for Horizon {
    type Output = Flector;
// Operative Statistics for this implementation:
//           add/sub      mul      div
//      f32        0        3        0
//    simd4        0        2        0
// Totals...
// yes simd        0        5        0
//  no simd        0       11        0
    fn anti_reject_orthogonally_from(self, other: Flector) -> Self::Output {
        use crate::elements::*;
let wedge = AntiScalar::from_groups(/* e1234 */other[e4] * self[e321] * -1.0);
let right_anti_dual = Flector::from_groups(/* e1, e2, e3, e4 */float4(float3(0.0), other[e321] * -1.0), /* e423, e431, e412, e321 */float4(other[e1], other[e2], other[e3], 0.0));
        return Flector::from_groups(/* e1, e2, e3, e4 */float4(wedge[e1234]) * right_anti_dual.group0(), /* e423, e431, e412, e321 */float4(wedge[e1234]) * right_anti_dual.group1());
    }
}
impl AntiRejectOrthogonallyFrom<Motor> for Horizon {
    type Output = Flector;
// Operative Statistics for this implementation:
//           add/sub      mul      div
//      f32        0        2        0
//    simd4        0        3        0
// Totals...
// yes simd        0        5        0
//  no simd        0       14        0
    fn anti_reject_orthogonally_from(self, other: Motor) -> Self::Output {
        use crate::elements::*;
let wedge = Horizon::from_groups(/* e321 */self[e321] * other[scalar]);
let right_anti_dual = Motor::from_groups(/* e41, e42, e43, e1234 */other.group1() * float4(-1.0, -1.0, -1.0, 1.0), /* e23, e31, e12, scalar */float4(0.0));
        return Flector::from_groups(/* e1, e2, e3, e4 */float4(wedge[e321], wedge[e321], wedge[e321], 0.0) * float4(right_anti_dual.group0().xyz, 0.0) * float4(1.0, 1.0, 1.0, 0.0), /* e423, e431, e412, e321 */float4(float3(0.0), wedge[e321] * right_anti_dual[e1234]));
    }
}
impl AntiRejectOrthogonallyFrom<MultiVector> for Horizon {
    type Output = MultiVector;
// Operative Statistics for this implementation:
//           add/sub      mul      div
//      f32       27       37        0
//    simd2        0        1        0
//    simd3        6       10        0
//    simd4        5        5        0
// Totals...
// yes simd       38       53        0
//  no simd       65       89        0
    fn anti_reject_orthogonally_from(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
let wedge = MultiVector::from_groups(/* scalar, e1234 */float2(1.0, self[e321] * other[e4]) * float2(0.0, -1.0), /* e1, e2, e3, e4 */float4(0.0), /* e41, e42, e43 */float3(0.0), /* e23, e31, e12 */float3(0.0), /* e423, e431, e412, e321 */float4(float3(0.0), self[e321] * other[scalar]));
let right_anti_dual = MultiVector::from_groups(/* scalar, e1234 */float2(0.0, other[scalar]), /* e1, e2, e3, e4 */float4(float3(0.0), other[e321] * -1.0), /* e41, e42, e43 */other.group3() * float3(-1.0), /* e23, e31, e12 */float3(0.0), /* e423, e431, e412, e321 */float4(other[e1], other[e2], other[e3], 0.0));
        return MultiVector::from_groups(/* scalar, e1234 */float2((right_anti_dual[scalar] * wedge[e1234]) + (right_anti_dual[e1234] * wedge[scalar]) + (right_anti_dual[e423] * wedge[e1]) + (right_anti_dual[e431] * wedge[e2]) + (right_anti_dual[e412] * wedge[e3]) + (right_anti_dual[e321] * wedge[e4]) - (right_anti_dual[e1] * wedge[e423]) - (right_anti_dual[e2] * wedge[e431]) - (right_anti_dual[e3] * wedge[e412]) - (right_anti_dual[e4] * wedge[e321]) - (right_anti_dual[e41] * wedge[e23]) - (right_anti_dual[e42] * wedge[e31]) - (right_anti_dual[e43] * wedge[e12]) - (right_anti_dual[e23] * wedge[e41]) - (right_anti_dual[e31] * wedge[e42]) - (right_anti_dual[e12] * wedge[e43]), right_anti_dual[e1234] * wedge[e1234]), /* e1, e2, e3, e4 */float4((right_anti_dual[e41] * wedge[e321]) + (right_anti_dual[e31] * wedge[e412]) + (right_anti_dual[e412] * wedge[e31]) + (right_anti_dual[e321] * wedge[e41]), (right_anti_dual[e42] * wedge[e321]) + (right_anti_dual[e12] * wedge[e423]) + (right_anti_dual[e423] * wedge[e12]) + (right_anti_dual[e321] * wedge[e42]), (right_anti_dual[e43] * wedge[e321]) + (right_anti_dual[e23] * wedge[e431]) + (right_anti_dual[e431] * wedge[e23]) + (right_anti_dual[e321] * wedge[e43]), -(right_anti_dual[e43] * wedge[e412]) - (right_anti_dual[e423] * wedge[e41]) - (right_anti_dual[e431] * wedge[e42]) - (right_anti_dual[e412] * wedge[e43])) + (float4(right_anti_dual[e1234]) * wedge.group1()) + (float4(wedge[e1234]) * right_anti_dual.group1()) - (wedge.group4().yzxx * float4(right_anti_dual.group3().zxy, right_anti_dual[e41])) - float4((wedge.group3().zxy * right_anti_dual.group4().yzx), right_anti_dual[e42] * wedge[e431]), /* e41, e42, e43 */(float3(right_anti_dual[e1234]) * wedge.group2()) + (float3(wedge[e1234]) * right_anti_dual.group2()) + (right_anti_dual.group4().yzx * wedge.group4().zxy) - (right_anti_dual.group4().zxy * wedge.group4().yzx), /* e23, e31, e12 */(float3(right_anti_dual[e1234]) * wedge.group3()) + (float3(right_anti_dual[e321]) * wedge.group4().xyz) + (float3(wedge[e1234]) * right_anti_dual.group3()) - (float3(wedge[e321]) * right_anti_dual.group4().xyz), /* e423, e431, e412, e321 */(float4(right_anti_dual[e1234]) * wedge.group4()) + (float4(wedge[e1234]) * right_anti_dual.group4()));
    }
}
impl AntiRejectOrthogonallyFrom<Point> for Horizon {
    type Output = Plane;
// Operative Statistics for this implementation:
//           add/sub      mul      div
//      f32        0        2        0
//    simd4        0        1        0
// Totals...
// yes simd        0        3        0
//  no simd        0        6        0
    fn anti_reject_orthogonally_from(self, other: Point) -> Self::Output {
        use crate::elements::*;
        return Plane::from_groups(/* e423, e431, e412, e321 */float4(self[e321] * other[e4] * -1.0) * float4(other[e1], other[e2], other[e3], 0.0));
    }
}
impl AntiRejectOrthogonallyFrom<Scalar> for Horizon {
    type Output = Horizon;
    fn anti_reject_orthogonally_from(self, other: Scalar) -> Self::Output {
        use crate::elements::*;
        return Horizon::from_groups(/* e321 */self[e321] * powi(other[scalar], 2));
    }
}
impl std::ops::Div<anti_reject_orthogonally_from> for Line {
    type Output = anti_reject_orthogonally_from_partial<Line>;
    fn div(self, _rhs: anti_reject_orthogonally_from) -> Self::Output {
        anti_reject_orthogonally_from_partial(self)
    }
}
impl AntiRejectOrthogonallyFrom<DualNum> for Line {
    type Output = Line;
// Operative Statistics for this implementation:
//          add/sub      mul      div
//   simd3        0        4        0
// no simd        0       12        0
    fn anti_reject_orthogonally_from(self, other: DualNum) -> Self::Output {
        use crate::elements::*;
let wedge = Line::from_groups(/* e41, e42, e43 */float3(other[scalar]) * self.group0(), /* e23, e31, e12 */float3(other[scalar]) * self.group1());
let right_anti_dual = AntiScalar::from_groups(/* e1234 */other[scalar]);
        return Line::from_groups(/* e41, e42, e43 */float3(right_anti_dual[e1234]) * wedge.group0(), /* e23, e31, e12 */float3(right_anti_dual[e1234]) * wedge.group1());
    }
}
impl AntiRejectOrthogonallyFrom<Flector> for Line {
    type Output = Motor;
// Operative Statistics for this implementation:
//           add/sub      mul      div
//      f32        6       15        0
//    simd3        1        2        0
//    simd4        2        2        0
// Totals...
// yes simd        9       19        0
//  no simd       17       29        0
    fn anti_reject_orthogonally_from(self, other: Flector) -> Self::Output {
        use crate::elements::*;
let wedge = Plane::from_groups(/* e423, e431, e412, e321 */float4((other[e3] * self[e42]) + (other[e4] * self[e23]), (other[e1] * self[e43]) + (other[e4] * self[e31]), (other[e2] * self[e41]) + (other[e4] * self[e12]), -(other[e2] * self[e31]) - (other[e3] * self[e12])) - (other.group0().yzxx * float4(self.group0().zxy, self[e23])));
let right_anti_dual = Flector::from_groups(/* e1, e2, e3, e4 */float4(float3(0.0), other[e321] * -1.0), /* e423, e431, e412, e321 */float4(other[e1], other[e2], other[e3], 0.0));
        return Motor::from_groups(/* e41, e42, e43, e1234 */float4(((right_anti_dual.group1().yzx * wedge.group0().zxy) - (right_anti_dual.group1().zxy * wedge.group0().yzx)), 0.0), /* e23, e31, e12, scalar */float4(right_anti_dual[e321] * wedge[e423], right_anti_dual[e321] * wedge[e431], right_anti_dual[e321] * wedge[e412], -(right_anti_dual[e2] * wedge[e431]) - (right_anti_dual[e3] * wedge[e412]) - (right_anti_dual[e4] * wedge[e321])) - (wedge.group0().wwwx * float4(right_anti_dual.group1().xyz, right_anti_dual[e1])));
    }
}
impl AntiRejectOrthogonallyFrom<Line> for Line {
    type Output = Line;
// Operative Statistics for this implementation:
//           add/sub      mul      div
//      f32        5        9        0
//    simd3        0        1        0
// Totals...
// yes simd        5       10        0
//  no simd        5       12        0
    fn anti_reject_orthogonally_from(self, other: Line) -> Self::Output {
        use crate::elements::*;
        return Line::from_groups(/* e41, e42, e43 */float3(-(other[e41] * self[e23]) - (other[e42] * self[e31]) - (other[e43] * self[e12]) - (other[e23] * self[e41]) - (other[e31] * self[e42]) - (other[e12] * self[e43])) * float3(other[e23] * -1.0, other[e31] * -1.0, other[e12] * -1.0), /* e23, e31, e12 */float3(0.0));
    }
}
impl AntiRejectOrthogonallyFrom<Motor> for Line {
    type Output = Motor;
// Operative Statistics for this implementation:
//           add/sub      mul      div
//      f32       10       13        0
//    simd3        1        2        0
//    simd4        2        7        0
// Totals...
// yes simd       13       22        0
//  no simd       21       47        0
    fn anti_reject_orthogonally_from(self, other: Motor) -> Self::Output {
        use crate::elements::*;
let wedge = Motor::from_groups(/* e41, e42, e43, e1234 */float4(other[scalar], other[scalar], other[scalar], 1.0) * float4(self.group0(), -(self[e41] * other[e23]) - (self[e42] * other[e31]) - (self[e43] * other[e12]) - (self[e23] * other[e41]) - (self[e31] * other[e42]) - (self[e12] * other[e43])), /* e23, e31, e12, scalar */float4(float3(1.0), 0.0) * float4(self.group1(), 0.0) * float4(other.group1().www, 0.0) * float4(1.0, 1.0, 1.0, 0.0));
let right_anti_dual = Motor::from_groups(/* e41, e42, e43, e1234 */other.group1() * float4(-1.0, -1.0, -1.0, 1.0), /* e23, e31, e12, scalar */float4(0.0));
        return Motor::from_groups(/* e41, e42, e43, e1234 */float4(((float3(right_anti_dual[e1234]) * wedge.group0().xyz) + (float3(wedge[e1234]) * right_anti_dual.group0().xyz)), right_anti_dual[e1234] * wedge[e1234]), /* e23, e31, e12, scalar */(float4(right_anti_dual[e1234]) * wedge.group1()) + (float4(wedge[e1234]) * right_anti_dual.group1()) + float4(float3(0.0), -(right_anti_dual[e41] * wedge[e23]) - (right_anti_dual[e42] * wedge[e31]) - (right_anti_dual[e43] * wedge[e12]) - (right_anti_dual[e23] * wedge[e41]) - (right_anti_dual[e31] * wedge[e42]) - (right_anti_dual[e12] * wedge[e43])));
    }
}
impl AntiRejectOrthogonallyFrom<MultiVector> for Line {
    type Output = MultiVector;
// Operative Statistics for this implementation:
//           add/sub      mul      div
//      f32       36       49        0
//    simd3        6       12        0
//    simd4        6        6        0
// Totals...
// yes simd       48       67        0
//  no simd       78      109        0
    fn anti_reject_orthogonally_from(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
let wedge = MultiVector::from_groups(/* scalar, e1234 */float2(0.0, -(self[e41] * other[e23]) - (self[e42] * other[e31]) - (self[e43] * other[e12]) - (self[e23] * other[e41]) - (self[e31] * other[e42]) - (self[e12] * other[e43])), /* e1, e2, e3, e4 */float4(0.0), /* e41, e42, e43 */float3(other[scalar]) * self.group0(), /* e23, e31, e12 */float3(other[scalar]) * self.group1(), /* e423, e431, e412, e321 */float4((self[e42] * other[e3]) + (self[e23] * other[e4]), (self[e43] * other[e1]) + (self[e31] * other[e4]), (self[e41] * other[e2]) + (self[e12] * other[e4]), -(self[e31] * other[e2]) - (self[e12] * other[e3])) - (other.group1().yzxx * float4(self.group0().zxy, self[e23])));
let right_anti_dual = MultiVector::from_groups(/* scalar, e1234 */float2(0.0, other[scalar]), /* e1, e2, e3, e4 */float4(float3(0.0), other[e321] * -1.0), /* e41, e42, e43 */other.group3() * float3(-1.0), /* e23, e31, e12 */float3(0.0), /* e423, e431, e412, e321 */float4(other[e1], other[e2], other[e3], 0.0));
        return MultiVector::from_groups(/* scalar, e1234 */float2((right_anti_dual[scalar] * wedge[e1234]) + (right_anti_dual[e1234] * wedge[scalar]) + (right_anti_dual[e423] * wedge[e1]) + (right_anti_dual[e431] * wedge[e2]) + (right_anti_dual[e412] * wedge[e3]) + (right_anti_dual[e321] * wedge[e4]) - (right_anti_dual[e1] * wedge[e423]) - (right_anti_dual[e2] * wedge[e431]) - (right_anti_dual[e3] * wedge[e412]) - (right_anti_dual[e4] * wedge[e321]) - (right_anti_dual[e41] * wedge[e23]) - (right_anti_dual[e42] * wedge[e31]) - (right_anti_dual[e43] * wedge[e12]) - (right_anti_dual[e23] * wedge[e41]) - (right_anti_dual[e31] * wedge[e42]) - (right_anti_dual[e12] * wedge[e43]), right_anti_dual[e1234] * wedge[e1234]), /* e1, e2, e3, e4 */float4((right_anti_dual[e41] * wedge[e321]) + (right_anti_dual[e31] * wedge[e412]) + (right_anti_dual[e412] * wedge[e31]) + (right_anti_dual[e321] * wedge[e41]), (right_anti_dual[e42] * wedge[e321]) + (right_anti_dual[e12] * wedge[e423]) + (right_anti_dual[e423] * wedge[e12]) + (right_anti_dual[e321] * wedge[e42]), (right_anti_dual[e43] * wedge[e321]) + (right_anti_dual[e23] * wedge[e431]) + (right_anti_dual[e431] * wedge[e23]) + (right_anti_dual[e321] * wedge[e43]), -(right_anti_dual[e43] * wedge[e412]) - (right_anti_dual[e423] * wedge[e41]) - (right_anti_dual[e431] * wedge[e42]) - (right_anti_dual[e412] * wedge[e43])) + (float4(right_anti_dual[e1234]) * wedge.group1()) + (float4(wedge[e1234]) * right_anti_dual.group1()) - (wedge.group4().yzxx * float4(right_anti_dual.group3().zxy, right_anti_dual[e41])) - float4((wedge.group3().zxy * right_anti_dual.group4().yzx), right_anti_dual[e42] * wedge[e431]), /* e41, e42, e43 */(float3(right_anti_dual[e1234]) * wedge.group2()) + (float3(wedge[e1234]) * right_anti_dual.group2()) + (right_anti_dual.group4().yzx * wedge.group4().zxy) - (right_anti_dual.group4().zxy * wedge.group4().yzx), /* e23, e31, e12 */(float3(right_anti_dual[e1234]) * wedge.group3()) + (float3(right_anti_dual[e321]) * wedge.group4().xyz) + (float3(wedge[e1234]) * right_anti_dual.group3()) - (float3(wedge[e321]) * right_anti_dual.group4().xyz), /* e423, e431, e412, e321 */(float4(right_anti_dual[e1234]) * wedge.group4()) + (float4(wedge[e1234]) * right_anti_dual.group4()));
    }
}
impl AntiRejectOrthogonallyFrom<Point> for Line {
    type Output = Line;
// Operative Statistics for this implementation:
//           add/sub      mul      div
//      f32        4        8        0
//    simd3        2        4        0
//    simd4        1        1        0
// Totals...
// yes simd        7       13        0
//  no simd       14       24        0
    fn anti_reject_orthogonally_from(self, other: Point) -> Self::Output {
        use crate::elements::*;
let wedge = Plane::from_groups(/* e423, e431, e412, e321 */float4((self[e42] * other[e3]) + (self[e23] * other[e4]), (self[e43] * other[e1]) + (self[e31] * other[e4]), (self[e41] * other[e2]) + (self[e12] * other[e4]), -(self[e31] * other[e2]) - (self[e12] * other[e3])) - (other.group0().yzxx * float4(self.group0().zxy, self[e23])));
let right_anti_dual = Plane::from_groups(/* e423, e431, e412, e321 */float4(other[e1], other[e2], other[e3], 0.0));
        return Line::from_groups(/* e41, e42, e43 */(right_anti_dual.group0().yzx * wedge.group0().zxy) - (right_anti_dual.group0().zxy * wedge.group0().yzx), /* e23, e31, e12 */(float3(right_anti_dual[e321]) * wedge.group0().xyz) - (float3(wedge[e321]) * right_anti_dual.group0().xyz));
    }
}
impl AntiRejectOrthogonallyFrom<Scalar> for Line {
    type Output = Line;
// Operative Statistics for this implementation:
//          add/sub      mul      div
//   simd3        0        4        0
// no simd        0       12        0
    fn anti_reject_orthogonally_from(self, other: Scalar) -> Self::Output {
        use crate::elements::*;
let wedge = Line::from_groups(/* e41, e42, e43 */float3(other[scalar]) * self.group0(), /* e23, e31, e12 */float3(other[scalar]) * self.group1());
let right_anti_dual = AntiScalar::from_groups(/* e1234 */other[scalar]);
        return Line::from_groups(/* e41, e42, e43 */float3(right_anti_dual[e1234]) * wedge.group0(), /* e23, e31, e12 */float3(right_anti_dual[e1234]) * wedge.group1());
    }
}
impl std::ops::Div<anti_reject_orthogonally_from> for Motor {
    type Output = anti_reject_orthogonally_from_partial<Motor>;
    fn div(self, _rhs: anti_reject_orthogonally_from) -> Self::Output {
        anti_reject_orthogonally_from_partial(self)
    }
}
impl AntiRejectOrthogonallyFrom<DualNum> for Motor {
    type Output = Motor;
// Operative Statistics for this implementation:
//           add/sub      mul      div
//      f32        1        2        0
//    simd4        0        4        0
// Totals...
// yes simd        1        6        0
//  no simd        1       18        0
    fn anti_reject_orthogonally_from(self, other: DualNum) -> Self::Output {
        use crate::elements::*;
let wedge = Motor::from_groups(/* e41, e42, e43, e1234 */float4(self[e41], self[e42], self[e43], 1.0) * float4(other.group0().xx, other[scalar], (other[scalar] * self[e1234]) + (other[e1234] * self[scalar])), /* e23, e31, e12, scalar */float4(other[scalar]) * self.group1());
let right_anti_dual = AntiScalar::from_groups(/* e1234 */other[scalar]);
        return Motor::from_groups(/* e41, e42, e43, e1234 */float4(right_anti_dual[e1234]) * wedge.group0(), /* e23, e31, e12, scalar */float4(right_anti_dual[e1234]) * wedge.group1());
    }
}
impl AntiRejectOrthogonallyFrom<Flector> for Motor {
    type Output = Motor;
// Operative Statistics for this implementation:
//           add/sub      mul      div
//      f32        9       16        0
//    simd3        1        3        0
//    simd4        4        4        0
// Totals...
// yes simd       14       23        0
//  no simd       28       41        0
    fn anti_reject_orthogonally_from(self, other: Flector) -> Self::Output {
        use crate::elements::*;
let wedge = Flector::from_groups(/* e1, e2, e3, e4 */float4(self[scalar]) * other.group0(), /* e423, e431, e412, e321 */float4((other[e4] * self[e23]) + (other[e423] * self[scalar]), (other[e4] * self[e31]) + (other[e431] * self[scalar]), (other[e4] * self[e12]) + (other[e412] * self[scalar]), -(other[e2] * self[e31]) - (other[e3] * self[e12])) + float4((other.group0().zxy * self.group0().yzx), other[e321] * self[scalar]) - (other.group0().yzxx * float4(self.group0().zxy, self[e23])));
let right_anti_dual = Flector::from_groups(/* e1, e2, e3, e4 */float4(float3(0.0), other[e321] * -1.0), /* e423, e431, e412, e321 */float4(other[e1], other[e2], other[e3], 0.0));
        return Motor::from_groups(/* e41, e42, e43, e1234 */float4(((right_anti_dual.group1().yzx * wedge.group1().zxy) - (right_anti_dual.group1().zxy * wedge.group1().yzx)), 0.0), /* e23, e31, e12, scalar */(right_anti_dual.group1().wwwx * float4(wedge.group1().xyz, wedge[e1])) + float4(float3(0.0), (right_anti_dual[e431] * wedge[e2]) + (right_anti_dual[e412] * wedge[e3]) + (right_anti_dual[e321] * wedge[e4]) - (right_anti_dual[e2] * wedge[e431]) - (right_anti_dual[e3] * wedge[e412]) - (right_anti_dual[e4] * wedge[e321])) - (wedge.group1().wwwx * float4(right_anti_dual.group1().xyz, right_anti_dual[e1])));
    }
}
impl AntiRejectOrthogonallyFrom<Horizon> for Motor {
    type Output = Scalar;
// Operative Statistics for this implementation:
//      add/sub      mul      div
// f32        0        1        0
    fn anti_reject_orthogonally_from(self, other: Horizon) -> Self::Output {
        use crate::elements::*;
        return Scalar::from_groups(/* scalar */powi(other[e321], 2) * self[scalar]);
    }
}
impl AntiRejectOrthogonallyFrom<Line> for Motor {
    type Output = Motor;
// Operative Statistics for this implementation:
//           add/sub      mul      div
//      f32       10       12        0
//    simd3        0        1        0
//    simd4        0        7        0
// Totals...
// yes simd       10       20        0
//  no simd       10       43        0
    fn anti_reject_orthogonally_from(self, other: Line) -> Self::Output {
        use crate::elements::*;
let wedge = Motor::from_groups(/* e41, e42, e43, e1234 */float4(self[scalar], self[scalar], self[scalar], 1.0) * float4(other.group0(), -(other[e41] * self[e23]) - (other[e42] * self[e31]) - (other[e43] * self[e12]) - (other[e23] * self[e41]) - (other[e31] * self[e42]) - (other[e12] * self[e43])), /* e23, e31, e12, scalar */float4(float3(1.0), 0.0) * float4(other.group1(), 0.0) * float4(self.group1().www, 0.0) * float4(1.0, 1.0, 1.0, 0.0));
let right_anti_dual = Line::from_groups(/* e41, e42, e43 */other.group1() * float3(-1.0), /* e23, e31, e12 */float3(0.0));
        return Motor::from_groups(/* e41, e42, e43, e1234 */float4(float3(1.0), 0.0) * float4(right_anti_dual.group0(), 0.0) * float4(wedge.group0().www, 0.0) * float4(1.0, 1.0, 1.0, 0.0), /* e23, e31, e12, scalar */float4(float3(0.0), -(right_anti_dual[e41] * wedge[e23]) - (right_anti_dual[e42] * wedge[e31]) - (right_anti_dual[e43] * wedge[e12]) - (right_anti_dual[e23] * wedge[e41]) - (right_anti_dual[e31] * wedge[e42]) - (right_anti_dual[e12] * wedge[e43])));
    }
}
impl AntiRejectOrthogonallyFrom<Motor> for Motor {
    type Output = Motor;
// Operative Statistics for this implementation:
//           add/sub      mul      div
//      f32       10       14        0
//    simd3        2        4        0
//    simd4        4        5        0
// Totals...
// yes simd       16       23        0
//  no simd       32       46        0
    fn anti_reject_orthogonally_from(self, other: Motor) -> Self::Output {
        use crate::elements::*;
let wedge = Motor::from_groups(/* e41, e42, e43, e1234 */(float4(other[scalar]) * self.group0()) + (float4(self[scalar]) * other.group0()) + float4(float3(0.0), -(other[e41] * self[e23]) - (other[e42] * self[e31]) - (other[e43] * self[e12]) - (other[e23] * self[e41]) - (other[e31] * self[e42]) - (other[e12] * self[e43])), /* e23, e31, e12, scalar */float4(((float3(other[scalar]) * self.group1().xyz) + (float3(self[scalar]) * other.group1().xyz)), other[scalar] * self[scalar]));
let right_anti_dual = Motor::from_groups(/* e41, e42, e43, e1234 */other.group1() * float4(-1.0, -1.0, -1.0, 1.0), /* e23, e31, e12, scalar */float4(0.0));
        return Motor::from_groups(/* e41, e42, e43, e1234 */float4(((float3(right_anti_dual[e1234]) * wedge.group0().xyz) + (float3(wedge[e1234]) * right_anti_dual.group0().xyz)), right_anti_dual[e1234] * wedge[e1234]), /* e23, e31, e12, scalar */(float4(right_anti_dual[e1234]) * wedge.group1()) + (float4(wedge[e1234]) * right_anti_dual.group1()) + float4(float3(0.0), -(right_anti_dual[e41] * wedge[e23]) - (right_anti_dual[e42] * wedge[e31]) - (right_anti_dual[e43] * wedge[e12]) - (right_anti_dual[e23] * wedge[e41]) - (right_anti_dual[e31] * wedge[e42]) - (right_anti_dual[e12] * wedge[e43])));
    }
}
impl AntiRejectOrthogonallyFrom<MultiVector> for Motor {
    type Output = MultiVector;
// Operative Statistics for this implementation:
//           add/sub      mul      div
//      f32       38       53        0
//    simd3        8       15        0
//    simd4        7        7        0
// Totals...
// yes simd       53       75        0
//  no simd       90      126        0
    fn anti_reject_orthogonally_from(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
let wedge = MultiVector::from_groups(/* scalar, e1234 */float2(self[scalar] * other[scalar], (self[e1234] * other[scalar]) + (self[scalar] * other[e1234]) - (self[e41] * other[e23]) - (self[e42] * other[e31]) - (self[e43] * other[e12]) - (self[e23] * other[e41]) - (self[e31] * other[e42]) - (self[e12] * other[e43])), /* e1, e2, e3, e4 */float4(self[scalar]) * other.group1(), /* e41, e42, e43 */(float3(self[scalar]) * other.group2()) + (float3(other[scalar]) * self.group0().xyz), /* e23, e31, e12 */(float3(self[scalar]) * other.group3()) + (float3(other[scalar]) * self.group1().xyz), /* e423, e431, e412, e321 */float4((self[e23] * other[e4]) + (self[scalar] * other[e423]), (self[e31] * other[e4]) + (self[scalar] * other[e431]), (self[e12] * other[e4]) + (self[scalar] * other[e412]), -(self[e31] * other[e2]) - (self[e12] * other[e3])) + float4((self.group0().yzx * other.group1().zxy), self[scalar] * other[e321]) - (other.group1().yzxx * float4(self.group0().zxy, self[e23])));
let right_anti_dual = MultiVector::from_groups(/* scalar, e1234 */float2(0.0, other[scalar]), /* e1, e2, e3, e4 */float4(float3(0.0), other[e321] * -1.0), /* e41, e42, e43 */other.group3() * float3(-1.0), /* e23, e31, e12 */float3(0.0), /* e423, e431, e412, e321 */float4(other[e1], other[e2], other[e3], 0.0));
        return MultiVector::from_groups(/* scalar, e1234 */float2((right_anti_dual[scalar] * wedge[e1234]) + (right_anti_dual[e1234] * wedge[scalar]) + (right_anti_dual[e423] * wedge[e1]) + (right_anti_dual[e431] * wedge[e2]) + (right_anti_dual[e412] * wedge[e3]) + (right_anti_dual[e321] * wedge[e4]) - (right_anti_dual[e1] * wedge[e423]) - (right_anti_dual[e2] * wedge[e431]) - (right_anti_dual[e3] * wedge[e412]) - (right_anti_dual[e4] * wedge[e321]) - (right_anti_dual[e41] * wedge[e23]) - (right_anti_dual[e42] * wedge[e31]) - (right_anti_dual[e43] * wedge[e12]) - (right_anti_dual[e23] * wedge[e41]) - (right_anti_dual[e31] * wedge[e42]) - (right_anti_dual[e12] * wedge[e43]), right_anti_dual[e1234] * wedge[e1234]), /* e1, e2, e3, e4 */float4((right_anti_dual[e41] * wedge[e321]) + (right_anti_dual[e31] * wedge[e412]) + (right_anti_dual[e412] * wedge[e31]) + (right_anti_dual[e321] * wedge[e41]), (right_anti_dual[e42] * wedge[e321]) + (right_anti_dual[e12] * wedge[e423]) + (right_anti_dual[e423] * wedge[e12]) + (right_anti_dual[e321] * wedge[e42]), (right_anti_dual[e43] * wedge[e321]) + (right_anti_dual[e23] * wedge[e431]) + (right_anti_dual[e431] * wedge[e23]) + (right_anti_dual[e321] * wedge[e43]), -(right_anti_dual[e43] * wedge[e412]) - (right_anti_dual[e423] * wedge[e41]) - (right_anti_dual[e431] * wedge[e42]) - (right_anti_dual[e412] * wedge[e43])) + (float4(right_anti_dual[e1234]) * wedge.group1()) + (float4(wedge[e1234]) * right_anti_dual.group1()) - (wedge.group4().yzxx * float4(right_anti_dual.group3().zxy, right_anti_dual[e41])) - float4((wedge.group3().zxy * right_anti_dual.group4().yzx), right_anti_dual[e42] * wedge[e431]), /* e41, e42, e43 */(float3(right_anti_dual[e1234]) * wedge.group2()) + (float3(wedge[e1234]) * right_anti_dual.group2()) + (right_anti_dual.group4().yzx * wedge.group4().zxy) - (right_anti_dual.group4().zxy * wedge.group4().yzx), /* e23, e31, e12 */(float3(right_anti_dual[e1234]) * wedge.group3()) + (float3(right_anti_dual[e321]) * wedge.group4().xyz) + (float3(wedge[e1234]) * right_anti_dual.group3()) - (float3(wedge[e321]) * right_anti_dual.group4().xyz), /* e423, e431, e412, e321 */(float4(right_anti_dual[e1234]) * wedge.group4()) + (float4(wedge[e1234]) * right_anti_dual.group4()));
    }
}
impl AntiRejectOrthogonallyFrom<Plane> for Motor {
    type Output = Scalar;
    fn anti_reject_orthogonally_from(self, other: Plane) -> Self::Output {
        use crate::elements::*;
        return Scalar::from_groups(/* scalar */self[scalar] * powi(other[e321], 2));
    }
}
impl AntiRejectOrthogonallyFrom<Point> for Motor {
    type Output = Motor;
// Operative Statistics for this implementation:
//           add/sub      mul      div
//      f32        6       17        0
//    simd3        1        2        0
//    simd4        2        3        0
// Totals...
// yes simd        9       22        0
//  no simd       17       35        0
    fn anti_reject_orthogonally_from(self, other: Point) -> Self::Output {
        use crate::elements::*;
let wedge = Flector::from_groups(/* e1, e2, e3, e4 */float4(self[scalar]) * other.group0(), /* e423, e431, e412, e321 */float4((self[e42] * other[e3]) + (self[e23] * other[e4]), (self[e43] * other[e1]) + (self[e31] * other[e4]), (self[e41] * other[e2]) + (self[e12] * other[e4]), -(self[e31] * other[e2]) - (self[e12] * other[e3])) - (other.group0().yzxx * float4(self.group0().zxy, self[e23])));
let right_anti_dual = Plane::from_groups(/* e423, e431, e412, e321 */float4(other[e1], other[e2], other[e3], 0.0));
        return Motor::from_groups(/* e41, e42, e43, e1234 */float4(((wedge.group1().zxy * right_anti_dual.group0().yzx) - (wedge.group1().yzx * right_anti_dual.group0().zxy)), 0.0), /* e23, e31, e12, scalar */float4(wedge[e321] * right_anti_dual[e423] * -1.0, wedge[e321] * right_anti_dual[e431] * -1.0, wedge[e321] * right_anti_dual[e412] * -1.0, (wedge[e2] * right_anti_dual[e431]) + (wedge[e3] * right_anti_dual[e412]) + (wedge[e4] * right_anti_dual[e321])) + (right_anti_dual.group0().wwwx * float4(wedge.group1().xyz, wedge[e1])));
    }
}
impl AntiRejectOrthogonallyFrom<Scalar> for Motor {
    type Output = Motor;
// Operative Statistics for this implementation:
//          add/sub      mul      div
//   simd4        0        4        0
// no simd        0       16        0
    fn anti_reject_orthogonally_from(self, other: Scalar) -> Self::Output {
        use crate::elements::*;
let wedge = Motor::from_groups(/* e41, e42, e43, e1234 */float4(other[scalar]) * self.group0(), /* e23, e31, e12, scalar */float4(other[scalar]) * self.group1());
let right_anti_dual = AntiScalar::from_groups(/* e1234 */other[scalar]);
        return Motor::from_groups(/* e41, e42, e43, e1234 */float4(right_anti_dual[e1234]) * wedge.group0(), /* e23, e31, e12, scalar */float4(right_anti_dual[e1234]) * wedge.group1());
    }
}
impl std::ops::Div<anti_reject_orthogonally_from> for MultiVector {
    type Output = anti_reject_orthogonally_from_partial<MultiVector>;
    fn div(self, _rhs: anti_reject_orthogonally_from) -> Self::Output {
        anti_reject_orthogonally_from_partial(self)
    }
}
impl AntiRejectOrthogonallyFrom<DualNum> for MultiVector {
    type Output = MultiVector;
// Operative Statistics for this implementation:
//           add/sub      mul      div
//      f32        1        3        0
//    simd2        0        1        0
//    simd3        0        4        0
//    simd4        0        4        0
// Totals...
// yes simd        1       12        0
//  no simd        1       33        0
    fn anti_reject_orthogonally_from(self, other: DualNum) -> Self::Output {
        use crate::elements::*;
let wedge = MultiVector::from_groups(/* scalar, e1234 */float2(other[scalar] * self[scalar], (other[scalar] * self[e1234]) + (other[e1234] * self[scalar])), /* e1, e2, e3, e4 */float4(other[scalar]) * self.group1(), /* e41, e42, e43 */float3(other[scalar]) * self.group2(), /* e23, e31, e12 */float3(other[scalar]) * self.group3(), /* e423, e431, e412, e321 */float4(other[scalar]) * self.group4());
let right_anti_dual = AntiScalar::from_groups(/* e1234 */other[scalar]);
        return MultiVector::from_groups(/* scalar, e1234 */float2(right_anti_dual[e1234]) * wedge.group0(), /* e1, e2, e3, e4 */float4(right_anti_dual[e1234]) * wedge.group1(), /* e41, e42, e43 */float3(right_anti_dual[e1234]) * wedge.group2(), /* e23, e31, e12 */float3(right_anti_dual[e1234]) * wedge.group3(), /* e423, e431, e412, e321 */float4(right_anti_dual[e1234]) * wedge.group4());
    }
}
impl AntiRejectOrthogonallyFrom<Flector> for MultiVector {
    type Output = MultiVector;
// Operative Statistics for this implementation:
//           add/sub      mul      div
//      f32       22       33        0
//    simd3        4        8        0
//    simd4        4        8        0
// Totals...
// yes simd       30       49        0
//  no simd       50       89        0
    fn anti_reject_orthogonally_from(self, other: Flector) -> Self::Output {
        use crate::elements::*;
let wedge = MultiVector::from_groups(/* scalar, e1234 */float2(0.0, (other[e423] * self[e1]) + (other[e431] * self[e2]) + (other[e412] * self[e3]) + (other[e321] * self[e4]) - (other[e1] * self[e423]) - (other[e2] * self[e431]) - (other[e3] * self[e412]) - (other[e4] * self[e321])), /* e1, e2, e3, e4 */float4(self[scalar]) * other.group0(), /* e41, e42, e43 */(float3(self[e4]) * other.group0().xyz) - (float3(other[e4]) * self.group1().xyz), /* e23, e31, e12 */(other.group0().zxy * self.group1().yzx) - (other.group0().yzx * self.group1().zxy), /* e423, e431, e412, e321 */float4((other[e3] * self[e42]) + (other[e4] * self[e23]), (other[e1] * self[e43]) + (other[e4] * self[e31]), (other[e2] * self[e41]) + (other[e4] * self[e12]), -(other[e2] * self[e31]) - (other[e3] * self[e12])) + (float4(self[scalar]) * other.group1()) - (other.group0().yzxx * float4(self.group2().zxy, self[e23])));
let right_anti_dual = Flector::from_groups(/* e1, e2, e3, e4 */float4(float3(0.0), other[e321] * -1.0), /* e423, e431, e412, e321 */float4(other[e1], other[e2], other[e3], 0.0));
        return MultiVector::from_groups(/* scalar, e1234 */float2((right_anti_dual[e423] * wedge[e1]) + (right_anti_dual[e431] * wedge[e2]) + (right_anti_dual[e412] * wedge[e3]) + (right_anti_dual[e321] * wedge[e4]) - (right_anti_dual[e1] * wedge[e423]) - (right_anti_dual[e2] * wedge[e431]) - (right_anti_dual[e3] * wedge[e412]) - (right_anti_dual[e4] * wedge[e321]), 0.0), /* e1, e2, e3, e4 */float4((right_anti_dual[e412] * wedge[e31]) + (right_anti_dual[e321] * wedge[e41]), (right_anti_dual[e423] * wedge[e12]) + (right_anti_dual[e321] * wedge[e42]), (right_anti_dual[e431] * wedge[e23]) + (right_anti_dual[e321] * wedge[e43]), -(right_anti_dual[e431] * wedge[e42]) - (right_anti_dual[e412] * wedge[e43])) + (float4(wedge[e1234]) * right_anti_dual.group0()) - (right_anti_dual.group1().yzxx * float4(wedge.group3().zxy, wedge[e41])), /* e41, e42, e43 */(right_anti_dual.group1().yzx * wedge.group4().zxy) - (right_anti_dual.group1().zxy * wedge.group4().yzx), /* e23, e31, e12 */(float3(right_anti_dual[e321]) * wedge.group4().xyz) - (float3(wedge[e321]) * right_anti_dual.group1().xyz), /* e423, e431, e412, e321 */float4(wedge.group0().yy, wedge[e1234], 0.0) * float4(float3(1.0), 0.0) * float4(right_anti_dual.group1().xyz, 0.0) * float4(1.0, 1.0, 1.0, 0.0));
    }
}
impl AntiRejectOrthogonallyFrom<Horizon> for MultiVector {
    type Output = MultiVector;
// Operative Statistics for this implementation:
//           add/sub      mul      div
//      f32        0        5        0
//    simd2        0        2        0
// Totals...
// yes simd        0        7        0
//  no simd        0        9        0
    fn anti_reject_orthogonally_from(self, other: Horizon) -> Self::Output {
        use crate::elements::*;
let wedge = MultiVector::from_groups(/* scalar, e1234 */float2(1.0, other[e321] * self[e4]) * float2(0.0, 1.0), /* e1, e2, e3, e4 */float4(0.0), /* e41, e42, e43 */float3(0.0), /* e23, e31, e12 */float3(0.0), /* e423, e431, e412, e321 */float4(float3(0.0), other[e321] * self[scalar]));
let right_anti_dual = Origin::from_groups(/* e4 */other[e321] * -1.0);
        return MultiVector::from_groups(/* scalar, e1234 */float2(wedge[e321] * right_anti_dual[e4], 1.0) * float2(-1.0, 0.0), /* e1, e2, e3, e4 */float4(float3(0.0), wedge[e1234] * right_anti_dual[e4]), /* e41, e42, e43 */float3(0.0), /* e23, e31, e12 */float3(0.0), /* e423, e431, e412, e321 */float4(0.0));
    }
}
impl AntiRejectOrthogonallyFrom<Line> for MultiVector {
    type Output = MultiVector;
// Operative Statistics for this implementation:
//           add/sub      mul      div
//      f32       18       28        0
//    simd3        0        4        0
//    simd4        2        2        0
// Totals...
// yes simd       20       34        0
//  no simd       26       48        0
    fn anti_reject_orthogonally_from(self, other: Line) -> Self::Output {
        use crate::elements::*;
let wedge = MultiVector::from_groups(/* scalar, e1234 */float2(0.0, -(other[e41] * self[e23]) - (other[e42] * self[e31]) - (other[e43] * self[e12]) - (other[e23] * self[e41]) - (other[e31] * self[e42]) - (other[e12] * self[e43])), /* e1, e2, e3, e4 */float4(0.0), /* e41, e42, e43 */float3(self[scalar]) * other.group0(), /* e23, e31, e12 */float3(self[scalar]) * other.group1(), /* e423, e431, e412, e321 */float4((other[e42] * self[e3]) + (other[e23] * self[e4]), (other[e43] * self[e1]) + (other[e31] * self[e4]), (other[e41] * self[e2]) + (other[e12] * self[e4]), -(other[e31] * self[e2]) - (other[e12] * self[e3])) - (self.group1().yzxx * float4(other.group0().zxy, other[e23])));
let right_anti_dual = Line::from_groups(/* e41, e42, e43 */other.group1() * float3(-1.0), /* e23, e31, e12 */float3(0.0));
        return MultiVector::from_groups(/* scalar, e1234 */float2(-(right_anti_dual[e41] * wedge[e23]) - (right_anti_dual[e42] * wedge[e31]) - (right_anti_dual[e43] * wedge[e12]) - (right_anti_dual[e23] * wedge[e41]) - (right_anti_dual[e31] * wedge[e42]) - (right_anti_dual[e12] * wedge[e43]), 0.0), /* e1, e2, e3, e4 */float4((right_anti_dual[e41] * wedge[e321]) + (right_anti_dual[e31] * wedge[e412]), (right_anti_dual[e42] * wedge[e321]) + (right_anti_dual[e12] * wedge[e423]), (right_anti_dual[e43] * wedge[e321]) + (right_anti_dual[e23] * wedge[e431]), -(right_anti_dual[e42] * wedge[e431]) - (right_anti_dual[e43] * wedge[e412])) - (wedge.group4().yzxx * float4(right_anti_dual.group1().zxy, right_anti_dual[e41])), /* e41, e42, e43 */float3(wedge[e1234]) * right_anti_dual.group0(), /* e23, e31, e12 */float3(0.0), /* e423, e431, e412, e321 */float4(0.0));
    }
}
impl AntiRejectOrthogonallyFrom<Motor> for MultiVector {
    type Output = MultiVector;
// Operative Statistics for this implementation:
//           add/sub      mul      div
//      f32       22       35        0
//    simd3        4        9        0
//    simd4        4        6        0
// Totals...
// yes simd       30       50        0
//  no simd       50       86        0
    fn anti_reject_orthogonally_from(self, other: Motor) -> Self::Output {
        use crate::elements::*;
let wedge = MultiVector::from_groups(/* scalar, e1234 */float2(other[scalar] * self[scalar], (other[e1234] * self[scalar]) + (other[scalar] * self[e1234]) - (other[e41] * self[e23]) - (other[e42] * self[e31]) - (other[e43] * self[e12]) - (other[e23] * self[e41]) - (other[e31] * self[e42]) - (other[e12] * self[e43])), /* e1, e2, e3, e4 */float4(other[scalar]) * self.group1(), /* e41, e42, e43 */(float3(other[scalar]) * self.group2()) + (float3(self[scalar]) * other.group0().xyz), /* e23, e31, e12 */(float3(other[scalar]) * self.group3()) + (float3(self[scalar]) * other.group1().xyz), /* e423, e431, e412, e321 */float4((other[e23] * self[e4]) + (other[scalar] * self[e423]), (other[e31] * self[e4]) + (other[scalar] * self[e431]), (other[e12] * self[e4]) + (other[scalar] * self[e412]), -(other[e31] * self[e2]) - (other[e12] * self[e3])) + float4((other.group0().yzx * self.group1().zxy), other[scalar] * self[e321]) - (self.group1().yzxx * float4(other.group0().zxy, other[e23])));
let right_anti_dual = Motor::from_groups(/* e41, e42, e43, e1234 */other.group1() * float4(-1.0, -1.0, -1.0, 1.0), /* e23, e31, e12, scalar */float4(0.0));
        return MultiVector::from_groups(/* scalar, e1234 */float2((right_anti_dual[e1234] * wedge[scalar]) + (right_anti_dual[scalar] * wedge[e1234]) - (right_anti_dual[e41] * wedge[e23]) - (right_anti_dual[e42] * wedge[e31]) - (right_anti_dual[e43] * wedge[e12]) - (right_anti_dual[e23] * wedge[e41]) - (right_anti_dual[e31] * wedge[e42]) - (right_anti_dual[e12] * wedge[e43]), right_anti_dual[e1234] * wedge[e1234]), /* e1, e2, e3, e4 */float4((right_anti_dual[e1234] * wedge[e1]) + (right_anti_dual[e31] * wedge[e412]), (right_anti_dual[e1234] * wedge[e2]) + (right_anti_dual[e12] * wedge[e423]), (right_anti_dual[e1234] * wedge[e3]) + (right_anti_dual[e23] * wedge[e431]), -(right_anti_dual[e42] * wedge[e431]) - (right_anti_dual[e43] * wedge[e412])) + (right_anti_dual.group0() * float4(wedge.group4().www, wedge[e4])) - (wedge.group4().yzxx * float4(right_anti_dual.group1().zxy, right_anti_dual[e41])), /* e41, e42, e43 */(float3(right_anti_dual[e1234]) * wedge.group2()) + (float3(wedge[e1234]) * right_anti_dual.group0().xyz), /* e23, e31, e12 */(float3(right_anti_dual[e1234]) * wedge.group3()) + (float3(wedge[e1234]) * right_anti_dual.group1().xyz), /* e423, e431, e412, e321 */float4(right_anti_dual[e1234]) * wedge.group4());
    }
}
impl AntiRejectOrthogonallyFrom<MultiVector> for MultiVector {
    type Output = MultiVector;
// Operative Statistics for this implementation:
//           add/sub      mul      div
//      f32       54       69        0
//    simd3       12       19        0
//    simd4       10       10        0
// Totals...
// yes simd       76       98        0
//  no simd      130      166        0
    fn anti_reject_orthogonally_from(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
let wedge = MultiVector::from_groups(/* scalar, e1234 */float2(other[scalar] * self[scalar], (other[scalar] * self[e1234]) + (other[e1234] * self[scalar]) + (other[e423] * self[e1]) + (other[e431] * self[e2]) + (other[e412] * self[e3]) + (other[e321] * self[e4]) - (other[e1] * self[e423]) - (other[e2] * self[e431]) - (other[e3] * self[e412]) - (other[e4] * self[e321]) - (other[e41] * self[e23]) - (other[e42] * self[e31]) - (other[e43] * self[e12]) - (other[e23] * self[e41]) - (other[e31] * self[e42]) - (other[e12] * self[e43])), /* e1, e2, e3, e4 */(float4(other[scalar]) * self.group1()) + (float4(self[scalar]) * other.group1()), /* e41, e42, e43 */(float3(other[scalar]) * self.group2()) + (float3(self[scalar]) * other.group2()) + (float3(self[e4]) * other.group1().xyz) - (float3(other[e4]) * self.group1().xyz), /* e23, e31, e12 */(float3(other[scalar]) * self.group3()) + (float3(self[scalar]) * other.group3()) + (other.group1().zxy * self.group1().yzx) - (other.group1().yzx * self.group1().zxy), /* e423, e431, e412, e321 */float4((other[e3] * self[e42]) + (other[e4] * self[e23]) + (other[e42] * self[e3]) + (other[e23] * self[e4]), (other[e1] * self[e43]) + (other[e4] * self[e31]) + (other[e43] * self[e1]) + (other[e31] * self[e4]), (other[e2] * self[e41]) + (other[e4] * self[e12]) + (other[e41] * self[e2]) + (other[e12] * self[e4]), -(other[e1] * self[e23]) - (other[e2] * self[e31]) - (other[e3] * self[e12]) - (other[e12] * self[e3])) + (float4(other[scalar]) * self.group4()) + (float4(self[scalar]) * other.group4()) - (self.group1().yzxx * float4(other.group2().zxy, other[e23])) - float4((self.group2().zxy * other.group1().yzx), other[e31] * self[e2]));
let right_anti_dual = MultiVector::from_groups(/* scalar, e1234 */float2(0.0, other[scalar]), /* e1, e2, e3, e4 */float4(float3(0.0), other[e321] * -1.0), /* e41, e42, e43 */other.group3() * float3(-1.0), /* e23, e31, e12 */float3(0.0), /* e423, e431, e412, e321 */float4(other[e1], other[e2], other[e3], 0.0));
        return MultiVector::from_groups(/* scalar, e1234 */float2((right_anti_dual[scalar] * wedge[e1234]) + (right_anti_dual[e1234] * wedge[scalar]) + (right_anti_dual[e423] * wedge[e1]) + (right_anti_dual[e431] * wedge[e2]) + (right_anti_dual[e412] * wedge[e3]) + (right_anti_dual[e321] * wedge[e4]) - (right_anti_dual[e1] * wedge[e423]) - (right_anti_dual[e2] * wedge[e431]) - (right_anti_dual[e3] * wedge[e412]) - (right_anti_dual[e4] * wedge[e321]) - (right_anti_dual[e41] * wedge[e23]) - (right_anti_dual[e42] * wedge[e31]) - (right_anti_dual[e43] * wedge[e12]) - (right_anti_dual[e23] * wedge[e41]) - (right_anti_dual[e31] * wedge[e42]) - (right_anti_dual[e12] * wedge[e43]), right_anti_dual[e1234] * wedge[e1234]), /* e1, e2, e3, e4 */float4((right_anti_dual[e41] * wedge[e321]) + (right_anti_dual[e31] * wedge[e412]) + (right_anti_dual[e412] * wedge[e31]) + (right_anti_dual[e321] * wedge[e41]), (right_anti_dual[e42] * wedge[e321]) + (right_anti_dual[e12] * wedge[e423]) + (right_anti_dual[e423] * wedge[e12]) + (right_anti_dual[e321] * wedge[e42]), (right_anti_dual[e43] * wedge[e321]) + (right_anti_dual[e23] * wedge[e431]) + (right_anti_dual[e431] * wedge[e23]) + (right_anti_dual[e321] * wedge[e43]), -(right_anti_dual[e43] * wedge[e412]) - (right_anti_dual[e423] * wedge[e41]) - (right_anti_dual[e431] * wedge[e42]) - (right_anti_dual[e412] * wedge[e43])) + (float4(right_anti_dual[e1234]) * wedge.group1()) + (float4(wedge[e1234]) * right_anti_dual.group1()) - (wedge.group4().yzxx * float4(right_anti_dual.group3().zxy, right_anti_dual[e41])) - float4((wedge.group3().zxy * right_anti_dual.group4().yzx), right_anti_dual[e42] * wedge[e431]), /* e41, e42, e43 */(float3(right_anti_dual[e1234]) * wedge.group2()) + (float3(wedge[e1234]) * right_anti_dual.group2()) + (right_anti_dual.group4().yzx * wedge.group4().zxy) - (right_anti_dual.group4().zxy * wedge.group4().yzx), /* e23, e31, e12 */(float3(right_anti_dual[e1234]) * wedge.group3()) + (float3(right_anti_dual[e321]) * wedge.group4().xyz) + (float3(wedge[e1234]) * right_anti_dual.group3()) - (float3(wedge[e321]) * right_anti_dual.group4().xyz), /* e423, e431, e412, e321 */(float4(right_anti_dual[e1234]) * wedge.group4()) + (float4(wedge[e1234]) * right_anti_dual.group4()));
    }
}
impl AntiRejectOrthogonallyFrom<Plane> for MultiVector {
    type Output = MultiVector;
// Operative Statistics for this implementation:
//           add/sub      mul      div
//      f32        3        7        0
//    simd2        0        1        0
//    simd4        0        1        0
// Totals...
// yes simd        3        9        0
//  no simd        3       13        0
    fn anti_reject_orthogonally_from(self, other: Plane) -> Self::Output {
        use crate::elements::*;
let wedge = MultiVector::from_groups(/* scalar, e1234 */float2(0.0, (self[e1] * other[e423]) + (self[e2] * other[e431]) + (self[e3] * other[e412]) + (self[e4] * other[e321])), /* e1, e2, e3, e4 */float4(0.0), /* e41, e42, e43 */float3(0.0), /* e23, e31, e12 */float3(0.0), /* e423, e431, e412, e321 */float4(self[scalar]) * other.group0());
let right_anti_dual = Origin::from_groups(/* e4 */other[e321] * -1.0);
        return MultiVector::from_groups(/* scalar, e1234 */float2(wedge[e321] * right_anti_dual[e4], 1.0) * float2(-1.0, 0.0), /* e1, e2, e3, e4 */float4(float3(0.0), wedge[e1234] * right_anti_dual[e4]), /* e41, e42, e43 */float3(0.0), /* e23, e31, e12 */float3(0.0), /* e423, e431, e412, e321 */float4(0.0));
    }
}
impl AntiRejectOrthogonallyFrom<Point> for MultiVector {
    type Output = MultiVector;
// Operative Statistics for this implementation:
//           add/sub      mul      div
//      f32       14       24        0
//    simd3        4        8        0
//    simd4        2        6        0
// Totals...
// yes simd       20       38        0
//  no simd       34       72        0
    fn anti_reject_orthogonally_from(self, other: Point) -> Self::Output {
        use crate::elements::*;
let wedge = MultiVector::from_groups(/* scalar, e1234 */float2(0.0, -(self[e423] * other[e1]) - (self[e431] * other[e2]) - (self[e412] * other[e3]) - (self[e321] * other[e4])), /* e1, e2, e3, e4 */float4(self[scalar]) * other.group0(), /* e41, e42, e43 */(float3(self[e4]) * other.group0().xyz) - (float3(other[e4]) * self.group1().xyz), /* e23, e31, e12 */(self.group1().yzx * other.group0().zxy) - (self.group1().zxy * other.group0().yzx), /* e423, e431, e412, e321 */float4((self[e42] * other[e3]) + (self[e23] * other[e4]), (self[e43] * other[e1]) + (self[e31] * other[e4]), (self[e41] * other[e2]) + (self[e12] * other[e4]), -(self[e31] * other[e2]) - (self[e12] * other[e3])) - (other.group0().yzxx * float4(self.group2().zxy, self[e23])));
let right_anti_dual = Plane::from_groups(/* e423, e431, e412, e321 */float4(other[e1], other[e2], other[e3], 0.0));
        return MultiVector::from_groups(/* scalar, e1234 */float2((wedge[e1] * right_anti_dual[e423]) + (wedge[e2] * right_anti_dual[e431]) + (wedge[e3] * right_anti_dual[e412]) + (wedge[e4] * right_anti_dual[e321]), 0.0), /* e1, e2, e3, e4 */float4((wedge[e41] * right_anti_dual[e321]) + (wedge[e31] * right_anti_dual[e412]), (wedge[e42] * right_anti_dual[e321]) + (wedge[e12] * right_anti_dual[e423]), (wedge[e43] * right_anti_dual[e321]) + (wedge[e23] * right_anti_dual[e431]), -(wedge[e42] * right_anti_dual[e431]) - (wedge[e43] * right_anti_dual[e412])) - (right_anti_dual.group0().yzxx * float4(wedge.group3().zxy, wedge[e41])), /* e41, e42, e43 */(wedge.group4().zxy * right_anti_dual.group0().yzx) - (wedge.group4().yzx * right_anti_dual.group0().zxy), /* e23, e31, e12 */(float3(right_anti_dual[e321]) * wedge.group4().xyz) - (float3(wedge[e321]) * right_anti_dual.group0().xyz), /* e423, e431, e412, e321 */float4(wedge.group0().yy, wedge[e1234], 0.0) * float4(float3(1.0), 0.0) * float4(right_anti_dual.group0().xyz, 0.0) * float4(1.0, 1.0, 1.0, 0.0));
    }
}
impl AntiRejectOrthogonallyFrom<Scalar> for MultiVector {
    type Output = MultiVector;
// Operative Statistics for this implementation:
//           add/sub      mul      div
//    simd2        0        2        0
//    simd3        0        4        0
//    simd4        0        4        0
// Totals...
// yes simd        0       10        0
//  no simd        0       32        0
    fn anti_reject_orthogonally_from(self, other: Scalar) -> Self::Output {
        use crate::elements::*;
let wedge = MultiVector::from_groups(/* scalar, e1234 */float2(other[scalar]) * self.group0(), /* e1, e2, e3, e4 */float4(other[scalar]) * self.group1(), /* e41, e42, e43 */float3(other[scalar]) * self.group2(), /* e23, e31, e12 */float3(other[scalar]) * self.group3(), /* e423, e431, e412, e321 */float4(other[scalar]) * self.group4());
let right_anti_dual = AntiScalar::from_groups(/* e1234 */other[scalar]);
        return MultiVector::from_groups(/* scalar, e1234 */float2(right_anti_dual[e1234]) * wedge.group0(), /* e1, e2, e3, e4 */float4(right_anti_dual[e1234]) * wedge.group1(), /* e41, e42, e43 */float3(right_anti_dual[e1234]) * wedge.group2(), /* e23, e31, e12 */float3(right_anti_dual[e1234]) * wedge.group3(), /* e423, e431, e412, e321 */float4(right_anti_dual[e1234]) * wedge.group4());
    }
}
impl std::ops::Div<anti_reject_orthogonally_from> for Origin {
    type Output = anti_reject_orthogonally_from_partial<Origin>;
    fn div(self, _rhs: anti_reject_orthogonally_from) -> Self::Output {
        anti_reject_orthogonally_from_partial(self)
    }
}
impl AntiRejectOrthogonallyFrom<DualNum> for Origin {
    type Output = Origin;
// Operative Statistics for this implementation:
//      add/sub      mul      div
// f32        0        1        0
    fn anti_reject_orthogonally_from(self, other: DualNum) -> Self::Output {
        use crate::elements::*;
        return Origin::from_groups(/* e4 */powi(other[scalar], 2) * self[e4]);
    }
}
impl AntiRejectOrthogonallyFrom<Flector> for Origin {
    type Output = Flector;
// Operative Statistics for this implementation:
//           add/sub      mul      div
//      f32        4        9        0
//    simd4        2        4        0
// Totals...
// yes simd        6       13        0
//  no simd       12       25        0
    fn anti_reject_orthogonally_from(self, other: Flector) -> Self::Output {
        use crate::elements::*;
let wedge = Motor::from_groups(/* e41, e42, e43, e1234 */float4(self[e4]) * float4(other.group0().xyz, other[e321]), /* e23, e31, e12, scalar */float4(0.0));
let right_anti_dual = Flector::from_groups(/* e1, e2, e3, e4 */float4(float3(0.0), other[e321] * -1.0), /* e423, e431, e412, e321 */float4(other[e1], other[e2], other[e3], 0.0));
        return Flector::from_groups(/* e1, e2, e3, e4 */float4((right_anti_dual[e412] * wedge[e31]) + (right_anti_dual[e321] * wedge[e41]), (right_anti_dual[e423] * wedge[e12]) + (right_anti_dual[e321] * wedge[e42]), (right_anti_dual[e431] * wedge[e23]) + (right_anti_dual[e321] * wedge[e43]), -(right_anti_dual[e431] * wedge[e42]) - (right_anti_dual[e412] * wedge[e43])) + (float4(wedge[e1234]) * right_anti_dual.group0()) - (right_anti_dual.group1().yzxx * float4(wedge.group1().zxy, wedge[e41])), /* e423, e431, e412, e321 */float4(wedge[e1234]) * right_anti_dual.group1());
    }
}
impl AntiRejectOrthogonallyFrom<Horizon> for Origin {
    type Output = Origin;
// Operative Statistics for this implementation:
//      add/sub      mul      div
// f32        0        2        0
    fn anti_reject_orthogonally_from(self, other: Horizon) -> Self::Output {
        use crate::elements::*;
        return Origin::from_groups(/* e4 */powi(other[e321], 2) * self[e4] * -1.0);
    }
}
impl AntiRejectOrthogonallyFrom<Line> for Origin {
    type Output = Origin;
// Operative Statistics for this implementation:
//           add/sub      mul      div
//      f32        2        3        0
//    simd3        0        1        0
//    simd4        0        2        0
// Totals...
// yes simd        2        6        0
//  no simd        2       14        0
    fn anti_reject_orthogonally_from(self, other: Line) -> Self::Output {
        use crate::elements::*;
let wedge = Plane::from_groups(/* e423, e431, e412, e321 */float4(self[e4], self[e4], self[e4], 0.0) * float4(other.group1(), 0.0) * float4(1.0, 1.0, 1.0, 0.0));
let right_anti_dual = Line::from_groups(/* e41, e42, e43 */other.group1() * float3(-1.0), /* e23, e31, e12 */float3(0.0));
        return Origin::from_groups(/* e4 */-(right_anti_dual[e41] * wedge[e423]) - (right_anti_dual[e42] * wedge[e431]) - (right_anti_dual[e43] * wedge[e412]));
    }
}
impl AntiRejectOrthogonallyFrom<Motor> for Origin {
    type Output = Flector;
// Operative Statistics for this implementation:
//           add/sub      mul      div
//      f32        4        9        0
//    simd4        2        6        0
// Totals...
// yes simd        6       15        0
//  no simd       12       33        0
    fn anti_reject_orthogonally_from(self, other: Motor) -> Self::Output {
        use crate::elements::*;
let wedge = Flector::from_groups(/* e1, e2, e3, e4 */float4(float3(0.0), other[scalar] * self[e4]), /* e423, e431, e412, e321 */float4(self[e4], self[e4], self[e4], 0.0) * float4(other.group1().xyz, 0.0) * float4(1.0, 1.0, 1.0, 0.0));
let right_anti_dual = Motor::from_groups(/* e41, e42, e43, e1234 */other.group1() * float4(-1.0, -1.0, -1.0, 1.0), /* e23, e31, e12, scalar */float4(0.0));
        return Flector::from_groups(/* e1, e2, e3, e4 */float4((wedge[e412] * right_anti_dual[e31]) + (wedge[e321] * right_anti_dual[e41]), (wedge[e423] * right_anti_dual[e12]) + (wedge[e321] * right_anti_dual[e42]), (wedge[e431] * right_anti_dual[e23]) + (wedge[e321] * right_anti_dual[e43]), -(wedge[e431] * right_anti_dual[e42]) - (wedge[e412] * right_anti_dual[e43])) + (float4(right_anti_dual[e1234]) * wedge.group0()) - (wedge.group1().yzxx * float4(right_anti_dual.group1().zxy, right_anti_dual[e41])), /* e423, e431, e412, e321 */float4(right_anti_dual[e1234]) * wedge.group1());
    }
}
impl AntiRejectOrthogonallyFrom<MultiVector> for Origin {
    type Output = MultiVector;
// Operative Statistics for this implementation:
//           add/sub      mul      div
//      f32       27       37        0
//    simd2        0        1        0
//    simd3        4        9        0
//    simd4        4        5        0
// Totals...
// yes simd       35       52        0
//  no simd       55       86        0
    fn anti_reject_orthogonally_from(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
let wedge = MultiVector::from_groups(/* scalar, e1234 */float2(1.0, other[e321] * self[e4]) * float2(0.0, 1.0), /* e1, e2, e3, e4 */float4(float3(0.0), other[scalar] * self[e4]), /* e41, e42, e43 */float3(self[e4]) * other.group1().xyz, /* e23, e31, e12 */float3(0.0), /* e423, e431, e412, e321 */float4(self[e4], self[e4], self[e4], 0.0) * float4(other.group3(), 0.0) * float4(1.0, 1.0, 1.0, 0.0));
let right_anti_dual = MultiVector::from_groups(/* scalar, e1234 */float2(0.0, other[scalar]), /* e1, e2, e3, e4 */float4(float3(0.0), other[e321] * -1.0), /* e41, e42, e43 */other.group3() * float3(-1.0), /* e23, e31, e12 */float3(0.0), /* e423, e431, e412, e321 */float4(other[e1], other[e2], other[e3], 0.0));
        return MultiVector::from_groups(/* scalar, e1234 */float2((right_anti_dual[scalar] * wedge[e1234]) + (right_anti_dual[e1234] * wedge[scalar]) + (right_anti_dual[e423] * wedge[e1]) + (right_anti_dual[e431] * wedge[e2]) + (right_anti_dual[e412] * wedge[e3]) + (right_anti_dual[e321] * wedge[e4]) - (right_anti_dual[e1] * wedge[e423]) - (right_anti_dual[e2] * wedge[e431]) - (right_anti_dual[e3] * wedge[e412]) - (right_anti_dual[e4] * wedge[e321]) - (right_anti_dual[e41] * wedge[e23]) - (right_anti_dual[e42] * wedge[e31]) - (right_anti_dual[e43] * wedge[e12]) - (right_anti_dual[e23] * wedge[e41]) - (right_anti_dual[e31] * wedge[e42]) - (right_anti_dual[e12] * wedge[e43]), right_anti_dual[e1234] * wedge[e1234]), /* e1, e2, e3, e4 */float4((right_anti_dual[e41] * wedge[e321]) + (right_anti_dual[e31] * wedge[e412]) + (right_anti_dual[e412] * wedge[e31]) + (right_anti_dual[e321] * wedge[e41]), (right_anti_dual[e42] * wedge[e321]) + (right_anti_dual[e12] * wedge[e423]) + (right_anti_dual[e423] * wedge[e12]) + (right_anti_dual[e321] * wedge[e42]), (right_anti_dual[e43] * wedge[e321]) + (right_anti_dual[e23] * wedge[e431]) + (right_anti_dual[e431] * wedge[e23]) + (right_anti_dual[e321] * wedge[e43]), -(right_anti_dual[e43] * wedge[e412]) - (right_anti_dual[e423] * wedge[e41]) - (right_anti_dual[e431] * wedge[e42]) - (right_anti_dual[e412] * wedge[e43])) + (float4(right_anti_dual[e1234]) * wedge.group1()) + (float4(wedge[e1234]) * right_anti_dual.group1()) - (wedge.group4().yzxx * float4(right_anti_dual.group3().zxy, right_anti_dual[e41])) - float4((wedge.group3().zxy * right_anti_dual.group4().yzx), right_anti_dual[e42] * wedge[e431]), /* e41, e42, e43 */(float3(right_anti_dual[e1234]) * wedge.group2()) + (float3(wedge[e1234]) * right_anti_dual.group2()) + (right_anti_dual.group4().yzx * wedge.group4().zxy) - (right_anti_dual.group4().zxy * wedge.group4().yzx), /* e23, e31, e12 */float3(0.0), /* e423, e431, e412, e321 */float4(((float3(right_anti_dual[e1234]) * wedge.group4().xyz) + (float3(wedge[e1234]) * right_anti_dual.group4().xyz)), 0.0));
    }
}
impl AntiRejectOrthogonallyFrom<Plane> for Origin {
    type Output = Origin;
// Operative Statistics for this implementation:
//      add/sub      mul      div
// f32        0        1        0
    fn anti_reject_orthogonally_from(self, other: Plane) -> Self::Output {
        use crate::elements::*;
        return Origin::from_groups(/* e4 */self[e4] * powi(other[e321], 2) * -1.0);
    }
}
impl AntiRejectOrthogonallyFrom<Point> for Origin {
    type Output = Origin;
// Operative Statistics for this implementation:
//           add/sub      mul      div
//      f32        2        3        0
//    simd3        0        1        0
// Totals...
// yes simd        2        4        0
//  no simd        2        6        0
    fn anti_reject_orthogonally_from(self, other: Point) -> Self::Output {
        use crate::elements::*;
let wedge = Line::from_groups(/* e41, e42, e43 */float3(self[e4]) * other.group0().xyz, /* e23, e31, e12 */float3(0.0));
let right_anti_dual = Plane::from_groups(/* e423, e431, e412, e321 */float4(other[e1], other[e2], other[e3], 0.0));
        return Origin::from_groups(/* e4 */-(wedge[e41] * right_anti_dual[e423]) - (wedge[e42] * right_anti_dual[e431]) - (wedge[e43] * right_anti_dual[e412]));
    }
}
impl AntiRejectOrthogonallyFrom<Scalar> for Origin {
    type Output = Origin;
    fn anti_reject_orthogonally_from(self, other: Scalar) -> Self::Output {
        use crate::elements::*;
        return Origin::from_groups(/* e4 */self[e4] * powi(other[scalar], 2));
    }
}
impl std::ops::Div<anti_reject_orthogonally_from> for Plane {
    type Output = anti_reject_orthogonally_from_partial<Plane>;
    fn div(self, _rhs: anti_reject_orthogonally_from) -> Self::Output {
        anti_reject_orthogonally_from_partial(self)
    }
}
impl AntiRejectOrthogonallyFrom<DualNum> for Plane {
    type Output = Plane;
// Operative Statistics for this implementation:
//           add/sub      mul      div
//      f32        0        4        0
//    simd4        0        1        0
// Totals...
// yes simd        0        5        0
//  no simd        0        8        0
    fn anti_reject_orthogonally_from(self, other: DualNum) -> Self::Output {
        use crate::elements::*;
        return Plane::from_groups(/* e423, e431, e412, e321 */float4(other[scalar]) * float4(other[scalar] * self[e423], other[scalar] * self[e431], other[scalar] * self[e412], other[scalar] * self[e321]));
    }
}
impl AntiRejectOrthogonallyFrom<Flector> for Plane {
    type Output = Flector;
// Operative Statistics for this implementation:
//           add/sub      mul      div
//      f32        3        5        0
//    simd4        0        3        0
// Totals...
// yes simd        3        8        0
//  no simd        3       17        0
    fn anti_reject_orthogonally_from(self, other: Flector) -> Self::Output {
        use crate::elements::*;
let wedge = AntiScalar::from_groups(/* e1234 */-(other[e1] * self[e423]) - (other[e2] * self[e431]) - (other[e3] * self[e412]) - (other[e4] * self[e321]));
let right_anti_dual = Flector::from_groups(/* e1, e2, e3, e4 */float4(float3(0.0), other[e321] * -1.0), /* e423, e431, e412, e321 */float4(other[e1], other[e2], other[e3], 0.0));
        return Flector::from_groups(/* e1, e2, e3, e4 */float4(wedge[e1234]) * right_anti_dual.group0(), /* e423, e431, e412, e321 */float4(wedge[e1234], wedge[e1234], wedge[e1234], 0.0) * float4(right_anti_dual.group1().xyz, 0.0) * float4(1.0, 1.0, 1.0, 0.0));
    }
}
impl AntiRejectOrthogonallyFrom<Motor> for Plane {
    type Output = Flector;
// Operative Statistics for this implementation:
//           add/sub      mul      div
//      f32        4        8        0
//    simd4        1        4        0
// Totals...
// yes simd        5       12        0
//  no simd        8       24        0
    fn anti_reject_orthogonally_from(self, other: Motor) -> Self::Output {
        use crate::elements::*;
let wedge = Plane::from_groups(/* e423, e431, e412, e321 */float4(other[scalar]) * self.group0());
let right_anti_dual = Motor::from_groups(/* e41, e42, e43, e1234 */other.group1() * float4(-1.0, -1.0, -1.0, 1.0), /* e23, e31, e12, scalar */float4(0.0));
        return Flector::from_groups(/* e1, e2, e3, e4 */float4((right_anti_dual[e41] * wedge[e321]) + (right_anti_dual[e31] * wedge[e412]), (right_anti_dual[e42] * wedge[e321]) + (right_anti_dual[e12] * wedge[e423]), (right_anti_dual[e43] * wedge[e321]) + (right_anti_dual[e23] * wedge[e431]), -(right_anti_dual[e42] * wedge[e431]) - (right_anti_dual[e43] * wedge[e412])) - (wedge.group0().yzxx * float4(right_anti_dual.group1().zxy, right_anti_dual[e41])), /* e423, e431, e412, e321 */float4(right_anti_dual[e1234]) * wedge.group0());
    }
}
impl AntiRejectOrthogonallyFrom<MultiVector> for Plane {
    type Output = MultiVector;
// Operative Statistics for this implementation:
//           add/sub      mul      div
//      f32       30       39        0
//    simd3        6       10        0
//    simd4        5        6        0
// Totals...
// yes simd       41       55        0
//  no simd       68       93        0
    fn anti_reject_orthogonally_from(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
let wedge = MultiVector::from_groups(/* scalar, e1234 */float2(0.0, -(other[e1] * self[e423]) - (other[e2] * self[e431]) - (other[e3] * self[e412]) - (other[e4] * self[e321])), /* e1, e2, e3, e4 */float4(0.0), /* e41, e42, e43 */float3(0.0), /* e23, e31, e12 */float3(0.0), /* e423, e431, e412, e321 */float4(other[scalar]) * self.group0());
let right_anti_dual = MultiVector::from_groups(/* scalar, e1234 */float2(0.0, other[scalar]), /* e1, e2, e3, e4 */float4(float3(0.0), other[e321] * -1.0), /* e41, e42, e43 */other.group3() * float3(-1.0), /* e23, e31, e12 */float3(0.0), /* e423, e431, e412, e321 */float4(other[e1], other[e2], other[e3], 0.0));
        return MultiVector::from_groups(/* scalar, e1234 */float2((right_anti_dual[scalar] * wedge[e1234]) + (right_anti_dual[e1234] * wedge[scalar]) + (right_anti_dual[e423] * wedge[e1]) + (right_anti_dual[e431] * wedge[e2]) + (right_anti_dual[e412] * wedge[e3]) + (right_anti_dual[e321] * wedge[e4]) - (right_anti_dual[e1] * wedge[e423]) - (right_anti_dual[e2] * wedge[e431]) - (right_anti_dual[e3] * wedge[e412]) - (right_anti_dual[e4] * wedge[e321]) - (right_anti_dual[e41] * wedge[e23]) - (right_anti_dual[e42] * wedge[e31]) - (right_anti_dual[e43] * wedge[e12]) - (right_anti_dual[e23] * wedge[e41]) - (right_anti_dual[e31] * wedge[e42]) - (right_anti_dual[e12] * wedge[e43]), right_anti_dual[e1234] * wedge[e1234]), /* e1, e2, e3, e4 */float4((right_anti_dual[e41] * wedge[e321]) + (right_anti_dual[e31] * wedge[e412]) + (right_anti_dual[e412] * wedge[e31]) + (right_anti_dual[e321] * wedge[e41]), (right_anti_dual[e42] * wedge[e321]) + (right_anti_dual[e12] * wedge[e423]) + (right_anti_dual[e423] * wedge[e12]) + (right_anti_dual[e321] * wedge[e42]), (right_anti_dual[e43] * wedge[e321]) + (right_anti_dual[e23] * wedge[e431]) + (right_anti_dual[e431] * wedge[e23]) + (right_anti_dual[e321] * wedge[e43]), -(right_anti_dual[e43] * wedge[e412]) - (right_anti_dual[e423] * wedge[e41]) - (right_anti_dual[e431] * wedge[e42]) - (right_anti_dual[e412] * wedge[e43])) + (float4(right_anti_dual[e1234]) * wedge.group1()) + (float4(wedge[e1234]) * right_anti_dual.group1()) - (wedge.group4().yzxx * float4(right_anti_dual.group3().zxy, right_anti_dual[e41])) - float4((wedge.group3().zxy * right_anti_dual.group4().yzx), right_anti_dual[e42] * wedge[e431]), /* e41, e42, e43 */(float3(right_anti_dual[e1234]) * wedge.group2()) + (float3(wedge[e1234]) * right_anti_dual.group2()) + (right_anti_dual.group4().yzx * wedge.group4().zxy) - (right_anti_dual.group4().zxy * wedge.group4().yzx), /* e23, e31, e12 */(float3(right_anti_dual[e1234]) * wedge.group3()) + (float3(right_anti_dual[e321]) * wedge.group4().xyz) + (float3(wedge[e1234]) * right_anti_dual.group3()) - (float3(wedge[e321]) * right_anti_dual.group4().xyz), /* e423, e431, e412, e321 */(float4(right_anti_dual[e1234]) * wedge.group4()) + (float4(wedge[e1234]) * right_anti_dual.group4()));
    }
}
impl AntiRejectOrthogonallyFrom<Point> for Plane {
    type Output = Plane;
// Operative Statistics for this implementation:
//           add/sub      mul      div
//      f32        3        4        0
//    simd4        0        2        0
// Totals...
// yes simd        3        6        0
//  no simd        3       12        0
    fn anti_reject_orthogonally_from(self, other: Point) -> Self::Output {
        use crate::elements::*;
let wedge = AntiScalar::from_groups(/* e1234 */-(self[e423] * other[e1]) - (self[e431] * other[e2]) - (self[e412] * other[e3]) - (self[e321] * other[e4]));
        return Plane::from_groups(/* e423, e431, e412, e321 */float4(wedge[e1234], wedge[e1234], wedge[e1234], 0.0) * float4(float4(other[e1], other[e2], other[e3], 0.0).xyz, 0.0) * float4(1.0, 1.0, 1.0, 0.0));
    }
}
impl AntiRejectOrthogonallyFrom<Scalar> for Plane {
    type Output = Plane;
// Operative Statistics for this implementation:
//           add/sub      mul      div
//      f32        0        4        0
//    simd4        0        1        0
// Totals...
// yes simd        0        5        0
//  no simd        0        8        0
    fn anti_reject_orthogonally_from(self, other: Scalar) -> Self::Output {
        use crate::elements::*;
        return Plane::from_groups(/* e423, e431, e412, e321 */float4(other[scalar]) * float4(self[e423] * other[scalar], self[e431] * other[scalar], self[e412] * other[scalar], self[e321] * other[scalar]));
    }
}
impl std::ops::Div<anti_reject_orthogonally_from> for Point {
    type Output = anti_reject_orthogonally_from_partial<Point>;
    fn div(self, _rhs: anti_reject_orthogonally_from) -> Self::Output {
        anti_reject_orthogonally_from_partial(self)
    }
}
impl AntiRejectOrthogonallyFrom<DualNum> for Point {
    type Output = Point;
// Operative Statistics for this implementation:
//           add/sub      mul      div
//      f32        0        4        0
//    simd4        0        1        0
// Totals...
// yes simd        0        5        0
//  no simd        0        8        0
    fn anti_reject_orthogonally_from(self, other: DualNum) -> Self::Output {
        use crate::elements::*;
        return Point::from_groups(/* e1, e2, e3, e4 */float4(other[scalar]) * float4(other[scalar] * self[e1], other[scalar] * self[e2], other[scalar] * self[e3], other[scalar] * self[e4]));
    }
}
impl AntiRejectOrthogonallyFrom<Flector> for Point {
    type Output = Flector;
// Operative Statistics for this implementation:
//           add/sub      mul      div
//      f32        6       18        0
//    simd3        1        2        0
//    simd4        3        6        0
// Totals...
// yes simd       10       26        0
//  no simd       21       48        0
    fn anti_reject_orthogonally_from(self, other: Flector) -> Self::Output {
        use crate::elements::*;
let wedge = Motor::from_groups(/* e41, e42, e43, e1234 */float4(other[e4] * self[e1] * -1.0, other[e4] * self[e2] * -1.0, other[e4] * self[e3] * -1.0, (other[e431] * self[e2]) + (other[e412] * self[e3]) + (other[e321] * self[e4])) + (self.group0().wwwx * float4(other.group0().xyz, other[e423])), /* e23, e31, e12, scalar */float4(((other.group0().zxy * self.group0().yzx) - (other.group0().yzx * self.group0().zxy)), 0.0));
let right_anti_dual = Flector::from_groups(/* e1, e2, e3, e4 */float4(float3(0.0), other[e321] * -1.0), /* e423, e431, e412, e321 */float4(other[e1], other[e2], other[e3], 0.0));
        return Flector::from_groups(/* e1, e2, e3, e4 */float4((right_anti_dual[e412] * wedge[e31]) + (right_anti_dual[e321] * wedge[e41]), (right_anti_dual[e423] * wedge[e12]) + (right_anti_dual[e321] * wedge[e42]), (right_anti_dual[e431] * wedge[e23]) + (right_anti_dual[e321] * wedge[e43]), -(right_anti_dual[e431] * wedge[e42]) - (right_anti_dual[e412] * wedge[e43])) + (float4(wedge[e1234]) * right_anti_dual.group0()) - (right_anti_dual.group1().yzxx * float4(wedge.group1().zxy, wedge[e41])), /* e423, e431, e412, e321 */float4(float3(1.0), 0.0) * float4(wedge.group0().www, 0.0) * float4(right_anti_dual.group1().xyz, 0.0) * float4(1.0, 1.0, 1.0, 0.0));
    }
}
impl AntiRejectOrthogonallyFrom<Horizon> for Point {
    type Output = Origin;
// Operative Statistics for this implementation:
//      add/sub      mul      div
// f32        0        2        0
    fn anti_reject_orthogonally_from(self, other: Horizon) -> Self::Output {
        use crate::elements::*;
        return Origin::from_groups(/* e4 */powi(other[e321], 2) * self[e4] * -1.0);
    }
}
impl AntiRejectOrthogonallyFrom<Line> for Point {
    type Output = Point;
// Operative Statistics for this implementation:
//           add/sub      mul      div
//      f32        8       16        0
//    simd3        0        1        0
//    simd4        2        2        0
// Totals...
// yes simd       10       19        0
//  no simd       16       27        0
    fn anti_reject_orthogonally_from(self, other: Line) -> Self::Output {
        use crate::elements::*;
let wedge = Plane::from_groups(/* e423, e431, e412, e321 */float4((other[e42] * self[e3]) + (other[e23] * self[e4]), (other[e43] * self[e1]) + (other[e31] * self[e4]), (other[e41] * self[e2]) + (other[e12] * self[e4]), -(other[e31] * self[e2]) - (other[e12] * self[e3])) - (self.group0().yzxx * float4(other.group0().zxy, other[e23])));
let right_anti_dual = Line::from_groups(/* e41, e42, e43 */other.group1() * float3(-1.0), /* e23, e31, e12 */float3(0.0));
        return Point::from_groups(/* e1, e2, e3, e4 */float4((right_anti_dual[e41] * wedge[e321]) + (right_anti_dual[e31] * wedge[e412]), (right_anti_dual[e42] * wedge[e321]) + (right_anti_dual[e12] * wedge[e423]), (right_anti_dual[e43] * wedge[e321]) + (right_anti_dual[e23] * wedge[e431]), -(right_anti_dual[e42] * wedge[e431]) - (right_anti_dual[e43] * wedge[e412])) - (wedge.group0().yzxx * float4(right_anti_dual.group1().zxy, right_anti_dual[e41])));
    }
}
impl AntiRejectOrthogonallyFrom<Motor> for Point {
    type Output = Flector;
// Operative Statistics for this implementation:
//           add/sub      mul      div
//      f32        8       16        0
//    simd4        3        6        0
// Totals...
// yes simd       11       22        0
//  no simd       20       40        0
    fn anti_reject_orthogonally_from(self, other: Motor) -> Self::Output {
        use crate::elements::*;
let wedge = Flector::from_groups(/* e1, e2, e3, e4 */float4(other[scalar]) * self.group0(), /* e423, e431, e412, e321 */float4((other[e42] * self[e3]) + (other[e23] * self[e4]), (other[e43] * self[e1]) + (other[e31] * self[e4]), (other[e41] * self[e2]) + (other[e12] * self[e4]), -(other[e31] * self[e2]) - (other[e12] * self[e3])) - (self.group0().yzxx * float4(other.group0().zxy, other[e23])));
let right_anti_dual = Motor::from_groups(/* e41, e42, e43, e1234 */other.group1() * float4(-1.0, -1.0, -1.0, 1.0), /* e23, e31, e12, scalar */float4(0.0));
        return Flector::from_groups(/* e1, e2, e3, e4 */float4((wedge[e412] * right_anti_dual[e31]) + (wedge[e321] * right_anti_dual[e41]), (wedge[e423] * right_anti_dual[e12]) + (wedge[e321] * right_anti_dual[e42]), (wedge[e431] * right_anti_dual[e23]) + (wedge[e321] * right_anti_dual[e43]), -(wedge[e431] * right_anti_dual[e42]) - (wedge[e412] * right_anti_dual[e43])) + (float4(right_anti_dual[e1234]) * wedge.group0()) - (wedge.group1().yzxx * float4(right_anti_dual.group1().zxy, right_anti_dual[e41])), /* e423, e431, e412, e321 */float4(right_anti_dual[e1234]) * wedge.group1());
    }
}
impl AntiRejectOrthogonallyFrom<MultiVector> for Point {
    type Output = MultiVector;
// Operative Statistics for this implementation:
//           add/sub      mul      div
//      f32       34       47        0
//    simd3        8       14        0
//    simd4        6        7        0
// Totals...
// yes simd       48       68        0
//  no simd       82      117        0
    fn anti_reject_orthogonally_from(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
let wedge = MultiVector::from_groups(/* scalar, e1234 */float2(0.0, (other[e423] * self[e1]) + (other[e431] * self[e2]) + (other[e412] * self[e3]) + (other[e321] * self[e4])), /* e1, e2, e3, e4 */float4(other[scalar]) * self.group0(), /* e41, e42, e43 */(float3(self[e4]) * other.group1().xyz) - (float3(other[e4]) * self.group0().xyz), /* e23, e31, e12 */(other.group1().zxy * self.group0().yzx) - (other.group1().yzx * self.group0().zxy), /* e423, e431, e412, e321 */float4((other[e42] * self[e3]) + (other[e23] * self[e4]), (other[e43] * self[e1]) + (other[e31] * self[e4]), (other[e41] * self[e2]) + (other[e12] * self[e4]), -(other[e31] * self[e2]) - (other[e12] * self[e3])) - (self.group0().yzxx * float4(other.group2().zxy, other[e23])));
let right_anti_dual = MultiVector::from_groups(/* scalar, e1234 */float2(0.0, other[scalar]), /* e1, e2, e3, e4 */float4(float3(0.0), other[e321] * -1.0), /* e41, e42, e43 */other.group3() * float3(-1.0), /* e23, e31, e12 */float3(0.0), /* e423, e431, e412, e321 */float4(other[e1], other[e2], other[e3], 0.0));
        return MultiVector::from_groups(/* scalar, e1234 */float2((right_anti_dual[scalar] * wedge[e1234]) + (right_anti_dual[e1234] * wedge[scalar]) + (right_anti_dual[e423] * wedge[e1]) + (right_anti_dual[e431] * wedge[e2]) + (right_anti_dual[e412] * wedge[e3]) + (right_anti_dual[e321] * wedge[e4]) - (right_anti_dual[e1] * wedge[e423]) - (right_anti_dual[e2] * wedge[e431]) - (right_anti_dual[e3] * wedge[e412]) - (right_anti_dual[e4] * wedge[e321]) - (right_anti_dual[e41] * wedge[e23]) - (right_anti_dual[e42] * wedge[e31]) - (right_anti_dual[e43] * wedge[e12]) - (right_anti_dual[e23] * wedge[e41]) - (right_anti_dual[e31] * wedge[e42]) - (right_anti_dual[e12] * wedge[e43]), right_anti_dual[e1234] * wedge[e1234]), /* e1, e2, e3, e4 */float4((right_anti_dual[e41] * wedge[e321]) + (right_anti_dual[e31] * wedge[e412]) + (right_anti_dual[e412] * wedge[e31]) + (right_anti_dual[e321] * wedge[e41]), (right_anti_dual[e42] * wedge[e321]) + (right_anti_dual[e12] * wedge[e423]) + (right_anti_dual[e423] * wedge[e12]) + (right_anti_dual[e321] * wedge[e42]), (right_anti_dual[e43] * wedge[e321]) + (right_anti_dual[e23] * wedge[e431]) + (right_anti_dual[e431] * wedge[e23]) + (right_anti_dual[e321] * wedge[e43]), -(right_anti_dual[e43] * wedge[e412]) - (right_anti_dual[e423] * wedge[e41]) - (right_anti_dual[e431] * wedge[e42]) - (right_anti_dual[e412] * wedge[e43])) + (float4(right_anti_dual[e1234]) * wedge.group1()) + (float4(wedge[e1234]) * right_anti_dual.group1()) - (wedge.group4().yzxx * float4(right_anti_dual.group3().zxy, right_anti_dual[e41])) - float4((wedge.group3().zxy * right_anti_dual.group4().yzx), right_anti_dual[e42] * wedge[e431]), /* e41, e42, e43 */(float3(right_anti_dual[e1234]) * wedge.group2()) + (float3(wedge[e1234]) * right_anti_dual.group2()) + (right_anti_dual.group4().yzx * wedge.group4().zxy) - (right_anti_dual.group4().zxy * wedge.group4().yzx), /* e23, e31, e12 */(float3(right_anti_dual[e1234]) * wedge.group3()) + (float3(right_anti_dual[e321]) * wedge.group4().xyz) + (float3(wedge[e1234]) * right_anti_dual.group3()) - (float3(wedge[e321]) * right_anti_dual.group4().xyz), /* e423, e431, e412, e321 */(float4(right_anti_dual[e1234]) * wedge.group4()) + (float4(wedge[e1234]) * right_anti_dual.group4()));
    }
}
impl AntiRejectOrthogonallyFrom<Plane> for Point {
    type Output = Origin;
// Operative Statistics for this implementation:
//      add/sub      mul      div
// f32        3        7        0
    fn anti_reject_orthogonally_from(self, other: Plane) -> Self::Output {
        use crate::elements::*;
        return Origin::from_groups(/* e4 */-(powi(other[e321], 2) * self[e4]) - (other[e423] * other[e321] * self[e1]) - (other[e431] * other[e321] * self[e2]) - (other[e412] * other[e321] * self[e3]));
    }
}
impl AntiRejectOrthogonallyFrom<Point> for Point {
    type Output = Point;
// Operative Statistics for this implementation:
//           add/sub      mul      div
//      f32        4        8        0
//    simd3        2        4        0
//    simd4        1        1        0
// Totals...
// yes simd        7       13        0
//  no simd       14       24        0
    fn anti_reject_orthogonally_from(self, other: Point) -> Self::Output {
        use crate::elements::*;
let wedge = Line::from_groups(/* e41, e42, e43 */(float3(self[e4]) * other.group0().xyz) - (float3(other[e4]) * self.group0().xyz), /* e23, e31, e12 */(other.group0().zxy * self.group0().yzx) - (other.group0().yzx * self.group0().zxy));
let right_anti_dual = Plane::from_groups(/* e423, e431, e412, e321 */float4(other[e1], other[e2], other[e3], 0.0));
        return Point::from_groups(/* e1, e2, e3, e4 */float4((wedge[e41] * right_anti_dual[e321]) + (wedge[e31] * right_anti_dual[e412]), (wedge[e42] * right_anti_dual[e321]) + (wedge[e12] * right_anti_dual[e423]), (wedge[e43] * right_anti_dual[e321]) + (wedge[e23] * right_anti_dual[e431]), -(wedge[e42] * right_anti_dual[e431]) - (wedge[e43] * right_anti_dual[e412])) - (right_anti_dual.group0().yzxx * float4(wedge.group1().zxy, wedge[e41])));
    }
}
impl AntiRejectOrthogonallyFrom<Scalar> for Point {
    type Output = Point;
// Operative Statistics for this implementation:
//           add/sub      mul      div
//      f32        0        4        0
//    simd4        0        1        0
// Totals...
// yes simd        0        5        0
//  no simd        0        8        0
    fn anti_reject_orthogonally_from(self, other: Scalar) -> Self::Output {
        use crate::elements::*;
        return Point::from_groups(/* e1, e2, e3, e4 */float4(other[scalar]) * float4(self[e1] * other[scalar], self[e2] * other[scalar], self[e3] * other[scalar], self[e4] * other[scalar]));
    }
}
impl std::ops::Div<anti_reject_orthogonally_from> for Scalar {
    type Output = anti_reject_orthogonally_from_partial<Scalar>;
    fn div(self, _rhs: anti_reject_orthogonally_from) -> Self::Output {
        anti_reject_orthogonally_from_partial(self)
    }
}
impl AntiRejectOrthogonallyFrom<DualNum> for Scalar {
    type Output = DualNum;
// Operative Statistics for this implementation:
//           add/sub      mul      div
//      f32        0        2        0
//    simd2        0        1        0
// Totals...
// yes simd        0        3        0
//  no simd        0        4        0
    fn anti_reject_orthogonally_from(self, other: DualNum) -> Self::Output {
        use crate::elements::*;
        return DualNum::from_groups(/* scalar, e1234 */float2(other[scalar]) * float2(other[scalar] * self[scalar], other[e1234] * self[scalar]));
    }
}
impl AntiRejectOrthogonallyFrom<Flector> for Scalar {
    type Output = Motor;
// Operative Statistics for this implementation:
//           add/sub      mul      div
//      f32        5        7        0
//    simd3        1        2        0
//    simd4        2        4        0
// Totals...
// yes simd        8       13        0
//  no simd       16       29        0
    fn anti_reject_orthogonally_from(self, other: Flector) -> Self::Output {
        use crate::elements::*;
let wedge = Flector::from_groups(/* e1, e2, e3, e4 */float4(self[scalar]) * other.group0(), /* e423, e431, e412, e321 */float4(self[scalar]) * other.group1());
let right_anti_dual = Flector::from_groups(/* e1, e2, e3, e4 */float4(float3(0.0), other[e321] * -1.0), /* e423, e431, e412, e321 */float4(other[e1], other[e2], other[e3], 0.0));
        return Motor::from_groups(/* e41, e42, e43, e1234 */float4(((right_anti_dual.group1().yzx * wedge.group1().zxy) - (right_anti_dual.group1().zxy * wedge.group1().yzx)), 0.0), /* e23, e31, e12, scalar */(right_anti_dual.group1().wwwx * float4(wedge.group1().xyz, wedge[e1])) + float4(float3(0.0), (right_anti_dual[e431] * wedge[e2]) + (right_anti_dual[e412] * wedge[e3]) + (right_anti_dual[e321] * wedge[e4]) - (right_anti_dual[e2] * wedge[e431]) - (right_anti_dual[e3] * wedge[e412]) - (right_anti_dual[e4] * wedge[e321])) - (wedge.group1().wwwx * float4(right_anti_dual.group1().xyz, right_anti_dual[e1])));
    }
}
impl AntiRejectOrthogonallyFrom<Horizon> for Scalar {
    type Output = Scalar;
// Operative Statistics for this implementation:
//      add/sub      mul      div
// f32        0        1        0
    fn anti_reject_orthogonally_from(self, other: Horizon) -> Self::Output {
        use crate::elements::*;
        return Scalar::from_groups(/* scalar */powi(other[e321], 2) * self[scalar]);
    }
}
impl AntiRejectOrthogonallyFrom<Line> for Scalar {
    type Output = Scalar;
// Operative Statistics for this implementation:
//           add/sub      mul      div
//      f32        5        6        0
//    simd3        0        3        0
// Totals...
// yes simd        5        9        0
//  no simd        5       15        0
    fn anti_reject_orthogonally_from(self, other: Line) -> Self::Output {
        use crate::elements::*;
let wedge = Line::from_groups(/* e41, e42, e43 */float3(self[scalar]) * other.group0(), /* e23, e31, e12 */float3(self[scalar]) * other.group1());
let right_anti_dual = Line::from_groups(/* e41, e42, e43 */other.group1() * float3(-1.0), /* e23, e31, e12 */float3(0.0));
        return Scalar::from_groups(/* scalar */-(right_anti_dual[e41] * wedge[e23]) - (right_anti_dual[e42] * wedge[e31]) - (right_anti_dual[e43] * wedge[e12]) - (right_anti_dual[e23] * wedge[e41]) - (right_anti_dual[e31] * wedge[e42]) - (right_anti_dual[e12] * wedge[e43]));
    }
}
impl AntiRejectOrthogonallyFrom<Motor> for Scalar {
    type Output = Motor;
// Operative Statistics for this implementation:
//           add/sub      mul      div
//      f32        5        7        0
//    simd3        1        2        0
//    simd4        2        5        0
// Totals...
// yes simd        8       14        0
//  no simd       16       33        0
    fn anti_reject_orthogonally_from(self, other: Motor) -> Self::Output {
        use crate::elements::*;
let wedge = Motor::from_groups(/* e41, e42, e43, e1234 */float4(self[scalar]) * other.group0(), /* e23, e31, e12, scalar */float4(self[scalar]) * other.group1());
let right_anti_dual = Motor::from_groups(/* e41, e42, e43, e1234 */other.group1() * float4(-1.0, -1.0, -1.0, 1.0), /* e23, e31, e12, scalar */float4(0.0));
        return Motor::from_groups(/* e41, e42, e43, e1234 */float4(((float3(right_anti_dual[e1234]) * wedge.group0().xyz) + (float3(wedge[e1234]) * right_anti_dual.group0().xyz)), right_anti_dual[e1234] * wedge[e1234]), /* e23, e31, e12, scalar */(float4(right_anti_dual[e1234]) * wedge.group1()) + (float4(wedge[e1234]) * right_anti_dual.group1()) + float4(float3(0.0), -(right_anti_dual[e41] * wedge[e23]) - (right_anti_dual[e42] * wedge[e31]) - (right_anti_dual[e43] * wedge[e12]) - (right_anti_dual[e23] * wedge[e41]) - (right_anti_dual[e31] * wedge[e42]) - (right_anti_dual[e12] * wedge[e43])));
    }
}
impl AntiRejectOrthogonallyFrom<MultiVector> for Scalar {
    type Output = MultiVector;
// Operative Statistics for this implementation:
//           add/sub      mul      div
//      f32       27       35        0
//    simd2        0        1        0
//    simd3        6       12        0
//    simd4        5        7        0
// Totals...
// yes simd       38       55        0
//  no simd       65      101        0
    fn anti_reject_orthogonally_from(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
let wedge = MultiVector::from_groups(/* scalar, e1234 */float2(self[scalar]) * other.group0(), /* e1, e2, e3, e4 */float4(self[scalar]) * other.group1(), /* e41, e42, e43 */float3(self[scalar]) * other.group2(), /* e23, e31, e12 */float3(self[scalar]) * other.group3(), /* e423, e431, e412, e321 */float4(self[scalar]) * other.group4());
let right_anti_dual = MultiVector::from_groups(/* scalar, e1234 */float2(0.0, other[scalar]), /* e1, e2, e3, e4 */float4(float3(0.0), other[e321] * -1.0), /* e41, e42, e43 */other.group3() * float3(-1.0), /* e23, e31, e12 */float3(0.0), /* e423, e431, e412, e321 */float4(other[e1], other[e2], other[e3], 0.0));
        return MultiVector::from_groups(/* scalar, e1234 */float2((right_anti_dual[scalar] * wedge[e1234]) + (right_anti_dual[e1234] * wedge[scalar]) + (right_anti_dual[e423] * wedge[e1]) + (right_anti_dual[e431] * wedge[e2]) + (right_anti_dual[e412] * wedge[e3]) + (right_anti_dual[e321] * wedge[e4]) - (right_anti_dual[e1] * wedge[e423]) - (right_anti_dual[e2] * wedge[e431]) - (right_anti_dual[e3] * wedge[e412]) - (right_anti_dual[e4] * wedge[e321]) - (right_anti_dual[e41] * wedge[e23]) - (right_anti_dual[e42] * wedge[e31]) - (right_anti_dual[e43] * wedge[e12]) - (right_anti_dual[e23] * wedge[e41]) - (right_anti_dual[e31] * wedge[e42]) - (right_anti_dual[e12] * wedge[e43]), right_anti_dual[e1234] * wedge[e1234]), /* e1, e2, e3, e4 */float4((right_anti_dual[e41] * wedge[e321]) + (right_anti_dual[e31] * wedge[e412]) + (right_anti_dual[e412] * wedge[e31]) + (right_anti_dual[e321] * wedge[e41]), (right_anti_dual[e42] * wedge[e321]) + (right_anti_dual[e12] * wedge[e423]) + (right_anti_dual[e423] * wedge[e12]) + (right_anti_dual[e321] * wedge[e42]), (right_anti_dual[e43] * wedge[e321]) + (right_anti_dual[e23] * wedge[e431]) + (right_anti_dual[e431] * wedge[e23]) + (right_anti_dual[e321] * wedge[e43]), -(right_anti_dual[e43] * wedge[e412]) - (right_anti_dual[e423] * wedge[e41]) - (right_anti_dual[e431] * wedge[e42]) - (right_anti_dual[e412] * wedge[e43])) + (float4(right_anti_dual[e1234]) * wedge.group1()) + (float4(wedge[e1234]) * right_anti_dual.group1()) - (wedge.group4().yzxx * float4(right_anti_dual.group3().zxy, right_anti_dual[e41])) - float4((wedge.group3().zxy * right_anti_dual.group4().yzx), right_anti_dual[e42] * wedge[e431]), /* e41, e42, e43 */(float3(right_anti_dual[e1234]) * wedge.group2()) + (float3(wedge[e1234]) * right_anti_dual.group2()) + (right_anti_dual.group4().yzx * wedge.group4().zxy) - (right_anti_dual.group4().zxy * wedge.group4().yzx), /* e23, e31, e12 */(float3(right_anti_dual[e1234]) * wedge.group3()) + (float3(right_anti_dual[e321]) * wedge.group4().xyz) + (float3(wedge[e1234]) * right_anti_dual.group3()) - (float3(wedge[e321]) * right_anti_dual.group4().xyz), /* e423, e431, e412, e321 */(float4(right_anti_dual[e1234]) * wedge.group4()) + (float4(wedge[e1234]) * right_anti_dual.group4()));
    }
}
impl AntiRejectOrthogonallyFrom<Plane> for Scalar {
    type Output = Scalar;
// Operative Statistics for this implementation:
//      add/sub      mul      div
// f32        0        1        0
    fn anti_reject_orthogonally_from(self, other: Plane) -> Self::Output {
        use crate::elements::*;
        return Scalar::from_groups(/* scalar */powi(other[e321], 2) * self[scalar]);
    }
}
impl AntiRejectOrthogonallyFrom<Point> for Scalar {
    type Output = Scalar;
// Operative Statistics for this implementation:
//           add/sub      mul      div
//      f32        3        4        0
//    simd4        0        1        0
// Totals...
// yes simd        3        5        0
//  no simd        3        8        0
    fn anti_reject_orthogonally_from(self, other: Point) -> Self::Output {
        use crate::elements::*;
let wedge = Point::from_groups(/* e1, e2, e3, e4 */float4(self[scalar]) * other.group0());
let right_anti_dual = Plane::from_groups(/* e423, e431, e412, e321 */float4(other[e1], other[e2], other[e3], 0.0));
        return Scalar::from_groups(/* scalar */(right_anti_dual[e423] * wedge[e1]) + (right_anti_dual[e431] * wedge[e2]) + (right_anti_dual[e412] * wedge[e3]) + (right_anti_dual[e321] * wedge[e4]));
    }
}
impl AntiRejectOrthogonallyFrom<Scalar> for Scalar {
    type Output = Scalar;
// Operative Statistics for this implementation:
//      add/sub      mul      div
// f32        0        1        0
    fn anti_reject_orthogonally_from(self, other: Scalar) -> Self::Output {
        use crate::elements::*;
        return Scalar::from_groups(/* scalar */powi(other[scalar], 2) * self[scalar]);
    }
}
