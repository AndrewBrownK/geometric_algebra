// Note on Operative Statistics:
// Operative Statistics are not a precise predictor of performance or performance comparisons.
// This is due to varying hardware capabilities and compiler optimizations.
// As always, where performance is a concern, there is no substitute for
// real measurements on real work-loads on real hardware.
// Disclaimer aside, enjoy the fun information =)
//
// Total Implementations: 5
//
// Yes SIMD:   add/sub     mul     div
//  Minimum:         0       0       0
//   Median:         0       0       0
//  Average:         0       1       0
//  Maximum:         0       3       0
//
//  No SIMD:   add/sub     mul     div
//  Minimum:         0       0       0
//   Median:         0       0       0
//  Average:         0       2       0
//  Maximum:         0       7       0
impl std::ops::Div<support> for Flector {
    type Output = Line;
    fn div(self, _rhs: support) -> Self::Output {
        self.support()
    }
}
impl Support for Flector {
    type Output = Line;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd3        0        1        0
    // Totals...
    // yes simd        0        2        0
    //  no simd        0        4        0
    fn support(self) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual = Flector::from_groups(
            // e1, e2, e3, e4
            Simd32x3::from(0.0).with_w(self[e321] * -1.0),
            // e423, e431, e412, e321
            Simd32x4::from([self[e1], self[e2], self[e3], 0.0]),
        );
        let self_2 = Origin::from_groups(/* e4 */ 1.0);
        return Line::from_groups(
            // e41, e42, e43
            Simd32x3::from(self_2[e4]) * right_anti_dual.group0().xyz(),
            // e23, e31, e12
            Simd32x3::from(0.0),
        );
    }
}
impl std::ops::Div<support> for Line {
    type Output = Scalar;
    fn div(self, _rhs: support) -> Self::Output {
        self.support()
    }
}
impl Support for Line {
    type Output = Scalar;
    fn support(self) -> Self::Output {
        return Scalar::from_groups(/* scalar */ 0.0);
    }
}
impl std::ops::Div<support> for Motor {
    type Output = Scalar;
    fn div(self, _rhs: support) -> Self::Output {
        self.support()
    }
}
impl Support for Motor {
    type Output = Scalar;
    fn support(self) -> Self::Output {
        return Scalar::from_groups(/* scalar */ 0.0);
    }
}
impl std::ops::Div<support> for MultiVector {
    type Output = Line;
    fn div(self, _rhs: support) -> Self::Output {
        self.support()
    }
}
impl Support for MultiVector {
    type Output = Line;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd3        0        2        0
    // Totals...
    // yes simd        0        3        0
    //  no simd        0        7        0
    fn support(self) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual = MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([0.0, self[scalar]]),
            // e1, e2, e3, e4
            Simd32x3::from(0.0).with_w(self[e321] * -1.0),
            // e41, e42, e43
            self.group3() * Simd32x3::from(-1.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e423, e431, e412, e321
            Simd32x4::from([self[e1], self[e2], self[e3], 0.0]),
        );
        let self_2 = Origin::from_groups(/* e4 */ 1.0);
        return Line::from_groups(
            // e41, e42, e43
            Simd32x3::from(self_2[e4]) * right_anti_dual.group1().xyz(),
            // e23, e31, e12
            Simd32x3::from(0.0),
        );
    }
}
impl std::ops::Div<support> for Point {
    type Output = Scalar;
    fn div(self, _rhs: support) -> Self::Output {
        self.support()
    }
}
impl Support for Point {
    type Output = Scalar;
    fn support(self) -> Self::Output {
        return Scalar::from_groups(/* scalar */ 0.0);
    }
}
