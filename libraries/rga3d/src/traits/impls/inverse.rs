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
    //           add/sub      mul      div
    //      f32        0        0        1
    //    simd2        0        1        0
    // Totals...
    // yes simd        0        1        1
    //  no simd        0        2        1
    fn inverse(self) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(/* scalar */ f32::powi(self[scalar], 2));
        let other = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return DualNum::from_groups(/* scalar, e1234 */ Simd32x2::from(other[scalar]) * self.group0());
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
    //      f32        3        0        1
    //    simd4        0        2        0
    // Totals...
    // yes simd        3        2        1
    //  no simd        3        8        1
    fn inverse(self) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(/* scalar */ f32::powi(self[e1], 2) + f32::powi(self[e2], 2) + f32::powi(self[e3], 2) - f32::powi(self[e321], 2));
        let other = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return Flector::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from(other[scalar]) * self.group0(),
            // e423, e431, e412, e321
            Simd32x4::from(other[scalar]) * self.group1(),
        );
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
    // f32        0        2        1
    fn inverse(self) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(/* scalar */ f32::powi(self[e321], 2) * -1.0);
        let other = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return Horizon::from_groups(/* e321 */ self[e321] * other[scalar]);
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
    //      f32        2        0        1
    //    simd3        0        2        0
    // Totals...
    // yes simd        2        2        1
    //  no simd        2        6        1
    fn inverse(self) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(/* scalar */ -f32::powi(self[e23], 2) - f32::powi(self[e31], 2) - f32::powi(self[e12], 2));
        let other = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return Line::from_groups(
            // e41, e42, e43
            Simd32x3::from(other[scalar]) * self.group0(),
            // e23, e31, e12
            Simd32x3::from(other[scalar]) * self.group1(),
        );
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
    //      f32        3        0        1
    //    simd4        0        2        0
    // Totals...
    // yes simd        3        2        1
    //  no simd        3        8        1
    fn inverse(self) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(
            // scalar
            f32::powi(self[scalar], 2) - f32::powi(self[e23], 2) - f32::powi(self[e31], 2) - f32::powi(self[e12], 2),
        );
        let other = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return Motor::from_groups(
            // e41, e42, e43, e1234
            Simd32x4::from(other[scalar]) * self.group0(),
            // e23, e31, e12, scalar
            Simd32x4::from(other[scalar]) * self.group1(),
        );
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
    //      f32        7        0        1
    //    simd2        0        1        0
    //    simd3        0        2        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        7        5        1
    //  no simd        7       16        1
    fn inverse(self) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(
            // scalar
            f32::powi(self[scalar], 2) + f32::powi(self[e1], 2) + f32::powi(self[e2], 2) + f32::powi(self[e3], 2)
                - f32::powi(self[e23], 2)
                - f32::powi(self[e31], 2)
                - f32::powi(self[e12], 2)
                - f32::powi(self[e321], 2),
        );
        let other = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from(other[scalar]) * self.group0(),
            // e1, e2, e3, e4
            Simd32x4::from(other[scalar]) * self.group1(),
            // e41, e42, e43
            Simd32x3::from(other[scalar]) * self.group2(),
            // e23, e31, e12
            Simd32x3::from(other[scalar]) * self.group3(),
            // e423, e431, e412, e321
            Simd32x4::from(other[scalar]) * self.group4(),
        );
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
    //      f32        0        1        1
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        2        1
    //  no simd        0        5        1
    fn inverse(self) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(/* scalar */ f32::powi(self[e321], 2) * -1.0);
        let other = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return Plane::from_groups(/* e423, e431, e412, e321 */ Simd32x4::from(other[scalar]) * self.group0());
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
    //      f32        2        0        1
    //    simd4        0        1        0
    // Totals...
    // yes simd        2        1        1
    //  no simd        2        4        1
    fn inverse(self) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(/* scalar */ f32::powi(self[e1], 2) + f32::powi(self[e2], 2) + f32::powi(self[e3], 2));
        let other = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return Point::from_groups(/* e1, e2, e3, e4 */ Simd32x4::from(other[scalar]) * self.group0());
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
    // f32        0        1        1
    fn inverse(self) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(/* scalar */ f32::powi(self[scalar], 2));
        let other = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return Scalar::from_groups(/* scalar */ other[scalar] * self[scalar]);
    }
}
