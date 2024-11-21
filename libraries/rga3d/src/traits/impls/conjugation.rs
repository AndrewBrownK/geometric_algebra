// Note on Operative Statistics:
// Operative Statistics are not a precise predictor of performance or performance comparisons.
// This is due to varying hardware capabilities and compiler optimizations.
// As always, where performance is a concern, there is no substitute for
// real measurements on real work-loads on real hardware.
// Disclaimer aside, enjoy the fun information =)
//
// Total Implementations: 11
//
// Yes SIMD:   add/sub     mul     div
//  Minimum:         0       0       0
//   Median:         0       1       0
//  Average:         0       1       0
//  Maximum:         0       6       0
//
//  No SIMD:   add/sub     mul     div
//  Minimum:         0       0       0
//   Median:         0       1       0
//  Average:         0       2       0
//  Maximum:         0      10       0
impl std::ops::Div<conjugation> for AntiScalar {
    type Output = AntiScalar;
    fn div(self, _rhs: conjugation) -> Self::Output {
        self.conjugation()
    }
}
impl std::ops::DivAssign<conjugation> for AntiScalar {
    fn div_assign(&mut self, _rhs: conjugation) {
        *self = self.conjugation()
    }
}
impl Conjugation for AntiScalar {
    fn conjugation(self) -> Self {
        return self;
    }
}
impl std::ops::Div<conjugation> for DualNum {
    type Output = DualNum;
    fn div(self, _rhs: conjugation) -> Self::Output {
        self.conjugation()
    }
}
impl std::ops::DivAssign<conjugation> for DualNum {
    fn div_assign(&mut self, _rhs: conjugation) {
        *self = self.conjugation()
    }
}
impl Conjugation for DualNum {
    fn conjugation(self) -> Self {
        return self;
    }
}
impl std::ops::Div<conjugation> for Flector {
    type Output = Flector;
    fn div(self, _rhs: conjugation) -> Self::Output {
        self.conjugation()
    }
}
impl std::ops::DivAssign<conjugation> for Flector {
    fn div_assign(&mut self, _rhs: conjugation) {
        *self = self.conjugation()
    }
}
impl Conjugation for Flector {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn conjugation(self) -> Self {
        use crate::elements::*;
        return Flector::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from([self[e1], self[e2], self[e3], self[e4]]) * Simd32x4::from(-1.0),
            // e423, e431, e412, e321
            self.group1(),
        );
    }
}
impl std::ops::Div<conjugation> for Horizon {
    type Output = Horizon;
    fn div(self, _rhs: conjugation) -> Self::Output {
        self.conjugation()
    }
}
impl std::ops::DivAssign<conjugation> for Horizon {
    fn div_assign(&mut self, _rhs: conjugation) {
        *self = self.conjugation()
    }
}
impl Conjugation for Horizon {
    fn conjugation(self) -> Self {
        return self;
    }
}
impl std::ops::Div<conjugation> for Line {
    type Output = Line;
    fn div(self, _rhs: conjugation) -> Self::Output {
        self.conjugation()
    }
}
impl std::ops::DivAssign<conjugation> for Line {
    fn div_assign(&mut self, _rhs: conjugation) {
        *self = self.conjugation()
    }
}
impl Conjugation for Line {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        2        0
    // no simd        0        6        0
    fn conjugation(self) -> Self {
        use crate::elements::*;
        return Line::from_groups(
            // e41, e42, e43
            Simd32x3::from([self[e41], self[e42], self[e43]]) * Simd32x3::from(-1.0),
            // e23, e31, e12
            Simd32x3::from([self[e23], self[e31], self[e12]]) * Simd32x3::from(-1.0),
        );
    }
}
impl std::ops::Div<conjugation> for Motor {
    type Output = Motor;
    fn div(self, _rhs: conjugation) -> Self::Output {
        self.conjugation()
    }
}
impl std::ops::DivAssign<conjugation> for Motor {
    fn div_assign(&mut self, _rhs: conjugation) {
        *self = self.conjugation()
    }
}
impl Conjugation for Motor {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        6        0
    fn conjugation(self) -> Self {
        use crate::elements::*;
        return Motor::from_groups(
            // e41, e42, e43, e1234
            Simd32x4::from([self[e41] * -1.0, self[e42] * -1.0, self[e43] * -1.0, self[e1234]]),
            // e23, e31, e12, scalar
            Simd32x4::from([self[e23] * -1.0, self[e31] * -1.0, self[e12] * -1.0, self[scalar]]),
        );
    }
}
impl std::ops::Div<conjugation> for MultiVector {
    type Output = MultiVector;
    fn div(self, _rhs: conjugation) -> Self::Output {
        self.conjugation()
    }
}
impl std::ops::DivAssign<conjugation> for MultiVector {
    fn div_assign(&mut self, _rhs: conjugation) {
        *self = self.conjugation()
    }
}
impl Conjugation for MultiVector {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        0        2        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        3        0
    //  no simd        0       10        0
    fn conjugation(self) -> Self {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e1234
            self.group0(),
            // e1, e2, e3, e4
            Simd32x4::from([self[e1], self[e2], self[e3], self[e4]]) * Simd32x4::from(-1.0),
            // e41, e42, e43
            Simd32x3::from([self[e41], self[e42], self[e43]]) * Simd32x3::from(-1.0),
            // e23, e31, e12
            Simd32x3::from([self[e23], self[e31], self[e12]]) * Simd32x3::from(-1.0),
            // e423, e431, e412, e321
            self.group4(),
        );
    }
}
impl std::ops::Div<conjugation> for Origin {
    type Output = Origin;
    fn div(self, _rhs: conjugation) -> Self::Output {
        self.conjugation()
    }
}
impl std::ops::DivAssign<conjugation> for Origin {
    fn div_assign(&mut self, _rhs: conjugation) {
        *self = self.conjugation()
    }
}
impl Conjugation for Origin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn conjugation(self) -> Self {
        use crate::elements::*;
        return Origin::from_groups(/* e4 */ self[e4] * -1.0);
    }
}
impl std::ops::Div<conjugation> for Plane {
    type Output = Plane;
    fn div(self, _rhs: conjugation) -> Self::Output {
        self.conjugation()
    }
}
impl std::ops::DivAssign<conjugation> for Plane {
    fn div_assign(&mut self, _rhs: conjugation) {
        *self = self.conjugation()
    }
}
impl Conjugation for Plane {
    fn conjugation(self) -> Self {
        return self;
    }
}
impl std::ops::Div<conjugation> for Point {
    type Output = Point;
    fn div(self, _rhs: conjugation) -> Self::Output {
        self.conjugation()
    }
}
impl std::ops::DivAssign<conjugation> for Point {
    fn div_assign(&mut self, _rhs: conjugation) {
        *self = self.conjugation()
    }
}
impl Conjugation for Point {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn conjugation(self) -> Self {
        use crate::elements::*;
        return Point::from_groups(/* e1, e2, e3, e4 */ Simd32x4::from([self[e1], self[e2], self[e3], self[e4]]) * Simd32x4::from(-1.0));
    }
}
impl std::ops::Div<conjugation> for Scalar {
    type Output = Scalar;
    fn div(self, _rhs: conjugation) -> Self::Output {
        self.conjugation()
    }
}
impl std::ops::DivAssign<conjugation> for Scalar {
    fn div_assign(&mut self, _rhs: conjugation) {
        *self = self.conjugation()
    }
}
impl Conjugation for Scalar {
    fn conjugation(self) -> Self {
        return self;
    }
}