impl std::ops::Div<project_orthogonally_onto> for AntiScalar {
    type Output = project_orthogonally_onto_partial<AntiScalar>;
    fn div(self, _rhs: project_orthogonally_onto) -> Self::Output {
        project_orthogonally_onto_partial(self)
    }
}
impl ProjectOrthogonallyOnto<Motor> for AntiScalar {
    type Output = Motor;
// Operative Statistics for this implementation:
//           add/sub      mul      div
//      f32        0        1        0
//    simd4        0        2        0
// Totals...
// yes simd        0        3        0
//  no simd        0        9        0
    fn project_orthogonally_onto(self, other: Motor) -> Self::Output {
        use crate::elements::*;
let wedge = AntiScalar::from_groups(/* e1234 */self[e1234] * 0.0);
        return Motor::from_groups(/* e41, e42, e43, e1234 */float4(wedge[e1234]) * other.group0(), /* e23, e31, e12, scalar */float4(wedge[e1234]) * other.group1());
    }
}
impl ProjectOrthogonallyOnto<MultiVector> for AntiScalar {
    type Output = MultiVector;
// Operative Statistics for this implementation:
//           add/sub      mul      div
//      f32        0        1        0
//    simd2        0        1        0
//    simd3        0        2        0
//    simd4        0        2        0
// Totals...
// yes simd        0        6        0
//  no simd        0       17        0
    fn project_orthogonally_onto(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
let wedge = AntiScalar::from_groups(/* e1234 */self[e1234] * 0.0);
        return MultiVector::from_groups(/* scalar, e1234 */float2(wedge[e1234]) * other.group0(), /* e1, e2, e3, e4 */float4(wedge[e1234]) * other.group1(), /* e41, e42, e43 */float3(wedge[e1234]) * other.group2(), /* e23, e31, e12 */float3(wedge[e1234]) * other.group3(), /* e423, e431, e412, e321 */float4(wedge[e1234]) * other.group4());
    }
}
impl std::ops::Div<project_orthogonally_onto> for DualNum {
    type Output = project_orthogonally_onto_partial<DualNum>;
    fn div(self, _rhs: project_orthogonally_onto) -> Self::Output {
        project_orthogonally_onto_partial(self)
    }
}
impl ProjectOrthogonallyOnto<DualNum> for DualNum {
    type Output = DualNum;
// Operative Statistics for this implementation:
//           add/sub      mul      div
//      f32        0        1        0
//    simd2        0        1        0
// Totals...
// yes simd        0        2        0
//  no simd        0        3        0
    fn project_orthogonally_onto(self, other: DualNum) -> Self::Output {
        use crate::elements::*;
        return DualNum::from_groups(/* scalar, e1234 */float2(other[scalar] * self[scalar]) * other.group0());
    }
}
impl ProjectOrthogonallyOnto<Flector> for DualNum {
    type Output = Motor;
// Operative Statistics for this implementation:
//           add/sub      mul      div
//      f32        5        7        0
//    simd3        1        2        0
//    simd4        2        4        0
// Totals...
// yes simd        8       13        0
//  no simd       16       29        0
    fn project_orthogonally_onto(self, other: Flector) -> Self::Output {
        use crate::elements::*;
let right_anti_dual = Flector::from_groups(/* e1, e2, e3, e4 */float4(float3(0.0), other[e321] * -1.0), /* e423, e431, e412, e321 */float4(other[e1], other[e2], other[e3], 0.0));
let wedge = Flector::from_groups(/* e1, e2, e3, e4 */float4(self[scalar]) * right_anti_dual.group0(), /* e423, e431, e412, e321 */float4(self[scalar]) * right_anti_dual.group1());
        return Motor::from_groups(/* e41, e42, e43, e1234 */float4(((other.group1().zxy * wedge.group1().yzx) - (other.group1().yzx * wedge.group1().zxy)), 0.0), /* e23, e31, e12, scalar */(wedge.group1().wwwx * float4(other.group1().xyz, other[e1])) + float4(float3(0.0), (other[e2] * wedge[e431]) + (other[e3] * wedge[e412]) + (other[e4] * wedge[e321]) - (other[e431] * wedge[e2]) - (other[e412] * wedge[e3]) - (other[e321] * wedge[e4])) - (other.group1().wwwx * float4(wedge.group1().xyz, wedge[e1])));
    }
}
impl ProjectOrthogonallyOnto<Horizon> for DualNum {
    type Output = Scalar;
// Operative Statistics for this implementation:
//      add/sub      mul      div
// f32        0        1        0
    fn project_orthogonally_onto(self, other: Horizon) -> Self::Output {
        use crate::elements::*;
        return Scalar::from_groups(/* scalar */powi(other[e321], 2) * self[scalar]);
    }
}
impl ProjectOrthogonallyOnto<Line> for DualNum {
    type Output = Scalar;
// Operative Statistics for this implementation:
//           add/sub      mul      div
//      f32        5        6        0
//    simd3        0        3        0
// Totals...
// yes simd        5        9        0
//  no simd        5       15        0
    fn project_orthogonally_onto(self, other: Line) -> Self::Output {
        use crate::elements::*;
let right_anti_dual = Line::from_groups(/* e41, e42, e43 */other.group1() * float3(-1.0), /* e23, e31, e12 */float3(0.0));
let wedge = Line::from_groups(/* e41, e42, e43 */float3(self[scalar]) * right_anti_dual.group0(), /* e23, e31, e12 */float3(self[scalar]) * right_anti_dual.group1());
        return Scalar::from_groups(/* scalar */-(other[e41] * wedge[e23]) - (other[e42] * wedge[e31]) - (other[e43] * wedge[e12]) - (other[e23] * wedge[e41]) - (other[e31] * wedge[e42]) - (other[e12] * wedge[e43]));
    }
}
impl ProjectOrthogonallyOnto<Motor> for DualNum {
    type Output = Motor;
// Operative Statistics for this implementation:
//           add/sub      mul      div
//      f32        6        9        0
//    simd3        1        2        0
//    simd4        2        5        0
// Totals...
// yes simd        9       16        0
//  no simd       17       35        0
    fn project_orthogonally_onto(self, other: Motor) -> Self::Output {
        use crate::elements::*;
let right_anti_dual = Motor::from_groups(/* e41, e42, e43, e1234 */other.group1() * float4(-1.0, -1.0, -1.0, 1.0), /* e23, e31, e12, scalar */float4(0.0));
let wedge = Motor::from_groups(/* e41, e42, e43, e1234 */float4(right_anti_dual[e41], right_anti_dual[e42], right_anti_dual[e43], 1.0) * float4(self.group0().xx, self[scalar], (self[scalar] * right_anti_dual[e1234]) + (self[e1234] * right_anti_dual[scalar])), /* e23, e31, e12, scalar */float4(self[scalar]) * right_anti_dual.group1());
        return Motor::from_groups(/* e41, e42, e43, e1234 */float4(((float3(other[e1234]) * wedge.group0().xyz) + (float3(wedge[e1234]) * other.group0().xyz)), other[e1234] * wedge[e1234]), /* e23, e31, e12, scalar */(float4(other[e1234]) * wedge.group1()) + (float4(wedge[e1234]) * other.group1()) + float4(float3(0.0), -(other[e41] * wedge[e23]) - (other[e42] * wedge[e31]) - (other[e43] * wedge[e12]) - (other[e23] * wedge[e41]) - (other[e31] * wedge[e42]) - (other[e12] * wedge[e43])));
    }
}
impl ProjectOrthogonallyOnto<MultiVector> for DualNum {
    type Output = MultiVector;
// Operative Statistics for this implementation:
//           add/sub      mul      div
//      f32       28       38        0
//    simd3        6       12        0
//    simd4        5        7        0
// Totals...
// yes simd       39       57        0
//  no simd       66      102        0
    fn project_orthogonally_onto(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
let right_anti_dual = MultiVector::from_groups(/* scalar, e1234 */float2(0.0, other[scalar]), /* e1, e2, e3, e4 */float4(float3(0.0), other[e321] * -1.0), /* e41, e42, e43 */other.group3() * float3(-1.0), /* e23, e31, e12 */float3(0.0), /* e423, e431, e412, e321 */float4(other[e1], other[e2], other[e3], 0.0));
let wedge = MultiVector::from_groups(/* scalar, e1234 */float2(self[scalar] * right_anti_dual[scalar], (self[scalar] * right_anti_dual[e1234]) + (self[e1234] * right_anti_dual[scalar])), /* e1, e2, e3, e4 */float4(self[scalar]) * right_anti_dual.group1(), /* e41, e42, e43 */float3(self[scalar]) * right_anti_dual.group2(), /* e23, e31, e12 */float3(self[scalar]) * right_anti_dual.group3(), /* e423, e431, e412, e321 */float4(self[scalar]) * right_anti_dual.group4());
        return MultiVector::from_groups(/* scalar, e1234 */float2((other[scalar] * wedge[e1234]) + (other[e1234] * wedge[scalar]) + (other[e1] * wedge[e423]) + (other[e2] * wedge[e431]) + (other[e3] * wedge[e412]) + (other[e4] * wedge[e321]) - (other[e41] * wedge[e23]) - (other[e42] * wedge[e31]) - (other[e43] * wedge[e12]) - (other[e23] * wedge[e41]) - (other[e31] * wedge[e42]) - (other[e12] * wedge[e43]) - (other[e423] * wedge[e1]) - (other[e431] * wedge[e2]) - (other[e412] * wedge[e3]) - (other[e321] * wedge[e4]), other[e1234] * wedge[e1234]), /* e1, e2, e3, e4 */float4((other[e41] * wedge[e321]) + (other[e31] * wedge[e412]) + (other[e412] * wedge[e31]) + (other[e321] * wedge[e41]), (other[e42] * wedge[e321]) + (other[e12] * wedge[e423]) + (other[e423] * wedge[e12]) + (other[e321] * wedge[e42]), (other[e43] * wedge[e321]) + (other[e23] * wedge[e431]) + (other[e431] * wedge[e23]) + (other[e321] * wedge[e43]), -(other[e43] * wedge[e412]) - (other[e423] * wedge[e41]) - (other[e431] * wedge[e42]) - (other[e412] * wedge[e43])) + (float4(other[e1234]) * wedge.group1()) + (float4(wedge[e1234]) * other.group1()) - (wedge.group4().yzxx * float4(other.group3().zxy, other[e41])) - float4((wedge.group3().zxy * other.group4().yzx), other[e42] * wedge[e431]), /* e41, e42, e43 */(float3(other[e1234]) * wedge.group2()) + (float3(wedge[e1234]) * other.group2()) + (other.group4().zxy * wedge.group4().yzx) - (other.group4().yzx * wedge.group4().zxy), /* e23, e31, e12 */(float3(other[e1234]) * wedge.group3()) + (float3(wedge[e1234]) * other.group3()) + (float3(wedge[e321]) * other.group4().xyz) - (float3(other[e321]) * wedge.group4().xyz), /* e423, e431, e412, e321 */(float4(other[e1234]) * wedge.group4()) + (float4(wedge[e1234]) * other.group4()));
    }
}
impl ProjectOrthogonallyOnto<Plane> for DualNum {
    type Output = Scalar;
// Operative Statistics for this implementation:
//      add/sub      mul      div
// f32        0        1        0
    fn project_orthogonally_onto(self, other: Plane) -> Self::Output {
        use crate::elements::*;
        return Scalar::from_groups(/* scalar */powi(other[e321], 2) * self[scalar]);
    }
}
impl ProjectOrthogonallyOnto<Point> for DualNum {
    type Output = Scalar;
// Operative Statistics for this implementation:
//           add/sub      mul      div
//      f32        3        4        0
//    simd4        0        1        0
// Totals...
// yes simd        3        5        0
//  no simd        3        8        0
    fn project_orthogonally_onto(self, other: Point) -> Self::Output {
        use crate::elements::*;
let wedge = Plane::from_groups(/* e423, e431, e412, e321 */float4(self[scalar]) * float4(other[e1], other[e2], other[e3], 0.0));
        return Scalar::from_groups(/* scalar */(wedge[e423] * other[e1]) + (wedge[e431] * other[e2]) + (wedge[e412] * other[e3]) + (wedge[e321] * other[e4]));
    }
}
impl ProjectOrthogonallyOnto<Scalar> for DualNum {
    type Output = Scalar;
// Operative Statistics for this implementation:
//      add/sub      mul      div
// f32        0        1        0
    fn project_orthogonally_onto(self, other: Scalar) -> Self::Output {
        use crate::elements::*;
        return Scalar::from_groups(/* scalar */powi(other[scalar], 2) * self[scalar]);
    }
}
impl std::ops::Div<project_orthogonally_onto> for Flector {
    type Output = project_orthogonally_onto_partial<Flector>;
    fn div(self, _rhs: project_orthogonally_onto) -> Self::Output {
        project_orthogonally_onto_partial(self)
    }
}
impl ProjectOrthogonallyOnto<Flector> for Flector {
    type Output = Flector;
// Operative Statistics for this implementation:
//           add/sub      mul      div
//      f32        9       15        0
//    simd3        1        2        0
//    simd4        4        5        0
// Totals...
// yes simd       14       22        0
//  no simd       28       41        0
    fn project_orthogonally_onto(self, other: Flector) -> Self::Output {
        use crate::elements::*;
let right_anti_dual = Flector::from_groups(/* e1, e2, e3, e4 */float4(float3(0.0), other[e321] * -1.0), /* e423, e431, e412, e321 */float4(other[e1], other[e2], other[e3], 0.0));
let wedge = Motor::from_groups(/* e41, e42, e43, e1234 */(self.group0().wwwx * float4(right_anti_dual.group0().xyz, right_anti_dual[e423])) + float4(float3(0.0), (right_anti_dual[e431] * self[e2]) + (right_anti_dual[e412] * self[e3]) + (right_anti_dual[e321] * self[e4]) - (right_anti_dual[e2] * self[e431]) - (right_anti_dual[e3] * self[e412]) - (right_anti_dual[e4] * self[e321])) - (right_anti_dual.group0().wwwx * float4(self.group0().xyz, self[e423])), /* e23, e31, e12, scalar */float4(((right_anti_dual.group0().zxy * self.group0().yzx) - (right_anti_dual.group0().yzx * self.group0().zxy)), 0.0));
        return Flector::from_groups(/* e1, e2, e3, e4 */float4((other[e412] * wedge[e31]) + (other[e321] * wedge[e41]), (other[e423] * wedge[e12]) + (other[e321] * wedge[e42]), (other[e431] * wedge[e23]) + (other[e321] * wedge[e43]), -(other[e431] * wedge[e42]) - (other[e412] * wedge[e43])) + (float4(wedge[e1234]) * other.group0()) - (other.group1().yzxx * float4(wedge.group1().zxy, wedge[e41])), /* e423, e431, e412, e321 */float4(wedge[e1234]) * other.group1());
    }
}
impl ProjectOrthogonallyOnto<Horizon> for Flector {
    type Output = Flector;
// Operative Statistics for this implementation:
//           add/sub      mul      div
//      f32        0        2        0
//    simd4        0        4        0
// Totals...
// yes simd        0        6        0
//  no simd        0       18        0
    fn project_orthogonally_onto(self, other: Horizon) -> Self::Output {
        use crate::elements::*;
let wedge = Motor::from_groups(/* e41, e42, e43, e1234 */float4(other[e321] * -1.0) * float4(self.group0().xyz, self[e321]) * float4(-1.0), /* e23, e31, e12, scalar */float4(0.0));
        return Flector::from_groups(/* e1, e2, e3, e4 */float4(other[e321], other[e321], other[e321], 0.0) * float4(wedge.group0().xyz, 0.0) * float4(1.0, 1.0, 1.0, 0.0), /* e423, e431, e412, e321 */float4(float3(0.0), other[e321] * wedge[e1234]));
    }
}
impl ProjectOrthogonallyOnto<Line> for Flector {
    type Output = Point;
// Operative Statistics for this implementation:
//           add/sub      mul      div
//      f32        4        8        0
//    simd3        0        4        0
//    simd4        3        1        0
// Totals...
// yes simd        7       13        0
//  no simd       16       24        0
    fn project_orthogonally_onto(self, other: Line) -> Self::Output {
        use crate::elements::*;
let right_anti_dual = Line::from_groups(/* e41, e42, e43 */other.group1() * float3(-1.0), /* e23, e31, e12 */float3(0.0));
let wedge = Plane::from_groups(/* e423, e431, e412, e321 */float4((float3(self[e4]) * right_anti_dual.group1()), 0.0) + float4((right_anti_dual.group0().yzx * self.group0().zxy), 0.0) - float4((right_anti_dual.group0().zxy * self.group0().yzx), 0.0));
        return Point::from_groups(/* e1, e2, e3, e4 */float4((other[e41] * wedge[e321]) + (other[e31] * wedge[e412]), (other[e42] * wedge[e321]) + (other[e12] * wedge[e423]), (other[e43] * wedge[e321]) + (other[e23] * wedge[e431]), -(other[e42] * wedge[e431]) - (other[e43] * wedge[e412])) - (wedge.group0().yzxx * float4(other.group1().zxy, other[e41])));
    }
}
impl ProjectOrthogonallyOnto<Motor> for Flector {
    type Output = Flector;
// Operative Statistics for this implementation:
//           add/sub      mul      div
//      f32        4        8        0
//    simd3        0        4        0
//    simd4        5        5        0
// Totals...
// yes simd        9       17        0
//  no simd       24       40        0
    fn project_orthogonally_onto(self, other: Motor) -> Self::Output {
        use crate::elements::*;
let right_anti_dual = Motor::from_groups(/* e41, e42, e43, e1234 */other.group1() * float4(-1.0, -1.0, -1.0, 1.0), /* e23, e31, e12, scalar */float4(0.0));
let wedge = Flector::from_groups(/* e1, e2, e3, e4 */float4(right_anti_dual[scalar]) * self.group0(), /* e423, e431, e412, e321 */float4((float3(self[e4]) * right_anti_dual.group1().xyz), 0.0) + float4((float3(right_anti_dual[scalar]) * self.group1().xyz), 0.0) + float4((self.group0().zxy * right_anti_dual.group0().yzx), 0.0) - float4((self.group0().yzx * right_anti_dual.group0().zxy), 0.0));
        return Flector::from_groups(/* e1, e2, e3, e4 */float4((wedge[e412] * other[e31]) + (wedge[e321] * other[e41]), (wedge[e423] * other[e12]) + (wedge[e321] * other[e42]), (wedge[e431] * other[e23]) + (wedge[e321] * other[e43]), -(wedge[e431] * other[e42]) - (wedge[e412] * other[e43])) + (float4(other[e1234]) * wedge.group0()) - (wedge.group1().yzxx * float4(other.group1().zxy, other[e41])), /* e423, e431, e412, e321 */float4(other[e1234]) * wedge.group1());
    }
}
impl ProjectOrthogonallyOnto<MultiVector> for Flector {
    type Output = MultiVector;
// Operative Statistics for this implementation:
//           add/sub      mul      div
//      f32       34       43        0
//    simd3        8       18        0
//    simd4        8        6        0
// Totals...
// yes simd       50       67        0
//  no simd       90      121        0
    fn project_orthogonally_onto(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
let right_anti_dual = MultiVector::from_groups(/* scalar, e1234 */float2(0.0, other[scalar]), /* e1, e2, e3, e4 */float4(float3(0.0), other[e321] * -1.0), /* e41, e42, e43 */other.group3() * float3(-1.0), /* e23, e31, e12 */float3(0.0), /* e423, e431, e412, e321 */float4(other[e1], other[e2], other[e3], 0.0));
let wedge = MultiVector::from_groups(/* scalar, e1234 */float2(0.0, (self[e1] * right_anti_dual[e423]) + (self[e2] * right_anti_dual[e431]) + (self[e3] * right_anti_dual[e412]) + (self[e4] * right_anti_dual[e321]) - (self[e423] * right_anti_dual[e1]) - (self[e431] * right_anti_dual[e2]) - (self[e412] * right_anti_dual[e3]) - (self[e321] * right_anti_dual[e4])), /* e1, e2, e3, e4 */float4(right_anti_dual[scalar]) * self.group0(), /* e41, e42, e43 */(float3(self[e4]) * right_anti_dual.group1().xyz) - (float3(right_anti_dual[e4]) * self.group0().xyz), /* e23, e31, e12 */(self.group0().yzx * right_anti_dual.group1().zxy) - (self.group0().zxy * right_anti_dual.group1().yzx), /* e423, e431, e412, e321 */float4((float3(self[e4]) * right_anti_dual.group3()), 0.0) + float4((float3(right_anti_dual[scalar]) * self.group1().xyz), 0.0) + float4((right_anti_dual.group2().yzx * self.group0().zxy), 0.0) - float4((right_anti_dual.group2().zxy * self.group0().yzx), 0.0));
        return MultiVector::from_groups(/* scalar, e1234 */float2((other[scalar] * wedge[e1234]) + (other[e1234] * wedge[scalar]) + (other[e1] * wedge[e423]) + (other[e2] * wedge[e431]) + (other[e3] * wedge[e412]) + (other[e4] * wedge[e321]) - (other[e41] * wedge[e23]) - (other[e42] * wedge[e31]) - (other[e43] * wedge[e12]) - (other[e23] * wedge[e41]) - (other[e31] * wedge[e42]) - (other[e12] * wedge[e43]) - (other[e423] * wedge[e1]) - (other[e431] * wedge[e2]) - (other[e412] * wedge[e3]) - (other[e321] * wedge[e4]), other[e1234] * wedge[e1234]), /* e1, e2, e3, e4 */float4((other[e41] * wedge[e321]) + (other[e31] * wedge[e412]) + (other[e412] * wedge[e31]) + (other[e321] * wedge[e41]), (other[e42] * wedge[e321]) + (other[e12] * wedge[e423]) + (other[e423] * wedge[e12]) + (other[e321] * wedge[e42]), (other[e43] * wedge[e321]) + (other[e23] * wedge[e431]) + (other[e431] * wedge[e23]) + (other[e321] * wedge[e43]), -(other[e43] * wedge[e412]) - (other[e423] * wedge[e41]) - (other[e431] * wedge[e42]) - (other[e412] * wedge[e43])) + (float4(other[e1234]) * wedge.group1()) + (float4(wedge[e1234]) * other.group1()) - (wedge.group4().yzxx * float4(other.group3().zxy, other[e41])) - float4((wedge.group3().zxy * other.group4().yzx), other[e42] * wedge[e431]), /* e41, e42, e43 */(float3(other[e1234]) * wedge.group2()) + (float3(wedge[e1234]) * other.group2()) + (other.group4().zxy * wedge.group4().yzx) - (other.group4().yzx * wedge.group4().zxy), /* e23, e31, e12 */(float3(other[e1234]) * wedge.group3()) + (float3(wedge[e1234]) * other.group3()) + (float3(wedge[e321]) * other.group4().xyz) - (float3(other[e321]) * wedge.group4().xyz), /* e423, e431, e412, e321 */(float4(other[e1234]) * wedge.group4()) + (float4(wedge[e1234]) * other.group4()));
    }
}
impl ProjectOrthogonallyOnto<Plane> for Flector {
    type Output = Flector;
// Operative Statistics for this implementation:
//           add/sub      mul      div
//      f32        4        9        0
//    simd4        1        4        0
// Totals...
// yes simd        5       13        0
//  no simd        8       25        0
    fn project_orthogonally_onto(self, other: Plane) -> Self::Output {
        use crate::elements::*;
let wedge = Motor::from_groups(/* e41, e42, e43, e1234 */float4(other[e321] * -1.0) * float4(self.group0().xyz, self[e321]) * float4(-1.0), /* e23, e31, e12, scalar */float4(0.0));
        return Flector::from_groups(/* e1, e2, e3, e4 */float4((wedge[e41] * other[e321]) + (wedge[e31] * other[e412]), (wedge[e42] * other[e321]) + (wedge[e12] * other[e423]), (wedge[e43] * other[e321]) + (wedge[e23] * other[e431]), -(wedge[e42] * other[e431]) - (wedge[e43] * other[e412])) - (other.group0().yzxx * float4(wedge.group1().zxy, wedge[e41])), /* e423, e431, e412, e321 */float4(wedge[e1234]) * other.group0());
    }
}
impl ProjectOrthogonallyOnto<Point> for Flector {
    type Output = Point;
// Operative Statistics for this implementation:
//           add/sub      mul      div
//      f32        3        4        0
//    simd4        0        1        0
// Totals...
// yes simd        3        5        0
//  no simd        3        8        0
    fn project_orthogonally_onto(self, other: Point) -> Self::Output {
        use crate::elements::*;
let right_anti_dual = Plane::from_groups(/* e423, e431, e412, e321 */float4(other[e1], other[e2], other[e3], 0.0));
        return Point::from_groups(/* e1, e2, e3, e4 */float4((self[e1] * right_anti_dual[e423]) + (self[e2] * right_anti_dual[e431]) + (self[e3] * right_anti_dual[e412]) + (self[e4] * right_anti_dual[e321])) * other.group0());
    }
}
impl std::ops::Div<project_orthogonally_onto> for Horizon {
    type Output = project_orthogonally_onto_partial<Horizon>;
    fn div(self, _rhs: project_orthogonally_onto) -> Self::Output {
        project_orthogonally_onto_partial(self)
    }
}
impl ProjectOrthogonallyOnto<Flector> for Horizon {
    type Output = Flector;
// Operative Statistics for this implementation:
//           add/sub      mul      div
//      f32        0        3        0
//    simd4        0        2        0
// Totals...
// yes simd        0        5        0
//  no simd        0       11        0
    fn project_orthogonally_onto(self, other: Flector) -> Self::Output {
        use crate::elements::*;
let wedge = AntiScalar::from_groups(/* e1234 */float4(float3(0.0), other[e321] * -1.0)[3] * self[e321] * -1.0);
        return Flector::from_groups(/* e1, e2, e3, e4 */float4(wedge[e1234]) * other.group0(), /* e423, e431, e412, e321 */float4(wedge[e1234]) * other.group1());
    }
}
impl ProjectOrthogonallyOnto<Horizon> for Horizon {
    type Output = Horizon;
// Operative Statistics for this implementation:
//      add/sub      mul      div
// f32        0        1        0
    fn project_orthogonally_onto(self, other: Horizon) -> Self::Output {
        use crate::elements::*;
        return Horizon::from_groups(/* e321 */powi(other[e321], 2) * self[e321]);
    }
}
impl ProjectOrthogonallyOnto<Motor> for Horizon {
    type Output = Flector;
// Operative Statistics for this implementation:
//           add/sub      mul      div
//      f32        0        2        0
//    simd4        0        2        0
// Totals...
// yes simd        0        4        0
//  no simd        0       10        0
    fn project_orthogonally_onto(self, other: Motor) -> Self::Output {
        use crate::elements::*;
let wedge = Horizon::from_groups(/* e321 */self[e321] * 0.0);
        return Flector::from_groups(/* e1, e2, e3, e4 */float4(wedge[e321], wedge[e321], wedge[e321], 0.0) * float4(other.group0().xyz, 0.0) * float4(1.0, 1.0, 1.0, 0.0), /* e423, e431, e412, e321 */float4(float3(0.0), wedge[e321] * other[e1234]));
    }
}
impl ProjectOrthogonallyOnto<MultiVector> for Horizon {
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
    fn project_orthogonally_onto(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
let right_anti_dual = MultiVector::from_groups(/* scalar, e1234 */float2(0.0, other[scalar]), /* e1, e2, e3, e4 */float4(float3(0.0), other[e321] * -1.0), /* e41, e42, e43 */other.group3() * float3(-1.0), /* e23, e31, e12 */float3(0.0), /* e423, e431, e412, e321 */float4(other[e1], other[e2], other[e3], 0.0));
let wedge = MultiVector::from_groups(/* scalar, e1234 */float2(1.0, self[e321] * right_anti_dual[e4]) * float2(0.0, -1.0), /* e1, e2, e3, e4 */float4(0.0), /* e41, e42, e43 */float3(0.0), /* e23, e31, e12 */float3(0.0), /* e423, e431, e412, e321 */float4(float3(0.0), self[e321] * right_anti_dual[scalar]));
        return MultiVector::from_groups(/* scalar, e1234 */float2((other[scalar] * wedge[e1234]) + (other[e1234] * wedge[scalar]) + (other[e1] * wedge[e423]) + (other[e2] * wedge[e431]) + (other[e3] * wedge[e412]) + (other[e4] * wedge[e321]) - (other[e41] * wedge[e23]) - (other[e42] * wedge[e31]) - (other[e43] * wedge[e12]) - (other[e23] * wedge[e41]) - (other[e31] * wedge[e42]) - (other[e12] * wedge[e43]) - (other[e423] * wedge[e1]) - (other[e431] * wedge[e2]) - (other[e412] * wedge[e3]) - (other[e321] * wedge[e4]), other[e1234] * wedge[e1234]), /* e1, e2, e3, e4 */float4((other[e41] * wedge[e321]) + (other[e31] * wedge[e412]) + (other[e412] * wedge[e31]) + (other[e321] * wedge[e41]), (other[e42] * wedge[e321]) + (other[e12] * wedge[e423]) + (other[e423] * wedge[e12]) + (other[e321] * wedge[e42]), (other[e43] * wedge[e321]) + (other[e23] * wedge[e431]) + (other[e431] * wedge[e23]) + (other[e321] * wedge[e43]), -(other[e43] * wedge[e412]) - (other[e423] * wedge[e41]) - (other[e431] * wedge[e42]) - (other[e412] * wedge[e43])) + (float4(other[e1234]) * wedge.group1()) + (float4(wedge[e1234]) * other.group1()) - (wedge.group4().yzxx * float4(other.group3().zxy, other[e41])) - float4((wedge.group3().zxy * other.group4().yzx), other[e42] * wedge[e431]), /* e41, e42, e43 */(float3(other[e1234]) * wedge.group2()) + (float3(wedge[e1234]) * other.group2()) + (other.group4().zxy * wedge.group4().yzx) - (other.group4().yzx * wedge.group4().zxy), /* e23, e31, e12 */(float3(other[e1234]) * wedge.group3()) + (float3(wedge[e1234]) * other.group3()) + (float3(wedge[e321]) * other.group4().xyz) - (float3(other[e321]) * wedge.group4().xyz), /* e423, e431, e412, e321 */(float4(other[e1234]) * wedge.group4()) + (float4(wedge[e1234]) * other.group4()));
    }
}
impl ProjectOrthogonallyOnto<Plane> for Horizon {
    type Output = Plane;
// Operative Statistics for this implementation:
//           add/sub      mul      div
//      f32        0        1        0
//    simd4        0        1        0
// Totals...
// yes simd        0        2        0
//  no simd        0        5        0
    fn project_orthogonally_onto(self, other: Plane) -> Self::Output {
        use crate::elements::*;
        return Plane::from_groups(/* e423, e431, e412, e321 */float4(self[e321] * other[e321]) * other.group0());
    }
}
impl std::ops::Div<project_orthogonally_onto> for Line {
    type Output = project_orthogonally_onto_partial<Line>;
    fn div(self, _rhs: project_orthogonally_onto) -> Self::Output {
        project_orthogonally_onto_partial(self)
    }
}
impl ProjectOrthogonallyOnto<Flector> for Line {
    type Output = Motor;
// Operative Statistics for this implementation:
//           add/sub      mul      div
//      f32        6       18        0
//    simd3        1        2        0
//    simd4        2        2        0
// Totals...
// yes simd        9       22        0
//  no simd       17       32        0
    fn project_orthogonally_onto(self, other: Flector) -> Self::Output {
        use crate::elements::*;
let right_anti_dual = Flector::from_groups(/* e1, e2, e3, e4 */float4(float3(0.0), other[e321] * -1.0), /* e423, e431, e412, e321 */float4(other[e1], other[e2], other[e3], 0.0));
let wedge = Plane::from_groups(/* e423, e431, e412, e321 */float4((right_anti_dual[e3] * self[e42]) + (right_anti_dual[e4] * self[e23]), (right_anti_dual[e1] * self[e43]) + (right_anti_dual[e4] * self[e31]), (right_anti_dual[e2] * self[e41]) + (right_anti_dual[e4] * self[e12]), -(right_anti_dual[e2] * self[e31]) - (right_anti_dual[e3] * self[e12])) - (right_anti_dual.group0().yzxx * float4(self.group0().zxy, self[e23])));
        return Motor::from_groups(/* e41, e42, e43, e1234 */float4(((other.group1().zxy * wedge.group0().yzx) - (other.group1().yzx * wedge.group0().zxy)), 0.0), /* e23, e31, e12, scalar */float4(other[e321] * wedge[e423] * -1.0, other[e321] * wedge[e431] * -1.0, other[e321] * wedge[e412] * -1.0, (other[e2] * wedge[e431]) + (other[e3] * wedge[e412]) + (other[e4] * wedge[e321])) + (wedge.group0().wwwx * float4(other.group1().xyz, other[e1])));
    }
}
impl ProjectOrthogonallyOnto<Horizon> for Line {
    type Output = Line;
// Operative Statistics for this implementation:
//           add/sub      mul      div
//      f32        0        4        0
//    simd3        0        2        0
// Totals...
// yes simd        0        6        0
//  no simd        0       10        0
    fn project_orthogonally_onto(self, other: Horizon) -> Self::Output {
        use crate::elements::*;
let right_anti_dual = Origin::from_groups(/* e4 */other[e321] * -1.0);
        return Line::from_groups(/* e41, e42, e43 */float3(0.0), /* e23, e31, e12 */float3(other[e321]) * float4(self[e23] * right_anti_dual[e4], self[e31] * right_anti_dual[e4], self[e12] * right_anti_dual[e4], 0.0).xyz * float3(-1.0));
    }
}
impl ProjectOrthogonallyOnto<Line> for Line {
    type Output = Line;
// Operative Statistics for this implementation:
//           add/sub      mul      div
//      f32        5        6        0
//    simd3        0        3        0
// Totals...
// yes simd        5        9        0
//  no simd        5       15        0
    fn project_orthogonally_onto(self, other: Line) -> Self::Output {
        use crate::elements::*;
let right_anti_dual = Line::from_groups(/* e41, e42, e43 */other.group1() * float3(-1.0), /* e23, e31, e12 */float3(0.0));
let wedge = AntiScalar::from_groups(/* e1234 */-(right_anti_dual[e41] * self[e23]) - (right_anti_dual[e42] * self[e31]) - (right_anti_dual[e43] * self[e12]) - (right_anti_dual[e23] * self[e41]) - (right_anti_dual[e31] * self[e42]) - (right_anti_dual[e12] * self[e43]));
        return Line::from_groups(/* e41, e42, e43 */float3(wedge[e1234]) * other.group0(), /* e23, e31, e12 */float3(wedge[e1234]) * other.group1());
    }
}
impl ProjectOrthogonallyOnto<Motor> for Line {
    type Output = Motor;
// Operative Statistics for this implementation:
//           add/sub      mul      div
//      f32       10       13        0
//    simd3        1        2        0
//    simd4        2        7        0
// Totals...
// yes simd       13       22        0
//  no simd       21       47        0
    fn project_orthogonally_onto(self, other: Motor) -> Self::Output {
        use crate::elements::*;
let right_anti_dual = Motor::from_groups(/* e41, e42, e43, e1234 */other.group1() * float4(-1.0, -1.0, -1.0, 1.0), /* e23, e31, e12, scalar */float4(0.0));
let wedge = Motor::from_groups(/* e41, e42, e43, e1234 */float4(right_anti_dual[scalar], right_anti_dual[scalar], right_anti_dual[scalar], 1.0) * float4(self.group0(), -(self[e41] * right_anti_dual[e23]) - (self[e42] * right_anti_dual[e31]) - (self[e43] * right_anti_dual[e12]) - (self[e23] * right_anti_dual[e41]) - (self[e31] * right_anti_dual[e42]) - (self[e12] * right_anti_dual[e43])), /* e23, e31, e12, scalar */float4(float3(1.0), 0.0) * float4(self.group1(), 0.0) * float4(right_anti_dual.group1().www, 0.0) * float4(1.0, 1.0, 1.0, 0.0));
        return Motor::from_groups(/* e41, e42, e43, e1234 */float4(((float3(other[e1234]) * wedge.group0().xyz) + (float3(wedge[e1234]) * other.group0().xyz)), other[e1234] * wedge[e1234]), /* e23, e31, e12, scalar */(float4(other[e1234]) * wedge.group1()) + (float4(wedge[e1234]) * other.group1()) + float4(float3(0.0), -(other[e41] * wedge[e23]) - (other[e42] * wedge[e31]) - (other[e43] * wedge[e12]) - (other[e23] * wedge[e41]) - (other[e31] * wedge[e42]) - (other[e12] * wedge[e43])));
    }
}
impl ProjectOrthogonallyOnto<MultiVector> for Line {
    type Output = MultiVector;
// Operative Statistics for this implementation:
//           add/sub      mul      div
//      f32       36       49        0
//    simd3        6       12        0
//    simd4        6        6        0
// Totals...
// yes simd       48       67        0
//  no simd       78      109        0
    fn project_orthogonally_onto(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
let right_anti_dual = MultiVector::from_groups(/* scalar, e1234 */float2(0.0, other[scalar]), /* e1, e2, e3, e4 */float4(float3(0.0), other[e321] * -1.0), /* e41, e42, e43 */other.group3() * float3(-1.0), /* e23, e31, e12 */float3(0.0), /* e423, e431, e412, e321 */float4(other[e1], other[e2], other[e3], 0.0));
let wedge = MultiVector::from_groups(/* scalar, e1234 */float2(0.0, -(self[e41] * right_anti_dual[e23]) - (self[e42] * right_anti_dual[e31]) - (self[e43] * right_anti_dual[e12]) - (self[e23] * right_anti_dual[e41]) - (self[e31] * right_anti_dual[e42]) - (self[e12] * right_anti_dual[e43])), /* e1, e2, e3, e4 */float4(0.0), /* e41, e42, e43 */float3(right_anti_dual[scalar]) * self.group0(), /* e23, e31, e12 */float3(right_anti_dual[scalar]) * self.group1(), /* e423, e431, e412, e321 */float4((self[e42] * right_anti_dual[e3]) + (self[e23] * right_anti_dual[e4]), (self[e43] * right_anti_dual[e1]) + (self[e31] * right_anti_dual[e4]), (self[e41] * right_anti_dual[e2]) + (self[e12] * right_anti_dual[e4]), -(self[e31] * right_anti_dual[e2]) - (self[e12] * right_anti_dual[e3])) - (right_anti_dual.group1().yzxx * float4(self.group0().zxy, self[e23])));
        return MultiVector::from_groups(/* scalar, e1234 */float2((other[scalar] * wedge[e1234]) + (other[e1234] * wedge[scalar]) + (other[e1] * wedge[e423]) + (other[e2] * wedge[e431]) + (other[e3] * wedge[e412]) + (other[e4] * wedge[e321]) - (other[e41] * wedge[e23]) - (other[e42] * wedge[e31]) - (other[e43] * wedge[e12]) - (other[e23] * wedge[e41]) - (other[e31] * wedge[e42]) - (other[e12] * wedge[e43]) - (other[e423] * wedge[e1]) - (other[e431] * wedge[e2]) - (other[e412] * wedge[e3]) - (other[e321] * wedge[e4]), other[e1234] * wedge[e1234]), /* e1, e2, e3, e4 */float4((other[e41] * wedge[e321]) + (other[e31] * wedge[e412]) + (other[e412] * wedge[e31]) + (other[e321] * wedge[e41]), (other[e42] * wedge[e321]) + (other[e12] * wedge[e423]) + (other[e423] * wedge[e12]) + (other[e321] * wedge[e42]), (other[e43] * wedge[e321]) + (other[e23] * wedge[e431]) + (other[e431] * wedge[e23]) + (other[e321] * wedge[e43]), -(other[e43] * wedge[e412]) - (other[e423] * wedge[e41]) - (other[e431] * wedge[e42]) - (other[e412] * wedge[e43])) + (float4(other[e1234]) * wedge.group1()) + (float4(wedge[e1234]) * other.group1()) - (wedge.group4().yzxx * float4(other.group3().zxy, other[e41])) - float4((wedge.group3().zxy * other.group4().yzx), other[e42] * wedge[e431]), /* e41, e42, e43 */(float3(other[e1234]) * wedge.group2()) + (float3(wedge[e1234]) * other.group2()) + (other.group4().zxy * wedge.group4().yzx) - (other.group4().yzx * wedge.group4().zxy), /* e23, e31, e12 */(float3(other[e1234]) * wedge.group3()) + (float3(wedge[e1234]) * other.group3()) + (float3(wedge[e321]) * other.group4().xyz) - (float3(other[e321]) * wedge.group4().xyz), /* e423, e431, e412, e321 */(float4(other[e1234]) * wedge.group4()) + (float4(wedge[e1234]) * other.group4()));
    }
}
impl ProjectOrthogonallyOnto<Plane> for Line {
    type Output = Line;
// Operative Statistics for this implementation:
//           add/sub      mul      div
//      f32        0        1        0
//    simd3        2        4        0
//    simd4        0        2        0
// Totals...
// yes simd        2        7        0
//  no simd        6       21        0
    fn project_orthogonally_onto(self, other: Plane) -> Self::Output {
        use crate::elements::*;
let right_anti_dual = Origin::from_groups(/* e4 */other[e321] * -1.0);
let wedge = Plane::from_groups(/* e423, e431, e412, e321 */float4(right_anti_dual[e4], right_anti_dual[e4], right_anti_dual[e4], 0.0) * float4(self.group1(), 0.0) * float4(1.0, 1.0, 1.0, 0.0));
        return Line::from_groups(/* e41, e42, e43 */(other.group0().zxy * wedge.group0().yzx) - (other.group0().yzx * wedge.group0().zxy), /* e23, e31, e12 */(float3(wedge[e321]) * other.group0().xyz) - (float3(other[e321]) * wedge.group0().xyz));
    }
}
impl std::ops::Div<project_orthogonally_onto> for Motor {
    type Output = project_orthogonally_onto_partial<Motor>;
    fn div(self, _rhs: project_orthogonally_onto) -> Self::Output {
        project_orthogonally_onto_partial(self)
    }
}
impl ProjectOrthogonallyOnto<DualNum> for Motor {
    type Output = DualNum;
// Operative Statistics for this implementation:
//           add/sub      mul      div
//      f32        0        1        0
//    simd2        0        1        0
// Totals...
// yes simd        0        2        0
//  no simd        0        3        0
    fn project_orthogonally_onto(self, other: DualNum) -> Self::Output {
        use crate::elements::*;
        return DualNum::from_groups(/* scalar, e1234 */float2(other[scalar] * self[scalar]) * other.group0());
    }
}
impl ProjectOrthogonallyOnto<Flector> for Motor {
    type Output = Motor;
// Operative Statistics for this implementation:
//           add/sub      mul      div
//      f32        9       16        0
//    simd3        1        3        0
//    simd4        4        4        0
// Totals...
// yes simd       14       23        0
//  no simd       28       41        0
    fn project_orthogonally_onto(self, other: Flector) -> Self::Output {
        use crate::elements::*;
let right_anti_dual = Flector::from_groups(/* e1, e2, e3, e4 */float4(float3(0.0), other[e321] * -1.0), /* e423, e431, e412, e321 */float4(other[e1], other[e2], other[e3], 0.0));
let wedge = Flector::from_groups(/* e1, e2, e3, e4 */float4(self[scalar]) * right_anti_dual.group0(), /* e423, e431, e412, e321 */float4((right_anti_dual[e4] * self[e23]) + (right_anti_dual[e423] * self[scalar]), (right_anti_dual[e4] * self[e31]) + (right_anti_dual[e431] * self[scalar]), (right_anti_dual[e4] * self[e12]) + (right_anti_dual[e412] * self[scalar]), -(right_anti_dual[e2] * self[e31]) - (right_anti_dual[e3] * self[e12])) + float4((right_anti_dual.group0().zxy * self.group0().yzx), right_anti_dual[e321] * self[scalar]) - (right_anti_dual.group0().yzxx * float4(self.group0().zxy, self[e23])));
        return Motor::from_groups(/* e41, e42, e43, e1234 */float4(((other.group1().zxy * wedge.group1().yzx) - (other.group1().yzx * wedge.group1().zxy)), 0.0), /* e23, e31, e12, scalar */(wedge.group1().wwwx * float4(other.group1().xyz, other[e1])) + float4(float3(0.0), (other[e2] * wedge[e431]) + (other[e3] * wedge[e412]) + (other[e4] * wedge[e321]) - (other[e431] * wedge[e2]) - (other[e412] * wedge[e3]) - (other[e321] * wedge[e4])) - (other.group1().wwwx * float4(wedge.group1().xyz, wedge[e1])));
    }
}
impl ProjectOrthogonallyOnto<Horizon> for Motor {
    type Output = Motor;
// Operative Statistics for this implementation:
//           add/sub      mul      div
//      f32        0        2        0
//    simd4        0        4        0
// Totals...
// yes simd        0        6        0
//  no simd        0       18        0
    fn project_orthogonally_onto(self, other: Horizon) -> Self::Output {
        use crate::elements::*;
let right_anti_dual = Origin::from_groups(/* e4 */other[e321] * -1.0);
let wedge = Flector::from_groups(/* e1, e2, e3, e4 */float4(float3(0.0), self[scalar] * right_anti_dual[e4]), /* e423, e431, e412, e321 */float4(right_anti_dual[e4], right_anti_dual[e4], right_anti_dual[e4], 0.0) * float4(self.group1().xyz, 0.0) * float4(1.0, 1.0, 1.0, 0.0));
        return Motor::from_groups(/* e41, e42, e43, e1234 */float4(0.0), /* e23, e31, e12, scalar */float4(other[e321]) * float4(wedge.group1().xyz, wedge[e4]) * float4(-1.0));
    }
}
impl ProjectOrthogonallyOnto<Line> for Motor {
    type Output = Motor;
// Operative Statistics for this implementation:
//           add/sub      mul      div
//      f32       10       12        0
//    simd3        0        1        0
//    simd4        0        8        0
// Totals...
// yes simd       10       21        0
//  no simd       10       47        0
    fn project_orthogonally_onto(self, other: Line) -> Self::Output {
        use crate::elements::*;
let right_anti_dual = Line::from_groups(/* e41, e42, e43 */other.group1() * float3(-1.0), /* e23, e31, e12 */float3(0.0));
let wedge = Motor::from_groups(/* e41, e42, e43, e1234 */float4(self[scalar], self[scalar], self[scalar], 1.0) * float4(right_anti_dual.group0(), -(right_anti_dual[e41] * self[e23]) - (right_anti_dual[e42] * self[e31]) - (right_anti_dual[e43] * self[e12]) - (right_anti_dual[e23] * self[e41]) - (right_anti_dual[e31] * self[e42]) - (right_anti_dual[e12] * self[e43])), /* e23, e31, e12, scalar */float4(float3(1.0), 0.0) * float4(right_anti_dual.group1(), 0.0) * float4(self.group1().www, 0.0) * float4(1.0, 1.0, 1.0, 0.0));
        return Motor::from_groups(/* e41, e42, e43, e1234 */float4(float3(1.0), 0.0) * float4(other.group0(), 0.0) * float4(wedge.group0().www, 0.0) * float4(1.0, 1.0, 1.0, 0.0), /* e23, e31, e12, scalar */float4(wedge[e1234], wedge[e1234], wedge[e1234], 1.0) * float4(other.group1(), -(other[e41] * wedge[e23]) - (other[e42] * wedge[e31]) - (other[e43] * wedge[e12]) - (other[e23] * wedge[e41]) - (other[e31] * wedge[e42]) - (other[e12] * wedge[e43])));
    }
}
impl ProjectOrthogonallyOnto<Motor> for Motor {
    type Output = Motor;
// Operative Statistics for this implementation:
//           add/sub      mul      div
//      f32       10       14        0
//    simd3        1        2        0
//    simd4        4        5        0
// Totals...
// yes simd       15       21        0
//  no simd       29       40        0
    fn project_orthogonally_onto(self, other: Motor) -> Self::Output {
        use crate::elements::*;
let right_anti_dual = Motor::from_groups(/* e41, e42, e43, e1234 */other.group1() * float4(-1.0, -1.0, -1.0, 1.0), /* e23, e31, e12, scalar */float4(0.0));
let wedge = Motor::from_groups(/* e41, e42, e43, e1234 */(float4(right_anti_dual[scalar]) * self.group0()) + (float4(self[scalar]) * right_anti_dual.group0()) + float4(float3(0.0), -(right_anti_dual[e41] * self[e23]) - (right_anti_dual[e42] * self[e31]) - (right_anti_dual[e43] * self[e12]) - (right_anti_dual[e23] * self[e41]) - (right_anti_dual[e31] * self[e42]) - (right_anti_dual[e12] * self[e43])), /* e23, e31, e12, scalar */float4(float3(0.0), right_anti_dual[scalar] * self[scalar]));
        return Motor::from_groups(/* e41, e42, e43, e1234 */float4(((float3(other[e1234]) * wedge.group0().xyz) + (float3(wedge[e1234]) * other.group0().xyz)), other[e1234] * wedge[e1234]), /* e23, e31, e12, scalar */(float4(other[e1234]) * wedge.group1()) + (float4(wedge[e1234]) * other.group1()) + float4(float3(0.0), -(other[e41] * wedge[e23]) - (other[e42] * wedge[e31]) - (other[e43] * wedge[e12]) - (other[e23] * wedge[e41]) - (other[e31] * wedge[e42]) - (other[e12] * wedge[e43])));
    }
}
impl ProjectOrthogonallyOnto<MultiVector> for Motor {
    type Output = MultiVector;
// Operative Statistics for this implementation:
//           add/sub      mul      div
//      f32       38       53        0
//    simd3        7       13        0
//    simd4        7        7        0
// Totals...
// yes simd       52       73        0
//  no simd       87      120        0
    fn project_orthogonally_onto(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
let right_anti_dual = MultiVector::from_groups(/* scalar, e1234 */float2(0.0, other[scalar]), /* e1, e2, e3, e4 */float4(float3(0.0), other[e321] * -1.0), /* e41, e42, e43 */other.group3() * float3(-1.0), /* e23, e31, e12 */float3(0.0), /* e423, e431, e412, e321 */float4(other[e1], other[e2], other[e3], 0.0));
let wedge = MultiVector::from_groups(/* scalar, e1234 */float2(self[scalar] * right_anti_dual[scalar], (self[e1234] * right_anti_dual[scalar]) + (self[scalar] * right_anti_dual[e1234]) - (self[e41] * right_anti_dual[e23]) - (self[e42] * right_anti_dual[e31]) - (self[e43] * right_anti_dual[e12]) - (self[e23] * right_anti_dual[e41]) - (self[e31] * right_anti_dual[e42]) - (self[e12] * right_anti_dual[e43])), /* e1, e2, e3, e4 */float4(self[scalar]) * right_anti_dual.group1(), /* e41, e42, e43 */(float3(self[scalar]) * right_anti_dual.group2()) + (float3(right_anti_dual[scalar]) * self.group0().xyz), /* e23, e31, e12 */float3(0.0), /* e423, e431, e412, e321 */float4((self[e23] * right_anti_dual[e4]) + (self[scalar] * right_anti_dual[e423]), (self[e31] * right_anti_dual[e4]) + (self[scalar] * right_anti_dual[e431]), (self[e12] * right_anti_dual[e4]) + (self[scalar] * right_anti_dual[e412]), -(self[e31] * right_anti_dual[e2]) - (self[e12] * right_anti_dual[e3])) + float4((self.group0().yzx * right_anti_dual.group1().zxy), self[scalar] * right_anti_dual[e321]) - (right_anti_dual.group1().yzxx * float4(self.group0().zxy, self[e23])));
        return MultiVector::from_groups(/* scalar, e1234 */float2((other[scalar] * wedge[e1234]) + (other[e1234] * wedge[scalar]) + (other[e1] * wedge[e423]) + (other[e2] * wedge[e431]) + (other[e3] * wedge[e412]) + (other[e4] * wedge[e321]) - (other[e41] * wedge[e23]) - (other[e42] * wedge[e31]) - (other[e43] * wedge[e12]) - (other[e23] * wedge[e41]) - (other[e31] * wedge[e42]) - (other[e12] * wedge[e43]) - (other[e423] * wedge[e1]) - (other[e431] * wedge[e2]) - (other[e412] * wedge[e3]) - (other[e321] * wedge[e4]), other[e1234] * wedge[e1234]), /* e1, e2, e3, e4 */float4((other[e41] * wedge[e321]) + (other[e31] * wedge[e412]) + (other[e412] * wedge[e31]) + (other[e321] * wedge[e41]), (other[e42] * wedge[e321]) + (other[e12] * wedge[e423]) + (other[e423] * wedge[e12]) + (other[e321] * wedge[e42]), (other[e43] * wedge[e321]) + (other[e23] * wedge[e431]) + (other[e431] * wedge[e23]) + (other[e321] * wedge[e43]), -(other[e43] * wedge[e412]) - (other[e423] * wedge[e41]) - (other[e431] * wedge[e42]) - (other[e412] * wedge[e43])) + (float4(other[e1234]) * wedge.group1()) + (float4(wedge[e1234]) * other.group1()) - (wedge.group4().yzxx * float4(other.group3().zxy, other[e41])) - float4((wedge.group3().zxy * other.group4().yzx), other[e42] * wedge[e431]), /* e41, e42, e43 */(float3(other[e1234]) * wedge.group2()) + (float3(wedge[e1234]) * other.group2()) + (other.group4().zxy * wedge.group4().yzx) - (other.group4().yzx * wedge.group4().zxy), /* e23, e31, e12 */(float3(other[e1234]) * wedge.group3()) + (float3(wedge[e1234]) * other.group3()) + (float3(wedge[e321]) * other.group4().xyz) - (float3(other[e321]) * wedge.group4().xyz), /* e423, e431, e412, e321 */(float4(other[e1234]) * wedge.group4()) + (float4(wedge[e1234]) * other.group4()));
    }
}
impl ProjectOrthogonallyOnto<Plane> for Motor {
    type Output = Motor;
// Operative Statistics for this implementation:
//           add/sub      mul      div
//      f32        2        8        0
//    simd3        1        2        0
//    simd4        1        3        0
// Totals...
// yes simd        4       13        0
//  no simd        9       26        0
    fn project_orthogonally_onto(self, other: Plane) -> Self::Output {
        use crate::elements::*;
let right_anti_dual = Origin::from_groups(/* e4 */other[e321] * -1.0);
let wedge = Flector::from_groups(/* e1, e2, e3, e4 */float4(float3(0.0), self[scalar] * right_anti_dual[e4]), /* e423, e431, e412, e321 */float4(right_anti_dual[e4], right_anti_dual[e4], right_anti_dual[e4], 0.0) * float4(self.group1().xyz, 0.0) * float4(1.0, 1.0, 1.0, 0.0));
        return Motor::from_groups(/* e41, e42, e43, e1234 */float4(((wedge.group1().yzx * other.group0().zxy) - (wedge.group1().zxy * other.group0().yzx)), 0.0), /* e23, e31, e12, scalar */float4(wedge[e321] * other[e423], wedge[e321] * other[e431], wedge[e321] * other[e412], -(wedge[e2] * other[e431]) - (wedge[e3] * other[e412]) - (wedge[e4] * other[e321])) - (other.group0().wwwx * float4(wedge.group1().xyz, wedge[e1])));
    }
}
impl ProjectOrthogonallyOnto<Point> for Motor {
    type Output = Scalar;
// Operative Statistics for this implementation:
//           add/sub      mul      div
//      f32        3        4        0
//    simd4        0        1        0
// Totals...
// yes simd        3        5        0
//  no simd        3        8        0
    fn project_orthogonally_onto(self, other: Point) -> Self::Output {
        use crate::elements::*;
let wedge = Plane::from_groups(/* e423, e431, e412, e321 */float4(self[scalar]) * float4(other[e1], other[e2], other[e3], 0.0));
        return Scalar::from_groups(/* scalar */(wedge[e423] * other[e1]) + (wedge[e431] * other[e2]) + (wedge[e412] * other[e3]) + (wedge[e321] * other[e4]));
    }
}
impl ProjectOrthogonallyOnto<Scalar> for Motor {
    type Output = Scalar;
// Operative Statistics for this implementation:
//      add/sub      mul      div
// f32        0        1        0
    fn project_orthogonally_onto(self, other: Scalar) -> Self::Output {
        use crate::elements::*;
        return Scalar::from_groups(/* scalar */powi(other[scalar], 2) * self[scalar]);
    }
}
impl std::ops::Div<project_orthogonally_onto> for MultiVector {
    type Output = project_orthogonally_onto_partial<MultiVector>;
    fn div(self, _rhs: project_orthogonally_onto) -> Self::Output {
        project_orthogonally_onto_partial(self)
    }
}
impl ProjectOrthogonallyOnto<DualNum> for MultiVector {
    type Output = DualNum;
// Operative Statistics for this implementation:
//           add/sub      mul      div
//      f32        0        1        0
//    simd2        0        1        0
// Totals...
// yes simd        0        2        0
//  no simd        0        3        0
    fn project_orthogonally_onto(self, other: DualNum) -> Self::Output {
        use crate::elements::*;
        return DualNum::from_groups(/* scalar, e1234 */float2(other[scalar] * self[scalar]) * other.group0());
    }
}
impl ProjectOrthogonallyOnto<Flector> for MultiVector {
    type Output = MultiVector;
// Operative Statistics for this implementation:
//           add/sub      mul      div
//      f32       22       33        0
//    simd3        4        8        0
//    simd4        4        6        0
// Totals...
// yes simd       30       47        0
//  no simd       50       81        0
    fn project_orthogonally_onto(self, other: Flector) -> Self::Output {
        use crate::elements::*;
let right_anti_dual = Flector::from_groups(/* e1, e2, e3, e4 */float4(float3(0.0), other[e321] * -1.0), /* e423, e431, e412, e321 */float4(other[e1], other[e2], other[e3], 0.0));
let wedge = MultiVector::from_groups(/* scalar, e1234 */float2(0.0, (right_anti_dual[e423] * self[e1]) + (right_anti_dual[e431] * self[e2]) + (right_anti_dual[e412] * self[e3]) + (right_anti_dual[e321] * self[e4]) - (right_anti_dual[e1] * self[e423]) - (right_anti_dual[e2] * self[e431]) - (right_anti_dual[e3] * self[e412]) - (right_anti_dual[e4] * self[e321])), /* e1, e2, e3, e4 */float4(self[scalar]) * right_anti_dual.group0(), /* e41, e42, e43 */(float3(self[e4]) * right_anti_dual.group0().xyz) - (float3(right_anti_dual[e4]) * self.group1().xyz), /* e23, e31, e12 */(right_anti_dual.group0().zxy * self.group1().yzx) - (right_anti_dual.group0().yzx * self.group1().zxy), /* e423, e431, e412, e321 */float4((right_anti_dual[e3] * self[e42]) + (right_anti_dual[e4] * self[e23]), (right_anti_dual[e1] * self[e43]) + (right_anti_dual[e4] * self[e31]), (right_anti_dual[e2] * self[e41]) + (right_anti_dual[e4] * self[e12]), -(right_anti_dual[e2] * self[e31]) - (right_anti_dual[e3] * self[e12])) + (float4(self[scalar]) * right_anti_dual.group1()) - (right_anti_dual.group0().yzxx * float4(self.group2().zxy, self[e23])));
        return MultiVector::from_groups(/* scalar, e1234 */float2((other[e1] * wedge[e423]) + (other[e2] * wedge[e431]) + (other[e3] * wedge[e412]) + (other[e4] * wedge[e321]) - (other[e423] * wedge[e1]) - (other[e431] * wedge[e2]) - (other[e412] * wedge[e3]) - (other[e321] * wedge[e4]), 0.0), /* e1, e2, e3, e4 */float4((other[e412] * wedge[e31]) + (other[e321] * wedge[e41]), (other[e423] * wedge[e12]) + (other[e321] * wedge[e42]), (other[e431] * wedge[e23]) + (other[e321] * wedge[e43]), -(other[e431] * wedge[e42]) - (other[e412] * wedge[e43])) + (float4(wedge[e1234]) * other.group0()) - (other.group1().yzxx * float4(wedge.group3().zxy, wedge[e41])), /* e41, e42, e43 */(other.group1().zxy * wedge.group4().yzx) - (other.group1().yzx * wedge.group4().zxy), /* e23, e31, e12 */(float3(wedge[e321]) * other.group1().xyz) - (float3(other[e321]) * wedge.group4().xyz), /* e423, e431, e412, e321 */float4(wedge[e1234]) * other.group1());
    }
}
impl ProjectOrthogonallyOnto<Horizon> for MultiVector {
    type Output = MultiVector;
// Operative Statistics for this implementation:
//           add/sub      mul      div
//      f32        0        5        0
//    simd2        0        2        0
//    simd3        0        4        0
//    simd4        0        4        0
// Totals...
// yes simd        0       15        0
//  no simd        0       37        0
    fn project_orthogonally_onto(self, other: Horizon) -> Self::Output {
        use crate::elements::*;
let right_anti_dual = Origin::from_groups(/* e4 */other[e321] * -1.0);
let wedge = MultiVector::from_groups(/* scalar, e1234 */float2(1.0, self[e321] * right_anti_dual[e4]) * float2(0.0, -1.0), /* e1, e2, e3, e4 */float4(float3(0.0), self[scalar] * right_anti_dual[e4]), /* e41, e42, e43 */float3(right_anti_dual[e4]) * self.group1().xyz * float3(-1.0), /* e23, e31, e12 */float3(0.0), /* e423, e431, e412, e321 */float4(right_anti_dual[e4], right_anti_dual[e4], right_anti_dual[e4], 0.0) * float4(self.group3(), 0.0) * float4(1.0, 1.0, 1.0, 0.0));
        return MultiVector::from_groups(/* scalar, e1234 */float2(other[e321] * wedge[e4], 1.0) * float2(-1.0, 0.0), /* e1, e2, e3, e4 */float4(other[e321], other[e321], other[e321], 0.0) * float4(wedge.group2(), 0.0) * float4(1.0, 1.0, 1.0, 0.0), /* e41, e42, e43 */float3(0.0), /* e23, e31, e12 */float3(other[e321]) * wedge.group4().xyz * float3(-1.0), /* e423, e431, e412, e321 */float4(float3(0.0), other[e321] * wedge[e1234]));
    }
}
impl ProjectOrthogonallyOnto<Line> for MultiVector {
    type Output = MultiVector;
// Operative Statistics for this implementation:
//           add/sub      mul      div
//      f32       14       20        0
//    simd3        0        8        0
//    simd4        3        1        0
// Totals...
// yes simd       17       29        0
//  no simd       26       48        0
    fn project_orthogonally_onto(self, other: Line) -> Self::Output {
        use crate::elements::*;
let right_anti_dual = Line::from_groups(/* e41, e42, e43 */other.group1() * float3(-1.0), /* e23, e31, e12 */float3(0.0));
let wedge = MultiVector::from_groups(/* scalar, e1234 */float2(0.0, -(right_anti_dual[e41] * self[e23]) - (right_anti_dual[e42] * self[e31]) - (right_anti_dual[e43] * self[e12]) - (right_anti_dual[e23] * self[e41]) - (right_anti_dual[e31] * self[e42]) - (right_anti_dual[e12] * self[e43])), /* e1, e2, e3, e4 */float4(0.0), /* e41, e42, e43 */float3(self[scalar]) * right_anti_dual.group0(), /* e23, e31, e12 */float3(self[scalar]) * right_anti_dual.group1(), /* e423, e431, e412, e321 */float4((float3(self[e4]) * right_anti_dual.group1()), 0.0) + float4((right_anti_dual.group0().yzx * self.group1().zxy), 0.0) - float4((right_anti_dual.group0().zxy * self.group1().yzx), 0.0));
        return MultiVector::from_groups(/* scalar, e1234 */float2(-(other[e41] * wedge[e23]) - (other[e42] * wedge[e31]) - (other[e43] * wedge[e12]) - (other[e23] * wedge[e41]) - (other[e31] * wedge[e42]) - (other[e12] * wedge[e43]), 0.0), /* e1, e2, e3, e4 */float4((other[e41] * wedge[e321]) + (other[e31] * wedge[e412]), (other[e42] * wedge[e321]) + (other[e12] * wedge[e423]), (other[e43] * wedge[e321]) + (other[e23] * wedge[e431]), -(other[e42] * wedge[e431]) - (other[e43] * wedge[e412])) - (wedge.group4().yzxx * float4(other.group1().zxy, other[e41])), /* e41, e42, e43 */float3(wedge[e1234]) * other.group0(), /* e23, e31, e12 */float3(wedge[e1234]) * other.group1(), /* e423, e431, e412, e321 */float4(0.0));
    }
}
impl ProjectOrthogonallyOnto<Motor> for MultiVector {
    type Output = MultiVector;
// Operative Statistics for this implementation:
//           add/sub      mul      div
//      f32       18       26        0
//    simd3        3       10        0
//    simd4        5        5        0
// Totals...
// yes simd       26       41        0
//  no simd       47       76        0
    fn project_orthogonally_onto(self, other: Motor) -> Self::Output {
        use crate::elements::*;
let right_anti_dual = Motor::from_groups(/* e41, e42, e43, e1234 */other.group1() * float4(-1.0, -1.0, -1.0, 1.0), /* e23, e31, e12, scalar */float4(0.0));
let wedge = MultiVector::from_groups(/* scalar, e1234 */float2(right_anti_dual[scalar] * self[scalar], (right_anti_dual[e1234] * self[scalar]) + (right_anti_dual[scalar] * self[e1234]) - (right_anti_dual[e41] * self[e23]) - (right_anti_dual[e42] * self[e31]) - (right_anti_dual[e43] * self[e12]) - (right_anti_dual[e23] * self[e41]) - (right_anti_dual[e31] * self[e42]) - (right_anti_dual[e12] * self[e43])), /* e1, e2, e3, e4 */float4(right_anti_dual[scalar]) * self.group1(), /* e41, e42, e43 */(float3(right_anti_dual[scalar]) * self.group2()) + (float3(self[scalar]) * right_anti_dual.group0().xyz), /* e23, e31, e12 */float3(0.0), /* e423, e431, e412, e321 */float4((float3(right_anti_dual[scalar]) * self.group4().xyz), 0.0) + float4((float3(self[e4]) * right_anti_dual.group1().xyz), 0.0) + float4((right_anti_dual.group0().yzx * self.group1().zxy), 0.0) - float4((right_anti_dual.group0().zxy * self.group1().yzx), 0.0));
        return MultiVector::from_groups(/* scalar, e1234 */float2((other[e1234] * wedge[scalar]) + (other[scalar] * wedge[e1234]) - (other[e41] * wedge[e23]) - (other[e42] * wedge[e31]) - (other[e43] * wedge[e12]) - (other[e23] * wedge[e41]) - (other[e31] * wedge[e42]) - (other[e12] * wedge[e43]), other[e1234] * wedge[e1234]), /* e1, e2, e3, e4 */float4((other[e1234] * wedge[e1]) + (other[e31] * wedge[e412]), (other[e1234] * wedge[e2]) + (other[e12] * wedge[e423]), (other[e1234] * wedge[e3]) + (other[e23] * wedge[e431]), -(other[e42] * wedge[e431]) - (other[e43] * wedge[e412])) + (other.group0() * float4(wedge.group4().www, wedge[e4])) - (wedge.group4().yzxx * float4(other.group1().zxy, other[e41])), /* e41, e42, e43 */(float3(other[e1234]) * wedge.group2()) + (float3(wedge[e1234]) * other.group0().xyz), /* e23, e31, e12 */(float3(other[e1234]) * wedge.group3()) + (float3(wedge[e1234]) * other.group1().xyz), /* e423, e431, e412, e321 */float4(other[e1234]) * wedge.group4());
    }
}
impl ProjectOrthogonallyOnto<MultiVector> for MultiVector {
    type Output = MultiVector;
// Operative Statistics for this implementation:
//           add/sub      mul      div
//      f32       54       69        0
//    simd3       12       19        0
//    simd4       10       10        0
// Totals...
// yes simd       76       98        0
//  no simd      130      166        0
    fn project_orthogonally_onto(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
let right_anti_dual = MultiVector::from_groups(/* scalar, e1234 */float2(0.0, other[scalar]), /* e1, e2, e3, e4 */float4(float3(0.0), other[e321] * -1.0), /* e41, e42, e43 */other.group3() * float3(-1.0), /* e23, e31, e12 */float3(0.0), /* e423, e431, e412, e321 */float4(other[e1], other[e2], other[e3], 0.0));
let wedge = MultiVector::from_groups(/* scalar, e1234 */float2(right_anti_dual[scalar] * self[scalar], (right_anti_dual[scalar] * self[e1234]) + (right_anti_dual[e1234] * self[scalar]) + (right_anti_dual[e423] * self[e1]) + (right_anti_dual[e431] * self[e2]) + (right_anti_dual[e412] * self[e3]) + (right_anti_dual[e321] * self[e4]) - (right_anti_dual[e1] * self[e423]) - (right_anti_dual[e2] * self[e431]) - (right_anti_dual[e3] * self[e412]) - (right_anti_dual[e4] * self[e321]) - (right_anti_dual[e41] * self[e23]) - (right_anti_dual[e42] * self[e31]) - (right_anti_dual[e43] * self[e12]) - (right_anti_dual[e23] * self[e41]) - (right_anti_dual[e31] * self[e42]) - (right_anti_dual[e12] * self[e43])), /* e1, e2, e3, e4 */(float4(right_anti_dual[scalar]) * self.group1()) + (float4(self[scalar]) * right_anti_dual.group1()), /* e41, e42, e43 */(float3(right_anti_dual[scalar]) * self.group2()) + (float3(self[scalar]) * right_anti_dual.group2()) + (float3(self[e4]) * right_anti_dual.group1().xyz) - (float3(right_anti_dual[e4]) * self.group1().xyz), /* e23, e31, e12 */(float3(right_anti_dual[scalar]) * self.group3()) + (float3(self[scalar]) * right_anti_dual.group3()) + (right_anti_dual.group1().zxy * self.group1().yzx) - (right_anti_dual.group1().yzx * self.group1().zxy), /* e423, e431, e412, e321 */float4((right_anti_dual[e3] * self[e42]) + (right_anti_dual[e4] * self[e23]) + (right_anti_dual[e42] * self[e3]) + (right_anti_dual[e23] * self[e4]), (right_anti_dual[e1] * self[e43]) + (right_anti_dual[e4] * self[e31]) + (right_anti_dual[e43] * self[e1]) + (right_anti_dual[e31] * self[e4]), (right_anti_dual[e2] * self[e41]) + (right_anti_dual[e4] * self[e12]) + (right_anti_dual[e41] * self[e2]) + (right_anti_dual[e12] * self[e4]), -(right_anti_dual[e1] * self[e23]) - (right_anti_dual[e2] * self[e31]) - (right_anti_dual[e3] * self[e12]) - (right_anti_dual[e12] * self[e3])) + (float4(right_anti_dual[scalar]) * self.group4()) + (float4(self[scalar]) * right_anti_dual.group4()) - (self.group1().yzxx * float4(right_anti_dual.group2().zxy, right_anti_dual[e23])) - float4((self.group2().zxy * right_anti_dual.group1().yzx), right_anti_dual[e31] * self[e2]));
        return MultiVector::from_groups(/* scalar, e1234 */float2((other[scalar] * wedge[e1234]) + (other[e1234] * wedge[scalar]) + (other[e1] * wedge[e423]) + (other[e2] * wedge[e431]) + (other[e3] * wedge[e412]) + (other[e4] * wedge[e321]) - (other[e41] * wedge[e23]) - (other[e42] * wedge[e31]) - (other[e43] * wedge[e12]) - (other[e23] * wedge[e41]) - (other[e31] * wedge[e42]) - (other[e12] * wedge[e43]) - (other[e423] * wedge[e1]) - (other[e431] * wedge[e2]) - (other[e412] * wedge[e3]) - (other[e321] * wedge[e4]), other[e1234] * wedge[e1234]), /* e1, e2, e3, e4 */float4((other[e41] * wedge[e321]) + (other[e31] * wedge[e412]) + (other[e412] * wedge[e31]) + (other[e321] * wedge[e41]), (other[e42] * wedge[e321]) + (other[e12] * wedge[e423]) + (other[e423] * wedge[e12]) + (other[e321] * wedge[e42]), (other[e43] * wedge[e321]) + (other[e23] * wedge[e431]) + (other[e431] * wedge[e23]) + (other[e321] * wedge[e43]), -(other[e43] * wedge[e412]) - (other[e423] * wedge[e41]) - (other[e431] * wedge[e42]) - (other[e412] * wedge[e43])) + (float4(other[e1234]) * wedge.group1()) + (float4(wedge[e1234]) * other.group1()) - (wedge.group4().yzxx * float4(other.group3().zxy, other[e41])) - float4((wedge.group3().zxy * other.group4().yzx), other[e42] * wedge[e431]), /* e41, e42, e43 */(float3(other[e1234]) * wedge.group2()) + (float3(wedge[e1234]) * other.group2()) + (other.group4().zxy * wedge.group4().yzx) - (other.group4().yzx * wedge.group4().zxy), /* e23, e31, e12 */(float3(other[e1234]) * wedge.group3()) + (float3(wedge[e1234]) * other.group3()) + (float3(wedge[e321]) * other.group4().xyz) - (float3(other[e321]) * wedge.group4().xyz), /* e423, e431, e412, e321 */(float4(other[e1234]) * wedge.group4()) + (float4(wedge[e1234]) * other.group4()));
    }
}
impl ProjectOrthogonallyOnto<Plane> for MultiVector {
    type Output = MultiVector;
// Operative Statistics for this implementation:
//           add/sub      mul      div
//      f32        7       15        0
//    simd2        0        1        0
//    simd3        2        6        0
//    simd4        1        4        0
// Totals...
// yes simd       10       26        0
//  no simd       17       51        0
    fn project_orthogonally_onto(self, other: Plane) -> Self::Output {
        use crate::elements::*;
let right_anti_dual = Origin::from_groups(/* e4 */other[e321] * -1.0);
let wedge = MultiVector::from_groups(/* scalar, e1234 */float2(1.0, self[e321] * right_anti_dual[e4]) * float2(0.0, -1.0), /* e1, e2, e3, e4 */float4(float3(0.0), self[scalar] * right_anti_dual[e4]), /* e41, e42, e43 */float3(right_anti_dual[e4]) * self.group1().xyz * float3(-1.0), /* e23, e31, e12 */float3(0.0), /* e423, e431, e412, e321 */float4(right_anti_dual[e4], right_anti_dual[e4], right_anti_dual[e4], 0.0) * float4(self.group3(), 0.0) * float4(1.0, 1.0, 1.0, 0.0));
        return MultiVector::from_groups(/* scalar, e1234 */float2(-(wedge[e1] * other[e423]) - (wedge[e2] * other[e431]) - (wedge[e3] * other[e412]) - (wedge[e4] * other[e321]), 0.0), /* e1, e2, e3, e4 */float4((wedge[e41] * other[e321]) + (wedge[e31] * other[e412]), (wedge[e42] * other[e321]) + (wedge[e12] * other[e423]), (wedge[e43] * other[e321]) + (wedge[e23] * other[e431]), -(wedge[e42] * other[e431]) - (wedge[e43] * other[e412])) - (other.group0().yzxx * float4(wedge.group3().zxy, wedge[e41])), /* e41, e42, e43 */(wedge.group4().yzx * other.group0().zxy) - (wedge.group4().zxy * other.group0().yzx), /* e23, e31, e12 */(float3(wedge[e321]) * other.group0().xyz) - (float3(other[e321]) * wedge.group4().xyz), /* e423, e431, e412, e321 */float4(wedge[e1234]) * other.group0());
    }
}
impl ProjectOrthogonallyOnto<Point> for MultiVector {
    type Output = MultiVector;
// Operative Statistics for this implementation:
//           add/sub      mul      div
//      f32        6        8        0
//    simd4        0        2        0
// Totals...
// yes simd        6       10        0
//  no simd        6       16        0
    fn project_orthogonally_onto(self, other: Point) -> Self::Output {
        use crate::elements::*;
let right_anti_dual = Plane::from_groups(/* e423, e431, e412, e321 */float4(other[e1], other[e2], other[e3], 0.0));
let wedge = MultiVector::from_groups(/* scalar, e1234 */float2(0.0, (self[e1] * right_anti_dual[e423]) + (self[e2] * right_anti_dual[e431]) + (self[e3] * right_anti_dual[e412]) + (self[e4] * right_anti_dual[e321])), /* e1, e2, e3, e4 */float4(0.0), /* e41, e42, e43 */float3(0.0), /* e23, e31, e12 */float3(0.0), /* e423, e431, e412, e321 */float4(self[scalar]) * right_anti_dual.group0());
        return MultiVector::from_groups(/* scalar, e1234 */float2((wedge[e423] * other[e1]) + (wedge[e431] * other[e2]) + (wedge[e412] * other[e3]) + (wedge[e321] * other[e4]), 0.0), /* e1, e2, e3, e4 */float4(wedge[e1234]) * other.group0(), /* e41, e42, e43 */float3(0.0), /* e23, e31, e12 */float3(0.0), /* e423, e431, e412, e321 */float4(0.0));
    }
}
impl ProjectOrthogonallyOnto<Scalar> for MultiVector {
    type Output = Scalar;
// Operative Statistics for this implementation:
//      add/sub      mul      div
// f32        0        1        0
    fn project_orthogonally_onto(self, other: Scalar) -> Self::Output {
        use crate::elements::*;
        return Scalar::from_groups(/* scalar */powi(other[scalar], 2) * self[scalar]);
    }
}
impl std::ops::Div<project_orthogonally_onto> for Origin {
    type Output = project_orthogonally_onto_partial<Origin>;
    fn div(self, _rhs: project_orthogonally_onto) -> Self::Output {
        project_orthogonally_onto_partial(self)
    }
}
impl ProjectOrthogonallyOnto<Flector> for Origin {
    type Output = Flector;
// Operative Statistics for this implementation:
//           add/sub      mul      div
//      f32        4        9        0
//    simd4        2        4        0
// Totals...
// yes simd        6       13        0
//  no simd       12       25        0
    fn project_orthogonally_onto(self, other: Flector) -> Self::Output {
        use crate::elements::*;
let right_anti_dual = Flector::from_groups(/* e1, e2, e3, e4 */float4(float3(0.0), other[e321] * -1.0), /* e423, e431, e412, e321 */float4(other[e1], other[e2], other[e3], 0.0));
let wedge = Motor::from_groups(/* e41, e42, e43, e1234 */float4(self[e4]) * float4(right_anti_dual.group0().xyz, right_anti_dual[e321]), /* e23, e31, e12, scalar */float4(0.0));
        return Flector::from_groups(/* e1, e2, e3, e4 */float4((other[e412] * wedge[e31]) + (other[e321] * wedge[e41]), (other[e423] * wedge[e12]) + (other[e321] * wedge[e42]), (other[e431] * wedge[e23]) + (other[e321] * wedge[e43]), -(other[e431] * wedge[e42]) - (other[e412] * wedge[e43])) + (float4(wedge[e1234]) * other.group0()) - (other.group1().yzxx * float4(wedge.group1().zxy, wedge[e41])), /* e423, e431, e412, e321 */float4(wedge[e1234]) * other.group1());
    }
}
impl ProjectOrthogonallyOnto<Motor> for Origin {
    type Output = Flector;
// Operative Statistics for this implementation:
//           add/sub      mul      div
//      f32        4        9        0
//    simd4        2        6        0
// Totals...
// yes simd        6       15        0
//  no simd       12       33        0
    fn project_orthogonally_onto(self, other: Motor) -> Self::Output {
        use crate::elements::*;
let right_anti_dual = Motor::from_groups(/* e41, e42, e43, e1234 */other.group1() * float4(-1.0, -1.0, -1.0, 1.0), /* e23, e31, e12, scalar */float4(0.0));
let wedge = Flector::from_groups(/* e1, e2, e3, e4 */float4(float3(0.0), right_anti_dual[scalar] * self[e4]), /* e423, e431, e412, e321 */float4(self[e4], self[e4], self[e4], 0.0) * float4(right_anti_dual.group1().xyz, 0.0) * float4(1.0, 1.0, 1.0, 0.0));
        return Flector::from_groups(/* e1, e2, e3, e4 */float4((wedge[e412] * other[e31]) + (wedge[e321] * other[e41]), (wedge[e423] * other[e12]) + (wedge[e321] * other[e42]), (wedge[e431] * other[e23]) + (wedge[e321] * other[e43]), -(wedge[e431] * other[e42]) - (wedge[e412] * other[e43])) + (float4(other[e1234]) * wedge.group0()) - (wedge.group1().yzxx * float4(other.group1().zxy, other[e41])), /* e423, e431, e412, e321 */float4(other[e1234]) * wedge.group1());
    }
}
impl ProjectOrthogonallyOnto<MultiVector> for Origin {
    type Output = MultiVector;
// Operative Statistics for this implementation:
//           add/sub      mul      div
//      f32       27       37        0
//    simd2        0        1        0
//    simd3        3        7        0
//    simd4        4        5        0
// Totals...
// yes simd       34       50        0
//  no simd       52       80        0
    fn project_orthogonally_onto(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
let right_anti_dual = MultiVector::from_groups(/* scalar, e1234 */float2(0.0, other[scalar]), /* e1, e2, e3, e4 */float4(float3(0.0), other[e321] * -1.0), /* e41, e42, e43 */other.group3() * float3(-1.0), /* e23, e31, e12 */float3(0.0), /* e423, e431, e412, e321 */float4(other[e1], other[e2], other[e3], 0.0));
let wedge = MultiVector::from_groups(/* scalar, e1234 */float2(1.0, right_anti_dual[e321] * self[e4]) * float2(0.0, 1.0), /* e1, e2, e3, e4 */float4(float3(0.0), right_anti_dual[scalar] * self[e4]), /* e41, e42, e43 */float3(self[e4]) * right_anti_dual.group1().xyz, /* e23, e31, e12 */float3(0.0), /* e423, e431, e412, e321 */float4(self[e4], self[e4], self[e4], 0.0) * float4(right_anti_dual.group3(), 0.0) * float4(1.0, 1.0, 1.0, 0.0));
        return MultiVector::from_groups(/* scalar, e1234 */float2((other[scalar] * wedge[e1234]) + (other[e1234] * wedge[scalar]) + (other[e1] * wedge[e423]) + (other[e2] * wedge[e431]) + (other[e3] * wedge[e412]) + (other[e4] * wedge[e321]) - (other[e41] * wedge[e23]) - (other[e42] * wedge[e31]) - (other[e43] * wedge[e12]) - (other[e23] * wedge[e41]) - (other[e31] * wedge[e42]) - (other[e12] * wedge[e43]) - (other[e423] * wedge[e1]) - (other[e431] * wedge[e2]) - (other[e412] * wedge[e3]) - (other[e321] * wedge[e4]), other[e1234] * wedge[e1234]), /* e1, e2, e3, e4 */float4((other[e41] * wedge[e321]) + (other[e31] * wedge[e412]) + (other[e412] * wedge[e31]) + (other[e321] * wedge[e41]), (other[e42] * wedge[e321]) + (other[e12] * wedge[e423]) + (other[e423] * wedge[e12]) + (other[e321] * wedge[e42]), (other[e43] * wedge[e321]) + (other[e23] * wedge[e431]) + (other[e431] * wedge[e23]) + (other[e321] * wedge[e43]), -(other[e43] * wedge[e412]) - (other[e423] * wedge[e41]) - (other[e431] * wedge[e42]) - (other[e412] * wedge[e43])) + (float4(other[e1234]) * wedge.group1()) + (float4(wedge[e1234]) * other.group1()) - (wedge.group4().yzxx * float4(other.group3().zxy, other[e41])) - float4((wedge.group3().zxy * other.group4().yzx), other[e42] * wedge[e431]), /* e41, e42, e43 */(float3(other[e1234]) * wedge.group2()) + (float3(wedge[e1234]) * other.group2()) + (other.group4().zxy * wedge.group4().yzx) - (other.group4().yzx * wedge.group4().zxy), /* e23, e31, e12 */float3(0.0), /* e423, e431, e412, e321 */float4(0.0));
    }
}
impl ProjectOrthogonallyOnto<Point> for Origin {
    type Output = Point;
// Operative Statistics for this implementation:
//           add/sub      mul      div
//      f32        0        1        0
//    simd4        0        1        0
// Totals...
// yes simd        0        2        0
//  no simd        0        5        0
    fn project_orthogonally_onto(self, other: Point) -> Self::Output {
        use crate::elements::*;
        return Point::from_groups(/* e1, e2, e3, e4 */float4(self[e4] * 0.0) * other.group0());
    }
}
impl std::ops::Div<project_orthogonally_onto> for Plane {
    type Output = project_orthogonally_onto_partial<Plane>;
    fn div(self, _rhs: project_orthogonally_onto) -> Self::Output {
        project_orthogonally_onto_partial(self)
    }
}
impl ProjectOrthogonallyOnto<Flector> for Plane {
    type Output = Flector;
// Operative Statistics for this implementation:
//           add/sub      mul      div
//      f32        3        5        0
//    simd4        0        2        0
// Totals...
// yes simd        3        7        0
//  no simd        3       13        0
    fn project_orthogonally_onto(self, other: Flector) -> Self::Output {
        use crate::elements::*;
let right_anti_dual = Flector::from_groups(/* e1, e2, e3, e4 */float4(float3(0.0), other[e321] * -1.0), /* e423, e431, e412, e321 */float4(other[e1], other[e2], other[e3], 0.0));
let wedge = AntiScalar::from_groups(/* e1234 */-(right_anti_dual[e1] * self[e423]) - (right_anti_dual[e2] * self[e431]) - (right_anti_dual[e3] * self[e412]) - (right_anti_dual[e4] * self[e321]));
        return Flector::from_groups(/* e1, e2, e3, e4 */float4(wedge[e1234]) * other.group0(), /* e423, e431, e412, e321 */float4(wedge[e1234]) * other.group1());
    }
}
impl ProjectOrthogonallyOnto<Horizon> for Plane {
    type Output = Horizon;
// Operative Statistics for this implementation:
//      add/sub      mul      div
// f32        0        1        0
    fn project_orthogonally_onto(self, other: Horizon) -> Self::Output {
        use crate::elements::*;
        return Horizon::from_groups(/* e321 */powi(other[e321], 2) * self[e321]);
    }
}
impl ProjectOrthogonallyOnto<Motor> for Plane {
    type Output = Plane;
// Operative Statistics for this implementation:
//           add/sub      mul      div
//      f32        0        4        0
//    simd4        0        2        0
// Totals...
// yes simd        0        6        0
//  no simd        0       12        0
    fn project_orthogonally_onto(self, other: Motor) -> Self::Output {
        use crate::elements::*;
let right_anti_dual = Motor::from_groups(/* e41, e42, e43, e1234 */other.group1() * float4(-1.0, -1.0, -1.0, 1.0), /* e23, e31, e12, scalar */float4(0.0));
        return Plane::from_groups(/* e423, e431, e412, e321 */float4(other[e1234]) * float4(right_anti_dual[scalar] * self[e423], right_anti_dual[scalar] * self[e431], right_anti_dual[scalar] * self[e412], right_anti_dual[scalar] * self[e321]));
    }
}
impl ProjectOrthogonallyOnto<MultiVector> for Plane {
    type Output = MultiVector;
// Operative Statistics for this implementation:
//           add/sub      mul      div
//      f32       30       39        0
//    simd3        6       10        0
//    simd4        5        6        0
// Totals...
// yes simd       41       55        0
//  no simd       68       93        0
    fn project_orthogonally_onto(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
let right_anti_dual = MultiVector::from_groups(/* scalar, e1234 */float2(0.0, other[scalar]), /* e1, e2, e3, e4 */float4(float3(0.0), other[e321] * -1.0), /* e41, e42, e43 */other.group3() * float3(-1.0), /* e23, e31, e12 */float3(0.0), /* e423, e431, e412, e321 */float4(other[e1], other[e2], other[e3], 0.0));
let wedge = MultiVector::from_groups(/* scalar, e1234 */float2(0.0, -(right_anti_dual[e1] * self[e423]) - (right_anti_dual[e2] * self[e431]) - (right_anti_dual[e3] * self[e412]) - (right_anti_dual[e4] * self[e321])), /* e1, e2, e3, e4 */float4(0.0), /* e41, e42, e43 */float3(0.0), /* e23, e31, e12 */float3(0.0), /* e423, e431, e412, e321 */float4(right_anti_dual[scalar]) * self.group0());
        return MultiVector::from_groups(/* scalar, e1234 */float2((other[scalar] * wedge[e1234]) + (other[e1234] * wedge[scalar]) + (other[e1] * wedge[e423]) + (other[e2] * wedge[e431]) + (other[e3] * wedge[e412]) + (other[e4] * wedge[e321]) - (other[e41] * wedge[e23]) - (other[e42] * wedge[e31]) - (other[e43] * wedge[e12]) - (other[e23] * wedge[e41]) - (other[e31] * wedge[e42]) - (other[e12] * wedge[e43]) - (other[e423] * wedge[e1]) - (other[e431] * wedge[e2]) - (other[e412] * wedge[e3]) - (other[e321] * wedge[e4]), other[e1234] * wedge[e1234]), /* e1, e2, e3, e4 */float4((other[e41] * wedge[e321]) + (other[e31] * wedge[e412]) + (other[e412] * wedge[e31]) + (other[e321] * wedge[e41]), (other[e42] * wedge[e321]) + (other[e12] * wedge[e423]) + (other[e423] * wedge[e12]) + (other[e321] * wedge[e42]), (other[e43] * wedge[e321]) + (other[e23] * wedge[e431]) + (other[e431] * wedge[e23]) + (other[e321] * wedge[e43]), -(other[e43] * wedge[e412]) - (other[e423] * wedge[e41]) - (other[e431] * wedge[e42]) - (other[e412] * wedge[e43])) + (float4(other[e1234]) * wedge.group1()) + (float4(wedge[e1234]) * other.group1()) - (wedge.group4().yzxx * float4(other.group3().zxy, other[e41])) - float4((wedge.group3().zxy * other.group4().yzx), other[e42] * wedge[e431]), /* e41, e42, e43 */(float3(other[e1234]) * wedge.group2()) + (float3(wedge[e1234]) * other.group2()) + (other.group4().zxy * wedge.group4().yzx) - (other.group4().yzx * wedge.group4().zxy), /* e23, e31, e12 */(float3(other[e1234]) * wedge.group3()) + (float3(wedge[e1234]) * other.group3()) + (float3(wedge[e321]) * other.group4().xyz) - (float3(other[e321]) * wedge.group4().xyz), /* e423, e431, e412, e321 */(float4(other[e1234]) * wedge.group4()) + (float4(wedge[e1234]) * other.group4()));
    }
}
impl ProjectOrthogonallyOnto<Plane> for Plane {
    type Output = Plane;
// Operative Statistics for this implementation:
//           add/sub      mul      div
//      f32        0        1        0
//    simd4        0        1        0
// Totals...
// yes simd        0        2        0
//  no simd        0        5        0
    fn project_orthogonally_onto(self, other: Plane) -> Self::Output {
        use crate::elements::*;
        return Plane::from_groups(/* e423, e431, e412, e321 */float4(other[e321] * self[e321]) * other.group0());
    }
}
impl std::ops::Div<project_orthogonally_onto> for Point {
    type Output = project_orthogonally_onto_partial<Point>;
    fn div(self, _rhs: project_orthogonally_onto) -> Self::Output {
        project_orthogonally_onto_partial(self)
    }
}
impl ProjectOrthogonallyOnto<Flector> for Point {
    type Output = Flector;
// Operative Statistics for this implementation:
//           add/sub      mul      div
//      f32        6       18        0
//    simd3        1        2        0
//    simd4        3        4        0
// Totals...
// yes simd       10       24        0
//  no simd       21       40        0
    fn project_orthogonally_onto(self, other: Flector) -> Self::Output {
        use crate::elements::*;
let right_anti_dual = Flector::from_groups(/* e1, e2, e3, e4 */float4(float3(0.0), other[e321] * -1.0), /* e423, e431, e412, e321 */float4(other[e1], other[e2], other[e3], 0.0));
let wedge = Motor::from_groups(/* e41, e42, e43, e1234 */float4(right_anti_dual[e4] * self[e1] * -1.0, right_anti_dual[e4] * self[e2] * -1.0, right_anti_dual[e4] * self[e3] * -1.0, (right_anti_dual[e431] * self[e2]) + (right_anti_dual[e412] * self[e3]) + (right_anti_dual[e321] * self[e4])) + (self.group0().wwwx * float4(right_anti_dual.group0().xyz, right_anti_dual[e423])), /* e23, e31, e12, scalar */float4(((right_anti_dual.group0().zxy * self.group0().yzx) - (right_anti_dual.group0().yzx * self.group0().zxy)), 0.0));
        return Flector::from_groups(/* e1, e2, e3, e4 */float4((other[e412] * wedge[e31]) + (other[e321] * wedge[e41]), (other[e423] * wedge[e12]) + (other[e321] * wedge[e42]), (other[e431] * wedge[e23]) + (other[e321] * wedge[e43]), -(other[e431] * wedge[e42]) - (other[e412] * wedge[e43])) + (float4(wedge[e1234]) * other.group0()) - (other.group1().yzxx * float4(wedge.group1().zxy, wedge[e41])), /* e423, e431, e412, e321 */float4(wedge[e1234]) * other.group1());
    }
}
impl ProjectOrthogonallyOnto<Horizon> for Point {
    type Output = Point;
// Operative Statistics for this implementation:
//           add/sub      mul      div
//      f32        0        7        0
//    simd4        0        2        0
// Totals...
// yes simd        0        9        0
//  no simd        0       15        0
    fn project_orthogonally_onto(self, other: Horizon) -> Self::Output {
        use crate::elements::*;
let right_anti_dual = Origin::from_groups(/* e4 */other[e321] * -1.0);
        return Point::from_groups(/* e1, e2, e3, e4 */float4(other[e321], other[e321], other[e321], 0.0) * float4(float3(right_anti_dual[e4] * self[e1] * -1.0, right_anti_dual[e4] * self[e2] * -1.0, right_anti_dual[e4] * self[e3] * -1.0), 0.0) * float4(1.0, 1.0, 1.0, 0.0));
    }
}
impl ProjectOrthogonallyOnto<Line> for Point {
    type Output = Point;
// Operative Statistics for this implementation:
//           add/sub      mul      div
//      f32        4        8        0
//    simd3        0        4        0
//    simd4        3        1        0
// Totals...
// yes simd        7       13        0
//  no simd       16       24        0
    fn project_orthogonally_onto(self, other: Line) -> Self::Output {
        use crate::elements::*;
let right_anti_dual = Line::from_groups(/* e41, e42, e43 */other.group1() * float3(-1.0), /* e23, e31, e12 */float3(0.0));
let wedge = Plane::from_groups(/* e423, e431, e412, e321 */float4((float3(self[e4]) * right_anti_dual.group1()), 0.0) + float4((right_anti_dual.group0().yzx * self.group0().zxy), 0.0) - float4((right_anti_dual.group0().zxy * self.group0().yzx), 0.0));
        return Point::from_groups(/* e1, e2, e3, e4 */float4((other[e41] * wedge[e321]) + (other[e31] * wedge[e412]), (other[e42] * wedge[e321]) + (other[e12] * wedge[e423]), (other[e43] * wedge[e321]) + (other[e23] * wedge[e431]), -(other[e42] * wedge[e431]) - (other[e43] * wedge[e412])) - (wedge.group0().yzxx * float4(other.group1().zxy, other[e41])));
    }
}
impl ProjectOrthogonallyOnto<Motor> for Point {
    type Output = Flector;
// Operative Statistics for this implementation:
//           add/sub      mul      div
//      f32        4        8        0
//    simd3        0        3        0
//    simd4        4        5        0
// Totals...
// yes simd        8       16        0
//  no simd       20       37        0
    fn project_orthogonally_onto(self, other: Motor) -> Self::Output {
        use crate::elements::*;
let right_anti_dual = Motor::from_groups(/* e41, e42, e43, e1234 */other.group1() * float4(-1.0, -1.0, -1.0, 1.0), /* e23, e31, e12, scalar */float4(0.0));
let wedge = Flector::from_groups(/* e1, e2, e3, e4 */float4(right_anti_dual[scalar]) * self.group0(), /* e423, e431, e412, e321 */float4((float3(self[e4]) * right_anti_dual.group1().xyz), 0.0) + float4((right_anti_dual.group0().yzx * self.group0().zxy), 0.0) - float4((right_anti_dual.group0().zxy * self.group0().yzx), 0.0));
        return Flector::from_groups(/* e1, e2, e3, e4 */float4((wedge[e412] * other[e31]) + (wedge[e321] * other[e41]), (wedge[e423] * other[e12]) + (wedge[e321] * other[e42]), (wedge[e431] * other[e23]) + (wedge[e321] * other[e43]), -(wedge[e431] * other[e42]) - (wedge[e412] * other[e43])) + (float4(other[e1234]) * wedge.group0()) - (wedge.group1().yzxx * float4(other.group1().zxy, other[e41])), /* e423, e431, e412, e321 */float4(other[e1234]) * wedge.group1());
    }
}
impl ProjectOrthogonallyOnto<MultiVector> for Point {
    type Output = MultiVector;
// Operative Statistics for this implementation:
//           add/sub      mul      div
//      f32       30       39        0
//    simd3        8       17        0
//    simd4        7        6        0
// Totals...
// yes simd       45       62        0
//  no simd       82      114        0
    fn project_orthogonally_onto(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
let right_anti_dual = MultiVector::from_groups(/* scalar, e1234 */float2(0.0, other[scalar]), /* e1, e2, e3, e4 */float4(float3(0.0), other[e321] * -1.0), /* e41, e42, e43 */other.group3() * float3(-1.0), /* e23, e31, e12 */float3(0.0), /* e423, e431, e412, e321 */float4(other[e1], other[e2], other[e3], 0.0));
let wedge = MultiVector::from_groups(/* scalar, e1234 */float2(0.0, (right_anti_dual[e423] * self[e1]) + (right_anti_dual[e431] * self[e2]) + (right_anti_dual[e412] * self[e3]) + (right_anti_dual[e321] * self[e4])), /* e1, e2, e3, e4 */float4(right_anti_dual[scalar]) * self.group0(), /* e41, e42, e43 */(float3(self[e4]) * right_anti_dual.group1().xyz) - (float3(right_anti_dual[e4]) * self.group0().xyz), /* e23, e31, e12 */(right_anti_dual.group1().zxy * self.group0().yzx) - (right_anti_dual.group1().yzx * self.group0().zxy), /* e423, e431, e412, e321 */float4((float3(self[e4]) * right_anti_dual.group3()), 0.0) + float4((right_anti_dual.group2().yzx * self.group0().zxy), 0.0) - float4((right_anti_dual.group2().zxy * self.group0().yzx), 0.0));
        return MultiVector::from_groups(/* scalar, e1234 */float2((other[scalar] * wedge[e1234]) + (other[e1234] * wedge[scalar]) + (other[e1] * wedge[e423]) + (other[e2] * wedge[e431]) + (other[e3] * wedge[e412]) + (other[e4] * wedge[e321]) - (other[e41] * wedge[e23]) - (other[e42] * wedge[e31]) - (other[e43] * wedge[e12]) - (other[e23] * wedge[e41]) - (other[e31] * wedge[e42]) - (other[e12] * wedge[e43]) - (other[e423] * wedge[e1]) - (other[e431] * wedge[e2]) - (other[e412] * wedge[e3]) - (other[e321] * wedge[e4]), other[e1234] * wedge[e1234]), /* e1, e2, e3, e4 */float4((other[e41] * wedge[e321]) + (other[e31] * wedge[e412]) + (other[e412] * wedge[e31]) + (other[e321] * wedge[e41]), (other[e42] * wedge[e321]) + (other[e12] * wedge[e423]) + (other[e423] * wedge[e12]) + (other[e321] * wedge[e42]), (other[e43] * wedge[e321]) + (other[e23] * wedge[e431]) + (other[e431] * wedge[e23]) + (other[e321] * wedge[e43]), -(other[e43] * wedge[e412]) - (other[e423] * wedge[e41]) - (other[e431] * wedge[e42]) - (other[e412] * wedge[e43])) + (float4(other[e1234]) * wedge.group1()) + (float4(wedge[e1234]) * other.group1()) - (wedge.group4().yzxx * float4(other.group3().zxy, other[e41])) - float4((wedge.group3().zxy * other.group4().yzx), other[e42] * wedge[e431]), /* e41, e42, e43 */(float3(other[e1234]) * wedge.group2()) + (float3(wedge[e1234]) * other.group2()) + (other.group4().zxy * wedge.group4().yzx) - (other.group4().yzx * wedge.group4().zxy), /* e23, e31, e12 */(float3(other[e1234]) * wedge.group3()) + (float3(wedge[e1234]) * other.group3()) + (float3(wedge[e321]) * other.group4().xyz) - (float3(other[e321]) * wedge.group4().xyz), /* e423, e431, e412, e321 */(float4(other[e1234]) * wedge.group4()) + (float4(wedge[e1234]) * other.group4()));
    }
}
impl ProjectOrthogonallyOnto<Plane> for Point {
    type Output = Point;
// Operative Statistics for this implementation:
//           add/sub      mul      div
//      f32        4        9        0
//    simd3        0        2        0
//    simd4        1        1        0
// Totals...
// yes simd        5       12        0
//  no simd        8       19        0
    fn project_orthogonally_onto(self, other: Plane) -> Self::Output {
        use crate::elements::*;
let wedge = Line::from_groups(/* e41, e42, e43 */float3(other[e321] * -1.0) * self.group0().xyz * float3(-1.0), /* e23, e31, e12 */float3(0.0));
        return Point::from_groups(/* e1, e2, e3, e4 */float4((wedge[e41] * other[e321]) + (wedge[e31] * other[e412]), (wedge[e42] * other[e321]) + (wedge[e12] * other[e423]), (wedge[e43] * other[e321]) + (wedge[e23] * other[e431]), -(wedge[e42] * other[e431]) - (wedge[e43] * other[e412])) - (other.group0().yzxx * float4(wedge.group1().zxy, wedge[e41])));
    }
}
impl ProjectOrthogonallyOnto<Point> for Point {
    type Output = Point;
// Operative Statistics for this implementation:
//           add/sub      mul      div
//      f32        3        4        0
//    simd4        0        1        0
// Totals...
// yes simd        3        5        0
//  no simd        3        8        0
    fn project_orthogonally_onto(self, other: Point) -> Self::Output {
        use crate::elements::*;
let right_anti_dual = Plane::from_groups(/* e423, e431, e412, e321 */float4(other[e1], other[e2], other[e3], 0.0));
        return Point::from_groups(/* e1, e2, e3, e4 */float4((right_anti_dual[e423] * self[e1]) + (right_anti_dual[e431] * self[e2]) + (right_anti_dual[e412] * self[e3]) + (right_anti_dual[e321] * self[e4])) * other.group0());
    }
}
impl std::ops::Div<project_orthogonally_onto> for Scalar {
    type Output = project_orthogonally_onto_partial<Scalar>;
    fn div(self, _rhs: project_orthogonally_onto) -> Self::Output {
        project_orthogonally_onto_partial(self)
    }
}
impl ProjectOrthogonallyOnto<DualNum> for Scalar {
    type Output = DualNum;
// Operative Statistics for this implementation:
//           add/sub      mul      div
//      f32        0        1        0
//    simd2        0        1        0
// Totals...
// yes simd        0        2        0
//  no simd        0        3        0
    fn project_orthogonally_onto(self, other: DualNum) -> Self::Output {
        use crate::elements::*;
        return DualNum::from_groups(/* scalar, e1234 */float2(other[scalar] * self[scalar]) * other.group0());
    }
}
impl ProjectOrthogonallyOnto<Flector> for Scalar {
    type Output = Motor;
// Operative Statistics for this implementation:
//           add/sub      mul      div
//      f32        5        7        0
//    simd3        1        2        0
//    simd4        2        4        0
// Totals...
// yes simd        8       13        0
//  no simd       16       29        0
    fn project_orthogonally_onto(self, other: Flector) -> Self::Output {
        use crate::elements::*;
let right_anti_dual = Flector::from_groups(/* e1, e2, e3, e4 */float4(float3(0.0), other[e321] * -1.0), /* e423, e431, e412, e321 */float4(other[e1], other[e2], other[e3], 0.0));
let wedge = Flector::from_groups(/* e1, e2, e3, e4 */float4(self[scalar]) * right_anti_dual.group0(), /* e423, e431, e412, e321 */float4(self[scalar]) * right_anti_dual.group1());
        return Motor::from_groups(/* e41, e42, e43, e1234 */float4(((other.group1().zxy * wedge.group1().yzx) - (other.group1().yzx * wedge.group1().zxy)), 0.0), /* e23, e31, e12, scalar */(wedge.group1().wwwx * float4(other.group1().xyz, other[e1])) + float4(float3(0.0), (other[e2] * wedge[e431]) + (other[e3] * wedge[e412]) + (other[e4] * wedge[e321]) - (other[e431] * wedge[e2]) - (other[e412] * wedge[e3]) - (other[e321] * wedge[e4])) - (other.group1().wwwx * float4(wedge.group1().xyz, wedge[e1])));
    }
}
impl ProjectOrthogonallyOnto<Horizon> for Scalar {
    type Output = Scalar;
// Operative Statistics for this implementation:
//      add/sub      mul      div
// f32        0        1        0
    fn project_orthogonally_onto(self, other: Horizon) -> Self::Output {
        use crate::elements::*;
        return Scalar::from_groups(/* scalar */powi(other[e321], 2) * self[scalar]);
    }
}
impl ProjectOrthogonallyOnto<Line> for Scalar {
    type Output = Scalar;
// Operative Statistics for this implementation:
//           add/sub      mul      div
//      f32        5        6        0
//    simd3        0        3        0
// Totals...
// yes simd        5        9        0
//  no simd        5       15        0
    fn project_orthogonally_onto(self, other: Line) -> Self::Output {
        use crate::elements::*;
let right_anti_dual = Line::from_groups(/* e41, e42, e43 */other.group1() * float3(-1.0), /* e23, e31, e12 */float3(0.0));
let wedge = Line::from_groups(/* e41, e42, e43 */float3(self[scalar]) * right_anti_dual.group0(), /* e23, e31, e12 */float3(self[scalar]) * right_anti_dual.group1());
        return Scalar::from_groups(/* scalar */-(other[e41] * wedge[e23]) - (other[e42] * wedge[e31]) - (other[e43] * wedge[e12]) - (other[e23] * wedge[e41]) - (other[e31] * wedge[e42]) - (other[e12] * wedge[e43]));
    }
}
impl ProjectOrthogonallyOnto<Motor> for Scalar {
    type Output = Motor;
// Operative Statistics for this implementation:
//           add/sub      mul      div
//      f32        5        7        0
//    simd3        1        2        0
//    simd4        2        5        0
// Totals...
// yes simd        8       14        0
//  no simd       16       33        0
    fn project_orthogonally_onto(self, other: Motor) -> Self::Output {
        use crate::elements::*;
let right_anti_dual = Motor::from_groups(/* e41, e42, e43, e1234 */other.group1() * float4(-1.0, -1.0, -1.0, 1.0), /* e23, e31, e12, scalar */float4(0.0));
let wedge = Motor::from_groups(/* e41, e42, e43, e1234 */float4(self[scalar]) * right_anti_dual.group0(), /* e23, e31, e12, scalar */float4(self[scalar]) * right_anti_dual.group1());
        return Motor::from_groups(/* e41, e42, e43, e1234 */float4(((float3(other[e1234]) * wedge.group0().xyz) + (float3(wedge[e1234]) * other.group0().xyz)), other[e1234] * wedge[e1234]), /* e23, e31, e12, scalar */(float4(other[e1234]) * wedge.group1()) + (float4(wedge[e1234]) * other.group1()) + float4(float3(0.0), -(other[e41] * wedge[e23]) - (other[e42] * wedge[e31]) - (other[e43] * wedge[e12]) - (other[e23] * wedge[e41]) - (other[e31] * wedge[e42]) - (other[e12] * wedge[e43])));
    }
}
impl ProjectOrthogonallyOnto<MultiVector> for Scalar {
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
    fn project_orthogonally_onto(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
let right_anti_dual = MultiVector::from_groups(/* scalar, e1234 */float2(0.0, other[scalar]), /* e1, e2, e3, e4 */float4(float3(0.0), other[e321] * -1.0), /* e41, e42, e43 */other.group3() * float3(-1.0), /* e23, e31, e12 */float3(0.0), /* e423, e431, e412, e321 */float4(other[e1], other[e2], other[e3], 0.0));
let wedge = MultiVector::from_groups(/* scalar, e1234 */float2(self[scalar]) * right_anti_dual.group0(), /* e1, e2, e3, e4 */float4(self[scalar]) * right_anti_dual.group1(), /* e41, e42, e43 */float3(self[scalar]) * right_anti_dual.group2(), /* e23, e31, e12 */float3(self[scalar]) * right_anti_dual.group3(), /* e423, e431, e412, e321 */float4(self[scalar]) * right_anti_dual.group4());
        return MultiVector::from_groups(/* scalar, e1234 */float2((other[scalar] * wedge[e1234]) + (other[e1234] * wedge[scalar]) + (other[e1] * wedge[e423]) + (other[e2] * wedge[e431]) + (other[e3] * wedge[e412]) + (other[e4] * wedge[e321]) - (other[e41] * wedge[e23]) - (other[e42] * wedge[e31]) - (other[e43] * wedge[e12]) - (other[e23] * wedge[e41]) - (other[e31] * wedge[e42]) - (other[e12] * wedge[e43]) - (other[e423] * wedge[e1]) - (other[e431] * wedge[e2]) - (other[e412] * wedge[e3]) - (other[e321] * wedge[e4]), other[e1234] * wedge[e1234]), /* e1, e2, e3, e4 */float4((other[e41] * wedge[e321]) + (other[e31] * wedge[e412]) + (other[e412] * wedge[e31]) + (other[e321] * wedge[e41]), (other[e42] * wedge[e321]) + (other[e12] * wedge[e423]) + (other[e423] * wedge[e12]) + (other[e321] * wedge[e42]), (other[e43] * wedge[e321]) + (other[e23] * wedge[e431]) + (other[e431] * wedge[e23]) + (other[e321] * wedge[e43]), -(other[e43] * wedge[e412]) - (other[e423] * wedge[e41]) - (other[e431] * wedge[e42]) - (other[e412] * wedge[e43])) + (float4(other[e1234]) * wedge.group1()) + (float4(wedge[e1234]) * other.group1()) - (wedge.group4().yzxx * float4(other.group3().zxy, other[e41])) - float4((wedge.group3().zxy * other.group4().yzx), other[e42] * wedge[e431]), /* e41, e42, e43 */(float3(other[e1234]) * wedge.group2()) + (float3(wedge[e1234]) * other.group2()) + (other.group4().zxy * wedge.group4().yzx) - (other.group4().yzx * wedge.group4().zxy), /* e23, e31, e12 */(float3(other[e1234]) * wedge.group3()) + (float3(wedge[e1234]) * other.group3()) + (float3(wedge[e321]) * other.group4().xyz) - (float3(other[e321]) * wedge.group4().xyz), /* e423, e431, e412, e321 */(float4(other[e1234]) * wedge.group4()) + (float4(wedge[e1234]) * other.group4()));
    }
}
impl ProjectOrthogonallyOnto<Plane> for Scalar {
    type Output = Scalar;
// Operative Statistics for this implementation:
//      add/sub      mul      div
// f32        0        1        0
    fn project_orthogonally_onto(self, other: Plane) -> Self::Output {
        use crate::elements::*;
        return Scalar::from_groups(/* scalar */powi(other[e321], 2) * self[scalar]);
    }
}
impl ProjectOrthogonallyOnto<Point> for Scalar {
    type Output = Scalar;
// Operative Statistics for this implementation:
//           add/sub      mul      div
//      f32        3        4        0
//    simd4        0        1        0
// Totals...
// yes simd        3        5        0
//  no simd        3        8        0
    fn project_orthogonally_onto(self, other: Point) -> Self::Output {
        use crate::elements::*;
let wedge = Plane::from_groups(/* e423, e431, e412, e321 */float4(self[scalar]) * float4(other[e1], other[e2], other[e3], 0.0));
        return Scalar::from_groups(/* scalar */(wedge[e423] * other[e1]) + (wedge[e431] * other[e2]) + (wedge[e412] * other[e3]) + (wedge[e321] * other[e4]));
    }
}
impl ProjectOrthogonallyOnto<Scalar> for Scalar {
    type Output = Scalar;
// Operative Statistics for this implementation:
//      add/sub      mul      div
// f32        0        1        0
    fn project_orthogonally_onto(self, other: Scalar) -> Self::Output {
        use crate::elements::*;
        return Scalar::from_groups(/* scalar */powi(other[scalar], 2) * self[scalar]);
    }
}
