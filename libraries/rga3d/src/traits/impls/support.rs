// Note on Operative Statistics:
// Operative Statistics are not a precise predictor of performance or performance comparisons.
// This is due to varying hardware capabilities and compiler optimizations.
// As always, where performance is a concern, there is no substitute for
// real measurements on real work-loads on real hardware.
// Disclaimer aside, enjoy the fun information =)
//
// Total Implementations: 2
//
// Yes SIMD:   add/sub     mul     div
//  Minimum:         0       1       0
//   Median:         0       1       0
//  Average:         0       1       0
//  Maximum:         0       1       0
//
//  No SIMD:   add/sub     mul     div
//  Minimum:         0       1       0
//   Median:         0       1       0
//  Average:         0       1       0
//  Maximum:         0       1       0
impl std::ops::Div<SupportPrefixOrPostfix> for Flector {
    type Output = Line;
    fn div(self, _rhs: SupportPrefixOrPostfix) -> Self::Output {
        self.support()
    }
}
impl Support for Flector {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn support(self) -> Line {
        use crate::elements::*;
        return Line::from_groups(
            // e41, e42, e43
            Simd32x3::from(0.0).with_w(self[e321] * -1.0).xyz(),
            // e23, e31, e12
            Simd32x3::from(0.0),
        );
    }
}
impl std::ops::Div<SupportPrefixOrPostfix> for MultiVector {
    type Output = Line;
    fn div(self, _rhs: SupportPrefixOrPostfix) -> Self::Output {
        self.support()
    }
}
impl Support for MultiVector {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn support(self) -> Line {
        use crate::elements::*;
        return Line::from_groups(
            // e41, e42, e43
            Simd32x3::from(0.0).with_w(self[e321] * -1.0).xyz(),
            // e23, e31, e12
            Simd32x3::from(0.0),
        );
    }
}
