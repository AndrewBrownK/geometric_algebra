// Note on Operative Statistics:
// Operative Statistics are not a precise predictor of performance or performance comparisons.
// This is due to varying hardware capabilities and compiler optimizations.
// As always, where performance is a concern, there is no substitute for
// real measurements on real work-loads on real hardware.
// Disclaimer aside, enjoy the fun information =)
//
// Total Implementations: 9
//
// Yes SIMD:   add/sub     mul     div
//  Minimum:         0       0       0
//   Median:         0       1       0
//  Average:         0       1       0
//  Maximum:         0       8       0
//
//  No SIMD:   add/sub     mul     div
//  Minimum:         0       0       0
//   Median:         0       3       0
//  Average:         0       4       0
//  Maximum:         0      17       0
impl std::ops::Div<AntiSupportPrefixOrPostfix> for DualNum {
    type Output = Horizon;
    fn div(self, _rhs: AntiSupportPrefixOrPostfix) -> Self::Output {
        self.anti_support()
    }
}
impl AntiSupport for DualNum {
    type Output = Horizon;
    fn anti_support(self) -> Self::Output {
        use crate::elements::*;
        return Horizon::from_groups(/* e321 */ self[scalar]);
    }
}
impl std::ops::Div<AntiSupportPrefixOrPostfix> for Flector {
    type Output = Motor;
    fn div(self, _rhs: AntiSupportPrefixOrPostfix) -> Self::Output {
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
        let right_dual = Flector::from_groups(
            // e1, e2, e3, e4
            Simd32x3::from(0.0).with_w(self[e321] * -1.0),
            // e423, e431, e412, e321
            self.group0().xyz().with_w(0.0),
        );
        return Motor::from_groups(
            // e41, e42, e43, e1234
            Simd32x4::from(0.0),
            // e23, e31, e12, scalar
            right_dual.group1().xyz().with_w(right_dual[e4]) * Simd32x4::from(-1.0),
        );
    }
}
impl std::ops::Div<AntiSupportPrefixOrPostfix> for Horizon {
    type Output = Scalar;
    fn div(self, _rhs: AntiSupportPrefixOrPostfix) -> Self::Output {
        self.anti_support()
    }
}
impl AntiSupport for Horizon {
    type Output = Scalar;
    fn anti_support(self) -> Self::Output {
        use crate::elements::*;
        return Scalar::from_groups(/* scalar */ self[e321]);
    }
}
impl std::ops::Div<AntiSupportPrefixOrPostfix> for Line {
    type Output = Point;
    fn div(self, _rhs: AntiSupportPrefixOrPostfix) -> Self::Output {
        self.anti_support()
    }
}
impl AntiSupport for Line {
    type Output = Point;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        1        0
    // no simd        0        3        0
    fn anti_support(self) -> Self::Output {
        return Point::from_groups(/* e1, e2, e3, e4 */ (self.group1() * Simd32x3::from(-1.0)).with_w(0.0));
    }
}
impl std::ops::Div<AntiSupportPrefixOrPostfix> for Motor {
    type Output = Flector;
    fn div(self, _rhs: AntiSupportPrefixOrPostfix) -> Self::Output {
        self.anti_support()
    }
}
impl AntiSupport for Motor {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd3        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        3        0
    //  no simd        0        8        0
    fn anti_support(self) -> Self::Output {
        use crate::elements::*;
        let right_complement = Horizon::from_groups(/* e321 */ 1.0);
        let right_dual = Motor::from_groups(
            // e41, e42, e43, e1234
            self.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e23, e31, e12, scalar
            Simd32x4::from(0.0),
        );
        return Flector::from_groups(
            // e1, e2, e3, e4
            (Simd32x3::from(right_complement[e321]) * right_dual.group0().xyz()).with_w(0.0),
            // e423, e431, e412, e321
            Simd32x3::from(0.0).with_w(right_complement[e321] * right_dual[e1234]),
        );
    }
}
impl std::ops::Div<AntiSupportPrefixOrPostfix> for MultiVector {
    type Output = MultiVector;
    fn div(self, _rhs: AntiSupportPrefixOrPostfix) -> Self::Output {
        self.anti_support()
    }
}
impl std::ops::DivAssign<AntiSupportPrefixOrPostfix> for MultiVector {
    fn div_assign(&mut self, _rhs: AntiSupportPrefixOrPostfix) {
        *self = self.anti_support()
    }
}
impl AntiSupport for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        3        0
    //    simd2        0        1        0
    //    simd3        0        4        0
    // Totals...
    // yes simd        0        8        0
    //  no simd        0       17        0
    fn anti_support(self) -> Self::Output {
        use crate::elements::*;
        let right_complement = Horizon::from_groups(/* e321 */ 1.0);
        let right_dual = MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([0.0, self[scalar]]),
            // e1, e2, e3, e4
            Simd32x3::from(0.0).with_w(self[e321] * -1.0),
            // e41, e42, e43
            self.group3() * Simd32x3::from(-1.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e423, e431, e412, e321
            self.group1().xyz().with_w(0.0),
        );
        return MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([right_complement[e321] * right_dual[e4], 1.0]) * Simd32x2::from([-1.0, 0.0]),
            // e1, e2, e3, e4
            (Simd32x3::from(right_complement[e321]) * right_dual.group2()).with_w(0.0),
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(right_complement[e321]) * right_dual.group4().xyz() * Simd32x3::from(-1.0),
            // e423, e431, e412, e321
            Simd32x3::from(0.0).with_w(right_complement[e321] * right_dual[e1234]),
        );
    }
}
impl std::ops::Div<AntiSupportPrefixOrPostfix> for Plane {
    type Output = Scalar;
    fn div(self, _rhs: AntiSupportPrefixOrPostfix) -> Self::Output {
        self.anti_support()
    }
}
impl AntiSupport for Plane {
    type Output = Scalar;
    fn anti_support(self) -> Self::Output {
        use crate::elements::*;
        return Scalar::from_groups(/* scalar */ self[e321]);
    }
}
impl std::ops::Div<AntiSupportPrefixOrPostfix> for Point {
    type Output = Line;
    fn div(self, _rhs: AntiSupportPrefixOrPostfix) -> Self::Output {
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
        return Line::from_groups(/* e41, e42, e43 */ Simd32x3::from(0.0), /* e23, e31, e12 */ self.group0().xyz() * Simd32x3::from(-1.0));
    }
}
impl std::ops::Div<AntiSupportPrefixOrPostfix> for Scalar {
    type Output = Horizon;
    fn div(self, _rhs: AntiSupportPrefixOrPostfix) -> Self::Output {
        self.anti_support()
    }
}
impl AntiSupport for Scalar {
    type Output = Horizon;
    fn anti_support(self) -> Self::Output {
        use crate::elements::*;
        return Horizon::from_groups(/* e321 */ self[scalar]);
    }
}
