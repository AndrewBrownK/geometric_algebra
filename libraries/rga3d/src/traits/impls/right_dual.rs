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
//  Maximum:         0       4       0
//
//  No SIMD:   add/sub     mul     div
//  Minimum:         0       0       0
//   Median:         0       1       0
//  Average:         0       1       0
//  Maximum:         0       4       0
impl RightDual for DualNum {
    type Output = AntiScalar;
    fn right_dual(self) -> Self::Output {
        return AntiScalar::from_groups(/* e1234 */ self.group0()[0]);
    }
}
impl RightDual for Flector {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn right_dual(self) -> Self::Output {
        return Flector::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from([0.0, 0.0, 0.0, (self.group1()[3] * -1.0)]),
            // e423, e431, e412, e321
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], 0.0]),
        );
    }
}
impl RightDual for Horizon {
    type Output = Origin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn right_dual(self) -> Self::Output {
        use crate::elements::*;
        return Origin::from_groups(/* e4 */ (self[e321] * -1.0));
    }
}
impl RightDual for Line {
    type Output = Line;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        3        0
    fn right_dual(self) -> Self::Output {
        return Line::from_groups(
            // e41, e42, e43
            Simd32x3::from([(self.group1()[0] * -1.0), (self.group1()[1] * -1.0), (self.group1()[2] * -1.0)]),
            // e23, e31, e12
            Simd32x3::from(0.0),
        );
    }
}
impl RightDual for Motor {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        3        0
    fn right_dual(self) -> Self::Output {
        return Motor::from_groups(
            // e41, e42, e43, e1234
            Simd32x4::from([(self.group1()[0] * -1.0), (self.group1()[1] * -1.0), (self.group1()[2] * -1.0), self.group1()[3]]),
            // e23, e31, e12, scalar
            Simd32x4::from(0.0),
        );
    }
}
impl RightDual for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        4        0
    fn right_dual(self) -> Self::Output {
        return MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([0.0, self.group0()[0]]),
            // e1, e2, e3, e4
            Simd32x4::from([0.0, 0.0, 0.0, (self.group4()[3] * -1.0)]),
            // e41, e42, e43
            Simd32x3::from([(self.group3()[0] * -1.0), (self.group3()[1] * -1.0), (self.group3()[2] * -1.0)]),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e423, e431, e412, e321
            Simd32x4::from([self.group1()[0], self.group1()[1], self.group1()[2], 0.0]),
        );
    }
}
impl RightDual for Plane {
    type Output = Origin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn right_dual(self) -> Self::Output {
        return Origin::from_groups(/* e4 */ (self.group0()[3] * -1.0));
    }
}
impl RightDual for Point {
    type Output = Plane;
    fn right_dual(self) -> Self::Output {
        return Plane::from_groups(/* e423, e431, e412, e321 */ Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], 0.0]));
    }
}
impl RightDual for Scalar {
    type Output = Scalar;
    fn right_dual(self) -> Self::Output {
        return self;
    }
}
