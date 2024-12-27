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
//   Median:         2       3       0
//  Average:         1       3       0
//  Maximum:         7       8       1
//
//  No SIMD:   add/sub     mul     div
//  Minimum:         0       0       0
//   Median:         2       8       0
//  Average:         1       9       0
//  Maximum:         7      26       1
impl std::ops::Div<InversePrefixOrPostfix> for DualNum {
    type Output = DualNum;
    fn div(self, _rhs: InversePrefixOrPostfix) -> Self::Output {
        self.inverse()
    }
}
impl std::ops::DivAssign<InversePrefixOrPostfix> for DualNum {
    fn div_assign(&mut self, _rhs: InversePrefixOrPostfix) {
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
        return DualNum::from_groups(/* scalar, e1234 */ Simd32x2::from(f32::powi(self[scalar], -2)) * self.group0());
    }
}
impl std::ops::Div<InversePrefixOrPostfix> for Flector {
    type Output = Flector;
    fn div(self, _rhs: InversePrefixOrPostfix) -> Self::Output {
        self.inverse()
    }
}
impl std::ops::DivAssign<InversePrefixOrPostfix> for Flector {
    fn div_assign(&mut self, _rhs: InversePrefixOrPostfix) {
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
        let reverse = Flector::from_groups(/* e1, e2, e3, e4 */ self.group0(), /* e423, e431, e412, e321 */ self.group1() * Simd32x4::from(-1.0));
        let other = Scalar::from_groups(/* scalar */ f32::powi(self[e1], 2) + f32::powi(self[e2], 2) + f32::powi(self[e3], 2) + f32::powi(self[e321], 2));
        return Flector::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from(other[scalar]) * reverse.group0(),
            // e423, e431, e412, e321
            Simd32x4::from(other[scalar]) * reverse.group1(),
        );
    }
}
impl std::ops::Div<InversePrefixOrPostfix> for Horizon {
    type Output = Horizon;
    fn div(self, _rhs: InversePrefixOrPostfix) -> Self::Output {
        self.inverse()
    }
}
impl std::ops::DivAssign<InversePrefixOrPostfix> for Horizon {
    fn div_assign(&mut self, _rhs: InversePrefixOrPostfix) {
        *self = self.inverse()
    }
}
impl Inverse for Horizon {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        1
    fn inverse(self) -> Self {
        use crate::elements::*;
        return Horizon::from_groups(/* e321 */ 1.0 / self[e321] * -1.0);
    }
}
impl std::ops::Div<InversePrefixOrPostfix> for Line {
    type Output = Line;
    fn div(self, _rhs: InversePrefixOrPostfix) -> Self::Output {
        self.inverse()
    }
}
impl std::ops::DivAssign<InversePrefixOrPostfix> for Line {
    fn div_assign(&mut self, _rhs: InversePrefixOrPostfix) {
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
        let reverse = Line::from_groups(
            // e41, e42, e43
            self.group0() * Simd32x3::from(-1.0),
            // e23, e31, e12
            self.group1() * Simd32x3::from(-1.0),
        );
        let other = Scalar::from_groups(/* scalar */ f32::powi(self[e23], 2) + f32::powi(self[e31], 2) + f32::powi(self[e12], 2));
        return Line::from_groups(
            // e41, e42, e43
            Simd32x3::from(other[scalar]) * reverse.group0(),
            // e23, e31, e12
            Simd32x3::from(other[scalar]) * reverse.group1(),
        );
    }
}
impl std::ops::Div<InversePrefixOrPostfix> for Motor {
    type Output = Motor;
    fn div(self, _rhs: InversePrefixOrPostfix) -> Self::Output {
        self.inverse()
    }
}
impl std::ops::DivAssign<InversePrefixOrPostfix> for Motor {
    fn div_assign(&mut self, _rhs: InversePrefixOrPostfix) {
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
        let reverse = Motor::from_groups(
            // e41, e42, e43, e1234
            self.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e23, e31, e12, scalar
            self.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
        );
        let other = Scalar::from_groups(
            // scalar
            f32::powi(self[e23], 2) + f32::powi(self[e31], 2) + f32::powi(self[e12], 2) + f32::powi(self[scalar], 2),
        );
        return Motor::from_groups(
            // e41, e42, e43, e1234
            Simd32x4::from(other[scalar]) * reverse.group0(),
            // e23, e31, e12, scalar
            Simd32x4::from(other[scalar]) * reverse.group1(),
        );
    }
}
impl std::ops::Div<InversePrefixOrPostfix> for MultiVector {
    type Output = MultiVector;
    fn div(self, _rhs: InversePrefixOrPostfix) -> Self::Output {
        self.inverse()
    }
}
impl std::ops::DivAssign<InversePrefixOrPostfix> for MultiVector {
    fn div_assign(&mut self, _rhs: InversePrefixOrPostfix) {
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
        let reverse = MultiVector::from_groups(
            // scalar, e1234
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e41, e42, e43
            self.group2() * Simd32x3::from(-1.0),
            // e23, e31, e12
            self.group3() * Simd32x3::from(-1.0),
            // e423, e431, e412, e321
            self.group4() * Simd32x4::from(-1.0),
        );
        let other = Scalar::from_groups(
            // scalar
            f32::powi(self[scalar], 2)
                + f32::powi(self[e1], 2)
                + f32::powi(self[e2], 2)
                + f32::powi(self[e3], 2)
                + f32::powi(self[e23], 2)
                + f32::powi(self[e31], 2)
                + f32::powi(self[e12], 2)
                + f32::powi(self[e321], 2),
        );
        return MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from(other[scalar]) * reverse.group0(),
            // e1, e2, e3, e4
            Simd32x4::from(other[scalar]) * reverse.group1(),
            // e41, e42, e43
            Simd32x3::from(other[scalar]) * reverse.group2(),
            // e23, e31, e12
            Simd32x3::from(other[scalar]) * reverse.group3(),
            // e423, e431, e412, e321
            Simd32x4::from(other[scalar]) * reverse.group4(),
        );
    }
}
impl std::ops::Div<InversePrefixOrPostfix> for Plane {
    type Output = Plane;
    fn div(self, _rhs: InversePrefixOrPostfix) -> Self::Output {
        self.inverse()
    }
}
impl std::ops::DivAssign<InversePrefixOrPostfix> for Plane {
    fn div_assign(&mut self, _rhs: InversePrefixOrPostfix) {
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
        return Plane::from_groups(
            // e423, e431, e412, e321
            Simd32x4::from(f32::powi(self[e321], -2)) * Simd32x4::from([self[e423] * -1.0, self[e431] * -1.0, self[e412] * -1.0, self[e321] * -1.0]),
        );
    }
}
impl std::ops::Div<InversePrefixOrPostfix> for Point {
    type Output = Point;
    fn div(self, _rhs: InversePrefixOrPostfix) -> Self::Output {
        self.inverse()
    }
}
impl std::ops::DivAssign<InversePrefixOrPostfix> for Point {
    fn div_assign(&mut self, _rhs: InversePrefixOrPostfix) {
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
        return Point::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from(f32::powi(self[e1], 2) + f32::powi(self[e2], 2) + f32::powi(self[e3], 2)) * self.group0(),
        );
    }
}
impl std::ops::Div<InversePrefixOrPostfix> for Scalar {
    type Output = Scalar;
    fn div(self, _rhs: InversePrefixOrPostfix) -> Self::Output {
        self.inverse()
    }
}
impl std::ops::DivAssign<InversePrefixOrPostfix> for Scalar {
    fn div_assign(&mut self, _rhs: InversePrefixOrPostfix) {
        *self = self.inverse()
    }
}
impl Inverse for Scalar {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        0        1
    fn inverse(self) -> Self {
        use crate::elements::*;
        return Scalar::from_groups(/* scalar */ 1.0 / self[scalar]);
    }
}
