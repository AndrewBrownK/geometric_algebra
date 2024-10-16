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
//  Average:         0       2       0
//  Maximum:         0      10       0
//
//  No SIMD:   add/sub     mul     div
//  Minimum:         0       0       0
//   Median:         0       1       0
//  Average:         0       2       0
//  Maximum:         0      10       0
impl AntiReverse for AntiScalar {
    fn anti_reverse(self) -> Self {
        return self;
    }
}
impl AntiReverse for DualNum {
    fn anti_reverse(self) -> Self {
        return self;
    }
}
impl AntiReverse for Flector {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        4        0
    fn anti_reverse(self) -> Self {
        return Flector::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from([(self.group0()[0] * -1.0), (self.group0()[1] * -1.0), (self.group0()[2] * -1.0), (self.group0()[3] * -1.0)]),
            // e423, e431, e412, e321
            self.group1(),
        );
    }
}
impl AntiReverse for Horizon {
    fn anti_reverse(self) -> Self {
        return self;
    }
}
impl AntiReverse for Line {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        6        0
    fn anti_reverse(self) -> Self {
        return Line::from_groups(
            // e41, e42, e43
            Simd32x3::from([(self.group0()[0] * -1.0), (self.group0()[1] * -1.0), (self.group0()[2] * -1.0)]),
            // e23, e31, e12
            Simd32x3::from([(self.group1()[0] * -1.0), (self.group1()[1] * -1.0), (self.group1()[2] * -1.0)]),
        );
    }
}
impl AntiReverse for Motor {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        6        0
    fn anti_reverse(self) -> Self {
        return Motor::from_groups(
            // e41, e42, e43, e1234
            Simd32x4::from([(self.group0()[0] * -1.0), (self.group0()[1] * -1.0), (self.group0()[2] * -1.0), self.group0()[3]]),
            // e23, e31, e12, scalar
            Simd32x4::from([(self.group1()[0] * -1.0), (self.group1()[1] * -1.0), (self.group1()[2] * -1.0), self.group1()[3]]),
        );
    }
}
impl AntiReverse for MultiVector {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0       10        0
    fn anti_reverse(self) -> Self {
        return MultiVector::from_groups(
            // scalar, e1234
            self.group0(),
            // e1, e2, e3, e4
            Simd32x4::from([(self.group1()[0] * -1.0), (self.group1()[1] * -1.0), (self.group1()[2] * -1.0), (self.group1()[3] * -1.0)]),
            // e41, e42, e43
            Simd32x3::from([(self.group2()[0] * -1.0), (self.group2()[1] * -1.0), (self.group2()[2] * -1.0)]),
            // e23, e31, e12
            Simd32x3::from([(self.group3()[0] * -1.0), (self.group3()[1] * -1.0), (self.group3()[2] * -1.0)]),
            // e423, e431, e412, e321
            self.group4(),
        );
    }
}
impl AntiReverse for Origin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn anti_reverse(self) -> Self {
        use crate::elements::*;
        return Origin::from_groups(/* e4 */ (self[e4] * -1.0));
    }
}
impl AntiReverse for Plane {
    fn anti_reverse(self) -> Self {
        return self;
    }
}
impl AntiReverse for Point {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        4        0
    fn anti_reverse(self) -> Self {
        return Point::from_groups(/* e1, e2, e3, e4 */ Simd32x4::from([
            (self.group0()[0] * -1.0),
            (self.group0()[1] * -1.0),
            (self.group0()[2] * -1.0),
            (self.group0()[3] * -1.0),
        ]));
    }
}
impl AntiReverse for Scalar {
    fn anti_reverse(self) -> Self {
        return self;
    }
}
