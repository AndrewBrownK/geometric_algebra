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
//  Minimum:         0       1       1
//   Median:         2       2       1
//  Average:         1       2       1
//  Maximum:         7       5       1
//
//  No SIMD:   add/sub     mul     div
//  Minimum:         0       1       1
//   Median:         2       5       1
//  Average:         1       5       1
//  Maximum:         7      16       1
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
    // f32        0        1        1
    fn anti_inverse(self) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(/* e1234 */ f32::powi(self[e1234], 2));
        let other = AntiScalar::from_groups(/* e1234 */ 1.0 / anti_dot_product[e1234]);
        return AntiScalar::from_groups(/* e1234 */ other[e1234] * self[e1234]);
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
    //           add/sub      mul      div
    //      f32        0        0        1
    //    simd2        0        1        0
    // Totals...
    // yes simd        0        1        1
    //  no simd        0        2        1
    fn anti_inverse(self) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(/* e1234 */ f32::powi(self[e1234], 2));
        let other = AntiScalar::from_groups(/* e1234 */ 1.0 / anti_dot_product[e1234]);
        return DualNum::from_groups(/* scalar, e1234 */ Simd32x2::from(other[e1234]) * self.group0());
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
    //      f32        3        0        1
    //    simd4        0        2        0
    // Totals...
    // yes simd        3        2        1
    //  no simd        3        8        1
    fn anti_inverse(self) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(/* e1234 */ f32::powi(self[e423], 2) + f32::powi(self[e431], 2) + f32::powi(self[e412], 2) - f32::powi(self[e4], 2));
        let other = AntiScalar::from_groups(/* e1234 */ 1.0 / anti_dot_product[e1234]);
        return Flector::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from(other[e1234]) * self.group0(),
            // e423, e431, e412, e321
            Simd32x4::from(other[e1234]) * self.group1(),
        );
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
    //      f32        2        0        1
    //    simd3        0        2        0
    // Totals...
    // yes simd        2        2        1
    //  no simd        2        6        1
    fn anti_inverse(self) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(/* e1234 */ -f32::powi(self[e41], 2) - f32::powi(self[e42], 2) - f32::powi(self[e43], 2));
        let other = AntiScalar::from_groups(/* e1234 */ 1.0 / anti_dot_product[e1234]);
        return Line::from_groups(
            // e41, e42, e43
            Simd32x3::from(other[e1234]) * self.group0(),
            // e23, e31, e12
            Simd32x3::from(other[e1234]) * self.group1(),
        );
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
    //      f32        3        0        1
    //    simd4        0        2        0
    // Totals...
    // yes simd        3        2        1
    //  no simd        3        8        1
    fn anti_inverse(self) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(/* e1234 */ f32::powi(self[e1234], 2) - f32::powi(self[e41], 2) - f32::powi(self[e42], 2) - f32::powi(self[e43], 2));
        let other = AntiScalar::from_groups(/* e1234 */ 1.0 / anti_dot_product[e1234]);
        return Motor::from_groups(
            // e41, e42, e43, e1234
            Simd32x4::from(other[e1234]) * self.group0(),
            // e23, e31, e12, scalar
            Simd32x4::from(other[e1234]) * self.group1(),
        );
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
    //      f32        7        0        1
    //    simd2        0        1        0
    //    simd3        0        2        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        7        5        1
    //  no simd        7       16        1
    fn anti_inverse(self) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(
            // e1234
            f32::powi(self[e1234], 2) + f32::powi(self[e423], 2) + f32::powi(self[e431], 2) + f32::powi(self[e412], 2)
                - f32::powi(self[e4], 2)
                - f32::powi(self[e41], 2)
                - f32::powi(self[e42], 2)
                - f32::powi(self[e43], 2),
        );
        let other = AntiScalar::from_groups(/* e1234 */ 1.0 / anti_dot_product[e1234]);
        return MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from(other[e1234]) * self.group0(),
            // e1, e2, e3, e4
            Simd32x4::from(other[e1234]) * self.group1(),
            // e41, e42, e43
            Simd32x3::from(other[e1234]) * self.group2(),
            // e23, e31, e12
            Simd32x3::from(other[e1234]) * self.group3(),
            // e423, e431, e412, e321
            Simd32x4::from(other[e1234]) * self.group4(),
        );
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
    // f32        0        2        1
    fn anti_inverse(self) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(/* e1234 */ f32::powi(self[e4], 2) * -1.0);
        let other = AntiScalar::from_groups(/* e1234 */ 1.0 / anti_dot_product[e1234]);
        return Origin::from_groups(/* e4 */ other[e1234] * self[e4]);
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
    //      f32        2        0        1
    //    simd4        0        1        0
    // Totals...
    // yes simd        2        1        1
    //  no simd        2        4        1
    fn anti_inverse(self) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(/* e1234 */ f32::powi(self[e423], 2) + f32::powi(self[e431], 2) + f32::powi(self[e412], 2));
        let other = AntiScalar::from_groups(/* e1234 */ 1.0 / anti_dot_product[e1234]);
        return Plane::from_groups(/* e423, e431, e412, e321 */ Simd32x4::from(other[e1234]) * self.group0());
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
    //      f32        0        1        1
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        2        1
    //  no simd        0        5        1
    fn anti_inverse(self) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(/* e1234 */ f32::powi(self[e4], 2) * -1.0);
        let other = AntiScalar::from_groups(/* e1234 */ 1.0 / anti_dot_product[e1234]);
        return Point::from_groups(/* e1, e2, e3, e4 */ Simd32x4::from(other[e1234]) * self.group0());
    }
}
