// Note on Operative Statistics:
// Operative Statistics are not a precise predictor of performance or performance comparisons.
// This is due to varying hardware capabilities and compiler optimizations.
// As always, where performance is a concern, there is no substitute for
// real measurements on real work-loads on real hardware.
// Disclaimer aside, enjoy the fun information =)
//
// Total Implementations: 99
//
// Yes SIMD:   add/sub     mul     div
//  Minimum:         0       1       1
//   Median:         2       2       1
//  Average:         1       1       1
//  Maximum:         7       6       1
//
//  No SIMD:   add/sub     mul     div
//  Minimum:         0       1       1
//   Median:         2       4       1
//  Average:         1       4       1
//  Maximum:         7      17       1
impl std::ops::Div<geometric_anti_quotient> for AntiScalar {
    type Output = geometric_anti_quotient_partial<AntiScalar>;
    fn div(self, _rhs: geometric_anti_quotient) -> Self::Output {
        geometric_anti_quotient_partial(self)
    }
}
impl GeometricAntiQuotient<AntiScalar> for AntiScalar {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        1
    fn geometric_anti_quotient(self, other: AntiScalar) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(/* e1234 */ f32::powi(other[e1234], 2));
        let anti_inverse = AntiScalar::from_groups(/* e1234 */ 1.0 / anti_dot_product[e1234]);
        return AntiScalar::from_groups(/* e1234 */ anti_inverse[e1234] * self[e1234]);
    }
}
impl GeometricAntiQuotient<DualNum> for AntiScalar {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        1
    fn geometric_anti_quotient(self, other: DualNum) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(/* e1234 */ f32::powi(other[e1234], 2));
        let anti_inverse = AntiScalar::from_groups(/* e1234 */ 1.0 / anti_dot_product[e1234]);
        return AntiScalar::from_groups(/* e1234 */ anti_inverse[e1234] * self[e1234]);
    }
}
impl GeometricAntiQuotient<Flector> for AntiScalar {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        1        1
    fn geometric_anti_quotient(self, other: Flector) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(
            // e1234
            f32::powi(other[e423], 2) + f32::powi(other[e431], 2) + f32::powi(other[e412], 2) - f32::powi(other[e4], 2),
        );
        let anti_inverse = AntiScalar::from_groups(/* e1234 */ 1.0 / anti_dot_product[e1234]);
        return AntiScalar::from_groups(/* e1234 */ anti_inverse[e1234] * self[e1234]);
    }
}
impl GeometricAntiQuotient<Line> for AntiScalar {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        1        1
    fn geometric_anti_quotient(self, other: Line) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(/* e1234 */ -f32::powi(other[e41], 2) - f32::powi(other[e42], 2) - f32::powi(other[e43], 2));
        let anti_inverse = AntiScalar::from_groups(/* e1234 */ 1.0 / anti_dot_product[e1234]);
        return AntiScalar::from_groups(/* e1234 */ anti_inverse[e1234] * self[e1234]);
    }
}
impl GeometricAntiQuotient<Motor> for AntiScalar {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        1        1
    fn geometric_anti_quotient(self, other: Motor) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(
            // e1234
            f32::powi(other[e1234], 2) - f32::powi(other[e41], 2) - f32::powi(other[e42], 2) - f32::powi(other[e43], 2),
        );
        let anti_inverse = AntiScalar::from_groups(/* e1234 */ 1.0 / anti_dot_product[e1234]);
        return AntiScalar::from_groups(/* e1234 */ anti_inverse[e1234] * self[e1234]);
    }
}
impl GeometricAntiQuotient<MultiVector> for AntiScalar {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        7        1        1
    fn geometric_anti_quotient(self, other: MultiVector) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(
            // e1234
            f32::powi(other[e1234], 2) + f32::powi(other[e423], 2) + f32::powi(other[e431], 2) + f32::powi(other[e412], 2)
                - f32::powi(other[e4], 2)
                - f32::powi(other[e41], 2)
                - f32::powi(other[e42], 2)
                - f32::powi(other[e43], 2),
        );
        let anti_inverse = AntiScalar::from_groups(/* e1234 */ 1.0 / anti_dot_product[e1234]);
        return AntiScalar::from_groups(/* e1234 */ anti_inverse[e1234] * self[e1234]);
    }
}
impl GeometricAntiQuotient<Origin> for AntiScalar {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        2        1
    fn geometric_anti_quotient(self, other: Origin) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(/* e1234 */ f32::powi(other[e4], 2) * -1.0);
        let anti_inverse = AntiScalar::from_groups(/* e1234 */ 1.0 / anti_dot_product[e1234]);
        return AntiScalar::from_groups(/* e1234 */ anti_inverse[e1234] * self[e1234]);
    }
}
impl GeometricAntiQuotient<Plane> for AntiScalar {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        1        1
    fn geometric_anti_quotient(self, other: Plane) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(/* e1234 */ f32::powi(other[e423], 2) + f32::powi(other[e431], 2) + f32::powi(other[e412], 2));
        let anti_inverse = AntiScalar::from_groups(/* e1234 */ 1.0 / anti_dot_product[e1234]);
        return AntiScalar::from_groups(/* e1234 */ anti_inverse[e1234] * self[e1234]);
    }
}
impl GeometricAntiQuotient<Point> for AntiScalar {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        2        1
    fn geometric_anti_quotient(self, other: Point) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(/* e1234 */ f32::powi(other[e4], 2) * -1.0);
        let anti_inverse = AntiScalar::from_groups(/* e1234 */ 1.0 / anti_dot_product[e1234]);
        return AntiScalar::from_groups(/* e1234 */ anti_inverse[e1234] * self[e1234]);
    }
}
impl std::ops::Div<geometric_anti_quotient> for DualNum {
    type Output = geometric_anti_quotient_partial<DualNum>;
    fn div(self, _rhs: geometric_anti_quotient) -> Self::Output {
        geometric_anti_quotient_partial(self)
    }
}
impl GeometricAntiQuotient<AntiScalar> for DualNum {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        0        1
    //    simd2        0        1        0
    // Totals...
    // yes simd        0        1        1
    //  no simd        0        2        1
    fn geometric_anti_quotient(self, other: AntiScalar) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(/* e1234 */ f32::powi(other[e1234], 2));
        let anti_inverse = AntiScalar::from_groups(/* e1234 */ 1.0 / anti_dot_product[e1234]);
        return DualNum::from_groups(/* scalar, e1234 */ Simd32x2::from(anti_inverse[e1234]) * Simd32x2::from([self[scalar], self[e1234]]));
    }
}
impl GeometricAntiQuotient<DualNum> for DualNum {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        0        1
    //    simd2        0        1        0
    // Totals...
    // yes simd        0        1        1
    //  no simd        0        2        1
    fn geometric_anti_quotient(self, other: DualNum) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(/* e1234 */ f32::powi(other[e1234], 2));
        let anti_inverse = AntiScalar::from_groups(/* e1234 */ 1.0 / anti_dot_product[e1234]);
        return DualNum::from_groups(/* scalar, e1234 */ Simd32x2::from(anti_inverse[e1234]) * Simd32x2::from([self[scalar], self[e1234]]));
    }
}
impl GeometricAntiQuotient<Flector> for DualNum {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        0        1
    //    simd2        0        1        0
    // Totals...
    // yes simd        3        1        1
    //  no simd        3        2        1
    fn geometric_anti_quotient(self, other: Flector) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(
            // e1234
            f32::powi(other[e423], 2) + f32::powi(other[e431], 2) + f32::powi(other[e412], 2) - f32::powi(other[e4], 2),
        );
        let anti_inverse = AntiScalar::from_groups(/* e1234 */ 1.0 / anti_dot_product[e1234]);
        return DualNum::from_groups(/* scalar, e1234 */ Simd32x2::from(anti_inverse[e1234]) * Simd32x2::from([self[scalar], self[e1234]]));
    }
}
impl GeometricAntiQuotient<Line> for DualNum {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        0        1
    //    simd2        0        1        0
    // Totals...
    // yes simd        2        1        1
    //  no simd        2        2        1
    fn geometric_anti_quotient(self, other: Line) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(/* e1234 */ -f32::powi(other[e41], 2) - f32::powi(other[e42], 2) - f32::powi(other[e43], 2));
        let anti_inverse = AntiScalar::from_groups(/* e1234 */ 1.0 / anti_dot_product[e1234]);
        return DualNum::from_groups(/* scalar, e1234 */ Simd32x2::from(anti_inverse[e1234]) * Simd32x2::from([self[scalar], self[e1234]]));
    }
}
impl GeometricAntiQuotient<Motor> for DualNum {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        0        1
    //    simd2        0        1        0
    // Totals...
    // yes simd        3        1        1
    //  no simd        3        2        1
    fn geometric_anti_quotient(self, other: Motor) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(
            // e1234
            f32::powi(other[e1234], 2) - f32::powi(other[e41], 2) - f32::powi(other[e42], 2) - f32::powi(other[e43], 2),
        );
        let anti_inverse = AntiScalar::from_groups(/* e1234 */ 1.0 / anti_dot_product[e1234]);
        return DualNum::from_groups(/* scalar, e1234 */ Simd32x2::from(anti_inverse[e1234]) * Simd32x2::from([self[scalar], self[e1234]]));
    }
}
impl GeometricAntiQuotient<MultiVector> for DualNum {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        7        0        1
    //    simd2        0        1        0
    // Totals...
    // yes simd        7        1        1
    //  no simd        7        2        1
    fn geometric_anti_quotient(self, other: MultiVector) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(
            // e1234
            f32::powi(other[e1234], 2) + f32::powi(other[e423], 2) + f32::powi(other[e431], 2) + f32::powi(other[e412], 2)
                - f32::powi(other[e4], 2)
                - f32::powi(other[e41], 2)
                - f32::powi(other[e42], 2)
                - f32::powi(other[e43], 2),
        );
        let anti_inverse = AntiScalar::from_groups(/* e1234 */ 1.0 / anti_dot_product[e1234]);
        return DualNum::from_groups(/* scalar, e1234 */ Simd32x2::from(anti_inverse[e1234]) * Simd32x2::from([self[scalar], self[e1234]]));
    }
}
impl GeometricAntiQuotient<Origin> for DualNum {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        1
    //    simd2        0        1        0
    // Totals...
    // yes simd        0        2        1
    //  no simd        0        3        1
    fn geometric_anti_quotient(self, other: Origin) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(/* e1234 */ f32::powi(other[e4], 2) * -1.0);
        let anti_inverse = AntiScalar::from_groups(/* e1234 */ 1.0 / anti_dot_product[e1234]);
        return DualNum::from_groups(/* scalar, e1234 */ Simd32x2::from(anti_inverse[e1234]) * Simd32x2::from([self[scalar], self[e1234]]));
    }
}
impl GeometricAntiQuotient<Plane> for DualNum {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        0        1
    //    simd2        0        1        0
    // Totals...
    // yes simd        2        1        1
    //  no simd        2        2        1
    fn geometric_anti_quotient(self, other: Plane) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(/* e1234 */ f32::powi(other[e423], 2) + f32::powi(other[e431], 2) + f32::powi(other[e412], 2));
        let anti_inverse = AntiScalar::from_groups(/* e1234 */ 1.0 / anti_dot_product[e1234]);
        return DualNum::from_groups(/* scalar, e1234 */ Simd32x2::from(anti_inverse[e1234]) * Simd32x2::from([self[scalar], self[e1234]]));
    }
}
impl GeometricAntiQuotient<Point> for DualNum {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        1
    //    simd2        0        1        0
    // Totals...
    // yes simd        0        2        1
    //  no simd        0        3        1
    fn geometric_anti_quotient(self, other: Point) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(/* e1234 */ f32::powi(other[e4], 2) * -1.0);
        let anti_inverse = AntiScalar::from_groups(/* e1234 */ 1.0 / anti_dot_product[e1234]);
        return DualNum::from_groups(/* scalar, e1234 */ Simd32x2::from(anti_inverse[e1234]) * Simd32x2::from([self[scalar], self[e1234]]));
    }
}
impl std::ops::Div<geometric_anti_quotient> for Flector {
    type Output = geometric_anti_quotient_partial<Flector>;
    fn div(self, _rhs: geometric_anti_quotient) -> Self::Output {
        geometric_anti_quotient_partial(self)
    }
}
impl GeometricAntiQuotient<AntiScalar> for Flector {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        0        1
    //    simd4        0        2        0
    // Totals...
    // yes simd        0        2        1
    //  no simd        0        8        1
    fn geometric_anti_quotient(self, other: AntiScalar) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(/* e1234 */ f32::powi(other[e1234], 2));
        let anti_inverse = AntiScalar::from_groups(/* e1234 */ 1.0 / anti_dot_product[e1234]);
        return Flector::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from(anti_inverse[e1234]) * Simd32x4::from([self[e1], self[e2], self[e3], self[e4]]),
            // e423, e431, e412, e321
            Simd32x4::from(anti_inverse[e1234]) * Simd32x4::from([self[e423], self[e431], self[e412], self[e321]]),
        );
    }
}
impl GeometricAntiQuotient<DualNum> for Flector {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        0        1
    //    simd4        0        2        0
    // Totals...
    // yes simd        0        2        1
    //  no simd        0        8        1
    fn geometric_anti_quotient(self, other: DualNum) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(/* e1234 */ f32::powi(other[e1234], 2));
        let anti_inverse = AntiScalar::from_groups(/* e1234 */ 1.0 / anti_dot_product[e1234]);
        return Flector::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from(anti_inverse[e1234]) * Simd32x4::from([self[e1], self[e2], self[e3], self[e4]]),
            // e423, e431, e412, e321
            Simd32x4::from(anti_inverse[e1234]) * Simd32x4::from([self[e423], self[e431], self[e412], self[e321]]),
        );
    }
}
impl GeometricAntiQuotient<Flector> for Flector {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        0        1
    //    simd4        0        2        0
    // Totals...
    // yes simd        3        2        1
    //  no simd        3        8        1
    fn geometric_anti_quotient(self, other: Flector) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(
            // e1234
            f32::powi(other[e423], 2) + f32::powi(other[e431], 2) + f32::powi(other[e412], 2) - f32::powi(other[e4], 2),
        );
        let anti_inverse = AntiScalar::from_groups(/* e1234 */ 1.0 / anti_dot_product[e1234]);
        return Flector::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from(anti_inverse[e1234]) * Simd32x4::from([self[e1], self[e2], self[e3], self[e4]]),
            // e423, e431, e412, e321
            Simd32x4::from(anti_inverse[e1234]) * Simd32x4::from([self[e423], self[e431], self[e412], self[e321]]),
        );
    }
}
impl GeometricAntiQuotient<Line> for Flector {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        0        1
    //    simd4        0        2        0
    // Totals...
    // yes simd        2        2        1
    //  no simd        2        8        1
    fn geometric_anti_quotient(self, other: Line) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(/* e1234 */ -f32::powi(other[e41], 2) - f32::powi(other[e42], 2) - f32::powi(other[e43], 2));
        let anti_inverse = AntiScalar::from_groups(/* e1234 */ 1.0 / anti_dot_product[e1234]);
        return Flector::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from(anti_inverse[e1234]) * Simd32x4::from([self[e1], self[e2], self[e3], self[e4]]),
            // e423, e431, e412, e321
            Simd32x4::from(anti_inverse[e1234]) * Simd32x4::from([self[e423], self[e431], self[e412], self[e321]]),
        );
    }
}
impl GeometricAntiQuotient<Motor> for Flector {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        0        1
    //    simd4        0        2        0
    // Totals...
    // yes simd        3        2        1
    //  no simd        3        8        1
    fn geometric_anti_quotient(self, other: Motor) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(
            // e1234
            f32::powi(other[e1234], 2) - f32::powi(other[e41], 2) - f32::powi(other[e42], 2) - f32::powi(other[e43], 2),
        );
        let anti_inverse = AntiScalar::from_groups(/* e1234 */ 1.0 / anti_dot_product[e1234]);
        return Flector::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from(anti_inverse[e1234]) * Simd32x4::from([self[e1], self[e2], self[e3], self[e4]]),
            // e423, e431, e412, e321
            Simd32x4::from(anti_inverse[e1234]) * Simd32x4::from([self[e423], self[e431], self[e412], self[e321]]),
        );
    }
}
impl GeometricAntiQuotient<MultiVector> for Flector {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        7        0        1
    //    simd4        0        2        0
    // Totals...
    // yes simd        7        2        1
    //  no simd        7        8        1
    fn geometric_anti_quotient(self, other: MultiVector) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(
            // e1234
            f32::powi(other[e1234], 2) + f32::powi(other[e423], 2) + f32::powi(other[e431], 2) + f32::powi(other[e412], 2)
                - f32::powi(other[e4], 2)
                - f32::powi(other[e41], 2)
                - f32::powi(other[e42], 2)
                - f32::powi(other[e43], 2),
        );
        let anti_inverse = AntiScalar::from_groups(/* e1234 */ 1.0 / anti_dot_product[e1234]);
        return Flector::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from(anti_inverse[e1234]) * Simd32x4::from([self[e1], self[e2], self[e3], self[e4]]),
            // e423, e431, e412, e321
            Simd32x4::from(anti_inverse[e1234]) * Simd32x4::from([self[e423], self[e431], self[e412], self[e321]]),
        );
    }
}
impl GeometricAntiQuotient<Origin> for Flector {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        1
    //    simd4        0        2        0
    // Totals...
    // yes simd        0        3        1
    //  no simd        0        9        1
    fn geometric_anti_quotient(self, other: Origin) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(/* e1234 */ f32::powi(other[e4], 2) * -1.0);
        let anti_inverse = AntiScalar::from_groups(/* e1234 */ 1.0 / anti_dot_product[e1234]);
        return Flector::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from(anti_inverse[e1234]) * Simd32x4::from([self[e1], self[e2], self[e3], self[e4]]),
            // e423, e431, e412, e321
            Simd32x4::from(anti_inverse[e1234]) * Simd32x4::from([self[e423], self[e431], self[e412], self[e321]]),
        );
    }
}
impl GeometricAntiQuotient<Plane> for Flector {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        0        1
    //    simd4        0        2        0
    // Totals...
    // yes simd        2        2        1
    //  no simd        2        8        1
    fn geometric_anti_quotient(self, other: Plane) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(/* e1234 */ f32::powi(other[e423], 2) + f32::powi(other[e431], 2) + f32::powi(other[e412], 2));
        let anti_inverse = AntiScalar::from_groups(/* e1234 */ 1.0 / anti_dot_product[e1234]);
        return Flector::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from(anti_inverse[e1234]) * Simd32x4::from([self[e1], self[e2], self[e3], self[e4]]),
            // e423, e431, e412, e321
            Simd32x4::from(anti_inverse[e1234]) * Simd32x4::from([self[e423], self[e431], self[e412], self[e321]]),
        );
    }
}
impl GeometricAntiQuotient<Point> for Flector {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        1
    //    simd4        0        2        0
    // Totals...
    // yes simd        0        3        1
    //  no simd        0        9        1
    fn geometric_anti_quotient(self, other: Point) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(/* e1234 */ f32::powi(other[e4], 2) * -1.0);
        let anti_inverse = AntiScalar::from_groups(/* e1234 */ 1.0 / anti_dot_product[e1234]);
        return Flector::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from(anti_inverse[e1234]) * Simd32x4::from([self[e1], self[e2], self[e3], self[e4]]),
            // e423, e431, e412, e321
            Simd32x4::from(anti_inverse[e1234]) * Simd32x4::from([self[e423], self[e431], self[e412], self[e321]]),
        );
    }
}
impl std::ops::Div<geometric_anti_quotient> for Horizon {
    type Output = geometric_anti_quotient_partial<Horizon>;
    fn div(self, _rhs: geometric_anti_quotient) -> Self::Output {
        geometric_anti_quotient_partial(self)
    }
}
impl GeometricAntiQuotient<AntiScalar> for Horizon {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        1
    fn geometric_anti_quotient(self, other: AntiScalar) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(/* e1234 */ f32::powi(other[e1234], 2));
        let anti_inverse = AntiScalar::from_groups(/* e1234 */ 1.0 / anti_dot_product[e1234]);
        return Horizon::from_groups(/* e321 */ anti_inverse[e1234] * self[e321]);
    }
}
impl GeometricAntiQuotient<DualNum> for Horizon {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        1
    fn geometric_anti_quotient(self, other: DualNum) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(/* e1234 */ f32::powi(other[e1234], 2));
        let anti_inverse = AntiScalar::from_groups(/* e1234 */ 1.0 / anti_dot_product[e1234]);
        return Horizon::from_groups(/* e321 */ anti_inverse[e1234] * self[e321]);
    }
}
impl GeometricAntiQuotient<Flector> for Horizon {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        1        1
    fn geometric_anti_quotient(self, other: Flector) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(
            // e1234
            f32::powi(other[e423], 2) + f32::powi(other[e431], 2) + f32::powi(other[e412], 2) - f32::powi(other[e4], 2),
        );
        let anti_inverse = AntiScalar::from_groups(/* e1234 */ 1.0 / anti_dot_product[e1234]);
        return Horizon::from_groups(/* e321 */ anti_inverse[e1234] * self[e321]);
    }
}
impl GeometricAntiQuotient<Line> for Horizon {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        1        1
    fn geometric_anti_quotient(self, other: Line) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(/* e1234 */ -f32::powi(other[e41], 2) - f32::powi(other[e42], 2) - f32::powi(other[e43], 2));
        let anti_inverse = AntiScalar::from_groups(/* e1234 */ 1.0 / anti_dot_product[e1234]);
        return Horizon::from_groups(/* e321 */ anti_inverse[e1234] * self[e321]);
    }
}
impl GeometricAntiQuotient<Motor> for Horizon {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        1        1
    fn geometric_anti_quotient(self, other: Motor) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(
            // e1234
            f32::powi(other[e1234], 2) - f32::powi(other[e41], 2) - f32::powi(other[e42], 2) - f32::powi(other[e43], 2),
        );
        let anti_inverse = AntiScalar::from_groups(/* e1234 */ 1.0 / anti_dot_product[e1234]);
        return Horizon::from_groups(/* e321 */ anti_inverse[e1234] * self[e321]);
    }
}
impl GeometricAntiQuotient<MultiVector> for Horizon {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        7        1        1
    fn geometric_anti_quotient(self, other: MultiVector) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(
            // e1234
            f32::powi(other[e1234], 2) + f32::powi(other[e423], 2) + f32::powi(other[e431], 2) + f32::powi(other[e412], 2)
                - f32::powi(other[e4], 2)
                - f32::powi(other[e41], 2)
                - f32::powi(other[e42], 2)
                - f32::powi(other[e43], 2),
        );
        let anti_inverse = AntiScalar::from_groups(/* e1234 */ 1.0 / anti_dot_product[e1234]);
        return Horizon::from_groups(/* e321 */ anti_inverse[e1234] * self[e321]);
    }
}
impl GeometricAntiQuotient<Origin> for Horizon {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        2        1
    fn geometric_anti_quotient(self, other: Origin) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(/* e1234 */ f32::powi(other[e4], 2) * -1.0);
        let anti_inverse = AntiScalar::from_groups(/* e1234 */ 1.0 / anti_dot_product[e1234]);
        return Horizon::from_groups(/* e321 */ anti_inverse[e1234] * self[e321]);
    }
}
impl GeometricAntiQuotient<Plane> for Horizon {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        1        1
    fn geometric_anti_quotient(self, other: Plane) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(/* e1234 */ f32::powi(other[e423], 2) + f32::powi(other[e431], 2) + f32::powi(other[e412], 2));
        let anti_inverse = AntiScalar::from_groups(/* e1234 */ 1.0 / anti_dot_product[e1234]);
        return Horizon::from_groups(/* e321 */ anti_inverse[e1234] * self[e321]);
    }
}
impl GeometricAntiQuotient<Point> for Horizon {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        2        1
    fn geometric_anti_quotient(self, other: Point) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(/* e1234 */ f32::powi(other[e4], 2) * -1.0);
        let anti_inverse = AntiScalar::from_groups(/* e1234 */ 1.0 / anti_dot_product[e1234]);
        return Horizon::from_groups(/* e321 */ anti_inverse[e1234] * self[e321]);
    }
}
impl std::ops::Div<geometric_anti_quotient> for Line {
    type Output = geometric_anti_quotient_partial<Line>;
    fn div(self, _rhs: geometric_anti_quotient) -> Self::Output {
        geometric_anti_quotient_partial(self)
    }
}
impl GeometricAntiQuotient<AntiScalar> for Line {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        0        1
    //    simd3        0        2        0
    // Totals...
    // yes simd        0        2        1
    //  no simd        0        6        1
    fn geometric_anti_quotient(self, other: AntiScalar) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(/* e1234 */ f32::powi(other[e1234], 2));
        let anti_inverse = AntiScalar::from_groups(/* e1234 */ 1.0 / anti_dot_product[e1234]);
        return Line::from_groups(
            // e41, e42, e43
            Simd32x3::from(anti_inverse[e1234]) * Simd32x3::from([self[e41], self[e42], self[e43]]),
            // e23, e31, e12
            Simd32x3::from(anti_inverse[e1234]) * Simd32x3::from([self[e23], self[e31], self[e12]]),
        );
    }
}
impl GeometricAntiQuotient<DualNum> for Line {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        0        1
    //    simd3        0        2        0
    // Totals...
    // yes simd        0        2        1
    //  no simd        0        6        1
    fn geometric_anti_quotient(self, other: DualNum) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(/* e1234 */ f32::powi(other[e1234], 2));
        let anti_inverse = AntiScalar::from_groups(/* e1234 */ 1.0 / anti_dot_product[e1234]);
        return Line::from_groups(
            // e41, e42, e43
            Simd32x3::from(anti_inverse[e1234]) * Simd32x3::from([self[e41], self[e42], self[e43]]),
            // e23, e31, e12
            Simd32x3::from(anti_inverse[e1234]) * Simd32x3::from([self[e23], self[e31], self[e12]]),
        );
    }
}
impl GeometricAntiQuotient<Flector> for Line {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        0        1
    //    simd3        0        2        0
    // Totals...
    // yes simd        3        2        1
    //  no simd        3        6        1
    fn geometric_anti_quotient(self, other: Flector) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(
            // e1234
            f32::powi(other[e423], 2) + f32::powi(other[e431], 2) + f32::powi(other[e412], 2) - f32::powi(other[e4], 2),
        );
        let anti_inverse = AntiScalar::from_groups(/* e1234 */ 1.0 / anti_dot_product[e1234]);
        return Line::from_groups(
            // e41, e42, e43
            Simd32x3::from(anti_inverse[e1234]) * Simd32x3::from([self[e41], self[e42], self[e43]]),
            // e23, e31, e12
            Simd32x3::from(anti_inverse[e1234]) * Simd32x3::from([self[e23], self[e31], self[e12]]),
        );
    }
}
impl GeometricAntiQuotient<Line> for Line {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        0        1
    //    simd3        0        2        0
    // Totals...
    // yes simd        2        2        1
    //  no simd        2        6        1
    fn geometric_anti_quotient(self, other: Line) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(/* e1234 */ -f32::powi(other[e41], 2) - f32::powi(other[e42], 2) - f32::powi(other[e43], 2));
        let anti_inverse = AntiScalar::from_groups(/* e1234 */ 1.0 / anti_dot_product[e1234]);
        return Line::from_groups(
            // e41, e42, e43
            Simd32x3::from(anti_inverse[e1234]) * Simd32x3::from([self[e41], self[e42], self[e43]]),
            // e23, e31, e12
            Simd32x3::from(anti_inverse[e1234]) * Simd32x3::from([self[e23], self[e31], self[e12]]),
        );
    }
}
impl GeometricAntiQuotient<Motor> for Line {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        0        1
    //    simd3        0        2        0
    // Totals...
    // yes simd        3        2        1
    //  no simd        3        6        1
    fn geometric_anti_quotient(self, other: Motor) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(
            // e1234
            f32::powi(other[e1234], 2) - f32::powi(other[e41], 2) - f32::powi(other[e42], 2) - f32::powi(other[e43], 2),
        );
        let anti_inverse = AntiScalar::from_groups(/* e1234 */ 1.0 / anti_dot_product[e1234]);
        return Line::from_groups(
            // e41, e42, e43
            Simd32x3::from(anti_inverse[e1234]) * Simd32x3::from([self[e41], self[e42], self[e43]]),
            // e23, e31, e12
            Simd32x3::from(anti_inverse[e1234]) * Simd32x3::from([self[e23], self[e31], self[e12]]),
        );
    }
}
impl GeometricAntiQuotient<MultiVector> for Line {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        7        0        1
    //    simd3        0        2        0
    // Totals...
    // yes simd        7        2        1
    //  no simd        7        6        1
    fn geometric_anti_quotient(self, other: MultiVector) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(
            // e1234
            f32::powi(other[e1234], 2) + f32::powi(other[e423], 2) + f32::powi(other[e431], 2) + f32::powi(other[e412], 2)
                - f32::powi(other[e4], 2)
                - f32::powi(other[e41], 2)
                - f32::powi(other[e42], 2)
                - f32::powi(other[e43], 2),
        );
        let anti_inverse = AntiScalar::from_groups(/* e1234 */ 1.0 / anti_dot_product[e1234]);
        return Line::from_groups(
            // e41, e42, e43
            Simd32x3::from(anti_inverse[e1234]) * Simd32x3::from([self[e41], self[e42], self[e43]]),
            // e23, e31, e12
            Simd32x3::from(anti_inverse[e1234]) * Simd32x3::from([self[e23], self[e31], self[e12]]),
        );
    }
}
impl GeometricAntiQuotient<Origin> for Line {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        1
    //    simd3        0        2        0
    // Totals...
    // yes simd        0        3        1
    //  no simd        0        7        1
    fn geometric_anti_quotient(self, other: Origin) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(/* e1234 */ f32::powi(other[e4], 2) * -1.0);
        let anti_inverse = AntiScalar::from_groups(/* e1234 */ 1.0 / anti_dot_product[e1234]);
        return Line::from_groups(
            // e41, e42, e43
            Simd32x3::from(anti_inverse[e1234]) * Simd32x3::from([self[e41], self[e42], self[e43]]),
            // e23, e31, e12
            Simd32x3::from(anti_inverse[e1234]) * Simd32x3::from([self[e23], self[e31], self[e12]]),
        );
    }
}
impl GeometricAntiQuotient<Plane> for Line {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        0        1
    //    simd3        0        2        0
    // Totals...
    // yes simd        2        2        1
    //  no simd        2        6        1
    fn geometric_anti_quotient(self, other: Plane) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(/* e1234 */ f32::powi(other[e423], 2) + f32::powi(other[e431], 2) + f32::powi(other[e412], 2));
        let anti_inverse = AntiScalar::from_groups(/* e1234 */ 1.0 / anti_dot_product[e1234]);
        return Line::from_groups(
            // e41, e42, e43
            Simd32x3::from(anti_inverse[e1234]) * Simd32x3::from([self[e41], self[e42], self[e43]]),
            // e23, e31, e12
            Simd32x3::from(anti_inverse[e1234]) * Simd32x3::from([self[e23], self[e31], self[e12]]),
        );
    }
}
impl GeometricAntiQuotient<Point> for Line {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        1
    //    simd3        0        2        0
    // Totals...
    // yes simd        0        3        1
    //  no simd        0        7        1
    fn geometric_anti_quotient(self, other: Point) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(/* e1234 */ f32::powi(other[e4], 2) * -1.0);
        let anti_inverse = AntiScalar::from_groups(/* e1234 */ 1.0 / anti_dot_product[e1234]);
        return Line::from_groups(
            // e41, e42, e43
            Simd32x3::from(anti_inverse[e1234]) * Simd32x3::from([self[e41], self[e42], self[e43]]),
            // e23, e31, e12
            Simd32x3::from(anti_inverse[e1234]) * Simd32x3::from([self[e23], self[e31], self[e12]]),
        );
    }
}
impl std::ops::Div<geometric_anti_quotient> for Motor {
    type Output = geometric_anti_quotient_partial<Motor>;
    fn div(self, _rhs: geometric_anti_quotient) -> Self::Output {
        geometric_anti_quotient_partial(self)
    }
}
impl GeometricAntiQuotient<AntiScalar> for Motor {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        0        1
    //    simd4        0        2        0
    // Totals...
    // yes simd        0        2        1
    //  no simd        0        8        1
    fn geometric_anti_quotient(self, other: AntiScalar) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(/* e1234 */ f32::powi(other[e1234], 2));
        let anti_inverse = AntiScalar::from_groups(/* e1234 */ 1.0 / anti_dot_product[e1234]);
        return Motor::from_groups(
            // e41, e42, e43, e1234
            Simd32x4::from(anti_inverse[e1234]) * Simd32x4::from([self[e41], self[e42], self[e43], self[e1234]]),
            // e23, e31, e12, scalar
            Simd32x4::from(anti_inverse[e1234]) * Simd32x4::from([self[e23], self[e31], self[e12], self[scalar]]),
        );
    }
}
impl GeometricAntiQuotient<DualNum> for Motor {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        0        1
    //    simd4        0        2        0
    // Totals...
    // yes simd        0        2        1
    //  no simd        0        8        1
    fn geometric_anti_quotient(self, other: DualNum) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(/* e1234 */ f32::powi(other[e1234], 2));
        let anti_inverse = AntiScalar::from_groups(/* e1234 */ 1.0 / anti_dot_product[e1234]);
        return Motor::from_groups(
            // e41, e42, e43, e1234
            Simd32x4::from(anti_inverse[e1234]) * Simd32x4::from([self[e41], self[e42], self[e43], self[e1234]]),
            // e23, e31, e12, scalar
            Simd32x4::from(anti_inverse[e1234]) * Simd32x4::from([self[e23], self[e31], self[e12], self[scalar]]),
        );
    }
}
impl GeometricAntiQuotient<Flector> for Motor {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        0        1
    //    simd4        0        2        0
    // Totals...
    // yes simd        3        2        1
    //  no simd        3        8        1
    fn geometric_anti_quotient(self, other: Flector) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(
            // e1234
            f32::powi(other[e423], 2) + f32::powi(other[e431], 2) + f32::powi(other[e412], 2) - f32::powi(other[e4], 2),
        );
        let anti_inverse = AntiScalar::from_groups(/* e1234 */ 1.0 / anti_dot_product[e1234]);
        return Motor::from_groups(
            // e41, e42, e43, e1234
            Simd32x4::from(anti_inverse[e1234]) * Simd32x4::from([self[e41], self[e42], self[e43], self[e1234]]),
            // e23, e31, e12, scalar
            Simd32x4::from(anti_inverse[e1234]) * Simd32x4::from([self[e23], self[e31], self[e12], self[scalar]]),
        );
    }
}
impl GeometricAntiQuotient<Line> for Motor {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        0        1
    //    simd4        0        2        0
    // Totals...
    // yes simd        2        2        1
    //  no simd        2        8        1
    fn geometric_anti_quotient(self, other: Line) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(/* e1234 */ -f32::powi(other[e41], 2) - f32::powi(other[e42], 2) - f32::powi(other[e43], 2));
        let anti_inverse = AntiScalar::from_groups(/* e1234 */ 1.0 / anti_dot_product[e1234]);
        return Motor::from_groups(
            // e41, e42, e43, e1234
            Simd32x4::from(anti_inverse[e1234]) * Simd32x4::from([self[e41], self[e42], self[e43], self[e1234]]),
            // e23, e31, e12, scalar
            Simd32x4::from(anti_inverse[e1234]) * Simd32x4::from([self[e23], self[e31], self[e12], self[scalar]]),
        );
    }
}
impl GeometricAntiQuotient<Motor> for Motor {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        0        1
    //    simd4        0        2        0
    // Totals...
    // yes simd        3        2        1
    //  no simd        3        8        1
    fn geometric_anti_quotient(self, other: Motor) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(
            // e1234
            f32::powi(other[e1234], 2) - f32::powi(other[e41], 2) - f32::powi(other[e42], 2) - f32::powi(other[e43], 2),
        );
        let anti_inverse = AntiScalar::from_groups(/* e1234 */ 1.0 / anti_dot_product[e1234]);
        return Motor::from_groups(
            // e41, e42, e43, e1234
            Simd32x4::from(anti_inverse[e1234]) * Simd32x4::from([self[e41], self[e42], self[e43], self[e1234]]),
            // e23, e31, e12, scalar
            Simd32x4::from(anti_inverse[e1234]) * Simd32x4::from([self[e23], self[e31], self[e12], self[scalar]]),
        );
    }
}
impl GeometricAntiQuotient<MultiVector> for Motor {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        7        0        1
    //    simd4        0        2        0
    // Totals...
    // yes simd        7        2        1
    //  no simd        7        8        1
    fn geometric_anti_quotient(self, other: MultiVector) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(
            // e1234
            f32::powi(other[e1234], 2) + f32::powi(other[e423], 2) + f32::powi(other[e431], 2) + f32::powi(other[e412], 2)
                - f32::powi(other[e4], 2)
                - f32::powi(other[e41], 2)
                - f32::powi(other[e42], 2)
                - f32::powi(other[e43], 2),
        );
        let anti_inverse = AntiScalar::from_groups(/* e1234 */ 1.0 / anti_dot_product[e1234]);
        return Motor::from_groups(
            // e41, e42, e43, e1234
            Simd32x4::from(anti_inverse[e1234]) * Simd32x4::from([self[e41], self[e42], self[e43], self[e1234]]),
            // e23, e31, e12, scalar
            Simd32x4::from(anti_inverse[e1234]) * Simd32x4::from([self[e23], self[e31], self[e12], self[scalar]]),
        );
    }
}
impl GeometricAntiQuotient<Origin> for Motor {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        1
    //    simd4        0        2        0
    // Totals...
    // yes simd        0        3        1
    //  no simd        0        9        1
    fn geometric_anti_quotient(self, other: Origin) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(/* e1234 */ f32::powi(other[e4], 2) * -1.0);
        let anti_inverse = AntiScalar::from_groups(/* e1234 */ 1.0 / anti_dot_product[e1234]);
        return Motor::from_groups(
            // e41, e42, e43, e1234
            Simd32x4::from(anti_inverse[e1234]) * Simd32x4::from([self[e41], self[e42], self[e43], self[e1234]]),
            // e23, e31, e12, scalar
            Simd32x4::from(anti_inverse[e1234]) * Simd32x4::from([self[e23], self[e31], self[e12], self[scalar]]),
        );
    }
}
impl GeometricAntiQuotient<Plane> for Motor {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        0        1
    //    simd4        0        2        0
    // Totals...
    // yes simd        2        2        1
    //  no simd        2        8        1
    fn geometric_anti_quotient(self, other: Plane) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(/* e1234 */ f32::powi(other[e423], 2) + f32::powi(other[e431], 2) + f32::powi(other[e412], 2));
        let anti_inverse = AntiScalar::from_groups(/* e1234 */ 1.0 / anti_dot_product[e1234]);
        return Motor::from_groups(
            // e41, e42, e43, e1234
            Simd32x4::from(anti_inverse[e1234]) * Simd32x4::from([self[e41], self[e42], self[e43], self[e1234]]),
            // e23, e31, e12, scalar
            Simd32x4::from(anti_inverse[e1234]) * Simd32x4::from([self[e23], self[e31], self[e12], self[scalar]]),
        );
    }
}
impl GeometricAntiQuotient<Point> for Motor {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        1
    //    simd4        0        2        0
    // Totals...
    // yes simd        0        3        1
    //  no simd        0        9        1
    fn geometric_anti_quotient(self, other: Point) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(/* e1234 */ f32::powi(other[e4], 2) * -1.0);
        let anti_inverse = AntiScalar::from_groups(/* e1234 */ 1.0 / anti_dot_product[e1234]);
        return Motor::from_groups(
            // e41, e42, e43, e1234
            Simd32x4::from(anti_inverse[e1234]) * Simd32x4::from([self[e41], self[e42], self[e43], self[e1234]]),
            // e23, e31, e12, scalar
            Simd32x4::from(anti_inverse[e1234]) * Simd32x4::from([self[e23], self[e31], self[e12], self[scalar]]),
        );
    }
}
impl std::ops::Div<geometric_anti_quotient> for MultiVector {
    type Output = geometric_anti_quotient_partial<MultiVector>;
    fn div(self, _rhs: geometric_anti_quotient) -> Self::Output {
        geometric_anti_quotient_partial(self)
    }
}
impl GeometricAntiQuotient<AntiScalar> for MultiVector {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        0        1
    //    simd2        0        1        0
    //    simd3        0        2        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        0        5        1
    //  no simd        0       16        1
    fn geometric_anti_quotient(self, other: AntiScalar) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(/* e1234 */ f32::powi(other[e1234], 2));
        let anti_inverse = AntiScalar::from_groups(/* e1234 */ 1.0 / anti_dot_product[e1234]);
        return MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from(anti_inverse[e1234]) * Simd32x2::from([self[scalar], self[e1234]]),
            // e1, e2, e3, e4
            Simd32x4::from(anti_inverse[e1234]) * Simd32x4::from([self[e1], self[e2], self[e3], self[e4]]),
            // e41, e42, e43
            Simd32x3::from(anti_inverse[e1234]) * Simd32x3::from([self[e41], self[e42], self[e43]]),
            // e23, e31, e12
            Simd32x3::from(anti_inverse[e1234]) * Simd32x3::from([self[e23], self[e31], self[e12]]),
            // e423, e431, e412, e321
            Simd32x4::from(anti_inverse[e1234]) * Simd32x4::from([self[e423], self[e431], self[e412], self[e321]]),
        );
    }
}
impl GeometricAntiQuotient<DualNum> for MultiVector {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        0        1
    //    simd2        0        1        0
    //    simd3        0        2        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        0        5        1
    //  no simd        0       16        1
    fn geometric_anti_quotient(self, other: DualNum) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(/* e1234 */ f32::powi(other[e1234], 2));
        let anti_inverse = AntiScalar::from_groups(/* e1234 */ 1.0 / anti_dot_product[e1234]);
        return MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from(anti_inverse[e1234]) * Simd32x2::from([self[scalar], self[e1234]]),
            // e1, e2, e3, e4
            Simd32x4::from(anti_inverse[e1234]) * Simd32x4::from([self[e1], self[e2], self[e3], self[e4]]),
            // e41, e42, e43
            Simd32x3::from(anti_inverse[e1234]) * Simd32x3::from([self[e41], self[e42], self[e43]]),
            // e23, e31, e12
            Simd32x3::from(anti_inverse[e1234]) * Simd32x3::from([self[e23], self[e31], self[e12]]),
            // e423, e431, e412, e321
            Simd32x4::from(anti_inverse[e1234]) * Simd32x4::from([self[e423], self[e431], self[e412], self[e321]]),
        );
    }
}
impl GeometricAntiQuotient<Flector> for MultiVector {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        0        1
    //    simd2        0        1        0
    //    simd3        0        2        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        3        5        1
    //  no simd        3       16        1
    fn geometric_anti_quotient(self, other: Flector) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(
            // e1234
            f32::powi(other[e423], 2) + f32::powi(other[e431], 2) + f32::powi(other[e412], 2) - f32::powi(other[e4], 2),
        );
        let anti_inverse = AntiScalar::from_groups(/* e1234 */ 1.0 / anti_dot_product[e1234]);
        return MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from(anti_inverse[e1234]) * Simd32x2::from([self[scalar], self[e1234]]),
            // e1, e2, e3, e4
            Simd32x4::from(anti_inverse[e1234]) * Simd32x4::from([self[e1], self[e2], self[e3], self[e4]]),
            // e41, e42, e43
            Simd32x3::from(anti_inverse[e1234]) * Simd32x3::from([self[e41], self[e42], self[e43]]),
            // e23, e31, e12
            Simd32x3::from(anti_inverse[e1234]) * Simd32x3::from([self[e23], self[e31], self[e12]]),
            // e423, e431, e412, e321
            Simd32x4::from(anti_inverse[e1234]) * Simd32x4::from([self[e423], self[e431], self[e412], self[e321]]),
        );
    }
}
impl GeometricAntiQuotient<Line> for MultiVector {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        0        1
    //    simd2        0        1        0
    //    simd3        0        2        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        2        5        1
    //  no simd        2       16        1
    fn geometric_anti_quotient(self, other: Line) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(/* e1234 */ -f32::powi(other[e41], 2) - f32::powi(other[e42], 2) - f32::powi(other[e43], 2));
        let anti_inverse = AntiScalar::from_groups(/* e1234 */ 1.0 / anti_dot_product[e1234]);
        return MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from(anti_inverse[e1234]) * Simd32x2::from([self[scalar], self[e1234]]),
            // e1, e2, e3, e4
            Simd32x4::from(anti_inverse[e1234]) * Simd32x4::from([self[e1], self[e2], self[e3], self[e4]]),
            // e41, e42, e43
            Simd32x3::from(anti_inverse[e1234]) * Simd32x3::from([self[e41], self[e42], self[e43]]),
            // e23, e31, e12
            Simd32x3::from(anti_inverse[e1234]) * Simd32x3::from([self[e23], self[e31], self[e12]]),
            // e423, e431, e412, e321
            Simd32x4::from(anti_inverse[e1234]) * Simd32x4::from([self[e423], self[e431], self[e412], self[e321]]),
        );
    }
}
impl GeometricAntiQuotient<Motor> for MultiVector {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        0        1
    //    simd2        0        1        0
    //    simd3        0        2        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        3        5        1
    //  no simd        3       16        1
    fn geometric_anti_quotient(self, other: Motor) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(
            // e1234
            f32::powi(other[e1234], 2) - f32::powi(other[e41], 2) - f32::powi(other[e42], 2) - f32::powi(other[e43], 2),
        );
        let anti_inverse = AntiScalar::from_groups(/* e1234 */ 1.0 / anti_dot_product[e1234]);
        return MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from(anti_inverse[e1234]) * Simd32x2::from([self[scalar], self[e1234]]),
            // e1, e2, e3, e4
            Simd32x4::from(anti_inverse[e1234]) * Simd32x4::from([self[e1], self[e2], self[e3], self[e4]]),
            // e41, e42, e43
            Simd32x3::from(anti_inverse[e1234]) * Simd32x3::from([self[e41], self[e42], self[e43]]),
            // e23, e31, e12
            Simd32x3::from(anti_inverse[e1234]) * Simd32x3::from([self[e23], self[e31], self[e12]]),
            // e423, e431, e412, e321
            Simd32x4::from(anti_inverse[e1234]) * Simd32x4::from([self[e423], self[e431], self[e412], self[e321]]),
        );
    }
}
impl GeometricAntiQuotient<MultiVector> for MultiVector {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        7        0        1
    //    simd2        0        1        0
    //    simd3        0        2        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        7        5        1
    //  no simd        7       16        1
    fn geometric_anti_quotient(self, other: MultiVector) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(
            // e1234
            f32::powi(other[e1234], 2) + f32::powi(other[e423], 2) + f32::powi(other[e431], 2) + f32::powi(other[e412], 2)
                - f32::powi(other[e4], 2)
                - f32::powi(other[e41], 2)
                - f32::powi(other[e42], 2)
                - f32::powi(other[e43], 2),
        );
        let anti_inverse = AntiScalar::from_groups(/* e1234 */ 1.0 / anti_dot_product[e1234]);
        return MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from(anti_inverse[e1234]) * Simd32x2::from([self[scalar], self[e1234]]),
            // e1, e2, e3, e4
            Simd32x4::from(anti_inverse[e1234]) * Simd32x4::from([self[e1], self[e2], self[e3], self[e4]]),
            // e41, e42, e43
            Simd32x3::from(anti_inverse[e1234]) * Simd32x3::from([self[e41], self[e42], self[e43]]),
            // e23, e31, e12
            Simd32x3::from(anti_inverse[e1234]) * Simd32x3::from([self[e23], self[e31], self[e12]]),
            // e423, e431, e412, e321
            Simd32x4::from(anti_inverse[e1234]) * Simd32x4::from([self[e423], self[e431], self[e412], self[e321]]),
        );
    }
}
impl GeometricAntiQuotient<Origin> for MultiVector {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        1
    //    simd2        0        1        0
    //    simd3        0        2        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        0        6        1
    //  no simd        0       17        1
    fn geometric_anti_quotient(self, other: Origin) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(/* e1234 */ f32::powi(other[e4], 2) * -1.0);
        let anti_inverse = AntiScalar::from_groups(/* e1234 */ 1.0 / anti_dot_product[e1234]);
        return MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from(anti_inverse[e1234]) * Simd32x2::from([self[scalar], self[e1234]]),
            // e1, e2, e3, e4
            Simd32x4::from(anti_inverse[e1234]) * Simd32x4::from([self[e1], self[e2], self[e3], self[e4]]),
            // e41, e42, e43
            Simd32x3::from(anti_inverse[e1234]) * Simd32x3::from([self[e41], self[e42], self[e43]]),
            // e23, e31, e12
            Simd32x3::from(anti_inverse[e1234]) * Simd32x3::from([self[e23], self[e31], self[e12]]),
            // e423, e431, e412, e321
            Simd32x4::from(anti_inverse[e1234]) * Simd32x4::from([self[e423], self[e431], self[e412], self[e321]]),
        );
    }
}
impl GeometricAntiQuotient<Plane> for MultiVector {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        0        1
    //    simd2        0        1        0
    //    simd3        0        2        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        2        5        1
    //  no simd        2       16        1
    fn geometric_anti_quotient(self, other: Plane) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(/* e1234 */ f32::powi(other[e423], 2) + f32::powi(other[e431], 2) + f32::powi(other[e412], 2));
        let anti_inverse = AntiScalar::from_groups(/* e1234 */ 1.0 / anti_dot_product[e1234]);
        return MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from(anti_inverse[e1234]) * Simd32x2::from([self[scalar], self[e1234]]),
            // e1, e2, e3, e4
            Simd32x4::from(anti_inverse[e1234]) * Simd32x4::from([self[e1], self[e2], self[e3], self[e4]]),
            // e41, e42, e43
            Simd32x3::from(anti_inverse[e1234]) * Simd32x3::from([self[e41], self[e42], self[e43]]),
            // e23, e31, e12
            Simd32x3::from(anti_inverse[e1234]) * Simd32x3::from([self[e23], self[e31], self[e12]]),
            // e423, e431, e412, e321
            Simd32x4::from(anti_inverse[e1234]) * Simd32x4::from([self[e423], self[e431], self[e412], self[e321]]),
        );
    }
}
impl GeometricAntiQuotient<Point> for MultiVector {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        1
    //    simd2        0        1        0
    //    simd3        0        2        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        0        6        1
    //  no simd        0       17        1
    fn geometric_anti_quotient(self, other: Point) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(/* e1234 */ f32::powi(other[e4], 2) * -1.0);
        let anti_inverse = AntiScalar::from_groups(/* e1234 */ 1.0 / anti_dot_product[e1234]);
        return MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from(anti_inverse[e1234]) * Simd32x2::from([self[scalar], self[e1234]]),
            // e1, e2, e3, e4
            Simd32x4::from(anti_inverse[e1234]) * Simd32x4::from([self[e1], self[e2], self[e3], self[e4]]),
            // e41, e42, e43
            Simd32x3::from(anti_inverse[e1234]) * Simd32x3::from([self[e41], self[e42], self[e43]]),
            // e23, e31, e12
            Simd32x3::from(anti_inverse[e1234]) * Simd32x3::from([self[e23], self[e31], self[e12]]),
            // e423, e431, e412, e321
            Simd32x4::from(anti_inverse[e1234]) * Simd32x4::from([self[e423], self[e431], self[e412], self[e321]]),
        );
    }
}
impl std::ops::Div<geometric_anti_quotient> for Origin {
    type Output = geometric_anti_quotient_partial<Origin>;
    fn div(self, _rhs: geometric_anti_quotient) -> Self::Output {
        geometric_anti_quotient_partial(self)
    }
}
impl GeometricAntiQuotient<AntiScalar> for Origin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        1
    fn geometric_anti_quotient(self, other: AntiScalar) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(/* e1234 */ f32::powi(other[e1234], 2));
        let anti_inverse = AntiScalar::from_groups(/* e1234 */ 1.0 / anti_dot_product[e1234]);
        return Origin::from_groups(/* e4 */ anti_inverse[e1234] * self[e4]);
    }
}
impl GeometricAntiQuotient<DualNum> for Origin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        1
    fn geometric_anti_quotient(self, other: DualNum) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(/* e1234 */ f32::powi(other[e1234], 2));
        let anti_inverse = AntiScalar::from_groups(/* e1234 */ 1.0 / anti_dot_product[e1234]);
        return Origin::from_groups(/* e4 */ anti_inverse[e1234] * self[e4]);
    }
}
impl GeometricAntiQuotient<Flector> for Origin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        1        1
    fn geometric_anti_quotient(self, other: Flector) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(
            // e1234
            f32::powi(other[e423], 2) + f32::powi(other[e431], 2) + f32::powi(other[e412], 2) - f32::powi(other[e4], 2),
        );
        let anti_inverse = AntiScalar::from_groups(/* e1234 */ 1.0 / anti_dot_product[e1234]);
        return Origin::from_groups(/* e4 */ anti_inverse[e1234] * self[e4]);
    }
}
impl GeometricAntiQuotient<Line> for Origin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        1        1
    fn geometric_anti_quotient(self, other: Line) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(/* e1234 */ -f32::powi(other[e41], 2) - f32::powi(other[e42], 2) - f32::powi(other[e43], 2));
        let anti_inverse = AntiScalar::from_groups(/* e1234 */ 1.0 / anti_dot_product[e1234]);
        return Origin::from_groups(/* e4 */ anti_inverse[e1234] * self[e4]);
    }
}
impl GeometricAntiQuotient<Motor> for Origin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        1        1
    fn geometric_anti_quotient(self, other: Motor) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(
            // e1234
            f32::powi(other[e1234], 2) - f32::powi(other[e41], 2) - f32::powi(other[e42], 2) - f32::powi(other[e43], 2),
        );
        let anti_inverse = AntiScalar::from_groups(/* e1234 */ 1.0 / anti_dot_product[e1234]);
        return Origin::from_groups(/* e4 */ anti_inverse[e1234] * self[e4]);
    }
}
impl GeometricAntiQuotient<MultiVector> for Origin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        7        1        1
    fn geometric_anti_quotient(self, other: MultiVector) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(
            // e1234
            f32::powi(other[e1234], 2) + f32::powi(other[e423], 2) + f32::powi(other[e431], 2) + f32::powi(other[e412], 2)
                - f32::powi(other[e4], 2)
                - f32::powi(other[e41], 2)
                - f32::powi(other[e42], 2)
                - f32::powi(other[e43], 2),
        );
        let anti_inverse = AntiScalar::from_groups(/* e1234 */ 1.0 / anti_dot_product[e1234]);
        return Origin::from_groups(/* e4 */ anti_inverse[e1234] * self[e4]);
    }
}
impl GeometricAntiQuotient<Origin> for Origin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        2        1
    fn geometric_anti_quotient(self, other: Origin) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(/* e1234 */ f32::powi(other[e4], 2) * -1.0);
        let anti_inverse = AntiScalar::from_groups(/* e1234 */ 1.0 / anti_dot_product[e1234]);
        return Origin::from_groups(/* e4 */ anti_inverse[e1234] * self[e4]);
    }
}
impl GeometricAntiQuotient<Plane> for Origin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        1        1
    fn geometric_anti_quotient(self, other: Plane) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(/* e1234 */ f32::powi(other[e423], 2) + f32::powi(other[e431], 2) + f32::powi(other[e412], 2));
        let anti_inverse = AntiScalar::from_groups(/* e1234 */ 1.0 / anti_dot_product[e1234]);
        return Origin::from_groups(/* e4 */ anti_inverse[e1234] * self[e4]);
    }
}
impl GeometricAntiQuotient<Point> for Origin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        2        1
    fn geometric_anti_quotient(self, other: Point) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(/* e1234 */ f32::powi(other[e4], 2) * -1.0);
        let anti_inverse = AntiScalar::from_groups(/* e1234 */ 1.0 / anti_dot_product[e1234]);
        return Origin::from_groups(/* e4 */ anti_inverse[e1234] * self[e4]);
    }
}
impl std::ops::Div<geometric_anti_quotient> for Plane {
    type Output = geometric_anti_quotient_partial<Plane>;
    fn div(self, _rhs: geometric_anti_quotient) -> Self::Output {
        geometric_anti_quotient_partial(self)
    }
}
impl GeometricAntiQuotient<AntiScalar> for Plane {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        0        1
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        1        1
    //  no simd        0        4        1
    fn geometric_anti_quotient(self, other: AntiScalar) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(/* e1234 */ f32::powi(other[e1234], 2));
        let anti_inverse = AntiScalar::from_groups(/* e1234 */ 1.0 / anti_dot_product[e1234]);
        return Plane::from_groups(
            // e423, e431, e412, e321
            Simd32x4::from(anti_inverse[e1234]) * Simd32x4::from([self[e423], self[e431], self[e412], self[e321]]),
        );
    }
}
impl GeometricAntiQuotient<DualNum> for Plane {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        0        1
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        1        1
    //  no simd        0        4        1
    fn geometric_anti_quotient(self, other: DualNum) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(/* e1234 */ f32::powi(other[e1234], 2));
        let anti_inverse = AntiScalar::from_groups(/* e1234 */ 1.0 / anti_dot_product[e1234]);
        return Plane::from_groups(
            // e423, e431, e412, e321
            Simd32x4::from(anti_inverse[e1234]) * Simd32x4::from([self[e423], self[e431], self[e412], self[e321]]),
        );
    }
}
impl GeometricAntiQuotient<Flector> for Plane {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        0        1
    //    simd4        0        1        0
    // Totals...
    // yes simd        3        1        1
    //  no simd        3        4        1
    fn geometric_anti_quotient(self, other: Flector) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(
            // e1234
            f32::powi(other[e423], 2) + f32::powi(other[e431], 2) + f32::powi(other[e412], 2) - f32::powi(other[e4], 2),
        );
        let anti_inverse = AntiScalar::from_groups(/* e1234 */ 1.0 / anti_dot_product[e1234]);
        return Plane::from_groups(
            // e423, e431, e412, e321
            Simd32x4::from(anti_inverse[e1234]) * Simd32x4::from([self[e423], self[e431], self[e412], self[e321]]),
        );
    }
}
impl GeometricAntiQuotient<Line> for Plane {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        0        1
    //    simd4        0        1        0
    // Totals...
    // yes simd        2        1        1
    //  no simd        2        4        1
    fn geometric_anti_quotient(self, other: Line) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(/* e1234 */ -f32::powi(other[e41], 2) - f32::powi(other[e42], 2) - f32::powi(other[e43], 2));
        let anti_inverse = AntiScalar::from_groups(/* e1234 */ 1.0 / anti_dot_product[e1234]);
        return Plane::from_groups(
            // e423, e431, e412, e321
            Simd32x4::from(anti_inverse[e1234]) * Simd32x4::from([self[e423], self[e431], self[e412], self[e321]]),
        );
    }
}
impl GeometricAntiQuotient<Motor> for Plane {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        0        1
    //    simd4        0        1        0
    // Totals...
    // yes simd        3        1        1
    //  no simd        3        4        1
    fn geometric_anti_quotient(self, other: Motor) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(
            // e1234
            f32::powi(other[e1234], 2) - f32::powi(other[e41], 2) - f32::powi(other[e42], 2) - f32::powi(other[e43], 2),
        );
        let anti_inverse = AntiScalar::from_groups(/* e1234 */ 1.0 / anti_dot_product[e1234]);
        return Plane::from_groups(
            // e423, e431, e412, e321
            Simd32x4::from(anti_inverse[e1234]) * Simd32x4::from([self[e423], self[e431], self[e412], self[e321]]),
        );
    }
}
impl GeometricAntiQuotient<MultiVector> for Plane {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        7        0        1
    //    simd4        0        1        0
    // Totals...
    // yes simd        7        1        1
    //  no simd        7        4        1
    fn geometric_anti_quotient(self, other: MultiVector) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(
            // e1234
            f32::powi(other[e1234], 2) + f32::powi(other[e423], 2) + f32::powi(other[e431], 2) + f32::powi(other[e412], 2)
                - f32::powi(other[e4], 2)
                - f32::powi(other[e41], 2)
                - f32::powi(other[e42], 2)
                - f32::powi(other[e43], 2),
        );
        let anti_inverse = AntiScalar::from_groups(/* e1234 */ 1.0 / anti_dot_product[e1234]);
        return Plane::from_groups(
            // e423, e431, e412, e321
            Simd32x4::from(anti_inverse[e1234]) * Simd32x4::from([self[e423], self[e431], self[e412], self[e321]]),
        );
    }
}
impl GeometricAntiQuotient<Origin> for Plane {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        1
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        2        1
    //  no simd        0        5        1
    fn geometric_anti_quotient(self, other: Origin) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(/* e1234 */ f32::powi(other[e4], 2) * -1.0);
        let anti_inverse = AntiScalar::from_groups(/* e1234 */ 1.0 / anti_dot_product[e1234]);
        return Plane::from_groups(
            // e423, e431, e412, e321
            Simd32x4::from(anti_inverse[e1234]) * Simd32x4::from([self[e423], self[e431], self[e412], self[e321]]),
        );
    }
}
impl GeometricAntiQuotient<Plane> for Plane {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        0        1
    //    simd4        0        1        0
    // Totals...
    // yes simd        2        1        1
    //  no simd        2        4        1
    fn geometric_anti_quotient(self, other: Plane) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(/* e1234 */ f32::powi(other[e423], 2) + f32::powi(other[e431], 2) + f32::powi(other[e412], 2));
        let anti_inverse = AntiScalar::from_groups(/* e1234 */ 1.0 / anti_dot_product[e1234]);
        return Plane::from_groups(
            // e423, e431, e412, e321
            Simd32x4::from(anti_inverse[e1234]) * Simd32x4::from([self[e423], self[e431], self[e412], self[e321]]),
        );
    }
}
impl GeometricAntiQuotient<Point> for Plane {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        1
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        2        1
    //  no simd        0        5        1
    fn geometric_anti_quotient(self, other: Point) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(/* e1234 */ f32::powi(other[e4], 2) * -1.0);
        let anti_inverse = AntiScalar::from_groups(/* e1234 */ 1.0 / anti_dot_product[e1234]);
        return Plane::from_groups(
            // e423, e431, e412, e321
            Simd32x4::from(anti_inverse[e1234]) * Simd32x4::from([self[e423], self[e431], self[e412], self[e321]]),
        );
    }
}
impl std::ops::Div<geometric_anti_quotient> for Point {
    type Output = geometric_anti_quotient_partial<Point>;
    fn div(self, _rhs: geometric_anti_quotient) -> Self::Output {
        geometric_anti_quotient_partial(self)
    }
}
impl GeometricAntiQuotient<AntiScalar> for Point {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        0        1
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        1        1
    //  no simd        0        4        1
    fn geometric_anti_quotient(self, other: AntiScalar) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(/* e1234 */ f32::powi(other[e1234], 2));
        let anti_inverse = AntiScalar::from_groups(/* e1234 */ 1.0 / anti_dot_product[e1234]);
        return Point::from_groups(/* e1, e2, e3, e4 */ Simd32x4::from(anti_inverse[e1234]) * Simd32x4::from([self[e1], self[e2], self[e3], self[e4]]));
    }
}
impl GeometricAntiQuotient<DualNum> for Point {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        0        1
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        1        1
    //  no simd        0        4        1
    fn geometric_anti_quotient(self, other: DualNum) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(/* e1234 */ f32::powi(other[e1234], 2));
        let anti_inverse = AntiScalar::from_groups(/* e1234 */ 1.0 / anti_dot_product[e1234]);
        return Point::from_groups(/* e1, e2, e3, e4 */ Simd32x4::from(anti_inverse[e1234]) * Simd32x4::from([self[e1], self[e2], self[e3], self[e4]]));
    }
}
impl GeometricAntiQuotient<Flector> for Point {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        0        1
    //    simd4        0        1        0
    // Totals...
    // yes simd        3        1        1
    //  no simd        3        4        1
    fn geometric_anti_quotient(self, other: Flector) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(
            // e1234
            f32::powi(other[e423], 2) + f32::powi(other[e431], 2) + f32::powi(other[e412], 2) - f32::powi(other[e4], 2),
        );
        let anti_inverse = AntiScalar::from_groups(/* e1234 */ 1.0 / anti_dot_product[e1234]);
        return Point::from_groups(/* e1, e2, e3, e4 */ Simd32x4::from(anti_inverse[e1234]) * Simd32x4::from([self[e1], self[e2], self[e3], self[e4]]));
    }
}
impl GeometricAntiQuotient<Line> for Point {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        0        1
    //    simd4        0        1        0
    // Totals...
    // yes simd        2        1        1
    //  no simd        2        4        1
    fn geometric_anti_quotient(self, other: Line) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(/* e1234 */ -f32::powi(other[e41], 2) - f32::powi(other[e42], 2) - f32::powi(other[e43], 2));
        let anti_inverse = AntiScalar::from_groups(/* e1234 */ 1.0 / anti_dot_product[e1234]);
        return Point::from_groups(/* e1, e2, e3, e4 */ Simd32x4::from(anti_inverse[e1234]) * Simd32x4::from([self[e1], self[e2], self[e3], self[e4]]));
    }
}
impl GeometricAntiQuotient<Motor> for Point {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        0        1
    //    simd4        0        1        0
    // Totals...
    // yes simd        3        1        1
    //  no simd        3        4        1
    fn geometric_anti_quotient(self, other: Motor) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(
            // e1234
            f32::powi(other[e1234], 2) - f32::powi(other[e41], 2) - f32::powi(other[e42], 2) - f32::powi(other[e43], 2),
        );
        let anti_inverse = AntiScalar::from_groups(/* e1234 */ 1.0 / anti_dot_product[e1234]);
        return Point::from_groups(/* e1, e2, e3, e4 */ Simd32x4::from(anti_inverse[e1234]) * Simd32x4::from([self[e1], self[e2], self[e3], self[e4]]));
    }
}
impl GeometricAntiQuotient<MultiVector> for Point {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        7        0        1
    //    simd4        0        1        0
    // Totals...
    // yes simd        7        1        1
    //  no simd        7        4        1
    fn geometric_anti_quotient(self, other: MultiVector) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(
            // e1234
            f32::powi(other[e1234], 2) + f32::powi(other[e423], 2) + f32::powi(other[e431], 2) + f32::powi(other[e412], 2)
                - f32::powi(other[e4], 2)
                - f32::powi(other[e41], 2)
                - f32::powi(other[e42], 2)
                - f32::powi(other[e43], 2),
        );
        let anti_inverse = AntiScalar::from_groups(/* e1234 */ 1.0 / anti_dot_product[e1234]);
        return Point::from_groups(/* e1, e2, e3, e4 */ Simd32x4::from(anti_inverse[e1234]) * Simd32x4::from([self[e1], self[e2], self[e3], self[e4]]));
    }
}
impl GeometricAntiQuotient<Origin> for Point {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        1
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        2        1
    //  no simd        0        5        1
    fn geometric_anti_quotient(self, other: Origin) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(/* e1234 */ f32::powi(other[e4], 2) * -1.0);
        let anti_inverse = AntiScalar::from_groups(/* e1234 */ 1.0 / anti_dot_product[e1234]);
        return Point::from_groups(/* e1, e2, e3, e4 */ Simd32x4::from(anti_inverse[e1234]) * Simd32x4::from([self[e1], self[e2], self[e3], self[e4]]));
    }
}
impl GeometricAntiQuotient<Plane> for Point {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        0        1
    //    simd4        0        1        0
    // Totals...
    // yes simd        2        1        1
    //  no simd        2        4        1
    fn geometric_anti_quotient(self, other: Plane) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(/* e1234 */ f32::powi(other[e423], 2) + f32::powi(other[e431], 2) + f32::powi(other[e412], 2));
        let anti_inverse = AntiScalar::from_groups(/* e1234 */ 1.0 / anti_dot_product[e1234]);
        return Point::from_groups(/* e1, e2, e3, e4 */ Simd32x4::from(anti_inverse[e1234]) * Simd32x4::from([self[e1], self[e2], self[e3], self[e4]]));
    }
}
impl GeometricAntiQuotient<Point> for Point {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        1
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        2        1
    //  no simd        0        5        1
    fn geometric_anti_quotient(self, other: Point) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(/* e1234 */ f32::powi(other[e4], 2) * -1.0);
        let anti_inverse = AntiScalar::from_groups(/* e1234 */ 1.0 / anti_dot_product[e1234]);
        return Point::from_groups(/* e1, e2, e3, e4 */ Simd32x4::from(anti_inverse[e1234]) * Simd32x4::from([self[e1], self[e2], self[e3], self[e4]]));
    }
}
impl std::ops::Div<geometric_anti_quotient> for Scalar {
    type Output = geometric_anti_quotient_partial<Scalar>;
    fn div(self, _rhs: geometric_anti_quotient) -> Self::Output {
        geometric_anti_quotient_partial(self)
    }
}
impl GeometricAntiQuotient<AntiScalar> for Scalar {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        1
    fn geometric_anti_quotient(self, other: AntiScalar) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(/* e1234 */ f32::powi(other[e1234], 2));
        let anti_inverse = AntiScalar::from_groups(/* e1234 */ 1.0 / anti_dot_product[e1234]);
        return Scalar::from_groups(/* scalar */ anti_inverse[e1234] * self[scalar]);
    }
}
impl GeometricAntiQuotient<DualNum> for Scalar {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        1
    fn geometric_anti_quotient(self, other: DualNum) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(/* e1234 */ f32::powi(other[e1234], 2));
        let anti_inverse = AntiScalar::from_groups(/* e1234 */ 1.0 / anti_dot_product[e1234]);
        return Scalar::from_groups(/* scalar */ anti_inverse[e1234] * self[scalar]);
    }
}
impl GeometricAntiQuotient<Flector> for Scalar {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        1        1
    fn geometric_anti_quotient(self, other: Flector) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(
            // e1234
            f32::powi(other[e423], 2) + f32::powi(other[e431], 2) + f32::powi(other[e412], 2) - f32::powi(other[e4], 2),
        );
        let anti_inverse = AntiScalar::from_groups(/* e1234 */ 1.0 / anti_dot_product[e1234]);
        return Scalar::from_groups(/* scalar */ anti_inverse[e1234] * self[scalar]);
    }
}
impl GeometricAntiQuotient<Line> for Scalar {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        1        1
    fn geometric_anti_quotient(self, other: Line) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(/* e1234 */ -f32::powi(other[e41], 2) - f32::powi(other[e42], 2) - f32::powi(other[e43], 2));
        let anti_inverse = AntiScalar::from_groups(/* e1234 */ 1.0 / anti_dot_product[e1234]);
        return Scalar::from_groups(/* scalar */ anti_inverse[e1234] * self[scalar]);
    }
}
impl GeometricAntiQuotient<Motor> for Scalar {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        1        1
    fn geometric_anti_quotient(self, other: Motor) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(
            // e1234
            f32::powi(other[e1234], 2) - f32::powi(other[e41], 2) - f32::powi(other[e42], 2) - f32::powi(other[e43], 2),
        );
        let anti_inverse = AntiScalar::from_groups(/* e1234 */ 1.0 / anti_dot_product[e1234]);
        return Scalar::from_groups(/* scalar */ anti_inverse[e1234] * self[scalar]);
    }
}
impl GeometricAntiQuotient<MultiVector> for Scalar {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        7        1        1
    fn geometric_anti_quotient(self, other: MultiVector) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(
            // e1234
            f32::powi(other[e1234], 2) + f32::powi(other[e423], 2) + f32::powi(other[e431], 2) + f32::powi(other[e412], 2)
                - f32::powi(other[e4], 2)
                - f32::powi(other[e41], 2)
                - f32::powi(other[e42], 2)
                - f32::powi(other[e43], 2),
        );
        let anti_inverse = AntiScalar::from_groups(/* e1234 */ 1.0 / anti_dot_product[e1234]);
        return Scalar::from_groups(/* scalar */ anti_inverse[e1234] * self[scalar]);
    }
}
impl GeometricAntiQuotient<Origin> for Scalar {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        2        1
    fn geometric_anti_quotient(self, other: Origin) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(/* e1234 */ f32::powi(other[e4], 2) * -1.0);
        let anti_inverse = AntiScalar::from_groups(/* e1234 */ 1.0 / anti_dot_product[e1234]);
        return Scalar::from_groups(/* scalar */ anti_inverse[e1234] * self[scalar]);
    }
}
impl GeometricAntiQuotient<Plane> for Scalar {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        1        1
    fn geometric_anti_quotient(self, other: Plane) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(/* e1234 */ f32::powi(other[e423], 2) + f32::powi(other[e431], 2) + f32::powi(other[e412], 2));
        let anti_inverse = AntiScalar::from_groups(/* e1234 */ 1.0 / anti_dot_product[e1234]);
        return Scalar::from_groups(/* scalar */ anti_inverse[e1234] * self[scalar]);
    }
}
impl GeometricAntiQuotient<Point> for Scalar {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        2        1
    fn geometric_anti_quotient(self, other: Point) -> Self {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(/* e1234 */ f32::powi(other[e4], 2) * -1.0);
        let anti_inverse = AntiScalar::from_groups(/* e1234 */ 1.0 / anti_dot_product[e1234]);
        return Scalar::from_groups(/* scalar */ anti_inverse[e1234] * self[scalar]);
    }
}
