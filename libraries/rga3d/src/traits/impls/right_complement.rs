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
impl std::ops::Div<right_complement> for AntiScalar {
    type Output = AntiScalar;
    fn div(self, _rhs: right_complement) -> Self::Output {
        self.right_complement()
    }
}
impl std::ops::DivAssign<right_complement> for AntiScalar {
    fn div_assign(&mut self, _rhs: right_complement) {
        *self = self.right_complement()
    }
}
impl RightComplement for AntiScalar {
    type Output = AntiScalar;
    fn right_complement(self) -> Self::Output {
        return self;
    }
}
impl std::ops::Div<right_complement> for DualNum {
    type Output = DualNum;
    fn div(self, _rhs: right_complement) -> Self::Output {
        self.right_complement()
    }
}
impl std::ops::DivAssign<right_complement> for DualNum {
    fn div_assign(&mut self, _rhs: right_complement) {
        *self = self.right_complement()
    }
}
impl RightComplement for DualNum {
    type Output = DualNum;
    fn right_complement(self) -> Self::Output {
        use crate::elements::*;
        return DualNum::from_groups(/* scalar, e1234 */ Simd32x2::from([self[e1234], self[scalar]]));
    }
}
impl std::ops::Div<right_complement> for Flector {
    type Output = Flector;
    fn div(self, _rhs: right_complement) -> Self::Output {
        self.right_complement()
    }
}
impl std::ops::DivAssign<right_complement> for Flector {
    fn div_assign(&mut self, _rhs: right_complement) {
        *self = self.right_complement()
    }
}
impl RightComplement for Flector {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn right_complement(self) -> Self::Output {
        use crate::elements::*;
        return Flector::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from([self[e423], self[e431], self[e412], self[e321]]) * Simd32x4::from(-1.0),
            // e423, e431, e412, e321
            self.group0(),
        );
    }
}
impl std::ops::Div<right_complement> for Horizon {
    type Output = Origin;
    fn div(self, _rhs: right_complement) -> Self::Output {
        self.right_complement()
    }
}
impl RightComplement for Horizon {
    type Output = Origin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn right_complement(self) -> Self::Output {
        use crate::elements::*;
        return Origin::from_groups(/* e4 */ self[e321] * -1.0);
    }
}
impl std::ops::Div<right_complement> for Line {
    type Output = Line;
    fn div(self, _rhs: right_complement) -> Self::Output {
        self.right_complement()
    }
}
impl std::ops::DivAssign<right_complement> for Line {
    fn div_assign(&mut self, _rhs: right_complement) {
        *self = self.right_complement()
    }
}
impl RightComplement for Line {
    type Output = Line;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        2        0
    // no simd        0        6        0
    fn right_complement(self) -> Self::Output {
        use crate::elements::*;
        return Line::from_groups(
            // e41, e42, e43
            Simd32x3::from([self[e23], self[e31], self[e12]]) * Simd32x3::from(-1.0),
            // e23, e31, e12
            Simd32x3::from([self[e41], self[e42], self[e43]]) * Simd32x3::from(-1.0),
        );
    }
}
impl std::ops::Div<right_complement> for Motor {
    type Output = Motor;
    fn div(self, _rhs: right_complement) -> Self::Output {
        self.right_complement()
    }
}
impl std::ops::DivAssign<right_complement> for Motor {
    fn div_assign(&mut self, _rhs: right_complement) {
        *self = self.right_complement()
    }
}
impl RightComplement for Motor {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        6        0
    fn right_complement(self) -> Self::Output {
        use crate::elements::*;
        return Motor::from_groups(
            // e41, e42, e43, e1234
            Simd32x4::from([self[e23] * -1.0, self[e31] * -1.0, self[e12] * -1.0, self[scalar]]),
            // e23, e31, e12, scalar
            Simd32x4::from([self[e41] * -1.0, self[e42] * -1.0, self[e43] * -1.0, self[e1234]]),
        );
    }
}
impl std::ops::Div<right_complement> for MultiVector {
    type Output = MultiVector;
    fn div(self, _rhs: right_complement) -> Self::Output {
        self.right_complement()
    }
}
impl std::ops::DivAssign<right_complement> for MultiVector {
    fn div_assign(&mut self, _rhs: right_complement) {
        *self = self.right_complement()
    }
}
impl RightComplement for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        0        2        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        3        0
    //  no simd        0       10        0
    fn right_complement(self) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([self[e1234], self[scalar]]),
            // e1, e2, e3, e4
            Simd32x4::from([self[e423], self[e431], self[e412], self[e321]]) * Simd32x4::from(-1.0),
            // e41, e42, e43
            Simd32x3::from([self[e23], self[e31], self[e12]]) * Simd32x3::from(-1.0),
            // e23, e31, e12
            Simd32x3::from([self[e41], self[e42], self[e43]]) * Simd32x3::from(-1.0),
            // e423, e431, e412, e321
            self.group1(),
        );
    }
}
impl std::ops::Div<right_complement> for Origin {
    type Output = Origin;
    fn div(self, _rhs: right_complement) -> Self::Output {
        self.right_complement()
    }
}
impl std::ops::DivAssign<right_complement> for Origin {
    fn div_assign(&mut self, _rhs: right_complement) {
        *self = self.right_complement()
    }
}
impl RightComplement for Origin {
    type Output = Origin;
    fn right_complement(self) -> Self::Output {
        return self;
    }
}
impl std::ops::Div<right_complement> for Plane {
    type Output = Point;
    fn div(self, _rhs: right_complement) -> Self::Output {
        self.right_complement()
    }
}
impl RightComplement for Plane {
    type Output = Point;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn right_complement(self) -> Self::Output {
        use crate::elements::*;
        return Point::from_groups(/* e1, e2, e3, e4 */ Simd32x4::from([self[e423], self[e431], self[e412], self[e321]]) * Simd32x4::from(-1.0));
    }
}
impl std::ops::Div<right_complement> for Point {
    type Output = Point;
    fn div(self, _rhs: right_complement) -> Self::Output {
        self.right_complement()
    }
}
impl std::ops::DivAssign<right_complement> for Point {
    fn div_assign(&mut self, _rhs: right_complement) {
        *self = self.right_complement()
    }
}
impl RightComplement for Point {
    type Output = Point;
    fn right_complement(self) -> Self::Output {
        return self;
    }
}
impl std::ops::Div<right_complement> for Scalar {
    type Output = Scalar;
    fn div(self, _rhs: right_complement) -> Self::Output {
        self.right_complement()
    }
}
impl std::ops::DivAssign<right_complement> for Scalar {
    fn div_assign(&mut self, _rhs: right_complement) {
        *self = self.right_complement()
    }
}
impl RightComplement for Scalar {
    type Output = Scalar;
    fn right_complement(self) -> Self::Output {
        return self;
    }
}
