use crate::traits::AntiSquareRoot;
use crate::traits::FlatWeightNormSquared;
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
//   Median:         2       4       1
//  Average:         1       4       1
//  Maximum:         7      13       1
//
//  No SIMD:   add/sub     mul     div
//  Minimum:         0       1       1
//   Median:         2       7       1
//  Average:         1       8       1
//  Maximum:         7      24       1
impl std::ops::Div<unitize> for AntiScalar {
    type Output = AntiScalar;
    fn div(self, _rhs: unitize) -> Self::Output {
        self.unitize()
    }
}
impl std::ops::DivAssign<unitize> for AntiScalar {
    fn div_assign(&mut self, _rhs: unitize) {
        *self = self.unitize()
    }
}
impl Unitize for AntiScalar {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        1
    fn unitize(self) -> Self {
        use crate::elements::*;
        return AntiScalar::from_groups(/* e1234 */ self[e1234] / (self.flat_weight_norm_squared().anti_square_root()[e1234]));
    }
}
impl std::ops::Div<unitize> for DualNum {
    type Output = DualNum;
    fn div(self, _rhs: unitize) -> Self::Output {
        self.unitize()
    }
}
impl std::ops::DivAssign<unitize> for DualNum {
    fn div_assign(&mut self, _rhs: unitize) {
        *self = self.unitize()
    }
}
impl Unitize for DualNum {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        1
    //    simd2        0        1        0
    // Totals...
    // yes simd        0        2        1
    //  no simd        0        3        1
    fn unitize(self) -> Self {
        use crate::elements::*;
        return DualNum::from_groups(/* scalar, e1234 */ Simd32x2::from(1.0 / self.flat_weight_norm_squared().anti_square_root()[e1234]) * self.group0());
    }
}
impl std::ops::Div<unitize> for Flector {
    type Output = Flector;
    fn div(self, _rhs: unitize) -> Self::Output {
        self.unitize()
    }
}
impl std::ops::DivAssign<unitize> for Flector {
    fn div_assign(&mut self, _rhs: unitize) {
        *self = self.unitize()
    }
}
impl Unitize for Flector {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        4        1
    //    simd4        0        2        0
    // Totals...
    // yes simd        3        6        1
    //  no simd        3       12        1
    fn unitize(self) -> Self {
        use crate::elements::*;
        let geometric_anti_product = AntiScalar::from_groups(/* e1234 */ 1.0 / self.flat_weight_norm_squared().anti_square_root()[e1234]);
        return Flector::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from(geometric_anti_product[e1234]) * self.group0(),
            // e423, e431, e412, e321
            Simd32x4::from(geometric_anti_product[e1234]) * self.group1(),
        );
    }
}
impl std::ops::Div<unitize> for Line {
    type Output = Line;
    fn div(self, _rhs: unitize) -> Self::Output {
        self.unitize()
    }
}
impl std::ops::DivAssign<unitize> for Line {
    fn div_assign(&mut self, _rhs: unitize) {
        *self = self.unitize()
    }
}
impl Unitize for Line {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        3        1
    //    simd3        0        2        0
    // Totals...
    // yes simd        2        5        1
    //  no simd        2        9        1
    fn unitize(self) -> Self {
        use crate::elements::*;
        let geometric_anti_product = AntiScalar::from_groups(/* e1234 */ 1.0 / self.flat_weight_norm_squared().anti_square_root()[e1234]);
        return Line::from_groups(
            // e41, e42, e43
            Simd32x3::from(geometric_anti_product[e1234]) * self.group0(),
            // e23, e31, e12
            Simd32x3::from(geometric_anti_product[e1234]) * self.group1(),
        );
    }
}
impl std::ops::Div<unitize> for Motor {
    type Output = Motor;
    fn div(self, _rhs: unitize) -> Self::Output {
        self.unitize()
    }
}
impl std::ops::DivAssign<unitize> for Motor {
    fn div_assign(&mut self, _rhs: unitize) {
        *self = self.unitize()
    }
}
impl Unitize for Motor {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        4        1
    //    simd4        0        2        0
    // Totals...
    // yes simd        3        6        1
    //  no simd        3       12        1
    fn unitize(self) -> Self {
        use crate::elements::*;
        let geometric_anti_product = AntiScalar::from_groups(/* e1234 */ 1.0 / self.flat_weight_norm_squared().anti_square_root()[e1234]);
        return Motor::from_groups(
            // e41, e42, e43, e1234
            Simd32x4::from(geometric_anti_product[e1234]) * self.group0(),
            // e23, e31, e12, scalar
            Simd32x4::from(geometric_anti_product[e1234]) * self.group1(),
        );
    }
}
impl std::ops::Div<unitize> for MultiVector {
    type Output = MultiVector;
    fn div(self, _rhs: unitize) -> Self::Output {
        self.unitize()
    }
}
impl std::ops::DivAssign<unitize> for MultiVector {
    fn div_assign(&mut self, _rhs: unitize) {
        *self = self.unitize()
    }
}
impl Unitize for MultiVector {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        7        8        1
    //    simd2        0        1        0
    //    simd3        0        2        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        7       13        1
    //  no simd        7       24        1
    fn unitize(self) -> Self {
        use crate::elements::*;
        let geometric_anti_product = AntiScalar::from_groups(/* e1234 */ 1.0 / self.flat_weight_norm_squared().anti_square_root()[e1234]);
        return MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from(geometric_anti_product[e1234]) * self.group0(),
            // e1, e2, e3, e4
            Simd32x4::from(geometric_anti_product[e1234]) * self.group1(),
            // e41, e42, e43
            Simd32x3::from(geometric_anti_product[e1234]) * self.group2(),
            // e23, e31, e12
            Simd32x3::from(geometric_anti_product[e1234]) * self.group3(),
            // e423, e431, e412, e321
            Simd32x4::from(geometric_anti_product[e1234]) * self.group4(),
        );
    }
}
impl std::ops::Div<unitize> for Origin {
    type Output = Origin;
    fn div(self, _rhs: unitize) -> Self::Output {
        self.unitize()
    }
}
impl std::ops::DivAssign<unitize> for Origin {
    fn div_assign(&mut self, _rhs: unitize) {
        *self = self.unitize()
    }
}
impl Unitize for Origin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        1
    fn unitize(self) -> Self {
        use crate::elements::*;
        return Origin::from_groups(/* e4 */ self[e4] / (self.flat_weight_norm_squared().anti_square_root()[e1234]));
    }
}
impl std::ops::Div<unitize> for Plane {
    type Output = Plane;
    fn div(self, _rhs: unitize) -> Self::Output {
        self.unitize()
    }
}
impl std::ops::DivAssign<unitize> for Plane {
    fn div_assign(&mut self, _rhs: unitize) {
        *self = self.unitize()
    }
}
impl Unitize for Plane {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        3        1
    //    simd4        0        1        0
    // Totals...
    // yes simd        2        4        1
    //  no simd        2        7        1
    fn unitize(self) -> Self {
        use crate::elements::*;
        return Plane::from_groups(
            // e423, e431, e412, e321
            Simd32x4::from(1.0 / self.flat_weight_norm_squared().anti_square_root()[e1234]) * self.group0(),
        );
    }
}
impl std::ops::Div<unitize> for Point {
    type Output = Point;
    fn div(self, _rhs: unitize) -> Self::Output {
        self.unitize()
    }
}
impl std::ops::DivAssign<unitize> for Point {
    fn div_assign(&mut self, _rhs: unitize) {
        *self = self.unitize()
    }
}
impl Unitize for Point {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        1
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        2        1
    //  no simd        0        5        1
    fn unitize(self) -> Self {
        use crate::elements::*;
        return Point::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from(1.0 / self.flat_weight_norm_squared().anti_square_root()[e1234]) * self.group0(),
        );
    }
}
