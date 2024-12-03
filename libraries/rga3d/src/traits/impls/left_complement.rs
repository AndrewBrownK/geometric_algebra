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
//  Average:         0       0       0
//  Maximum:         0       3       0
//
//  No SIMD:   add/sub     mul     div
//  Minimum:         0       0       0
//   Median:         0       1       0
//  Average:         0       3       0
//  Maximum:         0      10       0
impl std::ops::Div<left_complement> for AntiScalar {
    type Output = Scalar;
    fn div(self, _rhs: left_complement) -> Self::Output {
        self.left_complement()
    }
}
impl LeftComplement for AntiScalar {
    type Output = Scalar;
    fn left_complement(self) -> Self::Output {
        use crate::elements::*;
        return Scalar::from_groups(/* scalar */ self[e1234]);
    }
}
impl std::ops::Div<left_complement> for DualNum {
    type Output = DualNum;
    fn div(self, _rhs: left_complement) -> Self::Output {
        self.left_complement()
    }
}
impl std::ops::DivAssign<left_complement> for DualNum {
    fn div_assign(&mut self, _rhs: left_complement) {
        *self = self.left_complement()
    }
}
impl LeftComplement for DualNum {
    type Output = DualNum;
    fn left_complement(self) -> Self::Output {
        return DualNum::from_groups(/* scalar, e1234 */ self.group0().yx());
    }
}
impl std::ops::Div<left_complement> for Flector {
    type Output = Flector;
    fn div(self, _rhs: left_complement) -> Self::Output {
        self.left_complement()
    }
}
impl std::ops::DivAssign<left_complement> for Flector {
    fn div_assign(&mut self, _rhs: left_complement) {
        *self = self.left_complement()
    }
}
impl LeftComplement for Flector {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn left_complement(self) -> Self::Output {
        return Flector::from_groups(/* e1, e2, e3, e4 */ self.group1(), /* e423, e431, e412, e321 */ self.group0() * Simd32x4::from(-1.0));
    }
}
impl std::ops::Div<left_complement> for Horizon {
    type Output = Origin;
    fn div(self, _rhs: left_complement) -> Self::Output {
        self.left_complement()
    }
}
impl LeftComplement for Horizon {
    type Output = Origin;
    fn left_complement(self) -> Self::Output {
        use crate::elements::*;
        return Origin::from_groups(/* e4 */ self[e321]);
    }
}
impl std::ops::Div<left_complement> for Line {
    type Output = Line;
    fn div(self, _rhs: left_complement) -> Self::Output {
        self.left_complement()
    }
}
impl std::ops::DivAssign<left_complement> for Line {
    fn div_assign(&mut self, _rhs: left_complement) {
        *self = self.left_complement()
    }
}
impl LeftComplement for Line {
    type Output = Line;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        2        0
    // no simd        0        6        0
    fn left_complement(self) -> Self::Output {
        return Line::from_groups(
            // e41, e42, e43
            self.group1() * Simd32x3::from(-1.0),
            // e23, e31, e12
            self.group0() * Simd32x3::from(-1.0),
        );
    }
}
impl std::ops::Div<left_complement> for Motor {
    type Output = Motor;
    fn div(self, _rhs: left_complement) -> Self::Output {
        self.left_complement()
    }
}
impl std::ops::DivAssign<left_complement> for Motor {
    fn div_assign(&mut self, _rhs: left_complement) {
        *self = self.left_complement()
    }
}
impl LeftComplement for Motor {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        2        0
    // no simd        0        8        0
    fn left_complement(self) -> Self::Output {
        return Motor::from_groups(
            // e41, e42, e43, e1234
            self.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e23, e31, e12, scalar
            self.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
        );
    }
}
impl std::ops::Div<left_complement> for MultiVector {
    type Output = MultiVector;
    fn div(self, _rhs: left_complement) -> Self::Output {
        self.left_complement()
    }
}
impl std::ops::DivAssign<left_complement> for MultiVector {
    fn div_assign(&mut self, _rhs: left_complement) {
        *self = self.left_complement()
    }
}
impl LeftComplement for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        0        2        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        3        0
    //  no simd        0       10        0
    fn left_complement(self) -> Self::Output {
        return MultiVector::from_groups(
            // scalar, e1234
            self.group0().yx(),
            // e1, e2, e3, e4
            self.group4(),
            // e41, e42, e43
            self.group3() * Simd32x3::from(-1.0),
            // e23, e31, e12
            self.group2() * Simd32x3::from(-1.0),
            // e423, e431, e412, e321
            self.group1() * Simd32x4::from(-1.0),
        );
    }
}
impl std::ops::Div<left_complement> for Origin {
    type Output = Horizon;
    fn div(self, _rhs: left_complement) -> Self::Output {
        self.left_complement()
    }
}
impl LeftComplement for Origin {
    type Output = Horizon;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn left_complement(self) -> Self::Output {
        use crate::elements::*;
        return Horizon::from_groups(/* e321 */ self[e4] * -1.0);
    }
}
impl std::ops::Div<left_complement> for Plane {
    type Output = Point;
    fn div(self, _rhs: left_complement) -> Self::Output {
        self.left_complement()
    }
}
impl LeftComplement for Plane {
    type Output = Point;
    fn left_complement(self) -> Self::Output {
        return Point::from_groups(/* e1, e2, e3, e4 */ self.group0());
    }
}
impl std::ops::Div<left_complement> for Point {
    type Output = Plane;
    fn div(self, _rhs: left_complement) -> Self::Output {
        self.left_complement()
    }
}
impl LeftComplement for Point {
    type Output = Plane;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn left_complement(self) -> Self::Output {
        return Plane::from_groups(/* e423, e431, e412, e321 */ self.group0() * Simd32x4::from(-1.0));
    }
}
impl std::ops::Div<left_complement> for Scalar {
    type Output = AntiScalar;
    fn div(self, _rhs: left_complement) -> Self::Output {
        self.left_complement()
    }
}
impl LeftComplement for Scalar {
    type Output = AntiScalar;
    fn left_complement(self) -> Self::Output {
        use crate::elements::*;
        return AntiScalar::from_groups(/* e1234 */ self[scalar]);
    }
}
