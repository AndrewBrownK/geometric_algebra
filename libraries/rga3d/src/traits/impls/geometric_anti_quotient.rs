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
//  Minimum:         0       0       0
//   Median:         3       6       0
//  Average:         8      12       0
//  Maximum:        88      97       1
//
//  No SIMD:   add/sub     mul     div
//  Minimum:         0       0       0
//   Median:         3      17       0
//  Average:        14      27       0
//  Maximum:       188     208       1
impl std::ops::Div<geometric_anti_quotient> for AntiScalar {
    type Output = geometric_anti_quotient_partial<AntiScalar>;
    fn div(self, _rhs: geometric_anti_quotient) -> Self::Output {
        geometric_anti_quotient_partial(self)
    }
}
impl GeometricAntiQuotient<AntiScalar> for AntiScalar {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        0        1
    fn geometric_anti_quotient(self, other: AntiScalar) -> Self::Output {
        use crate::elements::*;
        return AntiScalar::from_groups(/* e1234 */ self[e1234] / (other[e1234]));
    }
}
impl GeometricAntiQuotient<DualNum> for AntiScalar {
    type Output = DualNum;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        2        0
    //    simd2        0        1        0
    // Totals...
    // yes simd        0        3        0
    //  no simd        0        4        0
    fn geometric_anti_quotient(self, other: DualNum) -> Self::Output {
        use crate::elements::*;
        let other_2 = AntiScalar::from_groups(/* e1234 */ f32::powi(other[e1234], -2));
        return DualNum::from_groups(
            // scalar, e1234
            Simd32x2::from(self[e1234]) * Simd32x2::from([other_2[e1234] * other[scalar], other_2[e1234] * other[e1234]]),
        );
    }
}
impl GeometricAntiQuotient<Flector> for AntiScalar {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        0        0
    //    simd4        0        4        0
    // Totals...
    // yes simd        3        4        0
    //  no simd        3       16        0
    fn geometric_anti_quotient(self, other: Flector) -> Self::Output {
        use crate::elements::*;
        let other_2 = AntiScalar::from_groups(
            // e1234
            f32::powi(other[e423], 2) + f32::powi(other[e431], 2) + f32::powi(other[e412], 2) - f32::powi(other[e4], 2),
        );
        let geometric_anti_product = Flector::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from(other_2[e1234]) * other.group0(),
            // e423, e431, e412, e321
            Simd32x4::from(other_2[e1234]) * other.group1(),
        );
        return Flector::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from(self[e1234]) * geometric_anti_product.group0(),
            // e423, e431, e412, e321
            Simd32x4::from(self[e1234]) * geometric_anti_product.group1(),
        );
    }
}
impl GeometricAntiQuotient<Line> for AntiScalar {
    type Output = Line;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        0        0
    //    simd3        0        4        0
    // Totals...
    // yes simd        2        4        0
    //  no simd        2       12        0
    fn geometric_anti_quotient(self, other: Line) -> Self::Output {
        use crate::elements::*;
        let other_2 = AntiScalar::from_groups(/* e1234 */ -f32::powi(other[e41], 2) - f32::powi(other[e42], 2) - f32::powi(other[e43], 2));
        let geometric_anti_product = Line::from_groups(
            // e41, e42, e43
            Simd32x3::from(other_2[e1234]) * other.group0(),
            // e23, e31, e12
            Simd32x3::from(other_2[e1234]) * other.group1(),
        );
        return Line::from_groups(
            // e41, e42, e43
            Simd32x3::from(self[e1234]) * geometric_anti_product.group0(),
            // e23, e31, e12
            Simd32x3::from(self[e1234]) * geometric_anti_product.group1(),
        );
    }
}
impl GeometricAntiQuotient<Motor> for AntiScalar {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        0        0
    //    simd4        0        4        0
    // Totals...
    // yes simd        3        4        0
    //  no simd        3       16        0
    fn geometric_anti_quotient(self, other: Motor) -> Self::Output {
        use crate::elements::*;
        let other_2 = AntiScalar::from_groups(
            // e1234
            f32::powi(other[e1234], 2) - f32::powi(other[e41], 2) - f32::powi(other[e42], 2) - f32::powi(other[e43], 2),
        );
        let geometric_anti_product = Motor::from_groups(
            // e41, e42, e43, e1234
            Simd32x4::from(other_2[e1234]) * other.group0(),
            // e23, e31, e12, scalar
            Simd32x4::from(other_2[e1234]) * other.group1(),
        );
        return Motor::from_groups(
            // e41, e42, e43, e1234
            Simd32x4::from(self[e1234]) * geometric_anti_product.group0(),
            // e23, e31, e12, scalar
            Simd32x4::from(self[e1234]) * geometric_anti_product.group1(),
        );
    }
}
impl GeometricAntiQuotient<MultiVector> for AntiScalar {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        7        0        0
    //    simd2        0        2        0
    //    simd3        0        4        0
    //    simd4        0        4        0
    // Totals...
    // yes simd        7       10        0
    //  no simd        7       32        0
    fn geometric_anti_quotient(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        let other_2 = AntiScalar::from_groups(
            // e1234
            f32::powi(other[e1234], 2) + f32::powi(other[e423], 2) + f32::powi(other[e431], 2) + f32::powi(other[e412], 2)
                - f32::powi(other[e4], 2)
                - f32::powi(other[e41], 2)
                - f32::powi(other[e42], 2)
                - f32::powi(other[e43], 2),
        );
        let geometric_anti_product = MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from(other_2[e1234]) * other.group0(),
            // e1, e2, e3, e4
            Simd32x4::from(other_2[e1234]) * other.group1(),
            // e41, e42, e43
            Simd32x3::from(other_2[e1234]) * other.group2(),
            // e23, e31, e12
            Simd32x3::from(other_2[e1234]) * other.group3(),
            // e423, e431, e412, e321
            Simd32x4::from(other_2[e1234]) * other.group4(),
        );
        return MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from(self[e1234]) * geometric_anti_product.group0(),
            // e1, e2, e3, e4
            Simd32x4::from(self[e1234]) * geometric_anti_product.group1(),
            // e41, e42, e43
            Simd32x3::from(self[e1234]) * geometric_anti_product.group2(),
            // e23, e31, e12
            Simd32x3::from(self[e1234]) * geometric_anti_product.group3(),
            // e423, e431, e412, e321
            Simd32x4::from(self[e1234]) * geometric_anti_product.group4(),
        );
    }
}
impl GeometricAntiQuotient<Origin> for AntiScalar {
    type Output = Origin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        1
    fn geometric_anti_quotient(self, other: Origin) -> Self::Output {
        use crate::elements::*;
        return Origin::from_groups(/* e4 */ self[e1234] / (other[e4]) * -1.0);
    }
}
impl GeometricAntiQuotient<Plane> for AntiScalar {
    type Output = Plane;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        4        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        2        5        0
    //  no simd        2        8        0
    fn geometric_anti_quotient(self, other: Plane) -> Self::Output {
        use crate::elements::*;
        let other_2 = AntiScalar::from_groups(/* e1234 */ f32::powi(other[e423], 2) + f32::powi(other[e431], 2) + f32::powi(other[e412], 2));
        return Plane::from_groups(
            // e423, e431, e412, e321
            Simd32x4::from(self[e1234]) * Simd32x4::from([other_2[e1234] * other[e423], other_2[e1234] * other[e431], other_2[e1234] * other[e412], other_2[e1234] * other[e321]]),
        );
    }
}
impl GeometricAntiQuotient<Point> for AntiScalar {
    type Output = Point;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        5        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        6        0
    //  no simd        0        9        0
    fn geometric_anti_quotient(self, other: Point) -> Self::Output {
        use crate::elements::*;
        let other_2 = AntiScalar::from_groups(/* e1234 */ f32::powi(other[e4], -2) * -1.0);
        return Point::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from(self[e1234]) * Simd32x4::from([other_2[e1234] * other[e1], other_2[e1234] * other[e2], other_2[e1234] * other[e3], other_2[e1234] * other[e4]]),
        );
    }
}
impl std::ops::Div<geometric_anti_quotient> for DualNum {
    type Output = geometric_anti_quotient_partial<DualNum>;
    fn div(self, _rhs: geometric_anti_quotient) -> Self::Output {
        geometric_anti_quotient_partial(self)
    }
}
impl GeometricAntiQuotient<AntiScalar> for DualNum {
    type Output = DualNum;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        0        1
    //    simd2        0        1        0
    // Totals...
    // yes simd        0        1        1
    //  no simd        0        2        1
    fn geometric_anti_quotient(self, other: AntiScalar) -> Self::Output {
        use crate::elements::*;
        return DualNum::from_groups(/* scalar, e1234 */ Simd32x2::from(1.0 / other[e1234]) * self.group0());
    }
}
impl GeometricAntiQuotient<DualNum> for DualNum {
    type Output = DualNum;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        3        0
    //    simd2        0        1        0
    // Totals...
    // yes simd        1        4        0
    //  no simd        1        5        0
    fn geometric_anti_quotient(self, other: DualNum) -> Self::Output {
        use crate::elements::*;
        let geometric_anti_product = DualNum::from_groups(/* scalar, e1234 */ Simd32x2::from(f32::powi(other[e1234], -2)) * other.group0());
        return DualNum::from_groups(/* scalar, e1234 */ Simd32x2::from([
            (geometric_anti_product[scalar] * self[e1234]) + (geometric_anti_product[e1234] * self[scalar]),
            geometric_anti_product[e1234] * self[e1234],
        ]));
    }
}
impl GeometricAntiQuotient<Flector> for DualNum {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        3        0
    //    simd3        1        2        0
    //    simd4        0        3        0
    // Totals...
    // yes simd        5        8        0
    //  no simd        7       21        0
    fn geometric_anti_quotient(self, other: Flector) -> Self::Output {
        use crate::elements::*;
        let other_2 = AntiScalar::from_groups(
            // e1234
            f32::powi(other[e423], 2) + f32::powi(other[e431], 2) + f32::powi(other[e412], 2) - f32::powi(other[e4], 2),
        );
        let geometric_anti_product = Flector::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from(other_2[e1234]) * other.group0(),
            // e423, e431, e412, e321
            Simd32x4::from(other_2[e1234]) * other.group1(),
        );
        return Flector::from_groups(
            // e1, e2, e3, e4
            ((Simd32x3::from(self[scalar]) * geometric_anti_product.group1().xyz()) + (Simd32x3::from(self[e1234]) * geometric_anti_product.group0().xyz()))
                .with_w(self[e1234] * geometric_anti_product[e4]),
            // e423, e431, e412, e321
            Simd32x4::from([geometric_anti_product[e423], geometric_anti_product[e431], geometric_anti_product[e412], 1.0])
                * self
                    .group0()
                    .yy()
                    .with_zw(self[e1234], (self[scalar] * geometric_anti_product[e4]) + (self[e1234] * geometric_anti_product[e321])),
        );
    }
}
impl GeometricAntiQuotient<Line> for DualNum {
    type Output = Line;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        0        0
    //    simd3        1        5        0
    // Totals...
    // yes simd        3        5        0
    //  no simd        5       15        0
    fn geometric_anti_quotient(self, other: Line) -> Self::Output {
        use crate::elements::*;
        let other_2 = AntiScalar::from_groups(/* e1234 */ -f32::powi(other[e41], 2) - f32::powi(other[e42], 2) - f32::powi(other[e43], 2));
        let geometric_anti_product = Line::from_groups(
            // e41, e42, e43
            Simd32x3::from(other_2[e1234]) * other.group0(),
            // e23, e31, e12
            Simd32x3::from(other_2[e1234]) * other.group1(),
        );
        return Line::from_groups(
            // e41, e42, e43
            Simd32x3::from(self[e1234]) * geometric_anti_product.group0(),
            // e23, e31, e12
            (Simd32x3::from(self[scalar]) * geometric_anti_product.group0()) + (Simd32x3::from(self[e1234]) * geometric_anti_product.group1()),
        );
    }
}
impl GeometricAntiQuotient<Motor> for DualNum {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        0        0
    //    simd4        1        5        0
    // Totals...
    // yes simd        4        5        0
    //  no simd        7       20        0
    fn geometric_anti_quotient(self, other: Motor) -> Self::Output {
        use crate::elements::*;
        let other_2 = AntiScalar::from_groups(
            // e1234
            f32::powi(other[e1234], 2) - f32::powi(other[e41], 2) - f32::powi(other[e42], 2) - f32::powi(other[e43], 2),
        );
        let geometric_anti_product = Motor::from_groups(
            // e41, e42, e43, e1234
            Simd32x4::from(other_2[e1234]) * other.group0(),
            // e23, e31, e12, scalar
            Simd32x4::from(other_2[e1234]) * other.group1(),
        );
        return Motor::from_groups(
            // e41, e42, e43, e1234
            Simd32x4::from(self[e1234]) * geometric_anti_product.group0(),
            // e23, e31, e12, scalar
            (Simd32x4::from(self[scalar]) * geometric_anti_product.group0()) + (Simd32x4::from(self[e1234]) * geometric_anti_product.group1()),
        );
    }
}
impl GeometricAntiQuotient<MultiVector> for DualNum {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        9        6        0
    //    simd2        0        1        0
    //    simd3        2        7        0
    //    simd4        0        3        0
    // Totals...
    // yes simd       11       17        0
    //  no simd       15       41        0
    fn geometric_anti_quotient(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        let other_2 = AntiScalar::from_groups(
            // e1234
            f32::powi(other[e1234], 2) + f32::powi(other[e423], 2) + f32::powi(other[e431], 2) + f32::powi(other[e412], 2)
                - f32::powi(other[e4], 2)
                - f32::powi(other[e41], 2)
                - f32::powi(other[e42], 2)
                - f32::powi(other[e43], 2),
        );
        let geometric_anti_product = MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from(other_2[e1234]) * other.group0(),
            // e1, e2, e3, e4
            Simd32x4::from(other_2[e1234]) * other.group1(),
            // e41, e42, e43
            Simd32x3::from(other_2[e1234]) * other.group2(),
            // e23, e31, e12
            Simd32x3::from(other_2[e1234]) * other.group3(),
            // e423, e431, e412, e321
            Simd32x4::from(other_2[e1234]) * other.group4(),
        );
        return MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([
                (self[scalar] * geometric_anti_product[e1234]) + (self[e1234] * geometric_anti_product[scalar]),
                self[e1234] * geometric_anti_product[e1234],
            ]),
            // e1, e2, e3, e4
            ((Simd32x3::from(self[scalar]) * geometric_anti_product.group4().xyz()) + (Simd32x3::from(self[e1234]) * geometric_anti_product.group1().xyz()))
                .with_w(self[e1234] * geometric_anti_product[e4]),
            // e41, e42, e43
            Simd32x3::from(self[e1234]) * geometric_anti_product.group2(),
            // e23, e31, e12
            (Simd32x3::from(self[scalar]) * geometric_anti_product.group2()) + (Simd32x3::from(self[e1234]) * geometric_anti_product.group3()),
            // e423, e431, e412, e321
            Simd32x4::from([geometric_anti_product[e423], geometric_anti_product[e431], geometric_anti_product[e412], 1.0])
                * self
                    .group0()
                    .yy()
                    .with_zw(self[e1234], (self[scalar] * geometric_anti_product[e4]) + (self[e1234] * geometric_anti_product[e321])),
        );
    }
}
impl GeometricAntiQuotient<Origin> for DualNum {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        3        1
    fn geometric_anti_quotient(self, other: Origin) -> Self::Output {
        use crate::elements::*;
        let geometric_anti_product = Origin::from_groups(/* e4 */ 1.0 / other[e4] * -1.0);
        return Flector::from_groups(
            // e1, e2, e3, e4
            Simd32x3::from(0.0).with_w(self[e1234] * geometric_anti_product[e4]),
            // e423, e431, e412, e321
            Simd32x3::from(0.0).with_w(self[scalar] * geometric_anti_product[e4]),
        );
    }
}
impl GeometricAntiQuotient<Plane> for DualNum {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        0        0
    //    simd4        0        5        0
    // Totals...
    // yes simd        2        5        0
    //  no simd        2       20        0
    fn geometric_anti_quotient(self, other: Plane) -> Self::Output {
        use crate::elements::*;
        let geometric_anti_product = Plane::from_groups(
            // e423, e431, e412, e321
            Simd32x4::from(f32::powi(other[e423], 2) + f32::powi(other[e431], 2) + f32::powi(other[e412], 2)) * other.group0(),
        );
        return Flector::from_groups(
            // e1, e2, e3, e4
            self.group0().xx().with_zw(self[scalar], 0.0)
                * Simd32x3::from(1.0).with_w(0.0)
                * geometric_anti_product.group0().xyz().with_w(0.0)
                * Simd32x4::from([1.0, 1.0, 1.0, 0.0]),
            // e423, e431, e412, e321
            Simd32x4::from(self[e1234]) * geometric_anti_product.group0(),
        );
    }
}
impl GeometricAntiQuotient<Point> for DualNum {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        2        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        0        4        0
    //  no simd        0       10        0
    fn geometric_anti_quotient(self, other: Point) -> Self::Output {
        use crate::elements::*;
        let geometric_anti_product = Point::from_groups(/* e1, e2, e3, e4 */ Simd32x4::from(f32::powi(other[e4], -2) * -1.0) * other.group0());
        return Flector::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from(self[e1234]) * geometric_anti_product.group0(),
            // e423, e431, e412, e321
            Simd32x3::from(0.0).with_w(self[scalar] * geometric_anti_product[e4]),
        );
    }
}
impl std::ops::Div<geometric_anti_quotient> for Flector {
    type Output = geometric_anti_quotient_partial<Flector>;
    fn div(self, _rhs: geometric_anti_quotient) -> Self::Output {
        geometric_anti_quotient_partial(self)
    }
}
impl GeometricAntiQuotient<AntiScalar> for Flector {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        0        1
    //    simd4        0        2        0
    // Totals...
    // yes simd        0        2        1
    //  no simd        0        8        1
    fn geometric_anti_quotient(self, other: AntiScalar) -> Self::Output {
        use crate::elements::*;
        let geometric_anti_product = AntiScalar::from_groups(/* e1234 */ 1.0 / other[e1234]);
        return Flector::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from(geometric_anti_product[e1234]) * self.group0(),
            // e423, e431, e412, e321
            Simd32x4::from(geometric_anti_product[e1234]) * self.group1(),
        );
    }
}
impl GeometricAntiQuotient<DualNum> for Flector {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        3        0
    //    simd2        0        1        0
    //    simd3        1        2        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        2        7        0
    //  no simd        4       15        0
    fn geometric_anti_quotient(self, other: DualNum) -> Self::Output {
        use crate::elements::*;
        let geometric_anti_product = DualNum::from_groups(/* scalar, e1234 */ Simd32x2::from(f32::powi(other[e1234], -2)) * other.group0());
        return Flector::from_groups(
            // e1, e2, e3, e4
            ((Simd32x3::from(geometric_anti_product[e1234]) * self.group0().xyz()) - (Simd32x3::from(geometric_anti_product[scalar]) * self.group1().xyz()))
                .with_w(geometric_anti_product[e1234] * self[e4]),
            // e423, e431, e412, e321
            Simd32x4::from([self[e423], self[e431], self[e412], 1.0])
                * geometric_anti_product.group0().yy().with_zw(
                    geometric_anti_product[e1234],
                    (geometric_anti_product[e1234] * self[e321]) - (geometric_anti_product[scalar] * self[e4]),
                ),
        );
    }
}
impl GeometricAntiQuotient<Flector> for Flector {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        7        8        0
    //    simd4        9       12        0
    // Totals...
    // yes simd       16       20        0
    //  no simd       43       56        0
    fn geometric_anti_quotient(self, other: Flector) -> Self::Output {
        use crate::elements::*;
        let other_2 = AntiScalar::from_groups(
            // e1234
            f32::powi(other[e423], 2) + f32::powi(other[e431], 2) + f32::powi(other[e412], 2) - f32::powi(other[e4], 2),
        );
        let geometric_anti_product = Flector::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from(other_2[e1234]) * other.group0(),
            // e423, e431, e412, e321
            Simd32x4::from(other_2[e1234]) * other.group1(),
        );
        return Motor::from_groups(
            // e41, e42, e43, e1234
            Simd32x4::from([
                -(geometric_anti_product[e423] * self[e4]) - (geometric_anti_product[e412] * self[e431]),
                -(geometric_anti_product[e423] * self[e412]) - (geometric_anti_product[e431] * self[e4]),
                -(geometric_anti_product[e431] * self[e423]) - (geometric_anti_product[e412] * self[e4]),
                (geometric_anti_product[e431] * self[e431]) + (geometric_anti_product[e412] * self[e412]),
            ]) + (geometric_anti_product.group1().yzxx() * self.group1().zxyx())
                - (Simd32x4::from(geometric_anti_product[e4]) * self.group1().xyz().with_w(self[e4])),
            // e23, e31, e12, scalar
            (Simd32x4::from(geometric_anti_product[e321]) * self.group1().xyz().with_w(self[e4]))
                + (Simd32x4::from([self[e4], self[e412], self[e423], self[e1]]) * geometric_anti_product.group0().xxy().with_w(geometric_anti_product[e423]))
                + (Simd32x4::from([self[e431], self[e4], self[e4], self[e2]]) * geometric_anti_product.group0().zyz().with_w(geometric_anti_product[e431]))
                + (geometric_anti_product.group1().yzxz() * self.group0().zxyz())
                - (Simd32x4::from([self[e2], self[e321], self[e321], self[e321]]) * geometric_anti_product.group1().zyz().with_w(geometric_anti_product[e4]))
                - (Simd32x4::from([self[e321], self[e3], self[e1], self[e412]]) * geometric_anti_product.group1().xxy().with_w(geometric_anti_product[e3]))
                - (geometric_anti_product.group0().yzxx() * self.group1().zxyx())
                - (geometric_anti_product.group0().wwwy() * self.group0().xyz().with_w(self[e431])),
        );
    }
}
impl GeometricAntiQuotient<Line> for Flector {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       10       14        0
    //    simd3        0        4        0
    //    simd4        6        4        0
    // Totals...
    // yes simd       16       22        0
    //  no simd       34       42        0
    fn geometric_anti_quotient(self, other: Line) -> Self::Output {
        use crate::elements::*;
        let other_2 = AntiScalar::from_groups(/* e1234 */ -f32::powi(other[e41], 2) - f32::powi(other[e42], 2) - f32::powi(other[e43], 2));
        let geometric_anti_product = Line::from_groups(
            // e41, e42, e43
            Simd32x3::from(other_2[e1234]) * other.group0(),
            // e23, e31, e12
            Simd32x3::from(other_2[e1234]) * other.group1(),
        );
        return Flector::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from([
                (self[e2] * geometric_anti_product[e43]) + (self[e412] * geometric_anti_product[e31]) + (self[e321] * geometric_anti_product[e41]),
                (self[e3] * geometric_anti_product[e41]) + (self[e423] * geometric_anti_product[e12]) + (self[e321] * geometric_anti_product[e42]),
                (self[e1] * geometric_anti_product[e42]) + (self[e431] * geometric_anti_product[e23]) + (self[e321] * geometric_anti_product[e43]),
                0.0,
            ]) - (Simd32x4::from([self[e4], self[e412], self[e423], self[e431]]) * geometric_anti_product.group1().xxy().with_w(geometric_anti_product[e42]))
                - (Simd32x4::from([self[e431], self[e4], self[e4], self[e412]]) * geometric_anti_product.group1().zyz().with_w(geometric_anti_product[e43]))
                - (geometric_anti_product.group0().yzx() * self.group0().zxy()).with_w(self[e423] * geometric_anti_product[e41]),
            // e423, e431, e412, e321
            (Simd32x4::from([self[e4], self[e412], self[e423], self[e423]]) * geometric_anti_product.group0().xxy().with_w(geometric_anti_product[e23]))
                + (Simd32x4::from([self[e431], self[e4], self[e4], self[e431]]) * geometric_anti_product.group0().zyz().with_w(geometric_anti_product[e31]))
                + Simd32x3::from(0.0).with_w((self[e412] * geometric_anti_product[e12]) - (self[e2] * geometric_anti_product[e42]) - (self[e3] * geometric_anti_product[e43]))
                - (geometric_anti_product.group0().yzx() * self.group1().zxy()).with_w(self[e1] * geometric_anti_product[e41]),
        );
    }
}
impl GeometricAntiQuotient<Motor> for Flector {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       15       18        0
    //    simd3        0        2        0
    //    simd4        8        8        0
    // Totals...
    // yes simd       23       28        0
    //  no simd       47       56        0
    fn geometric_anti_quotient(self, other: Motor) -> Self::Output {
        use crate::elements::*;
        let other_2 = AntiScalar::from_groups(
            // e1234
            f32::powi(other[e1234], 2) - f32::powi(other[e41], 2) - f32::powi(other[e42], 2) - f32::powi(other[e43], 2),
        );
        let geometric_anti_product = Motor::from_groups(
            // e41, e42, e43, e1234
            Simd32x4::from(other_2[e1234]) * other.group0(),
            // e23, e31, e12, scalar
            Simd32x4::from(other_2[e1234]) * other.group1(),
        );
        return Flector::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from([
                (self[e2] * geometric_anti_product[e43]) + (self[e412] * geometric_anti_product[e31]) + (self[e321] * geometric_anti_product[e41])
                    - (self[e431] * geometric_anti_product[e12]),
                (self[e3] * geometric_anti_product[e41]) + (self[e423] * geometric_anti_product[e12]) + (self[e321] * geometric_anti_product[e42])
                    - (self[e412] * geometric_anti_product[e23]),
                (self[e3] * geometric_anti_product[e1234]) + (self[e431] * geometric_anti_product[e23]) + (self[e321] * geometric_anti_product[e43])
                    - (self[e412] * geometric_anti_product[scalar]),
                0.0,
            ]) + (self.group0().xyxw() * geometric_anti_product.group0().wwyw())
                - (self.group1().xyxz() * geometric_anti_product.group1().wwy().with_w(geometric_anti_product[e43]))
                - (geometric_anti_product.group0().yzxx() * self.group0().zxy().with_w(self[e423]))
                - (self.group0().www() * geometric_anti_product.group1().xyz()).with_w(self[e431] * geometric_anti_product[e42]),
            // e423, e431, e412, e321
            (self.group1().xyxy() * geometric_anti_product.group0().wwy().with_w(geometric_anti_product[e31]))
                + (self.group1().yzzz() * geometric_anti_product.group0().zxw().with_w(geometric_anti_product[e12]))
                + Simd32x3::from(0.0).with_w(
                    (self[e321] * geometric_anti_product[e1234])
                        - (self[e2] * geometric_anti_product[e42])
                        - (self[e3] * geometric_anti_product[e43])
                        - (self[e4] * geometric_anti_product[scalar]),
                )
                + (self.group0().www() * geometric_anti_product.group0().xyz()).with_w(self[e423] * geometric_anti_product[e23])
                - (geometric_anti_product.group0().yzxx() * self.group1().zxy().with_w(self[e1])),
        );
    }
}
impl GeometricAntiQuotient<MultiVector> for Flector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       22       22        0
    //    simd2        4        5        0
    //    simd3       10       16        0
    //    simd4        8        8        0
    // Totals...
    // yes simd       44       51        0
    //  no simd       92      112        0
    fn geometric_anti_quotient(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        let other_2 = AntiScalar::from_groups(
            // e1234
            f32::powi(other[e1234], 2) + f32::powi(other[e423], 2) + f32::powi(other[e431], 2) + f32::powi(other[e412], 2)
                - f32::powi(other[e4], 2)
                - f32::powi(other[e41], 2)
                - f32::powi(other[e42], 2)
                - f32::powi(other[e43], 2),
        );
        let geometric_anti_product = MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from(other_2[e1234]) * other.group0(),
            // e1, e2, e3, e4
            Simd32x4::from(other_2[e1234]) * other.group1(),
            // e41, e42, e43
            Simd32x3::from(other_2[e1234]) * other.group2(),
            // e23, e31, e12
            Simd32x3::from(other_2[e1234]) * other.group3(),
            // e423, e431, e412, e321
            Simd32x4::from(other_2[e1234]) * other.group4(),
        );
        return MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([
                (self[e4] * geometric_anti_product[e321])
                    - (self[e431] * geometric_anti_product[e2])
                    - (self[e412] * geometric_anti_product[e3])
                    - (self[e321] * geometric_anti_product[e4]),
                0.0,
            ]) + (Simd32x2::from(geometric_anti_product[e423]) * Simd32x2::from([self[e1], self[e423]]))
                + (Simd32x2::from(geometric_anti_product[e431]) * Simd32x2::from([self[e2], self[e431]]))
                + (Simd32x2::from(geometric_anti_product[e412]) * Simd32x2::from([self[e3], self[e412]]))
                - (Simd32x2::from([self[e423], self[e4]]) * geometric_anti_product.group1().xw()),
            // e1, e2, e3, e4
            Simd32x4::from([
                (self[e2] * geometric_anti_product[e43]) + (self[e412] * geometric_anti_product[e31]) + (self[e321] * geometric_anti_product[e41])
                    - (self[e431] * geometric_anti_product[e12]),
                (self[e3] * geometric_anti_product[e41]) + (self[e423] * geometric_anti_product[e12]) + (self[e321] * geometric_anti_product[e42])
                    - (self[e4] * geometric_anti_product[e31]),
                (self[e1] * geometric_anti_product[e42]) + (self[e431] * geometric_anti_product[e23]) + (self[e321] * geometric_anti_product[e43])
                    - (self[e4] * geometric_anti_product[e12]),
                0.0,
            ]) + (Simd32x4::from(geometric_anti_product[e1234]) * self.group0())
                - (Simd32x4::from([self[e4], self[e412], self[e423], self[e412]]) * geometric_anti_product.group3().xxy().with_w(geometric_anti_product[e43]))
                - (self.group1().xyzx() * geometric_anti_product.group0().xx().with_zw(geometric_anti_product[scalar], geometric_anti_product[e41]))
                - (geometric_anti_product.group2().yzx() * self.group0().zxy()).with_w(self[e431] * geometric_anti_product[e42]),
            // e41, e42, e43
            (self.group1().zxy() * geometric_anti_product.group4().yzx())
                - (Simd32x3::from(self[e4]) * geometric_anti_product.group4().xyz())
                - (Simd32x3::from([geometric_anti_product[e4], geometric_anti_product[e4], geometric_anti_product[e431]]) * self.group1().xyx())
                - (Simd32x3::from([geometric_anti_product[e412], geometric_anti_product[e423], geometric_anti_product[e4]]) * self.group1().yzz()),
            // e23, e31, e12
            (Simd32x3::from(self[e4]) * geometric_anti_product.group1().xyz())
                + (Simd32x3::from([geometric_anti_product[e3], geometric_anti_product[e1], geometric_anti_product[e321]]) * self.group1().yzz())
                + (Simd32x3::from([geometric_anti_product[e321], geometric_anti_product[e321], geometric_anti_product[e2]]) * self.group1().xyx())
                + (self.group0().zxy() * geometric_anti_product.group4().yzx())
                - (Simd32x3::from(self[e321]) * geometric_anti_product.group4().xyz())
                - (Simd32x3::from([geometric_anti_product[e4], geometric_anti_product[e4], geometric_anti_product[e431]]) * self.group0().xyx())
                - (Simd32x3::from([geometric_anti_product[e412], geometric_anti_product[e423], geometric_anti_product[e4]]) * self.group0().yzz())
                - (self.group1().zxy() * geometric_anti_product.group1().yzx()),
            // e423, e431, e412, e321
            (Simd32x4::from(geometric_anti_product[e1234]) * self.group1())
                + (Simd32x4::from([self[e4], self[e412], self[e423], self[e423]]) * geometric_anti_product.group2().xxy().with_w(geometric_anti_product[e23]))
                + (Simd32x4::from([self[e431], self[e4], self[e4], self[e431]]) * geometric_anti_product.group2().zyz().with_w(geometric_anti_product[e31]))
                + Simd32x3::from(0.0).with_w(
                    (self[e412] * geometric_anti_product[e12])
                        - (self[e1] * geometric_anti_product[e41])
                        - (self[e2] * geometric_anti_product[e42])
                        - (self[e3] * geometric_anti_product[e43]),
                )
                - (geometric_anti_product.group2().yzx() * self.group1().zxy()).with_w(self[e4] * geometric_anti_product[scalar]),
        );
    }
}
impl GeometricAntiQuotient<Origin> for Flector {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        1
    //    simd4        0        4        0
    // Totals...
    // yes simd        0        5        1
    //  no simd        0       17        1
    fn geometric_anti_quotient(self, other: Origin) -> Self::Output {
        use crate::elements::*;
        let geometric_anti_product = Origin::from_groups(/* e4 */ 1.0 / other[e4] * -1.0);
        return Motor::from_groups(
            // e41, e42, e43, e1234
            Simd32x4::from(geometric_anti_product[e4]) * self.group1().xyz().with_w(self[e4]) * Simd32x4::from(-1.0),
            // e23, e31, e12, scalar
            Simd32x4::from(geometric_anti_product[e4]) * self.group0().xyz().with_w(self[e321]) * Simd32x4::from(-1.0),
        );
    }
}
impl GeometricAntiQuotient<Plane> for Flector {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       10       16        0
    //    simd4        3        4        0
    // Totals...
    // yes simd       13       20        0
    //  no simd       22       32        0
    fn geometric_anti_quotient(self, other: Plane) -> Self::Output {
        use crate::elements::*;
        let geometric_anti_product = Plane::from_groups(
            // e423, e431, e412, e321
            Simd32x4::from(f32::powi(other[e423], 2) + f32::powi(other[e431], 2) + f32::powi(other[e412], 2)) * other.group0(),
        );
        return Motor::from_groups(
            // e41, e42, e43, e1234
            Simd32x4::from([
                -(self[e4] * geometric_anti_product[e423]) - (self[e431] * geometric_anti_product[e412]),
                -(self[e4] * geometric_anti_product[e431]) - (self[e412] * geometric_anti_product[e423]),
                -(self[e4] * geometric_anti_product[e412]) - (self[e423] * geometric_anti_product[e431]),
                (self[e431] * geometric_anti_product[e431]) + (self[e412] * geometric_anti_product[e412]),
            ]) + (self.group1().zxyx() * geometric_anti_product.group0().yzxx()),
            // e23, e31, e12, scalar
            Simd32x4::from([
                -(self[e2] * geometric_anti_product[e412]) - (self[e321] * geometric_anti_product[e423]),
                -(self[e3] * geometric_anti_product[e423]) - (self[e321] * geometric_anti_product[e431]),
                -(self[e1] * geometric_anti_product[e431]) - (self[e321] * geometric_anti_product[e412]),
                (self[e3] * geometric_anti_product[e412]) + (self[e4] * geometric_anti_product[e321]),
            ]) + (self.group0().zxyx() * geometric_anti_product.group0().yzxx())
                + (geometric_anti_product.group0().wwwy() * self.group1().xyz().with_w(self[e2])),
        );
    }
}
impl GeometricAntiQuotient<Point> for Flector {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        9        0
    //    simd4        2        5        0
    // Totals...
    // yes simd        6       14        0
    //  no simd       12       29        0
    fn geometric_anti_quotient(self, other: Point) -> Self::Output {
        use crate::elements::*;
        let geometric_anti_product = Point::from_groups(/* e1, e2, e3, e4 */ Simd32x4::from(f32::powi(other[e4], -2) * -1.0) * other.group0());
        return Motor::from_groups(
            // e41, e42, e43, e1234
            Simd32x4::from(geometric_anti_product[e4]) * self.group1().xyz().with_w(self[e4]) * Simd32x4::from(-1.0),
            // e23, e31, e12, scalar
            Simd32x4::from([
                (self[e4] * geometric_anti_product[e1]) + (self[e431] * geometric_anti_product[e3]),
                (self[e4] * geometric_anti_product[e2]) + (self[e412] * geometric_anti_product[e1]),
                (self[e4] * geometric_anti_product[e3]) + (self[e423] * geometric_anti_product[e2]),
                -(self[e412] * geometric_anti_product[e3]) - (self[e321] * geometric_anti_product[e4]),
            ]) - (self.group1().zxyy() * geometric_anti_product.group0().yzxy())
                - (geometric_anti_product.group0().wwwx() * self.group0().xyz().with_w(self[e423])),
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
    type Output = Horizon;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        0        1
    fn geometric_anti_quotient(self, other: AntiScalar) -> Self::Output {
        use crate::elements::*;
        return Horizon::from_groups(/* e321 */ self[e321] / (other[e1234]));
    }
}
impl GeometricAntiQuotient<DualNum> for Horizon {
    type Output = Horizon;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        0        1
    fn geometric_anti_quotient(self, other: DualNum) -> Self::Output {
        use crate::elements::*;
        return Horizon::from_groups(/* e321 */ self[e321] / (other[e1234]));
    }
}
impl GeometricAntiQuotient<Flector> for Horizon {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        0        0
    //    simd4        0        4        0
    // Totals...
    // yes simd        3        4        0
    //  no simd        3       16        0
    fn geometric_anti_quotient(self, other: Flector) -> Self::Output {
        use crate::elements::*;
        let other_2 = AntiScalar::from_groups(
            // e1234
            f32::powi(other[e423], 2) + f32::powi(other[e431], 2) + f32::powi(other[e412], 2) - f32::powi(other[e4], 2),
        );
        let geometric_anti_product = Flector::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from(other_2[e1234]) * other.group0(),
            // e423, e431, e412, e321
            Simd32x4::from(other_2[e1234]) * other.group1(),
        );
        return Motor::from_groups(
            // e41, e42, e43, e1234
            Simd32x4::from(0.0),
            // e23, e31, e12, scalar
            Simd32x4::from(self[e321]) * geometric_anti_product.group1().xyz().with_w(geometric_anti_product[e4]) * Simd32x4::from(-1.0),
        );
    }
}
impl GeometricAntiQuotient<Line> for Horizon {
    type Output = Point;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        3        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        2        5        0
    //  no simd        2       11        0
    fn geometric_anti_quotient(self, other: Line) -> Self::Output {
        use crate::elements::*;
        let other_2 = AntiScalar::from_groups(/* e1234 */ -f32::powi(other[e41], 2) - f32::powi(other[e42], 2) - f32::powi(other[e43], 2));
        return Point::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from([self[e321], self[e321], self[e321], 0.0])
                * Simd32x3::from([other_2[e1234] * other[e41], other_2[e1234] * other[e42], other_2[e1234] * other[e43]]).with_w(0.0)
                * Simd32x4::from([1.0, 1.0, 1.0, 0.0]),
        );
    }
}
impl GeometricAntiQuotient<Motor> for Horizon {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        1        0
    //    simd4        0        4        0
    // Totals...
    // yes simd        3        5        0
    //  no simd        3       17        0
    fn geometric_anti_quotient(self, other: Motor) -> Self::Output {
        use crate::elements::*;
        let other_2 = AntiScalar::from_groups(
            // e1234
            f32::powi(other[e1234], 2) - f32::powi(other[e41], 2) - f32::powi(other[e42], 2) - f32::powi(other[e43], 2),
        );
        let geometric_anti_product = Motor::from_groups(
            // e41, e42, e43, e1234
            Simd32x4::from(other_2[e1234]) * other.group0(),
            // e23, e31, e12, scalar
            Simd32x4::from(other_2[e1234]) * other.group1(),
        );
        return Flector::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from([self[e321], self[e321], self[e321], 0.0]) * geometric_anti_product.group0().xyz().with_w(0.0) * Simd32x4::from([1.0, 1.0, 1.0, 0.0]),
            // e423, e431, e412, e321
            Simd32x3::from(0.0).with_w(self[e321] * geometric_anti_product[e1234]),
        );
    }
}
impl GeometricAntiQuotient<MultiVector> for Horizon {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        7        2        0
    //    simd2        0        2        0
    //    simd3        0        4        0
    //    simd4        0        4        0
    // Totals...
    // yes simd        7       12        0
    //  no simd        7       34        0
    fn geometric_anti_quotient(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        let other_2 = AntiScalar::from_groups(
            // e1234
            f32::powi(other[e1234], 2) + f32::powi(other[e423], 2) + f32::powi(other[e431], 2) + f32::powi(other[e412], 2)
                - f32::powi(other[e4], 2)
                - f32::powi(other[e41], 2)
                - f32::powi(other[e42], 2)
                - f32::powi(other[e43], 2),
        );
        let geometric_anti_product = MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from(other_2[e1234]) * other.group0(),
            // e1, e2, e3, e4
            Simd32x4::from(other_2[e1234]) * other.group1(),
            // e41, e42, e43
            Simd32x3::from(other_2[e1234]) * other.group2(),
            // e23, e31, e12
            Simd32x3::from(other_2[e1234]) * other.group3(),
            // e423, e431, e412, e321
            Simd32x4::from(other_2[e1234]) * other.group4(),
        );
        return MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([self[e321] * geometric_anti_product[e4], 1.0]) * Simd32x2::from([-1.0, 0.0]),
            // e1, e2, e3, e4
            Simd32x4::from([self[e321], self[e321], self[e321], 0.0]) * geometric_anti_product.group2().with_w(0.0) * Simd32x4::from([1.0, 1.0, 1.0, 0.0]),
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(self[e321]) * geometric_anti_product.group4().xyz() * Simd32x3::from(-1.0),
            // e423, e431, e412, e321
            Simd32x3::from(0.0).with_w(self[e321] * geometric_anti_product[e1234]),
        );
    }
}
impl GeometricAntiQuotient<Origin> for Horizon {
    type Output = Scalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        0        1
    fn geometric_anti_quotient(self, other: Origin) -> Self::Output {
        use crate::elements::*;
        return Scalar::from_groups(/* scalar */ self[e321] / (other[e4]));
    }
}
impl GeometricAntiQuotient<Plane> for Horizon {
    type Output = Line;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        4        0
    //    simd3        0        2        0
    // Totals...
    // yes simd        2        6        0
    //  no simd        2       10        0
    fn geometric_anti_quotient(self, other: Plane) -> Self::Output {
        use crate::elements::*;
        let other_2 = AntiScalar::from_groups(/* e1234 */ f32::powi(other[e423], 2) + f32::powi(other[e431], 2) + f32::powi(other[e412], 2));
        return Line::from_groups(
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(self[e321])
                * Simd32x4::from([other_2[e1234] * other[e423], other_2[e1234] * other[e431], other_2[e1234] * other[e412], other_2[e1234] * other[e321]]).xyz()
                * Simd32x3::from(-1.0),
        );
    }
}
impl GeometricAntiQuotient<Point> for Horizon {
    type Output = Scalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        0        1
    fn geometric_anti_quotient(self, other: Point) -> Self::Output {
        use crate::elements::*;
        return Scalar::from_groups(/* scalar */ self[e321] / (other[e4]));
    }
}
impl std::ops::Div<geometric_anti_quotient> for Line {
    type Output = geometric_anti_quotient_partial<Line>;
    fn div(self, _rhs: geometric_anti_quotient) -> Self::Output {
        geometric_anti_quotient_partial(self)
    }
}
impl GeometricAntiQuotient<AntiScalar> for Line {
    type Output = Line;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        0        1
    //    simd3        0        2        0
    // Totals...
    // yes simd        0        2        1
    //  no simd        0        6        1
    fn geometric_anti_quotient(self, other: AntiScalar) -> Self::Output {
        use crate::elements::*;
        let geometric_anti_product = AntiScalar::from_groups(/* e1234 */ 1.0 / other[e1234]);
        return Line::from_groups(
            // e41, e42, e43
            Simd32x3::from(geometric_anti_product[e1234]) * self.group0(),
            // e23, e31, e12
            Simd32x3::from(geometric_anti_product[e1234]) * self.group1(),
        );
    }
}
impl GeometricAntiQuotient<DualNum> for Line {
    type Output = Line;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd2        0        1        0
    //    simd3        1        3        0
    // Totals...
    // yes simd        1        4        0
    //  no simd        3       11        0
    fn geometric_anti_quotient(self, other: DualNum) -> Self::Output {
        use crate::elements::*;
        let geometric_anti_product = DualNum::from_groups(/* scalar, e1234 */ Simd32x2::from(f32::powi(other[e1234], -2)) * other.group0());
        return Line::from_groups(
            // e41, e42, e43
            Simd32x3::from(geometric_anti_product[e1234]) * self.group0(),
            // e23, e31, e12
            (Simd32x3::from(geometric_anti_product[scalar]) * self.group0()) + (Simd32x3::from(geometric_anti_product[e1234]) * self.group1()),
        );
    }
}
impl GeometricAntiQuotient<Flector> for Line {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       19       27        0
    //    simd3        0        2        0
    //    simd4        3        3        0
    // Totals...
    // yes simd       22       32        0
    //  no simd       31       45        0
    fn geometric_anti_quotient(self, other: Flector) -> Self::Output {
        use crate::elements::*;
        let other_2 = AntiScalar::from_groups(
            // e1234
            f32::powi(other[e423], 2) + f32::powi(other[e431], 2) + f32::powi(other[e412], 2) - f32::powi(other[e4], 2),
        );
        let geometric_anti_product = Flector::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from(other_2[e1234]) * other.group0(),
            // e423, e431, e412, e321
            Simd32x4::from(other_2[e1234]) * other.group1(),
        );
        return Flector::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from([
                (geometric_anti_product[e3] * self[e42])
                    + (geometric_anti_product[e4] * self[e23])
                    + (geometric_anti_product[e412] * self[e31])
                    + (geometric_anti_product[e321] * self[e41]),
                (geometric_anti_product[e1] * self[e43])
                    + (geometric_anti_product[e4] * self[e31])
                    + (geometric_anti_product[e423] * self[e12])
                    + (geometric_anti_product[e321] * self[e42]),
                (geometric_anti_product[e2] * self[e41])
                    + (geometric_anti_product[e4] * self[e12])
                    + (geometric_anti_product[e431] * self[e23])
                    + (geometric_anti_product[e321] * self[e43]),
                geometric_anti_product[e412] * self[e43] * -1.0,
            ]) - (geometric_anti_product.group1().yzxy() * self.group1().zxy().with_w(self[e42]))
                - (self.group0().zxy() * geometric_anti_product.group0().yzx()).with_w(geometric_anti_product[e423] * self[e41]),
            // e423, e431, e412, e321
            Simd32x4::from([
                (geometric_anti_product[e4] * self[e41]) + (geometric_anti_product[e412] * self[e42]),
                (geometric_anti_product[e4] * self[e42]) + (geometric_anti_product[e423] * self[e43]),
                (geometric_anti_product[e4] * self[e43]) + (geometric_anti_product[e431] * self[e41]),
                -(geometric_anti_product[e2] * self[e42])
                    - (geometric_anti_product[e3] * self[e43])
                    - (geometric_anti_product[e423] * self[e23])
                    - (geometric_anti_product[e431] * self[e31])
                    - (geometric_anti_product[e412] * self[e12]),
            ]) - (self.group0().zxy() * geometric_anti_product.group1().yzx()).with_w(geometric_anti_product[e1] * self[e41]),
        );
    }
}
impl GeometricAntiQuotient<Line> for Line {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        9       18        0
    //    simd3        0        5        0
    //    simd4        3        0        0
    // Totals...
    // yes simd       12       23        0
    //  no simd       21       33        0
    fn geometric_anti_quotient(self, other: Line) -> Self::Output {
        use crate::elements::*;
        let other_2 = AntiScalar::from_groups(/* e1234 */ -f32::powi(other[e41], 2) - f32::powi(other[e42], 2) - f32::powi(other[e43], 2));
        let geometric_anti_product = Line::from_groups(
            // e41, e42, e43
            Simd32x3::from(other_2[e1234]) * other.group0(),
            // e23, e31, e12
            Simd32x3::from(other_2[e1234]) * other.group1(),
        );
        return Motor::from_groups(
            // e41, e42, e43, e1234
            Simd32x4::from([
                geometric_anti_product[e43] * self[e42],
                geometric_anti_product[e41] * self[e43],
                geometric_anti_product[e42] * self[e41],
                -(geometric_anti_product[e42] * self[e42]) - (geometric_anti_product[e43] * self[e43]),
            ]) - (geometric_anti_product.group0().yzx() * self.group0().zxy()).with_w(geometric_anti_product[e41] * self[e41]),
            // e23, e31, e12, scalar
            Simd32x4::from([
                (geometric_anti_product[e43] * self[e31]) + (geometric_anti_product[e12] * self[e42]),
                (geometric_anti_product[e41] * self[e12]) + (geometric_anti_product[e23] * self[e43]),
                (geometric_anti_product[e42] * self[e23]) + (geometric_anti_product[e31] * self[e41]),
                -(geometric_anti_product[e43] * self[e12])
                    - (geometric_anti_product[e23] * self[e41])
                    - (geometric_anti_product[e31] * self[e42])
                    - (geometric_anti_product[e12] * self[e43]),
            ]) - (geometric_anti_product.group0().yzx() * self.group1().zxy()).with_w(geometric_anti_product[e41] * self[e23])
                - (geometric_anti_product.group1().yzx() * self.group0().zxy()).with_w(geometric_anti_product[e42] * self[e31]),
        );
    }
}
impl GeometricAntiQuotient<Motor> for Line {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       19       25        0
    //    simd3        0        1        0
    //    simd4        3        4        0
    // Totals...
    // yes simd       22       30        0
    //  no simd       31       44        0
    fn geometric_anti_quotient(self, other: Motor) -> Self::Output {
        use crate::elements::*;
        let other_2 = AntiScalar::from_groups(
            // e1234
            f32::powi(other[e1234], 2) - f32::powi(other[e41], 2) - f32::powi(other[e42], 2) - f32::powi(other[e43], 2),
        );
        let geometric_anti_product = Motor::from_groups(
            // e41, e42, e43, e1234
            Simd32x4::from(other_2[e1234]) * other.group0(),
            // e23, e31, e12, scalar
            Simd32x4::from(other_2[e1234]) * other.group1(),
        );
        return Motor::from_groups(
            // e41, e42, e43, e1234
            Simd32x4::from([
                (self[e41] * geometric_anti_product[e1234]) + (self[e42] * geometric_anti_product[e43]),
                (self[e42] * geometric_anti_product[e1234]) + (self[e43] * geometric_anti_product[e41]),
                (self[e41] * geometric_anti_product[e42]) + (self[e43] * geometric_anti_product[e1234]),
                -(self[e42] * geometric_anti_product[e42]) - (self[e43] * geometric_anti_product[e43]),
            ]) - (geometric_anti_product.group0().yzxx() * self.group0().zxy().with_w(self[e41])),
            // e23, e31, e12, scalar
            Simd32x4::from([
                (self[e41] * geometric_anti_product[scalar])
                    + (self[e42] * geometric_anti_product[e12])
                    + (self[e23] * geometric_anti_product[e1234])
                    + (self[e31] * geometric_anti_product[e43]),
                (self[e42] * geometric_anti_product[scalar])
                    + (self[e43] * geometric_anti_product[e23])
                    + (self[e31] * geometric_anti_product[e1234])
                    + (self[e12] * geometric_anti_product[e41]),
                (self[e41] * geometric_anti_product[e31])
                    + (self[e43] * geometric_anti_product[scalar])
                    + (self[e23] * geometric_anti_product[e42])
                    + (self[e12] * geometric_anti_product[e1234]),
                -(self[e43] * geometric_anti_product[e12])
                    - (self[e23] * geometric_anti_product[e41])
                    - (self[e31] * geometric_anti_product[e42])
                    - (self[e12] * geometric_anti_product[e43]),
            ]) - (geometric_anti_product.group1().yzxx() * self.group0().zxy().with_w(self[e41]))
                - (self.group1().zxy() * geometric_anti_product.group0().yzx()).with_w(self[e42] * geometric_anti_product[e31]),
        );
    }
}
impl GeometricAntiQuotient<MultiVector> for Line {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       25       30        0
    //    simd2        3        4        0
    //    simd3        7       13        0
    //    simd4        3        3        0
    // Totals...
    // yes simd       38       50        0
    //  no simd       64       89        0
    fn geometric_anti_quotient(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        let other_2 = AntiScalar::from_groups(
            // e1234
            f32::powi(other[e1234], 2) + f32::powi(other[e423], 2) + f32::powi(other[e431], 2) + f32::powi(other[e412], 2)
                - f32::powi(other[e4], 2)
                - f32::powi(other[e41], 2)
                - f32::powi(other[e42], 2)
                - f32::powi(other[e43], 2),
        );
        let geometric_anti_product = MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from(other_2[e1234]) * other.group0(),
            // e1, e2, e3, e4
            Simd32x4::from(other_2[e1234]) * other.group1(),
            // e41, e42, e43
            Simd32x3::from(other_2[e1234]) * other.group2(),
            // e23, e31, e12
            Simd32x3::from(other_2[e1234]) * other.group3(),
            // e423, e431, e412, e321
            Simd32x4::from(other_2[e1234]) * other.group4(),
        );
        return MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([
                -(self[e23] * geometric_anti_product[e41]) - (self[e31] * geometric_anti_product[e42]) - (self[e12] * geometric_anti_product[e43]),
                0.0,
            ]) - (Simd32x2::from(self[e41]) * Simd32x2::from([geometric_anti_product[e23], geometric_anti_product[e41]]))
                - (Simd32x2::from(self[e42]) * Simd32x2::from([geometric_anti_product[e31], geometric_anti_product[e42]]))
                - (Simd32x2::from(self[e43]) * Simd32x2::from([geometric_anti_product[e12], geometric_anti_product[e43]])),
            // e1, e2, e3, e4
            Simd32x4::from([
                (self[e41] * geometric_anti_product[e321])
                    + (self[e42] * geometric_anti_product[e3])
                    + (self[e23] * geometric_anti_product[e4])
                    + (self[e31] * geometric_anti_product[e412]),
                (self[e42] * geometric_anti_product[e321])
                    + (self[e43] * geometric_anti_product[e1])
                    + (self[e31] * geometric_anti_product[e4])
                    + (self[e12] * geometric_anti_product[e423]),
                (self[e41] * geometric_anti_product[e2])
                    + (self[e43] * geometric_anti_product[e321])
                    + (self[e23] * geometric_anti_product[e431])
                    + (self[e12] * geometric_anti_product[e4]),
                self[e43] * geometric_anti_product[e412] * -1.0,
            ]) - (geometric_anti_product.group4().yzxy() * self.group1().zxy().with_w(self[e42]))
                - (self.group0().zxy() * geometric_anti_product.group1().yzx()).with_w(self[e41] * geometric_anti_product[e423]),
            // e41, e42, e43
            (Simd32x3::from(geometric_anti_product[e1234]) * self.group0()) + (self.group0().yzx() * geometric_anti_product.group2().zxy())
                - (self.group0().zxy() * geometric_anti_product.group2().yzx()),
            // e23, e31, e12
            (Simd32x3::from(geometric_anti_product[scalar]) * self.group0())
                + (Simd32x3::from(geometric_anti_product[e1234]) * self.group1())
                + (self.group0().yzx() * geometric_anti_product.group3().zxy())
                + (self.group1().yzx() * geometric_anti_product.group2().zxy())
                - (self.group0().zxy() * geometric_anti_product.group3().yzx())
                - (self.group1().zxy() * geometric_anti_product.group2().yzx()),
            // e423, e431, e412, e321
            Simd32x4::from([
                (self[e41] * geometric_anti_product[e4]) + (self[e42] * geometric_anti_product[e412]),
                (self[e42] * geometric_anti_product[e4]) + (self[e43] * geometric_anti_product[e423]),
                (self[e41] * geometric_anti_product[e431]) + (self[e43] * geometric_anti_product[e4]),
                -(self[e42] * geometric_anti_product[e2])
                    - (self[e43] * geometric_anti_product[e3])
                    - (self[e23] * geometric_anti_product[e423])
                    - (self[e31] * geometric_anti_product[e431])
                    - (self[e12] * geometric_anti_product[e412]),
            ]) - (self.group0().zxy() * geometric_anti_product.group4().yzx()).with_w(self[e41] * geometric_anti_product[e1]),
        );
    }
}
impl GeometricAntiQuotient<Origin> for Line {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        1
    //    simd4        0        4        0
    // Totals...
    // yes simd        0        5        1
    //  no simd        0       17        1
    fn geometric_anti_quotient(self, other: Origin) -> Self::Output {
        use crate::elements::*;
        let geometric_anti_product = Origin::from_groups(/* e4 */ 1.0 / other[e4] * -1.0);
        return Flector::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from([geometric_anti_product[e4], geometric_anti_product[e4], geometric_anti_product[e4], 0.0])
                * self.group1().with_w(0.0)
                * Simd32x4::from([1.0, 1.0, 1.0, 0.0]),
            // e423, e431, e412, e321
            Simd32x4::from([geometric_anti_product[e4], geometric_anti_product[e4], geometric_anti_product[e4], 0.0])
                * self.group0().with_w(0.0)
                * Simd32x4::from([1.0, 1.0, 1.0, 0.0]),
        );
    }
}
impl GeometricAntiQuotient<Plane> for Line {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        7       13        0
    //    simd4        2        3        0
    // Totals...
    // yes simd        9       16        0
    //  no simd       15       25        0
    fn geometric_anti_quotient(self, other: Plane) -> Self::Output {
        use crate::elements::*;
        let geometric_anti_product = Plane::from_groups(
            // e423, e431, e412, e321
            Simd32x4::from(f32::powi(other[e423], 2) + f32::powi(other[e431], 2) + f32::powi(other[e412], 2)) * other.group0(),
        );
        return Flector::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from([
                (self[e41] * geometric_anti_product[e321]) + (self[e31] * geometric_anti_product[e412]),
                (self[e42] * geometric_anti_product[e321]) + (self[e12] * geometric_anti_product[e423]),
                (self[e43] * geometric_anti_product[e321]) + (self[e23] * geometric_anti_product[e431]),
                -(self[e42] * geometric_anti_product[e431]) - (self[e43] * geometric_anti_product[e412]),
            ]) - (geometric_anti_product.group0().yzxx() * self.group1().zxy().with_w(self[e41])),
            // e423, e431, e412, e321
            Simd32x4::from([
                self[e42] * geometric_anti_product[e412],
                self[e43] * geometric_anti_product[e423],
                self[e41] * geometric_anti_product[e431],
                -(self[e31] * geometric_anti_product[e431]) - (self[e12] * geometric_anti_product[e412]),
            ]) - (geometric_anti_product.group0().yzxx() * self.group0().zxy().with_w(self[e23])),
        );
    }
}
impl GeometricAntiQuotient<Point> for Line {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        4        0
    //    simd3        0        3        0
    //    simd4        2        2        0
    // Totals...
    // yes simd        4        9        0
    //  no simd       10       21        0
    fn geometric_anti_quotient(self, other: Point) -> Self::Output {
        use crate::elements::*;
        let geometric_anti_product = Point::from_groups(/* e1, e2, e3, e4 */ Simd32x4::from(f32::powi(other[e4], -2) * -1.0) * other.group0());
        return Flector::from_groups(
            // e1, e2, e3, e4
            (Simd32x3::from(geometric_anti_product[e4]) * self.group1()).with_w(0.0) + (self.group0().yzx() * geometric_anti_product.group0().zxy()).with_w(0.0)
                - (self.group0().zxy() * geometric_anti_product.group0().yzx()).with_w(0.0),
            // e423, e431, e412, e321
            Simd32x4::from([geometric_anti_product[e4], geometric_anti_product[e4], geometric_anti_product[e4], 1.0])
                * self
                    .group0()
                    .with_w(-(self[e41] * geometric_anti_product[e1]) - (self[e42] * geometric_anti_product[e2]) - (self[e43] * geometric_anti_product[e3])),
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
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        0        1
    //    simd4        0        2        0
    // Totals...
    // yes simd        0        2        1
    //  no simd        0        8        1
    fn geometric_anti_quotient(self, other: AntiScalar) -> Self::Output {
        use crate::elements::*;
        let geometric_anti_product = AntiScalar::from_groups(/* e1234 */ 1.0 / other[e1234]);
        return Motor::from_groups(
            // e41, e42, e43, e1234
            Simd32x4::from(geometric_anti_product[e1234]) * self.group0(),
            // e23, e31, e12, scalar
            Simd32x4::from(geometric_anti_product[e1234]) * self.group1(),
        );
    }
}
impl GeometricAntiQuotient<DualNum> for Motor {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd2        0        1        0
    //    simd4        1        3        0
    // Totals...
    // yes simd        1        4        0
    //  no simd        4       14        0
    fn geometric_anti_quotient(self, other: DualNum) -> Self::Output {
        use crate::elements::*;
        let geometric_anti_product = DualNum::from_groups(/* scalar, e1234 */ Simd32x2::from(f32::powi(other[e1234], -2)) * other.group0());
        return Motor::from_groups(
            // e41, e42, e43, e1234
            Simd32x4::from(geometric_anti_product[e1234]) * self.group0(),
            // e23, e31, e12, scalar
            (Simd32x4::from(geometric_anti_product[scalar]) * self.group0()) + (Simd32x4::from(geometric_anti_product[e1234]) * self.group1()),
        );
    }
}
impl GeometricAntiQuotient<Flector> for Motor {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       19       25        0
    //    simd4        6        8        0
    // Totals...
    // yes simd       25       33        0
    //  no simd       43       57        0
    fn geometric_anti_quotient(self, other: Flector) -> Self::Output {
        use crate::elements::*;
        let other_2 = AntiScalar::from_groups(
            // e1234
            f32::powi(other[e423], 2) + f32::powi(other[e431], 2) + f32::powi(other[e412], 2) - f32::powi(other[e4], 2),
        );
        let geometric_anti_product = Flector::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from(other_2[e1234]) * other.group0(),
            // e423, e431, e412, e321
            Simd32x4::from(other_2[e1234]) * other.group1(),
        );
        return Flector::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from([
                (geometric_anti_product[e3] * self[e42])
                    + (geometric_anti_product[e4] * self[e23])
                    + (geometric_anti_product[e423] * self[scalar])
                    + (geometric_anti_product[e412] * self[e31])
                    + (geometric_anti_product[e321] * self[e41]),
                (geometric_anti_product[e2] * self[e1234])
                    + (geometric_anti_product[e4] * self[e31])
                    + (geometric_anti_product[e423] * self[e12])
                    + (geometric_anti_product[e431] * self[scalar])
                    + (geometric_anti_product[e321] * self[e42]),
                (geometric_anti_product[e3] * self[e1234])
                    + (geometric_anti_product[e4] * self[e12])
                    + (geometric_anti_product[e431] * self[e23])
                    + (geometric_anti_product[e412] * self[scalar])
                    + (geometric_anti_product[e321] * self[e43]),
                geometric_anti_product[e412] * self[e43] * -1.0,
            ]) + (geometric_anti_product.group0().xxyw() * self.group0().wzxw())
                - (geometric_anti_product.group1().yzxy() * self.group1().zxy().with_w(self[e42]))
                - (self.group0().zxyx() * geometric_anti_product.group0().yzx().with_w(geometric_anti_product[e423])),
            // e423, e431, e412, e321
            Simd32x4::from([
                geometric_anti_product[e412] * self[e42],
                geometric_anti_product[e431] * self[e1234],
                geometric_anti_product[e412] * self[e1234],
                -(geometric_anti_product[e2] * self[e42])
                    - (geometric_anti_product[e3] * self[e43])
                    - (geometric_anti_product[e423] * self[e23])
                    - (geometric_anti_product[e431] * self[e31])
                    - (geometric_anti_product[e412] * self[e12]),
            ]) + (Simd32x4::from(geometric_anti_product[e4]) * self.group0().xyz().with_w(self[scalar]))
                + (geometric_anti_product.group1().xxyw() * self.group0().wzxw())
                - (self.group0().zxyx() * geometric_anti_product.group1().yzx().with_w(geometric_anti_product[e1])),
        );
    }
}
impl GeometricAntiQuotient<Line> for Motor {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       18       25        0
    //    simd3        0        3        0
    //    simd4        3        2        0
    // Totals...
    // yes simd       21       30        0
    //  no simd       30       42        0
    fn geometric_anti_quotient(self, other: Line) -> Self::Output {
        use crate::elements::*;
        let other_2 = AntiScalar::from_groups(/* e1234 */ -f32::powi(other[e41], 2) - f32::powi(other[e42], 2) - f32::powi(other[e43], 2));
        let geometric_anti_product = Line::from_groups(
            // e41, e42, e43
            Simd32x3::from(other_2[e1234]) * other.group0(),
            // e23, e31, e12
            Simd32x3::from(other_2[e1234]) * other.group1(),
        );
        return Motor::from_groups(
            // e41, e42, e43, e1234
            Simd32x4::from([
                (geometric_anti_product[e41] * self[e1234]) + (geometric_anti_product[e43] * self[e42]),
                (geometric_anti_product[e41] * self[e43]) + (geometric_anti_product[e42] * self[e1234]),
                (geometric_anti_product[e42] * self[e41]) + (geometric_anti_product[e43] * self[e1234]),
                -(geometric_anti_product[e42] * self[e42]) - (geometric_anti_product[e43] * self[e43]),
            ]) - (self.group0().zxyx() * geometric_anti_product.group0().yzx().with_w(geometric_anti_product[e41])),
            // e23, e31, e12, scalar
            Simd32x4::from([
                (geometric_anti_product[e41] * self[scalar])
                    + (geometric_anti_product[e43] * self[e31])
                    + (geometric_anti_product[e23] * self[e1234])
                    + (geometric_anti_product[e12] * self[e42]),
                (geometric_anti_product[e41] * self[e12])
                    + (geometric_anti_product[e42] * self[scalar])
                    + (geometric_anti_product[e23] * self[e43])
                    + (geometric_anti_product[e31] * self[e1234]),
                (geometric_anti_product[e42] * self[e23])
                    + (geometric_anti_product[e43] * self[scalar])
                    + (geometric_anti_product[e31] * self[e41])
                    + (geometric_anti_product[e12] * self[e1234]),
                -(geometric_anti_product[e43] * self[e12])
                    - (geometric_anti_product[e23] * self[e41])
                    - (geometric_anti_product[e31] * self[e42])
                    - (geometric_anti_product[e12] * self[e43]),
            ]) - (self.group1().zxyx() * geometric_anti_product.group0().yzx().with_w(geometric_anti_product[e41]))
                - (geometric_anti_product.group1().yzx() * self.group0().zxy()).with_w(geometric_anti_product[e42] * self[e31]),
        );
    }
}
impl GeometricAntiQuotient<Motor> for Motor {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       19       26        0
    //    simd3        0        2        0
    //    simd4        6        6        0
    // Totals...
    // yes simd       25       34        0
    //  no simd       43       56        0
    fn geometric_anti_quotient(self, other: Motor) -> Self::Output {
        use crate::elements::*;
        let other_2 = AntiScalar::from_groups(
            // e1234
            f32::powi(other[e1234], 2) - f32::powi(other[e41], 2) - f32::powi(other[e42], 2) - f32::powi(other[e43], 2),
        );
        let geometric_anti_product = Motor::from_groups(
            // e41, e42, e43, e1234
            Simd32x4::from(other_2[e1234]) * other.group0(),
            // e23, e31, e12, scalar
            Simd32x4::from(other_2[e1234]) * other.group1(),
        );
        return Motor::from_groups(
            // e41, e42, e43, e1234
            Simd32x4::from([
                (geometric_anti_product[e43] * self[e42]) + (geometric_anti_product[e1234] * self[e41]),
                (geometric_anti_product[e42] * self[e1234]) + (geometric_anti_product[e1234] * self[e42]),
                (geometric_anti_product[e43] * self[e1234]) + (geometric_anti_product[e1234] * self[e43]),
                -(geometric_anti_product[e42] * self[e42]) - (geometric_anti_product[e43] * self[e43]),
            ]) + (geometric_anti_product.group0().xxyw() * self.group0().wzxw())
                - (geometric_anti_product.group0().yzxx() * self.group0().zxyx()),
            // e23, e31, e12, scalar
            Simd32x4::from([
                (geometric_anti_product[e1234] * self[e23])
                    + (geometric_anti_product[e23] * self[e1234])
                    + (geometric_anti_product[e12] * self[e42])
                    + (geometric_anti_product[scalar] * self[e41]),
                (geometric_anti_product[e1234] * self[e31])
                    + (geometric_anti_product[e23] * self[e43])
                    + (geometric_anti_product[e31] * self[e1234])
                    + (geometric_anti_product[scalar] * self[e42]),
                (geometric_anti_product[e1234] * self[e12])
                    + (geometric_anti_product[e31] * self[e41])
                    + (geometric_anti_product[e12] * self[e1234])
                    + (geometric_anti_product[scalar] * self[e43]),
                -(geometric_anti_product[e43] * self[e12])
                    - (geometric_anti_product[e23] * self[e41])
                    - (geometric_anti_product[e31] * self[e42])
                    - (geometric_anti_product[e12] * self[e43]),
            ]) + (geometric_anti_product.group0().xxyw() * self.group1().wzxw())
                + (geometric_anti_product.group0().zyz() * self.group1().yww()).with_w(geometric_anti_product[scalar] * self[e1234])
                - (geometric_anti_product.group0().yzxx() * self.group1().zxyx())
                - (geometric_anti_product.group1().yzx() * self.group0().zxy()).with_w(geometric_anti_product[e42] * self[e31]),
        );
    }
}
impl GeometricAntiQuotient<MultiVector> for Motor {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       26       29        0
    //    simd2        4        5        0
    //    simd3       10       14        0
    //    simd4        6        8        0
    // Totals...
    // yes simd       46       56        0
    //  no simd       88      113        0
    fn geometric_anti_quotient(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        let other_2 = AntiScalar::from_groups(
            // e1234
            f32::powi(other[e1234], 2) + f32::powi(other[e423], 2) + f32::powi(other[e431], 2) + f32::powi(other[e412], 2)
                - f32::powi(other[e4], 2)
                - f32::powi(other[e41], 2)
                - f32::powi(other[e42], 2)
                - f32::powi(other[e43], 2),
        );
        let geometric_anti_product = MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from(other_2[e1234]) * other.group0(),
            // e1, e2, e3, e4
            Simd32x4::from(other_2[e1234]) * other.group1(),
            // e41, e42, e43
            Simd32x3::from(other_2[e1234]) * other.group2(),
            // e23, e31, e12
            Simd32x3::from(other_2[e1234]) * other.group3(),
            // e423, e431, e412, e321
            Simd32x4::from(other_2[e1234]) * other.group4(),
        );
        return MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([
                (self[scalar] * geometric_anti_product[e1234])
                    - (self[e41] * geometric_anti_product[e23])
                    - (self[e42] * geometric_anti_product[e31])
                    - (self[e43] * geometric_anti_product[e12]),
                0.0,
            ]) + (Simd32x2::from(self[e1234]) * geometric_anti_product.group0())
                - (Simd32x2::from(geometric_anti_product[e41]) * Simd32x2::from([self[e23], self[e41]]))
                - (Simd32x2::from(geometric_anti_product[e42]) * Simd32x2::from([self[e31], self[e42]]))
                - (Simd32x2::from(geometric_anti_product[e43]) * Simd32x2::from([self[e12], self[e43]])),
            // e1, e2, e3, e4
            Simd32x4::from([
                (self[e42] * geometric_anti_product[e3])
                    + (self[e1234] * geometric_anti_product[e1])
                    + (self[e23] * geometric_anti_product[e4])
                    + (self[e31] * geometric_anti_product[e412])
                    + (self[scalar] * geometric_anti_product[e423]),
                (self[e43] * geometric_anti_product[e1])
                    + (self[e1234] * geometric_anti_product[e2])
                    + (self[e31] * geometric_anti_product[e4])
                    + (self[e12] * geometric_anti_product[e423])
                    + (self[scalar] * geometric_anti_product[e431]),
                (self[e43] * geometric_anti_product[e321])
                    + (self[e1234] * geometric_anti_product[e3])
                    + (self[e23] * geometric_anti_product[e431])
                    + (self[e12] * geometric_anti_product[e4])
                    + (self[scalar] * geometric_anti_product[e412]),
                self[e43] * geometric_anti_product[e412] * -1.0,
            ]) + (self.group0().xyxw() * geometric_anti_product.group4().ww().with_zw(geometric_anti_product[e2], geometric_anti_product[e4]))
                - (self.group0().zxyx() * geometric_anti_product.group1().yzx().with_w(geometric_anti_product[e423]))
                - (geometric_anti_product.group4().yzxy() * self.group1().zxy().with_w(self[e42])),
            // e41, e42, e43
            (Simd32x3::from(geometric_anti_product[e1234]) * self.group0().xyz())
                + (geometric_anti_product.group2().xxy() * self.group0().wzx())
                + (geometric_anti_product.group2().zyz() * self.group0().yww())
                - (geometric_anti_product.group2().yzx() * self.group0().zxy()),
            // e23, e31, e12
            (Simd32x3::from(geometric_anti_product[scalar]) * self.group0().xyz())
                + (Simd32x3::from(geometric_anti_product[e1234]) * self.group1().xyz())
                + (geometric_anti_product.group2().xxy() * self.group1().wzx())
                + (geometric_anti_product.group2().zyz() * self.group1().yww())
                + (geometric_anti_product.group3().xxy() * self.group0().wzx())
                + (geometric_anti_product.group3().zyz() * self.group0().yww())
                - (geometric_anti_product.group2().yzx() * self.group1().zxy())
                - (geometric_anti_product.group3().yzx() * self.group0().zxy()),
            // e423, e431, e412, e321
            Simd32x4::from([
                self[e1234] * geometric_anti_product[e423],
                self[e1234] * geometric_anti_product[e431],
                self[e1234] * geometric_anti_product[e412],
                -(self[e42] * geometric_anti_product[e2])
                    - (self[e43] * geometric_anti_product[e3])
                    - (self[e23] * geometric_anti_product[e423])
                    - (self[e31] * geometric_anti_product[e431])
                    - (self[e12] * geometric_anti_product[e412]),
            ]) + (self.group0().xyxw() * geometric_anti_product.group1().ww().with_zw(geometric_anti_product[e431], geometric_anti_product[e321]))
                + (geometric_anti_product.group4().zx().with_zw(geometric_anti_product[e4], geometric_anti_product[e4]) * self.group0().yzz().with_w(self[scalar]))
                - (self.group0().zxyx() * geometric_anti_product.group4().yzx().with_w(geometric_anti_product[e1])),
        );
    }
}
impl GeometricAntiQuotient<Origin> for Motor {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        1
    //    simd4        0        2        0
    // Totals...
    // yes simd        0        3        1
    //  no simd        0        9        1
    fn geometric_anti_quotient(self, other: Origin) -> Self::Output {
        use crate::elements::*;
        let geometric_anti_product = Origin::from_groups(/* e4 */ 1.0 / other[e4] * -1.0);
        return Flector::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from(geometric_anti_product[e4]) * self.group1().xyz().with_w(self[e1234]),
            // e423, e431, e412, e321
            Simd32x4::from(geometric_anti_product[e4]) * self.group0().xyz().with_w(self[scalar]),
        );
    }
}
impl GeometricAntiQuotient<Plane> for Motor {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       10       16        0
    //    simd4        3        4        0
    // Totals...
    // yes simd       13       20        0
    //  no simd       22       32        0
    fn geometric_anti_quotient(self, other: Plane) -> Self::Output {
        use crate::elements::*;
        let geometric_anti_product = Plane::from_groups(
            // e423, e431, e412, e321
            Simd32x4::from(f32::powi(other[e423], 2) + f32::powi(other[e431], 2) + f32::powi(other[e412], 2)) * other.group0(),
        );
        return Flector::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from([
                (self[e41] * geometric_anti_product[e321]) + (self[e31] * geometric_anti_product[e412]) + (self[scalar] * geometric_anti_product[e423]),
                (self[e42] * geometric_anti_product[e321]) + (self[e12] * geometric_anti_product[e423]) + (self[scalar] * geometric_anti_product[e431]),
                (self[e43] * geometric_anti_product[e321]) + (self[e23] * geometric_anti_product[e431]) + (self[scalar] * geometric_anti_product[e412]),
                -(self[e42] * geometric_anti_product[e431]) - (self[e43] * geometric_anti_product[e412]),
            ]) - (geometric_anti_product.group0().yzxx() * self.group1().zxy().with_w(self[e41])),
            // e423, e431, e412, e321
            Simd32x4::from([
                self[e1234] * geometric_anti_product[e423],
                self[e1234] * geometric_anti_product[e431],
                self[e1234] * geometric_anti_product[e412],
                -(self[e31] * geometric_anti_product[e431]) - (self[e12] * geometric_anti_product[e412]),
            ]) + (self.group0().yzxw() * geometric_anti_product.group0().zxyw())
                - (geometric_anti_product.group0().yzxx() * self.group0().zxy().with_w(self[e23])),
        );
    }
}
impl GeometricAntiQuotient<Point> for Motor {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        6        0
    //    simd3        3        4        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        6       12        0
    //  no simd       12       26        0
    fn geometric_anti_quotient(self, other: Point) -> Self::Output {
        use crate::elements::*;
        let geometric_anti_product = Point::from_groups(/* e1, e2, e3, e4 */ Simd32x4::from(f32::powi(other[e4], -2) * -1.0) * other.group0());
        return Flector::from_groups(
            // e1, e2, e3, e4
            ((Simd32x3::from(self[e1234]) * geometric_anti_product.group0().xyz())
                + (Simd32x3::from(geometric_anti_product[e4]) * self.group1().xyz())
                + (self.group0().yzx() * geometric_anti_product.group0().zxy())
                - (self.group0().zxy() * geometric_anti_product.group0().yzx()))
            .with_w(self[e1234] * geometric_anti_product[e4]),
            // e423, e431, e412, e321
            Simd32x4::from([geometric_anti_product[e4], geometric_anti_product[e4], geometric_anti_product[e4], 1.0])
                * self.group0().xyz().with_w(
                    (self[scalar] * geometric_anti_product[e4])
                        - (self[e41] * geometric_anti_product[e1])
                        - (self[e42] * geometric_anti_product[e2])
                        - (self[e43] * geometric_anti_product[e3]),
                ),
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
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        0        1
    //    simd2        0        1        0
    //    simd3        0        2        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        0        5        1
    //  no simd        0       16        1
    fn geometric_anti_quotient(self, other: AntiScalar) -> Self::Output {
        use crate::elements::*;
        let geometric_anti_product = AntiScalar::from_groups(/* e1234 */ 1.0 / other[e1234]);
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
impl GeometricAntiQuotient<DualNum> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        6        0
    //    simd2        0        1        0
    //    simd3        2        5        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        4       13        0
    //  no simd        8       27        0
    fn geometric_anti_quotient(self, other: DualNum) -> Self::Output {
        use crate::elements::*;
        let geometric_anti_product = DualNum::from_groups(/* scalar, e1234 */ Simd32x2::from(f32::powi(other[e1234], -2)) * other.group0());
        return MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([
                (geometric_anti_product[scalar] * self[e1234]) + (geometric_anti_product[e1234] * self[scalar]),
                geometric_anti_product[e1234] * self[e1234],
            ]),
            // e1, e2, e3, e4
            ((Simd32x3::from(geometric_anti_product[e1234]) * self.group1().xyz()) - (Simd32x3::from(geometric_anti_product[scalar]) * self.group4().xyz()))
                .with_w(geometric_anti_product[e1234] * self[e4]),
            // e41, e42, e43
            Simd32x3::from(geometric_anti_product[e1234]) * self.group2(),
            // e23, e31, e12
            (Simd32x3::from(geometric_anti_product[scalar]) * self.group2()) + (Simd32x3::from(geometric_anti_product[e1234]) * self.group3()),
            // e423, e431, e412, e321
            Simd32x4::from([self[e423], self[e431], self[e412], 1.0])
                * geometric_anti_product.group0().yy().with_zw(
                    geometric_anti_product[e1234],
                    (geometric_anti_product[e1234] * self[e321]) - (geometric_anti_product[scalar] * self[e4]),
                ),
        );
    }
}
impl GeometricAntiQuotient<Flector> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       22       31        0
    //    simd2        4        4        0
    //    simd3       10       14        0
    //    simd4        6        6        0
    // Totals...
    // yes simd       42       55        0
    //  no simd       84      105        0
    fn geometric_anti_quotient(self, other: Flector) -> Self::Output {
        use crate::elements::*;
        let other_2 = AntiScalar::from_groups(
            // e1234
            f32::powi(other[e423], 2) + f32::powi(other[e431], 2) + f32::powi(other[e412], 2) - f32::powi(other[e4], 2),
        );
        let geometric_anti_product = Flector::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from(other_2[e1234]) * other.group0(),
            // e423, e431, e412, e321
            Simd32x4::from(other_2[e1234]) * other.group1(),
        );
        return MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([
                (geometric_anti_product[e321] * self[e4])
                    - (geometric_anti_product[e2] * self[e431])
                    - (geometric_anti_product[e3] * self[e412])
                    - (geometric_anti_product[e4] * self[e321]),
                0.0,
            ]) + (Simd32x2::from(geometric_anti_product[e423]) * Simd32x2::from([self[e1], self[e423]]))
                + (Simd32x2::from(geometric_anti_product[e431]) * Simd32x2::from([self[e2], self[e431]]))
                + (Simd32x2::from(geometric_anti_product[e412]) * Simd32x2::from([self[e3], self[e412]]))
                - (Simd32x2::from([self[e423], self[e4]]) * geometric_anti_product.group0().xw()),
            // e1, e2, e3, e4
            Simd32x4::from([
                (geometric_anti_product[e1] * self[e1234])
                    + (geometric_anti_product[e3] * self[e42])
                    + (geometric_anti_product[e4] * self[e23])
                    + (geometric_anti_product[e412] * self[e31])
                    + (geometric_anti_product[e321] * self[e41]),
                (geometric_anti_product[e1] * self[e43])
                    + (geometric_anti_product[e2] * self[e1234])
                    + (geometric_anti_product[e4] * self[e31])
                    + (geometric_anti_product[e423] * self[e12])
                    + (geometric_anti_product[e321] * self[e42]),
                (geometric_anti_product[e2] * self[e41])
                    + (geometric_anti_product[e3] * self[e1234])
                    + (geometric_anti_product[e4] * self[e12])
                    + (geometric_anti_product[e431] * self[e23])
                    + (geometric_anti_product[e321] * self[e43]),
                geometric_anti_product[e412] * self[e43] * -1.0,
            ]) + (self.group0().xx().with_zw(self[scalar], self[e1234]) * geometric_anti_product.group1().xyz().with_w(geometric_anti_product[e4]))
                - (geometric_anti_product.group1().yzxy() * self.group3().zxy().with_w(self[e42]))
                - (self.group2().zxy() * geometric_anti_product.group0().yzx()).with_w(geometric_anti_product[e423] * self[e41]),
            // e41, e42, e43
            (geometric_anti_product.group1().yzx() * self.group4().zxy())
                - (Simd32x3::from(geometric_anti_product[e4]) * self.group4().xyz())
                - (Simd32x3::from([self[e4], self[e412], self[e423]]) * geometric_anti_product.group1().xxy())
                - (Simd32x3::from([self[e431], self[e4], self[e4]]) * geometric_anti_product.group1().zyz()),
            // e23, e31, e12
            (Simd32x3::from(geometric_anti_product[e321]) * self.group4().xyz())
                + (Simd32x3::from([self[e4], self[e412], self[e423]]) * geometric_anti_product.group0().xxy())
                + (Simd32x3::from([self[e431], self[e4], self[e4]]) * geometric_anti_product.group0().zyz())
                + (geometric_anti_product.group1().yzx() * self.group1().zxy())
                - (Simd32x3::from(geometric_anti_product[e4]) * self.group1().xyz())
                - (Simd32x3::from([self[e2], self[e321], self[e321]]) * geometric_anti_product.group1().zyz())
                - (Simd32x3::from([self[e321], self[e3], self[e1]]) * geometric_anti_product.group1().xxy())
                - (geometric_anti_product.group0().yzx() * self.group4().zxy()),
            // e423, e431, e412, e321
            Simd32x4::from([
                geometric_anti_product[e412] * self[e42],
                geometric_anti_product[e423] * self[e43],
                geometric_anti_product[e4] * self[e43],
                -(geometric_anti_product[e2] * self[e42])
                    - (geometric_anti_product[e3] * self[e43])
                    - (geometric_anti_product[e423] * self[e23])
                    - (geometric_anti_product[e431] * self[e31])
                    - (geometric_anti_product[e412] * self[e12]),
            ]) + (self.group0().yy().with_zw(self[e1234], self[scalar]) * geometric_anti_product.group1().xyz().with_w(geometric_anti_product[e4]))
                + (geometric_anti_product.group0().ww().with_zw(geometric_anti_product[e431], geometric_anti_product[e321]) * self.group2().xyx().with_w(self[e1234]))
                - (self.group2().zxy() * geometric_anti_product.group1().yzx()).with_w(geometric_anti_product[e1] * self[e41]),
        );
    }
}
impl GeometricAntiQuotient<Line> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       12       17        0
    //    simd2        3        3        0
    //    simd3        7       13        0
    //    simd4        6        4        0
    // Totals...
    // yes simd       28       37        0
    //  no simd       63       78        0
    fn geometric_anti_quotient(self, other: Line) -> Self::Output {
        use crate::elements::*;
        let other_2 = AntiScalar::from_groups(/* e1234 */ -f32::powi(other[e41], 2) - f32::powi(other[e42], 2) - f32::powi(other[e43], 2));
        let geometric_anti_product = Line::from_groups(
            // e41, e42, e43
            Simd32x3::from(other_2[e1234]) * other.group0(),
            // e23, e31, e12
            Simd32x3::from(other_2[e1234]) * other.group1(),
        );
        return MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([
                -(geometric_anti_product[e23] * self[e41]) - (geometric_anti_product[e31] * self[e42]) - (geometric_anti_product[e12] * self[e43]),
                0.0,
            ]) - (Simd32x2::from(geometric_anti_product[e41]) * Simd32x2::from([self[e23], self[e41]]))
                - (Simd32x2::from(geometric_anti_product[e42]) * Simd32x2::from([self[e31], self[e42]]))
                - (Simd32x2::from(geometric_anti_product[e43]) * Simd32x2::from([self[e12], self[e43]])),
            // e1, e2, e3, e4
            Simd32x4::from([
                (geometric_anti_product[e41] * self[e321]) + (geometric_anti_product[e43] * self[e2]) + (geometric_anti_product[e31] * self[e412]),
                (geometric_anti_product[e41] * self[e3]) + (geometric_anti_product[e42] * self[e321]) + (geometric_anti_product[e12] * self[e423]),
                (geometric_anti_product[e42] * self[e1]) + (geometric_anti_product[e43] * self[e321]) + (geometric_anti_product[e23] * self[e431]),
                0.0,
            ]) - (Simd32x4::from([self[e4], self[e412], self[e423], self[e431]]) * geometric_anti_product.group1().xxy().with_w(geometric_anti_product[e42]))
                - (Simd32x4::from([self[e431], self[e4], self[e4], self[e412]]) * geometric_anti_product.group1().zyz().with_w(geometric_anti_product[e43]))
                - (geometric_anti_product.group0().yzx() * self.group1().zxy()).with_w(geometric_anti_product[e41] * self[e423]),
            // e41, e42, e43
            (Simd32x3::from(self[e1234]) * geometric_anti_product.group0()) + (geometric_anti_product.group0().zxy() * self.group2().yzx())
                - (geometric_anti_product.group0().yzx() * self.group2().zxy()),
            // e23, e31, e12
            (Simd32x3::from(self[scalar]) * geometric_anti_product.group0())
                + (Simd32x3::from(self[e1234]) * geometric_anti_product.group1())
                + (geometric_anti_product.group0().zxy() * self.group3().yzx())
                + (geometric_anti_product.group1().zxy() * self.group2().yzx())
                - (geometric_anti_product.group0().yzx() * self.group3().zxy())
                - (geometric_anti_product.group1().yzx() * self.group2().zxy()),
            // e423, e431, e412, e321
            (Simd32x4::from([self[e4], self[e412], self[e423], self[e423]]) * geometric_anti_product.group0().xxy().with_w(geometric_anti_product[e23]))
                + (Simd32x4::from([self[e431], self[e4], self[e4], self[e431]]) * geometric_anti_product.group0().zyz().with_w(geometric_anti_product[e31]))
                + Simd32x3::from(0.0).with_w((geometric_anti_product[e12] * self[e412]) - (geometric_anti_product[e42] * self[e2]) - (geometric_anti_product[e43] * self[e3]))
                - (geometric_anti_product.group0().yzx() * self.group4().zxy()).with_w(geometric_anti_product[e41] * self[e1]),
        );
    }
}
impl GeometricAntiQuotient<Motor> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       18       20        0
    //    simd2        4        4        0
    //    simd3       10       12        0
    //    simd4        8       10        0
    // Totals...
    // yes simd       40       46        0
    //  no simd       88      104        0
    fn geometric_anti_quotient(self, other: Motor) -> Self::Output {
        use crate::elements::*;
        let other_2 = AntiScalar::from_groups(
            // e1234
            f32::powi(other[e1234], 2) - f32::powi(other[e41], 2) - f32::powi(other[e42], 2) - f32::powi(other[e43], 2),
        );
        let geometric_anti_product = Motor::from_groups(
            // e41, e42, e43, e1234
            Simd32x4::from(other_2[e1234]) * other.group0(),
            // e23, e31, e12, scalar
            Simd32x4::from(other_2[e1234]) * other.group1(),
        );
        return MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([
                (geometric_anti_product[scalar] * self[e1234])
                    - (geometric_anti_product[e41] * self[e23])
                    - (geometric_anti_product[e42] * self[e31])
                    - (geometric_anti_product[e43] * self[e12]),
                0.0,
            ]) + (Simd32x2::from(geometric_anti_product[e1234]) * self.group0())
                - (Simd32x2::from(self[e41]) * Simd32x2::from([geometric_anti_product[e23], geometric_anti_product[e41]]))
                - (Simd32x2::from(self[e42]) * Simd32x2::from([geometric_anti_product[e31], geometric_anti_product[e42]]))
                - (Simd32x2::from(self[e43]) * Simd32x2::from([geometric_anti_product[e12], geometric_anti_product[e43]])),
            // e1, e2, e3, e4
            Simd32x4::from([
                (geometric_anti_product[e43] * self[e2]) + (geometric_anti_product[e1234] * self[e1]) + (geometric_anti_product[e31] * self[e412])
                    - (geometric_anti_product[scalar] * self[e423]),
                (geometric_anti_product[e42] * self[e321]) + (geometric_anti_product[e1234] * self[e2]) + (geometric_anti_product[e12] * self[e423])
                    - (geometric_anti_product[scalar] * self[e431]),
                (geometric_anti_product[e43] * self[e321]) + (geometric_anti_product[e1234] * self[e3]) + (geometric_anti_product[e23] * self[e431])
                    - (geometric_anti_product[scalar] * self[e412]),
                0.0,
            ]) + (Simd32x4::from([self[e321], self[e3], self[e1], self[e4]]) * geometric_anti_product.group0().xxyw())
                - (Simd32x4::from([self[e4], self[e412], self[e423], self[e431]]) * geometric_anti_product.group1().xxy().with_w(geometric_anti_product[e42]))
                - (Simd32x4::from([self[e431], self[e4], self[e4], self[e412]]) * geometric_anti_product.group1().zyz().with_w(geometric_anti_product[e43]))
                - (geometric_anti_product.group0().yzxx() * self.group1().zxy().with_w(self[e423])),
            // e41, e42, e43
            (Simd32x3::from(self[e1234]) * geometric_anti_product.group0().xyz())
                + (self.group2().xyx() * geometric_anti_product.group0().wwy())
                + (self.group2().yzz() * geometric_anti_product.group0().zxw())
                - (self.group2().zxy() * geometric_anti_product.group0().yzx()),
            // e23, e31, e12
            (Simd32x3::from(self[scalar]) * geometric_anti_product.group0().xyz())
                + (Simd32x3::from(self[e1234]) * geometric_anti_product.group1().xyz())
                + (self.group2().xyx() * geometric_anti_product.group1().wwy())
                + (self.group2().yzz() * geometric_anti_product.group1().zxw())
                + (self.group3().xyx() * geometric_anti_product.group0().wwy())
                + (self.group3().yzz() * geometric_anti_product.group0().zxw())
                - (self.group2().zxy() * geometric_anti_product.group1().yzx())
                - (self.group3().zxy() * geometric_anti_product.group0().yzx()),
            // e423, e431, e412, e321
            (Simd32x4::from([self[e4], self[e412], self[e423], self[e321]]) * geometric_anti_product.group0().xxyw())
                + (Simd32x4::from([self[e431], self[e4], self[e4], self[e423]]) * geometric_anti_product.group0().zyz().with_w(geometric_anti_product[e23]))
                + (self.group4().xyzy() * geometric_anti_product.group0().www().with_w(geometric_anti_product[e31]))
                + Simd32x3::from(0.0).with_w(
                    (geometric_anti_product[e12] * self[e412])
                        - (geometric_anti_product[e42] * self[e2])
                        - (geometric_anti_product[e43] * self[e3])
                        - (geometric_anti_product[scalar] * self[e4]),
                )
                - (geometric_anti_product.group0().yzxx() * self.group4().zxy().with_w(self[e1])),
        );
    }
}
impl GeometricAntiQuotient<MultiVector> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       42       44        0
    //    simd2        8        9        0
    //    simd3       22       30        0
    //    simd4       16       14        0
    // Totals...
    // yes simd       88       97        0
    //  no simd      188      208        0
    fn geometric_anti_quotient(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        let other_2 = AntiScalar::from_groups(
            // e1234
            f32::powi(other[e1234], 2) + f32::powi(other[e423], 2) + f32::powi(other[e431], 2) + f32::powi(other[e412], 2)
                - f32::powi(other[e4], 2)
                - f32::powi(other[e41], 2)
                - f32::powi(other[e42], 2)
                - f32::powi(other[e43], 2),
        );
        let geometric_anti_product = MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from(other_2[e1234]) * other.group0(),
            // e1, e2, e3, e4
            Simd32x4::from(other_2[e1234]) * other.group1(),
            // e41, e42, e43
            Simd32x3::from(other_2[e1234]) * other.group2(),
            // e23, e31, e12
            Simd32x3::from(other_2[e1234]) * other.group3(),
            // e423, e431, e412, e321
            Simd32x4::from(other_2[e1234]) * other.group4(),
        );
        return MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([
                (geometric_anti_product[e1234] * self[scalar]) + (geometric_anti_product[e321] * self[e4])
                    - (geometric_anti_product[e2] * self[e431])
                    - (geometric_anti_product[e3] * self[e412])
                    - (geometric_anti_product[e4] * self[e321])
                    - (geometric_anti_product[e23] * self[e41])
                    - (geometric_anti_product[e31] * self[e42])
                    - (geometric_anti_product[e12] * self[e43]),
                0.0,
            ]) + (Simd32x2::from(geometric_anti_product[e423]) * Simd32x2::from([self[e1], self[e423]]))
                + (Simd32x2::from(geometric_anti_product[e431]) * Simd32x2::from([self[e2], self[e431]]))
                + (Simd32x2::from(geometric_anti_product[e412]) * Simd32x2::from([self[e3], self[e412]]))
                + (Simd32x2::from(self[e1234]) * geometric_anti_product.group0())
                - (Simd32x2::from(geometric_anti_product[e41]) * Simd32x2::from([self[e23], self[e41]]))
                - (Simd32x2::from(geometric_anti_product[e42]) * Simd32x2::from([self[e31], self[e42]]))
                - (Simd32x2::from(geometric_anti_product[e43]) * Simd32x2::from([self[e12], self[e43]]))
                - (Simd32x2::from([self[e423], self[e4]]) * geometric_anti_product.group1().xw()),
            // e1, e2, e3, e4
            Simd32x4::from([
                (geometric_anti_product[e1] * self[e1234])
                    + (geometric_anti_product[e3] * self[e42])
                    + (geometric_anti_product[e4] * self[e23])
                    + (geometric_anti_product[e41] * self[e321])
                    + (geometric_anti_product[e43] * self[e2])
                    + (geometric_anti_product[e31] * self[e412])
                    + (geometric_anti_product[e412] * self[e31])
                    + (geometric_anti_product[e321] * self[e41]),
                (geometric_anti_product[e1] * self[e43])
                    + (geometric_anti_product[e2] * self[e1234])
                    + (geometric_anti_product[e4] * self[e31])
                    + (geometric_anti_product[e41] * self[e3])
                    + (geometric_anti_product[e42] * self[e321])
                    + (geometric_anti_product[e12] * self[e423])
                    + (geometric_anti_product[e423] * self[e12])
                    + (geometric_anti_product[e321] * self[e42]),
                (geometric_anti_product[e2] * self[e41])
                    + (geometric_anti_product[e3] * self[e1234])
                    + (geometric_anti_product[e4] * self[e12])
                    + (geometric_anti_product[e42] * self[e1])
                    + (geometric_anti_product[e43] * self[e321])
                    + (geometric_anti_product[e23] * self[e431])
                    + (geometric_anti_product[e431] * self[e23])
                    + (geometric_anti_product[e321] * self[e43]),
                0.0,
            ]) + (Simd32x4::from(geometric_anti_product[e1234]) * self.group1())
                + (self.group0().xx().with_zw(self[scalar], self[e1234]) * geometric_anti_product.group4().xyz().with_w(geometric_anti_product[e4]))
                - (Simd32x4::from([self[e4], self[e412], self[e423], self[e412]]) * geometric_anti_product.group3().xxy().with_w(geometric_anti_product[e43]))
                - (Simd32x4::from([self[e431], self[e4], self[e4], geometric_anti_product[e423]]) * geometric_anti_product.group3().zyz().with_w(self[e41]))
                - (geometric_anti_product.group4().yzxz() * self.group3().zxy().with_w(self[e43]))
                - (self.group4().xyzx() * geometric_anti_product.group0().xx().with_zw(geometric_anti_product[scalar], geometric_anti_product[e41]))
                - (geometric_anti_product.group2().yzx() * self.group1().zxy()).with_w(geometric_anti_product[e42] * self[e431])
                - (self.group2().zxy() * geometric_anti_product.group1().yzx()).with_w(geometric_anti_product[e431] * self[e42]),
            // e41, e42, e43
            (Simd32x3::from(geometric_anti_product[e1234]) * self.group2())
                + (Simd32x3::from(self[e1234]) * geometric_anti_product.group2())
                + (geometric_anti_product.group2().zxy() * self.group2().yzx())
                + (geometric_anti_product.group4().yzx() * self.group4().zxy())
                - (Simd32x3::from(geometric_anti_product[e4]) * self.group4().xyz())
                - (Simd32x3::from([self[e4], self[e412], self[e423]]) * geometric_anti_product.group4().xxy())
                - (Simd32x3::from([self[e431], self[e4], self[e4]]) * geometric_anti_product.group4().zyz())
                - (geometric_anti_product.group2().yzx() * self.group2().zxy()),
            // e23, e31, e12
            (Simd32x3::from(geometric_anti_product[scalar]) * self.group2())
                + (Simd32x3::from(geometric_anti_product[e1234]) * self.group3())
                + (Simd32x3::from(geometric_anti_product[e321]) * self.group4().xyz())
                + (Simd32x3::from(self[scalar]) * geometric_anti_product.group2())
                + (Simd32x3::from(self[e1234]) * geometric_anti_product.group3())
                + (Simd32x3::from([self[e4], self[e412], self[e423]]) * geometric_anti_product.group1().xxy())
                + (Simd32x3::from([self[e431], self[e4], self[e4]]) * geometric_anti_product.group1().zyz())
                + (geometric_anti_product.group2().zxy() * self.group3().yzx())
                + (geometric_anti_product.group3().zxy() * self.group2().yzx())
                + (geometric_anti_product.group4().yzx() * self.group1().zxy())
                - (Simd32x3::from(geometric_anti_product[e4]) * self.group1().xyz())
                - (Simd32x3::from([self[e2], self[e321], self[e321]]) * geometric_anti_product.group4().zyz())
                - (Simd32x3::from([self[e321], self[e3], self[e1]]) * geometric_anti_product.group4().xxy())
                - (geometric_anti_product.group2().yzx() * self.group3().zxy())
                - (geometric_anti_product.group3().yzx() * self.group2().zxy())
                - (geometric_anti_product.group1().yzx() * self.group4().zxy()),
            // e423, e431, e412, e321
            (Simd32x4::from(geometric_anti_product[e1234]) * self.group4())
                + (Simd32x4::from([self[e4], self[e412], self[e423], geometric_anti_product[e321]]) * geometric_anti_product.group2().xxy().with_w(self[e1234]))
                + (Simd32x4::from([self[e431], self[e4], self[e4], self[e423]]) * geometric_anti_product.group2().zyz().with_w(geometric_anti_product[e23]))
                + (self.group0().yy().with_zw(self[e1234], self[scalar]) * geometric_anti_product.group4().xyz().with_w(geometric_anti_product[e4]))
                + (geometric_anti_product.group1().ww().with_zw(geometric_anti_product[e431], self[e431]) * self.group2().xyx().with_w(geometric_anti_product[e31]))
                + (geometric_anti_product.group4().zx().with_zw(geometric_anti_product[e4], self[e412]) * self.group2().yzz().with_w(geometric_anti_product[e12]))
                + Simd32x3::from(0.0).with_w(
                    -(geometric_anti_product[e1] * self[e41])
                        - (geometric_anti_product[e2] * self[e42])
                        - (geometric_anti_product[e3] * self[e43])
                        - (geometric_anti_product[e42] * self[e2])
                        - (geometric_anti_product[e43] * self[e3])
                        - (geometric_anti_product[e423] * self[e23])
                        - (geometric_anti_product[e431] * self[e31])
                        - (geometric_anti_product[e412] * self[e12]),
                )
                - (geometric_anti_product.group2().yzx() * self.group4().zxy()).with_w(geometric_anti_product[scalar] * self[e4])
                - (self.group2().zxy() * geometric_anti_product.group4().yzx()).with_w(geometric_anti_product[e41] * self[e1]),
        );
    }
}
impl GeometricAntiQuotient<Origin> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        1
    //    simd2        0        2        0
    //    simd3        0        4        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        0        9        1
    //  no simd        0       25        1
    fn geometric_anti_quotient(self, other: Origin) -> Self::Output {
        use crate::elements::*;
        let geometric_anti_product = Origin::from_groups(/* e4 */ 1.0 / other[e4] * -1.0);
        return MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from(geometric_anti_product[e4]) * Simd32x2::from([self[e321], self[e4]]) * Simd32x2::from(-1.0),
            // e1, e2, e3, e4
            Simd32x4::from(geometric_anti_product[e4]) * self.group3().with_w(self[e1234]),
            // e41, e42, e43
            Simd32x3::from(geometric_anti_product[e4]) * self.group4().xyz() * Simd32x3::from(-1.0),
            // e23, e31, e12
            Simd32x3::from(geometric_anti_product[e4]) * self.group1().xyz() * Simd32x3::from(-1.0),
            // e423, e431, e412, e321
            Simd32x4::from(geometric_anti_product[e4]) * self.group2().with_w(self[scalar]),
        );
    }
}
impl GeometricAntiQuotient<Plane> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       10       17        0
    //    simd2        3        3        0
    //    simd3        5        7        0
    //    simd4        3        4        0
    // Totals...
    // yes simd       21       31        0
    //  no simd       43       60        0
    fn geometric_anti_quotient(self, other: Plane) -> Self::Output {
        use crate::elements::*;
        let geometric_anti_product = Plane::from_groups(
            // e423, e431, e412, e321
            Simd32x4::from(f32::powi(other[e423], 2) + f32::powi(other[e431], 2) + f32::powi(other[e412], 2)) * other.group0(),
        );
        return MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([self[e4] * geometric_anti_product[e321], 0.0])
                + (Simd32x2::from(geometric_anti_product[e423]) * Simd32x2::from([self[e1], self[e423]]))
                + (Simd32x2::from(geometric_anti_product[e431]) * Simd32x2::from([self[e2], self[e431]]))
                + (Simd32x2::from(geometric_anti_product[e412]) * Simd32x2::from([self[e3], self[e412]])),
            // e1, e2, e3, e4
            Simd32x4::from([
                (self[scalar] * geometric_anti_product[e423]) + (self[e41] * geometric_anti_product[e321]) + (self[e31] * geometric_anti_product[e412]),
                (self[scalar] * geometric_anti_product[e431]) + (self[e42] * geometric_anti_product[e321]) + (self[e12] * geometric_anti_product[e423]),
                (self[scalar] * geometric_anti_product[e412]) + (self[e43] * geometric_anti_product[e321]) + (self[e23] * geometric_anti_product[e431]),
                -(self[e42] * geometric_anti_product[e431]) - (self[e43] * geometric_anti_product[e412]),
            ]) - (geometric_anti_product.group0().yzxx() * self.group3().zxy().with_w(self[e41])),
            // e41, e42, e43
            (self.group4().zxy() * geometric_anti_product.group0().yzx())
                - (Simd32x3::from(self[e4]) * geometric_anti_product.group0().xyz())
                - (self.group4().yzx() * geometric_anti_product.group0().zxy()),
            // e23, e31, e12
            (Simd32x3::from(geometric_anti_product[e321]) * self.group4().xyz()) + (self.group1().zxy() * geometric_anti_product.group0().yzx())
                - (Simd32x3::from(self[e321]) * geometric_anti_product.group0().xyz())
                - (self.group1().yzx() * geometric_anti_product.group0().zxy()),
            // e423, e431, e412, e321
            Simd32x4::from([
                self[e42] * geometric_anti_product[e412],
                self[e43] * geometric_anti_product[e423],
                self[e41] * geometric_anti_product[e431],
                -(self[e31] * geometric_anti_product[e431]) - (self[e12] * geometric_anti_product[e412]),
            ]) + (Simd32x4::from(self[e1234]) * geometric_anti_product.group0())
                - (geometric_anti_product.group0().yzxx() * self.group2().zxy().with_w(self[e23])),
        );
    }
}
impl GeometricAntiQuotient<Point> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        6       11        0
    //    simd2        0        1        0
    //    simd3        6       10        0
    //    simd4        0        2        0
    // Totals...
    // yes simd       12       24        0
    //  no simd       24       51        0
    fn geometric_anti_quotient(self, other: Point) -> Self::Output {
        use crate::elements::*;
        let geometric_anti_product = Point::from_groups(/* e1, e2, e3, e4 */ Simd32x4::from(f32::powi(other[e4], -2) * -1.0) * other.group0());
        return MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([
                -(self[e423] * geometric_anti_product[e1])
                    - (self[e431] * geometric_anti_product[e2])
                    - (self[e412] * geometric_anti_product[e3])
                    - (self[e321] * geometric_anti_product[e4]),
                self[e4] * geometric_anti_product[e4],
            ]) * Simd32x2::from([1.0, -1.0]),
            // e1, e2, e3, e4
            ((Simd32x3::from(self[e1234]) * geometric_anti_product.group0().xyz())
                + (Simd32x3::from(geometric_anti_product[e4]) * self.group3())
                + (self.group2().yzx() * geometric_anti_product.group0().zxy())
                - (self.group2().zxy() * geometric_anti_product.group0().yzx()))
            .with_w(self[e1234] * geometric_anti_product[e4]),
            // e41, e42, e43
            Simd32x3::from(geometric_anti_product[e4]) * self.group4().xyz() * Simd32x3::from(-1.0),
            // e23, e31, e12
            (Simd32x3::from(self[e4]) * geometric_anti_product.group0().xyz()) + (self.group4().yzx() * geometric_anti_product.group0().zxy())
                - (Simd32x3::from(geometric_anti_product[e4]) * self.group1().xyz())
                - (self.group4().zxy() * geometric_anti_product.group0().yzx()),
            // e423, e431, e412, e321
            Simd32x4::from([geometric_anti_product[e4], geometric_anti_product[e4], geometric_anti_product[e4], 1.0])
                * self.group2().with_w(
                    (self[scalar] * geometric_anti_product[e4])
                        - (self[e41] * geometric_anti_product[e1])
                        - (self[e42] * geometric_anti_product[e2])
                        - (self[e43] * geometric_anti_product[e3]),
                ),
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
    type Output = Origin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        0        1
    fn geometric_anti_quotient(self, other: AntiScalar) -> Self::Output {
        use crate::elements::*;
        return Origin::from_groups(/* e4 */ self[e4] / (other[e1234]));
    }
}
impl GeometricAntiQuotient<DualNum> for Origin {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        3        0
    //    simd2        0        1        0
    // Totals...
    // yes simd        0        4        0
    //  no simd        0        5        0
    fn geometric_anti_quotient(self, other: DualNum) -> Self::Output {
        use crate::elements::*;
        let geometric_anti_product = DualNum::from_groups(/* scalar, e1234 */ Simd32x2::from(f32::powi(other[e1234], -2)) * other.group0());
        return Flector::from_groups(
            // e1, e2, e3, e4
            Simd32x3::from(0.0).with_w(geometric_anti_product[e1234] * self[e4]),
            // e423, e431, e412, e321
            Simd32x3::from(0.0).with_w(geometric_anti_product[scalar] * self[e4] * -1.0),
        );
    }
}
impl GeometricAntiQuotient<Flector> for Origin {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        0        0
    //    simd4        0        5        0
    // Totals...
    // yes simd        3        5        0
    //  no simd        3       20        0
    fn geometric_anti_quotient(self, other: Flector) -> Self::Output {
        use crate::elements::*;
        let other_2 = AntiScalar::from_groups(
            // e1234
            f32::powi(other[e423], 2) + f32::powi(other[e431], 2) + f32::powi(other[e412], 2) - f32::powi(other[e4], 2),
        );
        let geometric_anti_product = Flector::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from(other_2[e1234]) * other.group0(),
            // e423, e431, e412, e321
            Simd32x4::from(other_2[e1234]) * other.group1(),
        );
        return Motor::from_groups(
            // e41, e42, e43, e1234
            Simd32x4::from(self[e4]) * geometric_anti_product.group1().xyz().with_w(geometric_anti_product[e4]) * Simd32x4::from(-1.0),
            // e23, e31, e12, scalar
            Simd32x4::from(self[e4]) * geometric_anti_product.group0().xyz().with_w(geometric_anti_product[e321]),
        );
    }
}
impl GeometricAntiQuotient<Line> for Origin {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        0        0
    //    simd3        0        2        0
    //    simd4        0        4        0
    // Totals...
    // yes simd        2        6        0
    //  no simd        2       22        0
    fn geometric_anti_quotient(self, other: Line) -> Self::Output {
        use crate::elements::*;
        let other_2 = AntiScalar::from_groups(/* e1234 */ -f32::powi(other[e41], 2) - f32::powi(other[e42], 2) - f32::powi(other[e43], 2));
        let geometric_anti_product = Line::from_groups(
            // e41, e42, e43
            Simd32x3::from(other_2[e1234]) * other.group0(),
            // e23, e31, e12
            Simd32x3::from(other_2[e1234]) * other.group1(),
        );
        return Flector::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from([self[e4], self[e4], self[e4], 0.0]) * geometric_anti_product.group1().with_w(0.0) * Simd32x4::from([-1.0, -1.0, -1.0, 0.0]),
            // e423, e431, e412, e321
            Simd32x4::from([self[e4], self[e4], self[e4], 0.0]) * geometric_anti_product.group0().with_w(0.0) * Simd32x4::from([1.0, 1.0, 1.0, 0.0]),
        );
    }
}
impl GeometricAntiQuotient<Motor> for Origin {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        0        0
    //    simd4        0        6        0
    // Totals...
    // yes simd        3        6        0
    //  no simd        3       24        0
    fn geometric_anti_quotient(self, other: Motor) -> Self::Output {
        use crate::elements::*;
        let other_2 = AntiScalar::from_groups(
            // e1234
            f32::powi(other[e1234], 2) - f32::powi(other[e41], 2) - f32::powi(other[e42], 2) - f32::powi(other[e43], 2),
        );
        let geometric_anti_product = Motor::from_groups(
            // e41, e42, e43, e1234
            Simd32x4::from(other_2[e1234]) * other.group0(),
            // e23, e31, e12, scalar
            Simd32x4::from(other_2[e1234]) * other.group1(),
        );
        return Flector::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from(self[e4]) * geometric_anti_product.group1().xyz().with_w(geometric_anti_product[e1234]) * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e423, e431, e412, e321
            Simd32x4::from(self[e4]) * geometric_anti_product.group0().xyz().with_w(geometric_anti_product[scalar]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
        );
    }
}
impl GeometricAntiQuotient<MultiVector> for Origin {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        7        0        0
    //    simd2        0        3        0
    //    simd3        0        5        0
    //    simd4        0        6        0
    // Totals...
    // yes simd        7       14        0
    //  no simd        7       45        0
    fn geometric_anti_quotient(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        let other_2 = AntiScalar::from_groups(
            // e1234
            f32::powi(other[e1234], 2) + f32::powi(other[e423], 2) + f32::powi(other[e431], 2) + f32::powi(other[e412], 2)
                - f32::powi(other[e4], 2)
                - f32::powi(other[e41], 2)
                - f32::powi(other[e42], 2)
                - f32::powi(other[e43], 2),
        );
        let geometric_anti_product = MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from(other_2[e1234]) * other.group0(),
            // e1, e2, e3, e4
            Simd32x4::from(other_2[e1234]) * other.group1(),
            // e41, e42, e43
            Simd32x3::from(other_2[e1234]) * other.group2(),
            // e23, e31, e12
            Simd32x3::from(other_2[e1234]) * other.group3(),
            // e423, e431, e412, e321
            Simd32x4::from(other_2[e1234]) * other.group4(),
        );
        return MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from(self[e4]) * Simd32x2::from([geometric_anti_product[e321], geometric_anti_product[e4]]) * Simd32x2::from([1.0, -1.0]),
            // e1, e2, e3, e4
            Simd32x4::from(self[e4]) * geometric_anti_product.group3().with_w(geometric_anti_product[e1234]) * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e41, e42, e43
            Simd32x3::from(self[e4]) * geometric_anti_product.group4().xyz() * Simd32x3::from(-1.0),
            // e23, e31, e12
            Simd32x3::from(self[e4]) * geometric_anti_product.group1().xyz(),
            // e423, e431, e412, e321
            Simd32x4::from(self[e4]) * geometric_anti_product.group2().with_w(geometric_anti_product[scalar]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
        );
    }
}
impl GeometricAntiQuotient<Origin> for Origin {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        0        1
    fn geometric_anti_quotient(self, other: Origin) -> Self::Output {
        use crate::elements::*;
        return AntiScalar::from_groups(/* e1234 */ self[e4] / (other[e4]));
    }
}
impl GeometricAntiQuotient<Plane> for Origin {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        1        0
    //    simd4        0        3        0
    // Totals...
    // yes simd        2        4        0
    //  no simd        2       13        0
    fn geometric_anti_quotient(self, other: Plane) -> Self::Output {
        use crate::elements::*;
        let geometric_anti_product = Plane::from_groups(
            // e423, e431, e412, e321
            Simd32x4::from(f32::powi(other[e423], 2) + f32::powi(other[e431], 2) + f32::powi(other[e412], 2)) * other.group0(),
        );
        return Motor::from_groups(
            // e41, e42, e43, e1234
            Simd32x4::from([self[e4], self[e4], self[e4], 0.0]) * geometric_anti_product.group0().xyz().with_w(0.0) * Simd32x4::from([-1.0, -1.0, -1.0, 0.0]),
            // e23, e31, e12, scalar
            Simd32x3::from(0.0).with_w(self[e4] * geometric_anti_product[e321]),
        );
    }
}
impl GeometricAntiQuotient<Point> for Origin {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        3        0
    //    simd4        0        3        0
    // Totals...
    // yes simd        0        6        0
    //  no simd        0       15        0
    fn geometric_anti_quotient(self, other: Point) -> Self::Output {
        use crate::elements::*;
        let geometric_anti_product = Point::from_groups(/* e1, e2, e3, e4 */ Simd32x4::from(f32::powi(other[e4], -2) * -1.0) * other.group0());
        return Motor::from_groups(
            // e41, e42, e43, e1234
            Simd32x3::from(0.0).with_w(self[e4] * geometric_anti_product[e4] * -1.0),
            // e23, e31, e12, scalar
            Simd32x4::from([self[e4], self[e4], self[e4], 0.0]) * geometric_anti_product.group0().xyz().with_w(0.0) * Simd32x4::from([1.0, 1.0, 1.0, 0.0]),
        );
    }
}
impl std::ops::Div<geometric_anti_quotient> for Plane {
    type Output = geometric_anti_quotient_partial<Plane>;
    fn div(self, _rhs: geometric_anti_quotient) -> Self::Output {
        geometric_anti_quotient_partial(self)
    }
}
impl GeometricAntiQuotient<AntiScalar> for Plane {
    type Output = Plane;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        0        1
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        1        1
    //  no simd        0        4        1
    fn geometric_anti_quotient(self, other: AntiScalar) -> Self::Output {
        use crate::elements::*;
        return Plane::from_groups(/* e423, e431, e412, e321 */ Simd32x4::from(1.0 / other[e1234]) * self.group0());
    }
}
impl GeometricAntiQuotient<DualNum> for Plane {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd2        0        1        0
    //    simd4        0        4        0
    // Totals...
    // yes simd        0        5        0
    //  no simd        0       18        0
    fn geometric_anti_quotient(self, other: DualNum) -> Self::Output {
        use crate::elements::*;
        let geometric_anti_product = DualNum::from_groups(/* scalar, e1234 */ Simd32x2::from(f32::powi(other[e1234], -2)) * other.group0());
        return Flector::from_groups(
            // e1, e2, e3, e4
            geometric_anti_product.group0().xx().with_zw(geometric_anti_product[scalar], 0.0)
                * Simd32x3::from(1.0).with_w(0.0)
                * self.group0().xyz().with_w(0.0)
                * Simd32x4::from([-1.0, -1.0, -1.0, 0.0]),
            // e423, e431, e412, e321
            Simd32x4::from(geometric_anti_product[e1234]) * self.group0(),
        );
    }
}
impl GeometricAntiQuotient<Flector> for Plane {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       11       16        0
    //    simd4        3        5        0
    // Totals...
    // yes simd       14       21        0
    //  no simd       23       36        0
    fn geometric_anti_quotient(self, other: Flector) -> Self::Output {
        use crate::elements::*;
        let other_2 = AntiScalar::from_groups(
            // e1234
            f32::powi(other[e423], 2) + f32::powi(other[e431], 2) + f32::powi(other[e412], 2) - f32::powi(other[e4], 2),
        );
        let geometric_anti_product = Flector::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from(other_2[e1234]) * other.group0(),
            // e423, e431, e412, e321
            Simd32x4::from(other_2[e1234]) * other.group1(),
        );
        return Motor::from_groups(
            // e41, e42, e43, e1234
            Simd32x4::from([
                -(geometric_anti_product[e4] * self[e423]) - (geometric_anti_product[e412] * self[e431]),
                -(geometric_anti_product[e4] * self[e431]) - (geometric_anti_product[e423] * self[e412]),
                -(geometric_anti_product[e4] * self[e412]) - (geometric_anti_product[e431] * self[e423]),
                (geometric_anti_product[e431] * self[e431]) + (geometric_anti_product[e412] * self[e412]),
            ]) + (geometric_anti_product.group1().yzxx() * self.group0().zxyx()),
            // e23, e31, e12, scalar
            Simd32x4::from([
                (geometric_anti_product[e3] * self[e431]) + (geometric_anti_product[e321] * self[e423]),
                (geometric_anti_product[e1] * self[e412]) + (geometric_anti_product[e321] * self[e431]),
                (geometric_anti_product[e2] * self[e423]) + (geometric_anti_product[e321] * self[e412]),
                -(geometric_anti_product[e3] * self[e412]) - (geometric_anti_product[e4] * self[e321]),
            ]) - (geometric_anti_product.group0().yzxx() * self.group0().zxyx())
                - (self.group0().wwwy() * geometric_anti_product.group1().xyz().with_w(geometric_anti_product[e2])),
        );
    }
}
impl GeometricAntiQuotient<Line> for Plane {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        7       16        0
    //    simd3        0        2        0
    //    simd4        2        2        0
    // Totals...
    // yes simd        9       20        0
    //  no simd       15       30        0
    fn geometric_anti_quotient(self, other: Line) -> Self::Output {
        use crate::elements::*;
        let other_2 = AntiScalar::from_groups(/* e1234 */ -f32::powi(other[e41], 2) - f32::powi(other[e42], 2) - f32::powi(other[e43], 2));
        let geometric_anti_product = Line::from_groups(
            // e41, e42, e43
            Simd32x3::from(other_2[e1234]) * other.group0(),
            // e23, e31, e12
            Simd32x3::from(other_2[e1234]) * other.group1(),
        );
        return Flector::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from([
                (geometric_anti_product[e41] * self[e321]) + (geometric_anti_product[e31] * self[e412]),
                (geometric_anti_product[e42] * self[e321]) + (geometric_anti_product[e12] * self[e423]),
                (geometric_anti_product[e43] * self[e321]) + (geometric_anti_product[e23] * self[e431]),
                -(geometric_anti_product[e42] * self[e431]) - (geometric_anti_product[e43] * self[e412]),
            ]) - (self.group0().yzxx() * geometric_anti_product.group1().zxy().with_w(geometric_anti_product[e41])),
            // e423, e431, e412, e321
            Simd32x4::from([
                geometric_anti_product[e42] * self[e412] * -1.0,
                geometric_anti_product[e43] * self[e423] * -1.0,
                geometric_anti_product[e41] * self[e431] * -1.0,
                (geometric_anti_product[e31] * self[e431]) + (geometric_anti_product[e12] * self[e412]),
            ]) + (self.group0().yzxx() * geometric_anti_product.group0().zxy().with_w(geometric_anti_product[e23])),
        );
    }
}
impl GeometricAntiQuotient<Motor> for Plane {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        7       16        0
    //    simd4        4        6        0
    // Totals...
    // yes simd       11       22        0
    //  no simd       23       40        0
    fn geometric_anti_quotient(self, other: Motor) -> Self::Output {
        use crate::elements::*;
        let other_2 = AntiScalar::from_groups(
            // e1234
            f32::powi(other[e1234], 2) - f32::powi(other[e41], 2) - f32::powi(other[e42], 2) - f32::powi(other[e43], 2),
        );
        let geometric_anti_product = Motor::from_groups(
            // e41, e42, e43, e1234
            Simd32x4::from(other_2[e1234]) * other.group0(),
            // e23, e31, e12, scalar
            Simd32x4::from(other_2[e1234]) * other.group1(),
        );
        return Flector::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from([
                (geometric_anti_product[e41] * self[e321]) + (geometric_anti_product[e31] * self[e412]),
                (geometric_anti_product[e42] * self[e321]) + (geometric_anti_product[e12] * self[e423]),
                (geometric_anti_product[e43] * self[e321]) + (geometric_anti_product[e23] * self[e431]),
                geometric_anti_product[e43] * self[e412] * -1.0,
            ]) - (self.group0().xyzy() * geometric_anti_product.group1().www().with_w(geometric_anti_product[e42]))
                - (self.group0().yzxx() * geometric_anti_product.group1().zxy().with_w(geometric_anti_product[e41])),
            // e423, e431, e412, e321
            Simd32x4::from([
                geometric_anti_product[e42] * self[e412] * -1.0,
                geometric_anti_product[e43] * self[e423] * -1.0,
                geometric_anti_product[e41] * self[e431] * -1.0,
                (geometric_anti_product[e31] * self[e431]) + (geometric_anti_product[e12] * self[e412]),
            ]) + (geometric_anti_product.group0().zxyw() * self.group0().yzxw())
                + (self.group0().xyzx() * geometric_anti_product.group0().www().with_w(geometric_anti_product[e23])),
        );
    }
}
impl GeometricAntiQuotient<MultiVector> for Plane {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       16       23        0
    //    simd2        0        1        0
    //    simd3        5        9        0
    //    simd4        4        6        0
    // Totals...
    // yes simd       25       39        0
    //  no simd       47       76        0
    fn geometric_anti_quotient(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        let other_2 = AntiScalar::from_groups(
            // e1234
            f32::powi(other[e1234], 2) + f32::powi(other[e423], 2) + f32::powi(other[e431], 2) + f32::powi(other[e412], 2)
                - f32::powi(other[e4], 2)
                - f32::powi(other[e41], 2)
                - f32::powi(other[e42], 2)
                - f32::powi(other[e43], 2),
        );
        let geometric_anti_product = MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from(other_2[e1234]) * other.group0(),
            // e1, e2, e3, e4
            Simd32x4::from(other_2[e1234]) * other.group1(),
            // e41, e42, e43
            Simd32x3::from(other_2[e1234]) * other.group2(),
            // e23, e31, e12
            Simd32x3::from(other_2[e1234]) * other.group3(),
            // e423, e431, e412, e321
            Simd32x4::from(other_2[e1234]) * other.group4(),
        );
        return MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([
                -(geometric_anti_product[e1] * self[e423])
                    - (geometric_anti_product[e2] * self[e431])
                    - (geometric_anti_product[e3] * self[e412])
                    - (geometric_anti_product[e4] * self[e321]),
                (geometric_anti_product[e423] * self[e423]) + (geometric_anti_product[e431] * self[e431]) + (geometric_anti_product[e412] * self[e412]),
            ]),
            // e1, e2, e3, e4
            Simd32x4::from([
                (geometric_anti_product[e41] * self[e321]) + (geometric_anti_product[e31] * self[e412]),
                (geometric_anti_product[e42] * self[e321]) + (geometric_anti_product[e12] * self[e423]),
                (geometric_anti_product[e43] * self[e321]) + (geometric_anti_product[e23] * self[e431]),
                geometric_anti_product[e43] * self[e412] * -1.0,
            ]) - (self.group0().xyzx() * geometric_anti_product.group0().xx().with_zw(geometric_anti_product[scalar], geometric_anti_product[e41]))
                - (self.group0().yzxy() * geometric_anti_product.group3().zxy().with_w(geometric_anti_product[e42])),
            // e41, e42, e43
            (geometric_anti_product.group4().yzx() * self.group0().zxy())
                - (Simd32x3::from(geometric_anti_product[e4]) * self.group0().xyz())
                - (geometric_anti_product.group4().zxy() * self.group0().yzx()),
            // e23, e31, e12
            (Simd32x3::from(geometric_anti_product[e321]) * self.group0().xyz()) + (geometric_anti_product.group1().zxy() * self.group0().yzx())
                - (Simd32x3::from(self[e321]) * geometric_anti_product.group4().xyz())
                - (geometric_anti_product.group1().yzx() * self.group0().zxy()),
            // e423, e431, e412, e321
            Simd32x4::from([
                geometric_anti_product[e42] * self[e412] * -1.0,
                geometric_anti_product[e43] * self[e423] * -1.0,
                geometric_anti_product[e41] * self[e431] * -1.0,
                (geometric_anti_product[e31] * self[e431]) + (geometric_anti_product[e12] * self[e412]),
            ]) + (Simd32x4::from(geometric_anti_product[e1234]) * self.group0())
                + (self.group0().yzxx() * geometric_anti_product.group2().zxy().with_w(geometric_anti_product[e23])),
        );
    }
}
impl GeometricAntiQuotient<Origin> for Plane {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        3        1
    //    simd4        0        2        0
    // Totals...
    // yes simd        0        5        1
    //  no simd        0       11        1
    fn geometric_anti_quotient(self, other: Origin) -> Self::Output {
        use crate::elements::*;
        let geometric_anti_product = Origin::from_groups(/* e4 */ 1.0 / other[e4] * -1.0);
        return Motor::from_groups(
            // e41, e42, e43, e1234
            Simd32x4::from([geometric_anti_product[e4], geometric_anti_product[e4], geometric_anti_product[e4], 0.0])
                * self.group0().xyz().with_w(0.0)
                * Simd32x4::from([-1.0, -1.0, -1.0, 0.0]),
            // e23, e31, e12, scalar
            Simd32x3::from(0.0).with_w(geometric_anti_product[e4] * self[e321] * -1.0),
        );
    }
}
impl GeometricAntiQuotient<Plane> for Plane {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        8        0
    //    simd3        1        2        0
    //    simd4        1        2        0
    // Totals...
    // yes simd        5       12        0
    //  no simd       10       22        0
    fn geometric_anti_quotient(self, other: Plane) -> Self::Output {
        use crate::elements::*;
        let geometric_anti_product = Plane::from_groups(
            // e423, e431, e412, e321
            Simd32x4::from(f32::powi(other[e423], 2) + f32::powi(other[e431], 2) + f32::powi(other[e412], 2)) * other.group0(),
        );
        return Motor::from_groups(
            // e41, e42, e43, e1234
            Simd32x4::from([
                geometric_anti_product[e412] * self[e431] * -1.0,
                geometric_anti_product[e423] * self[e412] * -1.0,
                geometric_anti_product[e431] * self[e423] * -1.0,
                (geometric_anti_product[e431] * self[e431]) + (geometric_anti_product[e412] * self[e412]),
            ]) + (geometric_anti_product.group0().yzxx() * self.group0().zxyx()),
            // e23, e31, e12, scalar
            ((Simd32x3::from(geometric_anti_product[e321]) * self.group0().xyz()) - (Simd32x3::from(self[e321]) * geometric_anti_product.group0().xyz())).with_w(0.0),
        );
    }
}
impl GeometricAntiQuotient<Point> for Plane {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        7        0
    //    simd4        1        5        0
    // Totals...
    // yes simd        3       12        0
    //  no simd        6       27        0
    fn geometric_anti_quotient(self, other: Point) -> Self::Output {
        use crate::elements::*;
        let geometric_anti_product = Point::from_groups(/* e1, e2, e3, e4 */ Simd32x4::from(f32::powi(other[e4], -2) * -1.0) * other.group0());
        return Motor::from_groups(
            // e41, e42, e43, e1234
            Simd32x3::from(1.0).with_w(0.0) * geometric_anti_product.group0().www().with_w(0.0) * self.group0().xyz().with_w(0.0) * Simd32x4::from([-1.0, -1.0, -1.0, 0.0]),
            // e23, e31, e12, scalar
            Simd32x4::from([
                self[e431] * geometric_anti_product[e3],
                self[e412] * geometric_anti_product[e1],
                self[e423] * geometric_anti_product[e2],
                -(self[e431] * geometric_anti_product[e2]) - (self[e412] * geometric_anti_product[e3]) - (self[e321] * geometric_anti_product[e4]),
            ]) - (self.group0().zxyx() * geometric_anti_product.group0().yzxx()),
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
    type Output = Point;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        0        1
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        1        1
    //  no simd        0        4        1
    fn geometric_anti_quotient(self, other: AntiScalar) -> Self::Output {
        use crate::elements::*;
        return Point::from_groups(/* e1, e2, e3, e4 */ Simd32x4::from(1.0 / other[e1234]) * self.group0());
    }
}
impl GeometricAntiQuotient<DualNum> for Point {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        2        0
    //    simd2        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        4        0
    //  no simd        0        8        0
    fn geometric_anti_quotient(self, other: DualNum) -> Self::Output {
        use crate::elements::*;
        let geometric_anti_product = DualNum::from_groups(/* scalar, e1234 */ Simd32x2::from(f32::powi(other[e1234], -2)) * other.group0());
        return Flector::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from(geometric_anti_product[e1234]) * self.group0(),
            // e423, e431, e412, e321
            Simd32x3::from(0.0).with_w(geometric_anti_product[scalar] * self[e4] * -1.0),
        );
    }
}
impl GeometricAntiQuotient<Flector> for Point {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        7        8        0
    //    simd4        2        6        0
    // Totals...
    // yes simd        9       14        0
    //  no simd       15       32        0
    fn geometric_anti_quotient(self, other: Flector) -> Self::Output {
        use crate::elements::*;
        let other_2 = AntiScalar::from_groups(
            // e1234
            f32::powi(other[e423], 2) + f32::powi(other[e431], 2) + f32::powi(other[e412], 2) - f32::powi(other[e4], 2),
        );
        let geometric_anti_product = Flector::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from(other_2[e1234]) * other.group0(),
            // e423, e431, e412, e321
            Simd32x4::from(other_2[e1234]) * other.group1(),
        );
        return Motor::from_groups(
            // e41, e42, e43, e1234
            Simd32x4::from(self[e4]) * geometric_anti_product.group1().xyz().with_w(geometric_anti_product[e4]) * Simd32x4::from(-1.0),
            // e23, e31, e12, scalar
            Simd32x4::from([
                -(geometric_anti_product[e4] * self[e1]) - (geometric_anti_product[e412] * self[e2]),
                -(geometric_anti_product[e4] * self[e2]) - (geometric_anti_product[e423] * self[e3]),
                -(geometric_anti_product[e4] * self[e3]) - (geometric_anti_product[e431] * self[e1]),
                (geometric_anti_product[e412] * self[e3]) + (geometric_anti_product[e321] * self[e4]),
            ]) + (geometric_anti_product.group1().yzxy() * self.group0().zxyy())
                + (self.group0().wwwx() * geometric_anti_product.group0().xyz().with_w(geometric_anti_product[e423])),
        );
    }
}
impl GeometricAntiQuotient<Line> for Point {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        3        0
    //    simd3        0        5        0
    //    simd4        2        1        0
    // Totals...
    // yes simd        6        9        0
    //  no simd       12       22        0
    fn geometric_anti_quotient(self, other: Line) -> Self::Output {
        use crate::elements::*;
        let other_2 = AntiScalar::from_groups(/* e1234 */ -f32::powi(other[e41], 2) - f32::powi(other[e42], 2) - f32::powi(other[e43], 2));
        let geometric_anti_product = Line::from_groups(
            // e41, e42, e43
            Simd32x3::from(other_2[e1234]) * other.group0(),
            // e23, e31, e12
            Simd32x3::from(other_2[e1234]) * other.group1(),
        );
        return Flector::from_groups(
            // e1, e2, e3, e4
            (geometric_anti_product.group0().zxy() * self.group0().yzx()).with_w(0.0)
                - (Simd32x3::from(self[e4]) * geometric_anti_product.group1()).with_w(0.0)
                - (geometric_anti_product.group0().yzx() * self.group0().zxy()).with_w(0.0),
            // e423, e431, e412, e321
            Simd32x4::from([self[e4], self[e4], self[e4], 1.0])
                * geometric_anti_product
                    .group0()
                    .with_w(-(geometric_anti_product[e41] * self[e1]) - (geometric_anti_product[e42] * self[e2]) - (geometric_anti_product[e43] * self[e3])),
        );
    }
}
impl GeometricAntiQuotient<Motor> for Point {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        6        5        0
    //    simd3        3        4        0
    //    simd4        0        3        0
    // Totals...
    // yes simd        9       12        0
    //  no simd       15       29        0
    fn geometric_anti_quotient(self, other: Motor) -> Self::Output {
        use crate::elements::*;
        let other_2 = AntiScalar::from_groups(
            // e1234
            f32::powi(other[e1234], 2) - f32::powi(other[e41], 2) - f32::powi(other[e42], 2) - f32::powi(other[e43], 2),
        );
        let geometric_anti_product = Motor::from_groups(
            // e41, e42, e43, e1234
            Simd32x4::from(other_2[e1234]) * other.group0(),
            // e23, e31, e12, scalar
            Simd32x4::from(other_2[e1234]) * other.group1(),
        );
        return Flector::from_groups(
            // e1, e2, e3, e4
            ((Simd32x3::from(geometric_anti_product[e1234]) * self.group0().xyz()) + (geometric_anti_product.group0().zxy() * self.group0().yzx())
                - (Simd32x3::from(self[e4]) * geometric_anti_product.group1().xyz())
                - (geometric_anti_product.group0().yzx() * self.group0().zxy()))
            .with_w(geometric_anti_product[e1234] * self[e4]),
            // e423, e431, e412, e321
            Simd32x4::from([self[e4], self[e4], self[e4], 1.0])
                * geometric_anti_product.group0().xyz().with_w(
                    -(geometric_anti_product[e41] * self[e1])
                        - (geometric_anti_product[e42] * self[e2])
                        - (geometric_anti_product[e43] * self[e3])
                        - (geometric_anti_product[scalar] * self[e4]),
                ),
        );
    }
}
impl GeometricAntiQuotient<MultiVector> for Point {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       13       10        0
    //    simd2        0        2        0
    //    simd3        6       12        0
    //    simd4        0        3        0
    // Totals...
    // yes simd       19       27        0
    //  no simd       31       62        0
    fn geometric_anti_quotient(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        let other_2 = AntiScalar::from_groups(
            // e1234
            f32::powi(other[e1234], 2) + f32::powi(other[e423], 2) + f32::powi(other[e431], 2) + f32::powi(other[e412], 2)
                - f32::powi(other[e4], 2)
                - f32::powi(other[e41], 2)
                - f32::powi(other[e42], 2)
                - f32::powi(other[e43], 2),
        );
        let geometric_anti_product = MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from(other_2[e1234]) * other.group0(),
            // e1, e2, e3, e4
            Simd32x4::from(other_2[e1234]) * other.group1(),
            // e41, e42, e43
            Simd32x3::from(other_2[e1234]) * other.group2(),
            // e23, e31, e12
            Simd32x3::from(other_2[e1234]) * other.group3(),
            // e423, e431, e412, e321
            Simd32x4::from(other_2[e1234]) * other.group4(),
        );
        return MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([
                (geometric_anti_product[e423] * self[e1])
                    + (geometric_anti_product[e431] * self[e2])
                    + (geometric_anti_product[e412] * self[e3])
                    + (geometric_anti_product[e321] * self[e4]),
                geometric_anti_product[e4] * self[e4],
            ]) * Simd32x2::from([1.0, -1.0]),
            // e1, e2, e3, e4
            ((Simd32x3::from(geometric_anti_product[e1234]) * self.group0().xyz()) + (geometric_anti_product.group2().zxy() * self.group0().yzx())
                - (Simd32x3::from(self[e4]) * geometric_anti_product.group3())
                - (geometric_anti_product.group2().yzx() * self.group0().zxy()))
            .with_w(geometric_anti_product[e1234] * self[e4]),
            // e41, e42, e43
            Simd32x3::from(self[e4]) * geometric_anti_product.group4().xyz() * Simd32x3::from(-1.0),
            // e23, e31, e12
            (Simd32x3::from(self[e4]) * geometric_anti_product.group1().xyz()) + (geometric_anti_product.group4().yzx() * self.group0().zxy())
                - (Simd32x3::from(geometric_anti_product[e4]) * self.group0().xyz())
                - (geometric_anti_product.group4().zxy() * self.group0().yzx()),
            // e423, e431, e412, e321
            Simd32x4::from([self[e4], self[e4], self[e4], 1.0])
                * geometric_anti_product.group2().with_w(
                    -(geometric_anti_product[scalar] * self[e4])
                        - (geometric_anti_product[e41] * self[e1])
                        - (geometric_anti_product[e42] * self[e2])
                        - (geometric_anti_product[e43] * self[e3]),
                ),
        );
    }
}
impl GeometricAntiQuotient<Origin> for Point {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        3        1
    //    simd4        0        2        0
    // Totals...
    // yes simd        0        5        1
    //  no simd        0       11        1
    fn geometric_anti_quotient(self, other: Origin) -> Self::Output {
        use crate::elements::*;
        let geometric_anti_product = Origin::from_groups(/* e4 */ 1.0 / other[e4] * -1.0);
        return Motor::from_groups(
            // e41, e42, e43, e1234
            Simd32x3::from(0.0).with_w(geometric_anti_product[e4] * self[e4] * -1.0),
            // e23, e31, e12, scalar
            Simd32x4::from([geometric_anti_product[e4], geometric_anti_product[e4], geometric_anti_product[e4], 0.0])
                * self.group0().xyz().with_w(0.0)
                * Simd32x4::from([-1.0, -1.0, -1.0, 0.0]),
        );
    }
}
impl GeometricAntiQuotient<Plane> for Point {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        9        0
    //    simd4        1        5        0
    // Totals...
    // yes simd        5       14        0
    //  no simd        8       29        0
    fn geometric_anti_quotient(self, other: Plane) -> Self::Output {
        use crate::elements::*;
        let geometric_anti_product = Plane::from_groups(
            // e423, e431, e412, e321
            Simd32x4::from(f32::powi(other[e423], 2) + f32::powi(other[e431], 2) + f32::powi(other[e412], 2)) * other.group0(),
        );
        return Motor::from_groups(
            // e41, e42, e43, e1234
            Simd32x3::from(1.0).with_w(0.0) * self.group0().www().with_w(0.0) * geometric_anti_product.group0().xyz().with_w(0.0) * Simd32x4::from([-1.0, -1.0, -1.0, 0.0]),
            // e23, e31, e12, scalar
            Simd32x4::from([
                geometric_anti_product[e412] * self[e2] * -1.0,
                geometric_anti_product[e423] * self[e3] * -1.0,
                geometric_anti_product[e431] * self[e1] * -1.0,
                (geometric_anti_product[e431] * self[e2]) + (geometric_anti_product[e412] * self[e3]) + (geometric_anti_product[e321] * self[e4]),
            ]) + (geometric_anti_product.group0().yzxx() * self.group0().zxyx()),
        );
    }
}
impl GeometricAntiQuotient<Point> for Point {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        3        0
    //    simd3        1        2        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        1        6        0
    //  no simd        3       13        0
    fn geometric_anti_quotient(self, other: Point) -> Self::Output {
        use crate::elements::*;
        let geometric_anti_product = Point::from_groups(/* e1, e2, e3, e4 */ Simd32x4::from(f32::powi(other[e4], -2) * -1.0) * other.group0());
        return Motor::from_groups(
            // e41, e42, e43, e1234
            Simd32x3::from(0.0).with_w(geometric_anti_product[e4] * self[e4] * -1.0),
            // e23, e31, e12, scalar
            ((Simd32x3::from(self[e4]) * geometric_anti_product.group0().xyz()) - (Simd32x3::from(geometric_anti_product[e4]) * self.group0().xyz())).with_w(0.0),
        );
    }
}
impl std::ops::Div<geometric_anti_quotient> for Scalar {
    type Output = geometric_anti_quotient_partial<Scalar>;
    fn div(self, _rhs: geometric_anti_quotient) -> Self::Output {
        geometric_anti_quotient_partial(self)
    }
}
impl GeometricAntiQuotient<AntiScalar> for Scalar {
    type Output = Scalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        0        1
    fn geometric_anti_quotient(self, other: AntiScalar) -> Self::Output {
        use crate::elements::*;
        return Scalar::from_groups(/* scalar */ self[scalar] / (other[e1234]));
    }
}
impl GeometricAntiQuotient<DualNum> for Scalar {
    type Output = Scalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        0        1
    fn geometric_anti_quotient(self, other: DualNum) -> Self::Output {
        use crate::elements::*;
        return Scalar::from_groups(/* scalar */ self[scalar] / (other[e1234]));
    }
}
impl GeometricAntiQuotient<Flector> for Scalar {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        1        0
    //    simd4        0        4        0
    // Totals...
    // yes simd        3        5        0
    //  no simd        3       17        0
    fn geometric_anti_quotient(self, other: Flector) -> Self::Output {
        use crate::elements::*;
        let other_2 = AntiScalar::from_groups(
            // e1234
            f32::powi(other[e423], 2) + f32::powi(other[e431], 2) + f32::powi(other[e412], 2) - f32::powi(other[e4], 2),
        );
        let geometric_anti_product = Flector::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from(other_2[e1234]) * other.group0(),
            // e423, e431, e412, e321
            Simd32x4::from(other_2[e1234]) * other.group1(),
        );
        return Flector::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from([self[scalar], self[scalar], self[scalar], 0.0]) * geometric_anti_product.group1().xyz().with_w(0.0) * Simd32x4::from([1.0, 1.0, 1.0, 0.0]),
            // e423, e431, e412, e321
            Simd32x3::from(0.0).with_w(geometric_anti_product[e4] * self[scalar]),
        );
    }
}
impl GeometricAntiQuotient<Line> for Scalar {
    type Output = Line;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        3        0
    //    simd3        0        1        0
    // Totals...
    // yes simd        2        4        0
    //  no simd        2        6        0
    fn geometric_anti_quotient(self, other: Line) -> Self::Output {
        use crate::elements::*;
        let other_2 = AntiScalar::from_groups(/* e1234 */ -f32::powi(other[e41], 2) - f32::powi(other[e42], 2) - f32::powi(other[e43], 2));
        return Line::from_groups(
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(self[scalar]) * Simd32x3::from([other_2[e1234] * other[e41], other_2[e1234] * other[e42], other_2[e1234] * other[e43]]),
        );
    }
}
impl GeometricAntiQuotient<Motor> for Scalar {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        4        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        3        5        0
    //  no simd        3        8        0
    fn geometric_anti_quotient(self, other: Motor) -> Self::Output {
        use crate::elements::*;
        let other_2 = AntiScalar::from_groups(
            // e1234
            f32::powi(other[e1234], 2) - f32::powi(other[e41], 2) - f32::powi(other[e42], 2) - f32::powi(other[e43], 2),
        );
        return Motor::from_groups(
            // e41, e42, e43, e1234
            Simd32x4::from(0.0),
            // e23, e31, e12, scalar
            Simd32x4::from(self[scalar]) * Simd32x4::from([other_2[e1234] * other[e41], other_2[e1234] * other[e42], other_2[e1234] * other[e43], other_2[e1234] * other[e1234]]),
        );
    }
}
impl GeometricAntiQuotient<MultiVector> for Scalar {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        7        2        0
    //    simd2        0        2        0
    //    simd3        0        3        0
    //    simd4        0        4        0
    // Totals...
    // yes simd        7       11        0
    //  no simd        7       31        0
    fn geometric_anti_quotient(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        let other_2 = AntiScalar::from_groups(
            // e1234
            f32::powi(other[e1234], 2) + f32::powi(other[e423], 2) + f32::powi(other[e431], 2) + f32::powi(other[e412], 2)
                - f32::powi(other[e4], 2)
                - f32::powi(other[e41], 2)
                - f32::powi(other[e42], 2)
                - f32::powi(other[e43], 2),
        );
        let geometric_anti_product = MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from(other_2[e1234]) * other.group0(),
            // e1, e2, e3, e4
            Simd32x4::from(other_2[e1234]) * other.group1(),
            // e41, e42, e43
            Simd32x3::from(other_2[e1234]) * other.group2(),
            // e23, e31, e12
            Simd32x3::from(other_2[e1234]) * other.group3(),
            // e423, e431, e412, e321
            Simd32x4::from(other_2[e1234]) * other.group4(),
        );
        return MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([geometric_anti_product[e1234] * self[scalar], 1.0]) * Simd32x2::from([1.0, 0.0]),
            // e1, e2, e3, e4
            Simd32x4::from([self[scalar], self[scalar], self[scalar], 0.0]) * geometric_anti_product.group4().xyz().with_w(0.0) * Simd32x4::from([1.0, 1.0, 1.0, 0.0]),
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(self[scalar]) * geometric_anti_product.group2(),
            // e423, e431, e412, e321
            Simd32x3::from(0.0).with_w(geometric_anti_product[e4] * self[scalar]),
        );
    }
}
impl GeometricAntiQuotient<Origin> for Scalar {
    type Output = Horizon;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        1
    fn geometric_anti_quotient(self, other: Origin) -> Self::Output {
        use crate::elements::*;
        return Horizon::from_groups(/* e321 */ self[scalar] / (other[e4]) * -1.0);
    }
}
impl GeometricAntiQuotient<Plane> for Scalar {
    type Output = Point;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        4        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        2        6        0
    //  no simd        2       12        0
    fn geometric_anti_quotient(self, other: Plane) -> Self::Output {
        use crate::elements::*;
        let other_2 = AntiScalar::from_groups(/* e1234 */ f32::powi(other[e423], 2) + f32::powi(other[e431], 2) + f32::powi(other[e412], 2));
        return Point::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from([self[scalar], self[scalar], self[scalar], 0.0])
                * Simd32x4::from([other_2[e1234] * other[e423], other_2[e1234] * other[e431], other_2[e1234] * other[e412], other_2[e1234] * other[e321]])
                    .xyz()
                    .with_w(0.0)
                * Simd32x4::from([1.0, 1.0, 1.0, 0.0]),
        );
    }
}
impl GeometricAntiQuotient<Point> for Scalar {
    type Output = Horizon;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        1
    fn geometric_anti_quotient(self, other: Point) -> Self::Output {
        use crate::elements::*;
        return Horizon::from_groups(/* e321 */ self[scalar] / (other[e4]) * -1.0);
    }
}
