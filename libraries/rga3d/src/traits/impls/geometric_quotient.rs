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
//  Minimum:         0       2       0
//   Median:         3       6       0
//  Average:         8      13       0
//  Maximum:        88      97       1
//
//  No SIMD:   add/sub     mul     div
//  Minimum:         0       2       0
//   Median:         3      17       0
//  Average:        14      27       0
//  Maximum:       188     208       1
impl std::ops::Div<geometric_quotient> for AntiScalar {
    type Output = geometric_quotient_partial<AntiScalar>;
    fn div(self, _rhs: geometric_quotient) -> Self::Output {
        geometric_quotient_partial(self)
    }
}
impl GeometricQuotient<DualNum> for AntiScalar {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        2        0
    fn geometric_quotient(self, other: DualNum) -> Self::Output {
        use crate::elements::*;
        let other = Scalar::from_groups(/* scalar */ f32::powi(other[scalar], -2));
        return AntiScalar::from_groups(/* e1234 */ self[e1234] * other[scalar] * other[scalar]);
    }
}
impl GeometricQuotient<Flector> for AntiScalar {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        2        0
    //    simd4        0        4        0
    // Totals...
    // yes simd        3        6        0
    //  no simd        3       18        0
    fn geometric_quotient(self, other: Flector) -> Self::Output {
        use crate::elements::*;
        let other = Scalar::from_groups(
            // scalar
            f32::powi(other[e1], 2) + f32::powi(other[e2], 2) + f32::powi(other[e3], 2) - f32::powi(other[e321], 2),
        );
        let geometric_product = Flector::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from(other[scalar]) * other.group0(),
            // e423, e431, e412, e321
            Simd32x4::from(other[scalar]) * other.group1(),
        );
        return Flector::from_groups(
            // e1, e2, e3, e4
            Simd32x3::from(0.0).with_w(self[e1234] * geometric_product[e321] * -1.0),
            // e423, e431, e412, e321
            Simd32x4::from([self[e1234], self[e1234], self[e1234], 0.0]) * geometric_product.group0().xyz().with_w(0.0) * Simd32x4::from([-1.0, -1.0, -1.0, 0.0]),
        );
    }
}
impl GeometricQuotient<Horizon> for AntiScalar {
    type Output = Origin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        4        0
    fn geometric_quotient(self, other: Horizon) -> Self::Output {
        use crate::elements::*;
        let other = Scalar::from_groups(/* scalar */ f32::powi(other[e321], -2) * -1.0);
        return Origin::from_groups(/* e4 */ self[e1234] * other[e321] * other[scalar] * -1.0);
    }
}
impl GeometricQuotient<Line> for AntiScalar {
    type Output = Line;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        3        0
    //    simd3        0        1        0
    // Totals...
    // yes simd        2        4        0
    //  no simd        2        6        0
    fn geometric_quotient(self, other: Line) -> Self::Output {
        use crate::elements::*;
        let other = Scalar::from_groups(/* scalar */ -f32::powi(other[e23], 2) - f32::powi(other[e31], 2) - f32::powi(other[e12], 2));
        return Line::from_groups(
            // e41, e42, e43
            Simd32x3::from(self[e1234]) * Simd32x3::from([other[e23] * other[scalar], other[e31] * other[scalar], other[e12] * other[scalar]]),
            // e23, e31, e12
            Simd32x3::from(0.0),
        );
    }
}
impl GeometricQuotient<Motor> for AntiScalar {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        4        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        3        5        0
    //  no simd        3        8        0
    fn geometric_quotient(self, other: Motor) -> Self::Output {
        use crate::elements::*;
        let other = Scalar::from_groups(
            // scalar
            f32::powi(other[scalar], 2) - f32::powi(other[e23], 2) - f32::powi(other[e31], 2) - f32::powi(other[e12], 2),
        );
        return Motor::from_groups(
            // e41, e42, e43, e1234
            Simd32x4::from(self[e1234]) * Simd32x4::from([other[e23] * other[scalar], other[e31] * other[scalar], other[e12] * other[scalar], other[scalar] * other[scalar]]),
            // e23, e31, e12, scalar
            Simd32x4::from(0.0),
        );
    }
}
impl GeometricQuotient<MultiVector> for AntiScalar {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        7        3        0
    //    simd2        0        2        0
    //    simd3        0        3        0
    //    simd4        0        4        0
    // Totals...
    // yes simd        7       12        0
    //  no simd        7       32        0
    fn geometric_quotient(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        let other = Scalar::from_groups(
            // scalar
            f32::powi(other[scalar], 2) + f32::powi(other[e1], 2) + f32::powi(other[e2], 2) + f32::powi(other[e3], 2)
                - f32::powi(other[e23], 2)
                - f32::powi(other[e31], 2)
                - f32::powi(other[e12], 2)
                - f32::powi(other[e321], 2),
        );
        let geometric_product = MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from(other[scalar]) * other.group0(),
            // e1, e2, e3, e4
            Simd32x4::from(other[scalar]) * other.group1(),
            // e41, e42, e43
            Simd32x3::from(other[scalar]) * other.group2(),
            // e23, e31, e12
            Simd32x3::from(other[scalar]) * other.group3(),
            // e423, e431, e412, e321
            Simd32x4::from(other[scalar]) * other.group4(),
        );
        return MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([1.0, self[e1234] * geometric_product[scalar]]) * Simd32x2::from([0.0, 1.0]),
            // e1, e2, e3, e4
            Simd32x3::from(0.0).with_w(self[e1234] * geometric_product[e321] * -1.0),
            // e41, e42, e43
            Simd32x3::from(self[e1234]) * geometric_product.group3(),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e423, e431, e412, e321
            Simd32x4::from([self[e1234], self[e1234], self[e1234], 0.0]) * geometric_product.group1().xyz().with_w(0.0) * Simd32x4::from([-1.0, -1.0, -1.0, 0.0]),
        );
    }
}
impl GeometricQuotient<Plane> for AntiScalar {
    type Output = Origin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        4        0
    fn geometric_quotient(self, other: Plane) -> Self::Output {
        use crate::elements::*;
        let other = Scalar::from_groups(/* scalar */ f32::powi(other[e321], -2) * -1.0);
        return Origin::from_groups(/* e4 */ self[e1234] * other[e321] * other[scalar] * -1.0);
    }
}
impl GeometricQuotient<Point> for AntiScalar {
    type Output = Plane;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        4        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        2        6        0
    //  no simd        2       12        0
    fn geometric_quotient(self, other: Point) -> Self::Output {
        use crate::elements::*;
        let other = Scalar::from_groups(/* scalar */ f32::powi(other[e1], 2) + f32::powi(other[e2], 2) + f32::powi(other[e3], 2));
        return Plane::from_groups(
            // e423, e431, e412, e321
            Simd32x4::from([self[e1234], self[e1234], self[e1234], 0.0])
                * Simd32x4::from([other[e1] * other[scalar], other[e2] * other[scalar], other[e3] * other[scalar], other[e4] * other[scalar]])
                    .xyz()
                    .with_w(0.0)
                * Simd32x4::from([-1.0, -1.0, -1.0, 0.0]),
        );
    }
}
impl GeometricQuotient<Scalar> for AntiScalar {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        2        0
    fn geometric_quotient(self, other: Scalar) -> Self::Output {
        use crate::elements::*;
        let other = Scalar::from_groups(/* scalar */ f32::powi(other[scalar], -2));
        return AntiScalar::from_groups(/* e1234 */ self[e1234] * other[scalar] * other[scalar]);
    }
}
impl std::ops::Div<geometric_quotient> for DualNum {
    type Output = geometric_quotient_partial<DualNum>;
    fn div(self, _rhs: geometric_quotient) -> Self::Output {
        geometric_quotient_partial(self)
    }
}
impl GeometricQuotient<DualNum> for DualNum {
    type Output = DualNum;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        3        0
    //    simd2        0        1        0
    // Totals...
    // yes simd        1        4        0
    //  no simd        1        5        0
    fn geometric_quotient(self, other: DualNum) -> Self::Output {
        use crate::elements::*;
        let geometric_product = DualNum::from_groups(/* scalar, e1234 */ Simd32x2::from(f32::powi(other[scalar], -2)) * other.group0());
        return DualNum::from_groups(/* scalar, e1234 */ Simd32x2::from([
            geometric_product[scalar] * self[scalar],
            (geometric_product[scalar] * self[e1234]) + (geometric_product[e1234] * self[scalar]),
        ]));
    }
}
impl GeometricQuotient<Flector> for DualNum {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        3        0
    //    simd3        1        2        0
    //    simd4        0        3        0
    // Totals...
    // yes simd        5        8        0
    //  no simd        7       21        0
    fn geometric_quotient(self, other: Flector) -> Self::Output {
        use crate::elements::*;
        let other = Scalar::from_groups(
            // scalar
            f32::powi(other[e1], 2) + f32::powi(other[e2], 2) + f32::powi(other[e3], 2) - f32::powi(other[e321], 2),
        );
        let geometric_product = Flector::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from(other[scalar]) * other.group0(),
            // e423, e431, e412, e321
            Simd32x4::from(other[scalar]) * other.group1(),
        );
        return Flector::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from([geometric_product[e1], geometric_product[e2], geometric_product[e3], 1.0])
                * self
                    .group0()
                    .xx()
                    .with_zw(self[scalar], (self[scalar] * geometric_product[e4]) - (self[e1234] * geometric_product[e321])),
            // e423, e431, e412, e321
            ((Simd32x3::from(self[scalar]) * geometric_product.group1().xyz()) - (Simd32x3::from(self[e1234]) * geometric_product.group0().xyz()))
                .with_w(self[scalar] * geometric_product[e321]),
        );
    }
}
impl GeometricQuotient<Horizon> for DualNum {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        4        1
    fn geometric_quotient(self, other: Horizon) -> Self::Output {
        use crate::elements::*;
        let geometric_product = Horizon::from_groups(/* e321 */ 1.0 / other[e321] * -1.0);
        return Flector::from_groups(
            // e1, e2, e3, e4
            Simd32x3::from(0.0).with_w(self[e1234] * geometric_product[e321] * -1.0),
            // e423, e431, e412, e321
            Simd32x3::from(0.0).with_w(self[scalar] * geometric_product[e321]),
        );
    }
}
impl GeometricQuotient<Line> for DualNum {
    type Output = Line;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        0        0
    //    simd3        1        5        0
    // Totals...
    // yes simd        3        5        0
    //  no simd        5       15        0
    fn geometric_quotient(self, other: Line) -> Self::Output {
        use crate::elements::*;
        let other = Scalar::from_groups(/* scalar */ -f32::powi(other[e23], 2) - f32::powi(other[e31], 2) - f32::powi(other[e12], 2));
        let geometric_product = Line::from_groups(
            // e41, e42, e43
            Simd32x3::from(other[scalar]) * other.group0(),
            // e23, e31, e12
            Simd32x3::from(other[scalar]) * other.group1(),
        );
        return Line::from_groups(
            // e41, e42, e43
            (Simd32x3::from(self[scalar]) * geometric_product.group0()) + (Simd32x3::from(self[e1234]) * geometric_product.group1()),
            // e23, e31, e12
            Simd32x3::from(self[scalar]) * geometric_product.group1(),
        );
    }
}
impl GeometricQuotient<Motor> for DualNum {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        0        0
    //    simd4        1        5        0
    // Totals...
    // yes simd        4        5        0
    //  no simd        7       20        0
    fn geometric_quotient(self, other: Motor) -> Self::Output {
        use crate::elements::*;
        let other = Scalar::from_groups(
            // scalar
            f32::powi(other[scalar], 2) - f32::powi(other[e23], 2) - f32::powi(other[e31], 2) - f32::powi(other[e12], 2),
        );
        let geometric_product = Motor::from_groups(
            // e41, e42, e43, e1234
            Simd32x4::from(other[scalar]) * other.group0(),
            // e23, e31, e12, scalar
            Simd32x4::from(other[scalar]) * other.group1(),
        );
        return Motor::from_groups(
            // e41, e42, e43, e1234
            (Simd32x4::from(self[scalar]) * geometric_product.group0()) + (Simd32x4::from(self[e1234]) * geometric_product.group1()),
            // e23, e31, e12, scalar
            Simd32x4::from(self[scalar]) * geometric_product.group1(),
        );
    }
}
impl GeometricQuotient<MultiVector> for DualNum {
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
    fn geometric_quotient(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        let other = Scalar::from_groups(
            // scalar
            f32::powi(other[scalar], 2) + f32::powi(other[e1], 2) + f32::powi(other[e2], 2) + f32::powi(other[e3], 2)
                - f32::powi(other[e23], 2)
                - f32::powi(other[e31], 2)
                - f32::powi(other[e12], 2)
                - f32::powi(other[e321], 2),
        );
        let geometric_product = MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from(other[scalar]) * other.group0(),
            // e1, e2, e3, e4
            Simd32x4::from(other[scalar]) * other.group1(),
            // e41, e42, e43
            Simd32x3::from(other[scalar]) * other.group2(),
            // e23, e31, e12
            Simd32x3::from(other[scalar]) * other.group3(),
            // e423, e431, e412, e321
            Simd32x4::from(other[scalar]) * other.group4(),
        );
        return MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([
                self[scalar] * geometric_product[scalar],
                (self[scalar] * geometric_product[e1234]) + (self[e1234] * geometric_product[scalar]),
            ]),
            // e1, e2, e3, e4
            Simd32x4::from([geometric_product[e1], geometric_product[e2], geometric_product[e3], 1.0])
                * self
                    .group0()
                    .xx()
                    .with_zw(self[scalar], (self[scalar] * geometric_product[e4]) - (self[e1234] * geometric_product[e321])),
            // e41, e42, e43
            (Simd32x3::from(self[scalar]) * geometric_product.group2()) + (Simd32x3::from(self[e1234]) * geometric_product.group3()),
            // e23, e31, e12
            Simd32x3::from(self[scalar]) * geometric_product.group3(),
            // e423, e431, e412, e321
            ((Simd32x3::from(self[scalar]) * geometric_product.group4().xyz()) - (Simd32x3::from(self[e1234]) * geometric_product.group1().xyz()))
                .with_w(self[scalar] * geometric_product[e321]),
        );
    }
}
impl GeometricQuotient<Plane> for DualNum {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        3        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        0        5        0
    //  no simd        0       11        0
    fn geometric_quotient(self, other: Plane) -> Self::Output {
        use crate::elements::*;
        let geometric_product = Plane::from_groups(/* e423, e431, e412, e321 */ Simd32x4::from(f32::powi(other[e321], -2) * -1.0) * other.group0());
        return Flector::from_groups(
            // e1, e2, e3, e4
            Simd32x3::from(0.0).with_w(self[e1234] * geometric_product[e321] * -1.0),
            // e423, e431, e412, e321
            Simd32x4::from(self[scalar]) * geometric_product.group0(),
        );
    }
}
impl GeometricQuotient<Point> for DualNum {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        0        0
    //    simd4        0        5        0
    // Totals...
    // yes simd        2        5        0
    //  no simd        2       20        0
    fn geometric_quotient(self, other: Point) -> Self::Output {
        use crate::elements::*;
        let geometric_product = Point::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from(f32::powi(other[e1], 2) + f32::powi(other[e2], 2) + f32::powi(other[e3], 2)) * other.group0(),
        );
        return Flector::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from(self[scalar]) * geometric_product.group0(),
            // e423, e431, e412, e321
            self.group0().yy().with_zw(self[e1234], 0.0) * Simd32x3::from(1.0).with_w(0.0) * geometric_product.group0().xyz().with_w(0.0) * Simd32x4::from([-1.0, -1.0, -1.0, 0.0]),
        );
    }
}
impl GeometricQuotient<Scalar> for DualNum {
    type Output = DualNum;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd2        0        1        0
    // Totals...
    // yes simd        0        2        0
    //  no simd        0        3        0
    fn geometric_quotient(self, other: Scalar) -> Self::Output {
        use crate::elements::*;
        let other = Scalar::from_groups(/* scalar */ f32::powi(other[scalar], -2));
        return DualNum::from_groups(/* scalar, e1234 */ Simd32x2::from(other[scalar] * other[scalar]) * self.group0());
    }
}
impl std::ops::Div<geometric_quotient> for Flector {
    type Output = geometric_quotient_partial<Flector>;
    fn div(self, _rhs: geometric_quotient) -> Self::Output {
        geometric_quotient_partial(self)
    }
}
impl GeometricQuotient<DualNum> for Flector {
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
    fn geometric_quotient(self, other: DualNum) -> Self::Output {
        use crate::elements::*;
        let geometric_product = DualNum::from_groups(/* scalar, e1234 */ Simd32x2::from(f32::powi(other[scalar], -2)) * other.group0());
        return Flector::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from([self[e1], self[e2], self[e3], 1.0])
                * geometric_product
                    .group0()
                    .xx()
                    .with_zw(geometric_product[scalar], (geometric_product[scalar] * self[e4]) + (geometric_product[e1234] * self[e321])),
            // e423, e431, e412, e321
            ((Simd32x3::from(geometric_product[scalar]) * self.group1().xyz()) + (Simd32x3::from(geometric_product[e1234]) * self.group0().xyz()))
                .with_w(geometric_product[scalar] * self[e321]),
        );
    }
}
impl GeometricQuotient<Flector> for Flector {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        7        8        0
    //    simd4        9       12        0
    // Totals...
    // yes simd       16       20        0
    //  no simd       43       56        0
    fn geometric_quotient(self, other: Flector) -> Self::Output {
        use crate::elements::*;
        let other = Scalar::from_groups(
            // scalar
            f32::powi(other[e1], 2) + f32::powi(other[e2], 2) + f32::powi(other[e3], 2) - f32::powi(other[e321], 2),
        );
        let geometric_product = Flector::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from(other[scalar]) * other.group0(),
            // e423, e431, e412, e321
            Simd32x4::from(other[scalar]) * other.group1(),
        );
        return Motor::from_groups(
            // e41, e42, e43, e1234
            (Simd32x4::from(geometric_product[e321]) * self.group1().xyz().with_w(self[e4]))
                + (geometric_product.group1().zxyz() * self.group0().yzxz())
                + (self.group0().ww().with_zw(self[e431], self[e1]) * geometric_product.group0().xyx().with_w(geometric_product[e423]))
                + (self.group1().zx().with_zw(self[e4], self[e2]) * geometric_product.group0().yzz().with_w(geometric_product[e431]))
                - (geometric_product.group0().zxyx() * self.group1().yzxx())
                - (geometric_product.group0().wwwy() * self.group0().xyz().with_w(self[e431]))
                - (self.group0().zx().with_zw(self[e321], self[e321]) * geometric_product.group1().yzz().with_w(geometric_product[e4]))
                - (self.group1().ww().with_zw(self[e2], self[e412]) * geometric_product.group1().xyx().with_w(geometric_product[e3])),
            // e23, e31, e12, scalar
            Simd32x4::from([
                -(geometric_product[e2] * self[e3]) - (geometric_product[e321] * self[e1]),
                -(geometric_product[e3] * self[e1]) - (geometric_product[e321] * self[e2]),
                -(geometric_product[e3] * self[e321]) - (geometric_product[e321] * self[e3]),
                (geometric_product[e2] * self[e2]) + (geometric_product[e3] * self[e3]),
            ]) + (geometric_product.group0().zxyx() * self.group0().yzxx())
                - (self.group1().ww().with_zw(self[e2], self[e321]) * geometric_product.group0().xyx().with_w(geometric_product[e321])),
        );
    }
}
impl GeometricQuotient<Horizon> for Flector {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        1
    //    simd4        0        3        0
    // Totals...
    // yes simd        0        4        1
    //  no simd        0       13        1
    fn geometric_quotient(self, other: Horizon) -> Self::Output {
        use crate::elements::*;
        let geometric_product = Horizon::from_groups(/* e321 */ 1.0 / other[e321] * -1.0);
        return Motor::from_groups(
            // e41, e42, e43, e1234
            Simd32x4::from(geometric_product[e321]) * self.group1().xyz().with_w(self[e4]),
            // e23, e31, e12, scalar
            Simd32x4::from(geometric_product[e321]) * self.group0().xyz().with_w(self[e321]) * Simd32x4::from(-1.0),
        );
    }
}
impl GeometricQuotient<Line> for Flector {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       18       26        0
    //    simd3        0        3        0
    //    simd4        3        2        0
    // Totals...
    // yes simd       21       31        0
    //  no simd       30       43        0
    fn geometric_quotient(self, other: Line) -> Self::Output {
        use crate::elements::*;
        let other = Scalar::from_groups(/* scalar */ -f32::powi(other[e23], 2) - f32::powi(other[e31], 2) - f32::powi(other[e12], 2));
        let geometric_product = Line::from_groups(
            // e41, e42, e43
            Simd32x3::from(other[scalar]) * other.group0(),
            // e23, e31, e12
            Simd32x3::from(other[scalar]) * other.group1(),
        );
        return Flector::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from([
                (self[e3] * geometric_product[e31]) + (self[e321] * geometric_product[e23]),
                (self[e1] * geometric_product[e12]) + (self[e321] * geometric_product[e31]),
                (self[e2] * geometric_product[e23]) + (self[e321] * geometric_product[e12]),
                -(self[e2] * geometric_product[e42])
                    - (self[e3] * geometric_product[e43])
                    - (self[e423] * geometric_product[e23])
                    - (self[e431] * geometric_product[e31])
                    - (self[e412] * geometric_product[e12]),
            ]) - (self.group0().yzxx() * geometric_product.group1().zxy().with_w(geometric_product[e41])),
            // e423, e431, e412, e321
            Simd32x4::from([
                (self[e3] * geometric_product[e42]) + (self[e4] * geometric_product[e23]) + (self[e412] * geometric_product[e31]) + (self[e321] * geometric_product[e41]),
                (self[e1] * geometric_product[e43]) + (self[e4] * geometric_product[e31]) + (self[e423] * geometric_product[e12]) + (self[e321] * geometric_product[e42]),
                (self[e2] * geometric_product[e41]) + (self[e4] * geometric_product[e12]) + (self[e431] * geometric_product[e23]) + (self[e321] * geometric_product[e43]),
                self[e3] * geometric_product[e12] * -1.0,
            ]) - (self.group0().yzxx() * geometric_product.group0().zxy().with_w(geometric_product[e23]))
                - (geometric_product.group1().zxy() * self.group1().yzx()).with_w(self[e2] * geometric_product[e31]),
        );
    }
}
impl GeometricQuotient<Motor> for Flector {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       19       27        0
    //    simd3        0        2        0
    //    simd4        6        6        0
    // Totals...
    // yes simd       25       35        0
    //  no simd       43       57        0
    fn geometric_quotient(self, other: Motor) -> Self::Output {
        use crate::elements::*;
        let other = Scalar::from_groups(
            // scalar
            f32::powi(other[scalar], 2) - f32::powi(other[e23], 2) - f32::powi(other[e31], 2) - f32::powi(other[e12], 2),
        );
        let geometric_product = Motor::from_groups(
            // e41, e42, e43, e1234
            Simd32x4::from(other[scalar]) * other.group0(),
            // e23, e31, e12, scalar
            Simd32x4::from(other[scalar]) * other.group1(),
        );
        return Flector::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from([
                self[e321] * geometric_product[e23],
                self[e321] * geometric_product[e31],
                self[e321] * geometric_product[e12],
                -(self[e2] * geometric_product[e42])
                    - (self[e3] * geometric_product[e43])
                    - (self[e423] * geometric_product[e23])
                    - (self[e431] * geometric_product[e31])
                    - (self[e412] * geometric_product[e12]),
            ]) + (self.group0().xxyw() * geometric_product.group1().wzxw())
                + (self.group0().zyz() * geometric_product.group1().yww()).with_w(self[e321] * geometric_product[e1234])
                - (self.group0().yzxx() * geometric_product.group1().zxy().with_w(geometric_product[e41])),
            // e423, e431, e412, e321
            Simd32x4::from([
                (self[e3] * geometric_product[e42])
                    + (self[e4] * geometric_product[e23])
                    + (self[e423] * geometric_product[scalar])
                    + (self[e412] * geometric_product[e31])
                    + (self[e321] * geometric_product[e41]),
                (self[e2] * geometric_product[e1234])
                    + (self[e4] * geometric_product[e31])
                    + (self[e423] * geometric_product[e12])
                    + (self[e431] * geometric_product[scalar])
                    + (self[e321] * geometric_product[e42]),
                (self[e3] * geometric_product[e1234])
                    + (self[e4] * geometric_product[e12])
                    + (self[e431] * geometric_product[e23])
                    + (self[e412] * geometric_product[scalar])
                    + (self[e321] * geometric_product[e43]),
                self[e3] * geometric_product[e12] * -1.0,
            ]) + (self.group0().xxy() * geometric_product.group0().wzx()).with_w(self[e321] * geometric_product[scalar])
                - (self.group0().yzxx() * geometric_product.group0().zxy().with_w(geometric_product[e23]))
                - (geometric_product.group1().zxyy() * self.group1().yzx().with_w(self[e2])),
        );
    }
}
impl GeometricQuotient<MultiVector> for Flector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       26       30        0
    //    simd2        4        5        0
    //    simd3       10       15        0
    //    simd4        6        7        0
    // Totals...
    // yes simd       46       57        0
    //  no simd       88      113        0
    fn geometric_quotient(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        let other = Scalar::from_groups(
            // scalar
            f32::powi(other[scalar], 2) + f32::powi(other[e1], 2) + f32::powi(other[e2], 2) + f32::powi(other[e3], 2)
                - f32::powi(other[e23], 2)
                - f32::powi(other[e31], 2)
                - f32::powi(other[e12], 2)
                - f32::powi(other[e321], 2),
        );
        let geometric_product = MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from(other[scalar]) * other.group0(),
            // e1, e2, e3, e4
            Simd32x4::from(other[scalar]) * other.group1(),
            // e41, e42, e43
            Simd32x3::from(other[scalar]) * other.group2(),
            // e23, e31, e12
            Simd32x3::from(other[scalar]) * other.group3(),
            // e423, e431, e412, e321
            Simd32x4::from(other[scalar]) * other.group4(),
        );
        return MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([
                0.0,
                (self[e4] * geometric_product[e321]) - (self[e431] * geometric_product[e2]) - (self[e412] * geometric_product[e3]) - (self[e321] * geometric_product[e4]),
            ]) + (Simd32x2::from(self[e1]) * Simd32x2::from([geometric_product[e1], geometric_product[e423]]))
                + (Simd32x2::from(self[e2]) * Simd32x2::from([geometric_product[e2], geometric_product[e431]]))
                + (Simd32x2::from(self[e3]) * Simd32x2::from([geometric_product[e3], geometric_product[e412]]))
                - (Simd32x2::from([geometric_product[e321], geometric_product[e1]]) * self.group1().wx()),
            // e1, e2, e3, e4
            Simd32x4::from([
                self[e3] * geometric_product[e31],
                self[e1] * geometric_product[e12],
                self[e321] * geometric_product[e12],
                -(self[e2] * geometric_product[e42])
                    - (self[e3] * geometric_product[e43])
                    - (self[e423] * geometric_product[e23])
                    - (self[e431] * geometric_product[e31])
                    - (self[e412] * geometric_product[e12]),
            ]) + (Simd32x4::from(geometric_product[scalar]) * self.group0())
                + (self.group1().ww().with_zw(self[e2], self[e321]) * geometric_product.group3().xyx().with_w(geometric_product[e1234]))
                - (self.group0().yzxx() * geometric_product.group3().zxy().with_w(geometric_product[e41])),
            // e41, e42, e43
            (Simd32x3::from(self[e4]) * geometric_product.group1().xyz())
                + (Simd32x3::from([geometric_product[e2], geometric_product[e321], geometric_product[e321]]) * self.group1().zyz())
                + (Simd32x3::from([geometric_product[e321], geometric_product[e3], geometric_product[e1]]) * self.group1().xxy())
                + (self.group0().yzx() * geometric_product.group4().zxy())
                - (Simd32x3::from(self[e321]) * geometric_product.group4().xyz())
                - (Simd32x3::from([geometric_product[e4], geometric_product[e412], geometric_product[e423]]) * self.group0().xxy())
                - (Simd32x3::from([geometric_product[e431], geometric_product[e4], geometric_product[e4]]) * self.group0().zyz())
                - (self.group1().yzx() * geometric_product.group1().zxy()),
            // e23, e31, e12
            (self.group0().yzx() * geometric_product.group1().zxy())
                - (Simd32x3::from(self[e321]) * geometric_product.group1().xyz())
                - (Simd32x3::from([geometric_product[e2], geometric_product[e321], geometric_product[e321]]) * self.group0().zyz())
                - (Simd32x3::from([geometric_product[e321], geometric_product[e3], geometric_product[e1]]) * self.group0().xxy()),
            // e423, e431, e412, e321
            Simd32x4::from([
                (self[e1] * geometric_product[e1234])
                    + (self[e3] * geometric_product[e42])
                    + (self[e4] * geometric_product[e23])
                    + (self[e412] * geometric_product[e31])
                    + (self[e321] * geometric_product[e41]),
                (self[e1] * geometric_product[e43])
                    + (self[e2] * geometric_product[e1234])
                    + (self[e4] * geometric_product[e31])
                    + (self[e423] * geometric_product[e12])
                    + (self[e321] * geometric_product[e42]),
                (self[e2] * geometric_product[e41])
                    + (self[e3] * geometric_product[e1234])
                    + (self[e4] * geometric_product[e12])
                    + (self[e431] * geometric_product[e23])
                    + (self[e321] * geometric_product[e43]),
                self[e3] * geometric_product[e12] * -1.0,
            ]) + (Simd32x4::from(geometric_product[scalar]) * self.group1())
                - (self.group0().yzxx() * geometric_product.group2().zxy().with_w(geometric_product[e23]))
                - (geometric_product.group3().zxy() * self.group1().yzx()).with_w(self[e2] * geometric_product[e31]),
        );
    }
}
impl GeometricQuotient<Plane> for Flector {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        9        0
    //    simd4        2        5        0
    // Totals...
    // yes simd        6       14        0
    //  no simd       12       29        0
    fn geometric_quotient(self, other: Plane) -> Self::Output {
        use crate::elements::*;
        let geometric_product = Plane::from_groups(/* e423, e431, e412, e321 */ Simd32x4::from(f32::powi(other[e321], -2) * -1.0) * other.group0());
        return Motor::from_groups(
            // e41, e42, e43, e1234
            Simd32x4::from([
                -(self[e3] * geometric_product[e431]) - (self[e321] * geometric_product[e423]),
                -(self[e1] * geometric_product[e412]) - (self[e321] * geometric_product[e431]),
                -(self[e2] * geometric_product[e423]) - (self[e321] * geometric_product[e412]),
                (self[e3] * geometric_product[e412]) + (self[e4] * geometric_product[e321]),
            ]) + (self.group0().yzxx() * geometric_product.group0().zxyx())
                + (geometric_product.group0().wwwy() * self.group1().xyz().with_w(self[e2])),
            // e23, e31, e12, scalar
            Simd32x4::from(geometric_product[e321]) * self.group0().xyz().with_w(self[e321]) * Simd32x4::from(-1.0),
        );
    }
}
impl GeometricQuotient<Point> for Flector {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       10       16        0
    //    simd4        3        4        0
    // Totals...
    // yes simd       13       20        0
    //  no simd       22       32        0
    fn geometric_quotient(self, other: Point) -> Self::Output {
        use crate::elements::*;
        let geometric_product = Point::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from(f32::powi(other[e1], 2) + f32::powi(other[e2], 2) + f32::powi(other[e3], 2)) * other.group0(),
        );
        return Motor::from_groups(
            // e41, e42, e43, e1234
            Simd32x4::from([
                (self[e4] * geometric_product[e1]) + (self[e412] * geometric_product[e2]),
                (self[e4] * geometric_product[e2]) + (self[e423] * geometric_product[e3]),
                (self[e4] * geometric_product[e3]) + (self[e431] * geometric_product[e1]),
                -(self[e412] * geometric_product[e3]) - (self[e321] * geometric_product[e4]),
            ]) - (self.group1().yzxy() * geometric_product.group0().zxyy())
                - (geometric_product.group0().wwwx() * self.group0().xyz().with_w(self[e423])),
            // e23, e31, e12, scalar
            Simd32x4::from([
                -(self[e3] * geometric_product[e2]) - (self[e321] * geometric_product[e1]),
                -(self[e1] * geometric_product[e3]) - (self[e321] * geometric_product[e2]),
                -(self[e2] * geometric_product[e1]) - (self[e321] * geometric_product[e3]),
                (self[e2] * geometric_product[e2]) + (self[e3] * geometric_product[e3]),
            ]) + (self.group0().yzxx() * geometric_product.group0().zxyx()),
        );
    }
}
impl GeometricQuotient<Scalar> for Flector {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        0        1
    //    simd4        0        2        0
    // Totals...
    // yes simd        0        2        1
    //  no simd        0        8        1
    fn geometric_quotient(self, other: Scalar) -> Self::Output {
        use crate::elements::*;
        let geometric_product = Scalar::from_groups(/* scalar */ 1.0 / other[scalar]);
        return Flector::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from(geometric_product[scalar]) * self.group0(),
            // e423, e431, e412, e321
            Simd32x4::from(geometric_product[scalar]) * self.group1(),
        );
    }
}
impl std::ops::Div<geometric_quotient> for Horizon {
    type Output = geometric_quotient_partial<Horizon>;
    fn div(self, _rhs: geometric_quotient) -> Self::Output {
        geometric_quotient_partial(self)
    }
}
impl GeometricQuotient<DualNum> for Horizon {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        2        0
    //    simd2        0        1        0
    // Totals...
    // yes simd        0        3        0
    //  no simd        0        4        0
    fn geometric_quotient(self, other: DualNum) -> Self::Output {
        use crate::elements::*;
        let geometric_product = DualNum::from_groups(/* scalar, e1234 */ Simd32x2::from(f32::powi(other[scalar], -2)) * other.group0());
        return Flector::from_groups(
            // e1, e2, e3, e4
            Simd32x3::from(0.0).with_w(geometric_product[e1234] * self[e321]),
            // e423, e431, e412, e321
            Simd32x3::from(0.0).with_w(geometric_product[scalar] * self[e321]),
        );
    }
}
impl GeometricQuotient<Flector> for Horizon {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        0        0
    //    simd4        0        6        0
    // Totals...
    // yes simd        3        6        0
    //  no simd        3       24        0
    fn geometric_quotient(self, other: Flector) -> Self::Output {
        use crate::elements::*;
        let other = Scalar::from_groups(
            // scalar
            f32::powi(other[e1], 2) + f32::powi(other[e2], 2) + f32::powi(other[e3], 2) - f32::powi(other[e321], 2),
        );
        let geometric_product = Flector::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from(other[scalar]) * other.group0(),
            // e423, e431, e412, e321
            Simd32x4::from(other[scalar]) * other.group1(),
        );
        return Motor::from_groups(
            // e41, e42, e43, e1234
            Simd32x4::from(self[e321]) * geometric_product.group1().xyz().with_w(geometric_product[e4]) * Simd32x4::from(-1.0),
            // e23, e31, e12, scalar
            Simd32x4::from(self[e321]) * geometric_product.group0().xyz().with_w(geometric_product[e321]) * Simd32x4::from(-1.0),
        );
    }
}
impl GeometricQuotient<Horizon> for Horizon {
    type Output = Scalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        4        0
    fn geometric_quotient(self, other: Horizon) -> Self::Output {
        use crate::elements::*;
        let other = Scalar::from_groups(/* scalar */ f32::powi(other[e321], -2) * -1.0);
        return Scalar::from_groups(/* scalar */ other[e321] * self[e321] * other[scalar] * -1.0);
    }
}
impl GeometricQuotient<Line> for Horizon {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        0        0
    //    simd3        0        2        0
    //    simd4        0        4        0
    // Totals...
    // yes simd        2        6        0
    //  no simd        2       22        0
    fn geometric_quotient(self, other: Line) -> Self::Output {
        use crate::elements::*;
        let other = Scalar::from_groups(/* scalar */ -f32::powi(other[e23], 2) - f32::powi(other[e31], 2) - f32::powi(other[e12], 2));
        let geometric_product = Line::from_groups(
            // e41, e42, e43
            Simd32x3::from(other[scalar]) * other.group0(),
            // e23, e31, e12
            Simd32x3::from(other[scalar]) * other.group1(),
        );
        return Flector::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from([self[e321], self[e321], self[e321], 0.0]) * geometric_product.group1().with_w(0.0) * Simd32x4::from([1.0, 1.0, 1.0, 0.0]),
            // e423, e431, e412, e321
            Simd32x4::from([self[e321], self[e321], self[e321], 0.0]) * geometric_product.group0().with_w(0.0) * Simd32x4::from([1.0, 1.0, 1.0, 0.0]),
        );
    }
}
impl GeometricQuotient<Motor> for Horizon {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        0        0
    //    simd4        0        4        0
    // Totals...
    // yes simd        3        4        0
    //  no simd        3       16        0
    fn geometric_quotient(self, other: Motor) -> Self::Output {
        use crate::elements::*;
        let other = Scalar::from_groups(
            // scalar
            f32::powi(other[scalar], 2) - f32::powi(other[e23], 2) - f32::powi(other[e31], 2) - f32::powi(other[e12], 2),
        );
        let geometric_product = Motor::from_groups(
            // e41, e42, e43, e1234
            Simd32x4::from(other[scalar]) * other.group0(),
            // e23, e31, e12, scalar
            Simd32x4::from(other[scalar]) * other.group1(),
        );
        return Flector::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from(self[e321]) * geometric_product.group1().xyz().with_w(geometric_product[e1234]),
            // e423, e431, e412, e321
            Simd32x4::from(self[e321]) * geometric_product.group0().xyz().with_w(geometric_product[scalar]),
        );
    }
}
impl GeometricQuotient<MultiVector> for Horizon {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        7        0        0
    //    simd2        0        3        0
    //    simd3        0        6        0
    //    simd4        0        4        0
    // Totals...
    // yes simd        7       13        0
    //  no simd        7       40        0
    fn geometric_quotient(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        let other = Scalar::from_groups(
            // scalar
            f32::powi(other[scalar], 2) + f32::powi(other[e1], 2) + f32::powi(other[e2], 2) + f32::powi(other[e3], 2)
                - f32::powi(other[e23], 2)
                - f32::powi(other[e31], 2)
                - f32::powi(other[e12], 2)
                - f32::powi(other[e321], 2),
        );
        let geometric_product = MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from(other[scalar]) * other.group0(),
            // e1, e2, e3, e4
            Simd32x4::from(other[scalar]) * other.group1(),
            // e41, e42, e43
            Simd32x3::from(other[scalar]) * other.group2(),
            // e23, e31, e12
            Simd32x3::from(other[scalar]) * other.group3(),
            // e423, e431, e412, e321
            Simd32x4::from(other[scalar]) * other.group4(),
        );
        return MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from(self[e321]) * Simd32x2::from([geometric_product[e321], geometric_product[e4]]) * Simd32x2::from(-1.0),
            // e1, e2, e3, e4
            Simd32x4::from(self[e321]) * geometric_product.group3().with_w(geometric_product[e1234]),
            // e41, e42, e43
            Simd32x3::from(self[e321]) * geometric_product.group4().xyz() * Simd32x3::from(-1.0),
            // e23, e31, e12
            Simd32x3::from(self[e321]) * geometric_product.group1().xyz() * Simd32x3::from(-1.0),
            // e423, e431, e412, e321
            Simd32x4::from(self[e321]) * geometric_product.group2().with_w(geometric_product[scalar]),
        );
    }
}
impl GeometricQuotient<Plane> for Horizon {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        3        0
    //    simd4        0        3        0
    // Totals...
    // yes simd        0        6        0
    //  no simd        0       15        0
    fn geometric_quotient(self, other: Plane) -> Self::Output {
        use crate::elements::*;
        let geometric_product = Plane::from_groups(/* e423, e431, e412, e321 */ Simd32x4::from(f32::powi(other[e321], -2) * -1.0) * other.group0());
        return Motor::from_groups(
            // e41, e42, e43, e1234
            Simd32x4::from([self[e321], self[e321], self[e321], 0.0]) * geometric_product.group0().xyz().with_w(0.0) * Simd32x4::from([-1.0, -1.0, -1.0, 0.0]),
            // e23, e31, e12, scalar
            Simd32x3::from(0.0).with_w(self[e321] * geometric_product[e321] * -1.0),
        );
    }
}
impl GeometricQuotient<Point> for Horizon {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        2        0
    //    simd4        0        3        0
    // Totals...
    // yes simd        2        5        0
    //  no simd        2       14        0
    fn geometric_quotient(self, other: Point) -> Self::Output {
        use crate::elements::*;
        let geometric_product = Point::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from(f32::powi(other[e1], 2) + f32::powi(other[e2], 2) + f32::powi(other[e3], 2)) * other.group0(),
        );
        return Motor::from_groups(
            // e41, e42, e43, e1234
            Simd32x3::from(0.0).with_w(self[e321] * geometric_product[e4] * -1.0),
            // e23, e31, e12, scalar
            Simd32x4::from([self[e321], self[e321], self[e321], 0.0]) * geometric_product.group0().xyz().with_w(0.0) * Simd32x4::from([-1.0, -1.0, -1.0, 0.0]),
        );
    }
}
impl GeometricQuotient<Scalar> for Horizon {
    type Output = Horizon;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        2        0
    fn geometric_quotient(self, other: Scalar) -> Self::Output {
        use crate::elements::*;
        let other = Scalar::from_groups(/* scalar */ f32::powi(other[scalar], -2));
        return Horizon::from_groups(/* e321 */ self[e321] * other[scalar] * other[scalar]);
    }
}
impl std::ops::Div<geometric_quotient> for Line {
    type Output = geometric_quotient_partial<Line>;
    fn div(self, _rhs: geometric_quotient) -> Self::Output {
        geometric_quotient_partial(self)
    }
}
impl GeometricQuotient<DualNum> for Line {
    type Output = Line;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd2        0        1        0
    //    simd3        1        3        0
    // Totals...
    // yes simd        1        4        0
    //  no simd        3       11        0
    fn geometric_quotient(self, other: DualNum) -> Self::Output {
        use crate::elements::*;
        let geometric_product = DualNum::from_groups(/* scalar, e1234 */ Simd32x2::from(f32::powi(other[scalar], -2)) * other.group0());
        return Line::from_groups(
            // e41, e42, e43
            (Simd32x3::from(geometric_product[scalar]) * self.group0()) + (Simd32x3::from(geometric_product[e1234]) * self.group1()),
            // e23, e31, e12
            Simd32x3::from(geometric_product[scalar]) * self.group1(),
        );
    }
}
impl GeometricQuotient<Flector> for Line {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       11       14        0
    //    simd3        0        2        0
    //    simd4        6        6        0
    // Totals...
    // yes simd       17       22        0
    //  no simd       35       44        0
    fn geometric_quotient(self, other: Flector) -> Self::Output {
        use crate::elements::*;
        let other = Scalar::from_groups(
            // scalar
            f32::powi(other[e1], 2) + f32::powi(other[e2], 2) + f32::powi(other[e3], 2) - f32::powi(other[e321], 2),
        );
        let geometric_product = Flector::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from(other[scalar]) * other.group0(),
            // e423, e431, e412, e321
            Simd32x4::from(other[scalar]) * other.group1(),
        );
        return Flector::from_groups(
            // e1, e2, e3, e4
            (Simd32x4::from([geometric_product[e2], geometric_product[e321], geometric_product[e321], geometric_product[e2]]) * self.group1().zyz().with_w(self[e42]))
                + (Simd32x4::from([geometric_product[e321], geometric_product[e3], geometric_product[e1], geometric_product[e1]]) * self.group1().xxy().with_w(self[e41]))
                + Simd32x3::from(0.0).with_w((geometric_product[e3] * self[e43]) - (geometric_product[e431] * self[e31]) - (geometric_product[e412] * self[e12]))
                - (self.group1().yzx() * geometric_product.group0().zxy()).with_w(geometric_product[e423] * self[e23]),
            // e423, e431, e412, e321
            Simd32x4::from([
                (geometric_product[e3] * self[e42]) + (geometric_product[e4] * self[e23]) + (geometric_product[e431] * self[e12]),
                (geometric_product[e1] * self[e43]) + (geometric_product[e4] * self[e31]) + (geometric_product[e412] * self[e23]),
                (geometric_product[e2] * self[e41]) + (geometric_product[e4] * self[e12]) + (geometric_product[e423] * self[e31]),
                0.0,
            ]) - (Simd32x4::from([geometric_product[e2], geometric_product[e321], geometric_product[e321], geometric_product[e2]]) * self.group0().zyz().with_w(self[e31]))
                - (Simd32x4::from([geometric_product[e321], geometric_product[e3], geometric_product[e1], geometric_product[e1]]) * self.group0().xxy().with_w(self[e23]))
                - (self.group1().yzx() * geometric_product.group1().zxy()).with_w(geometric_product[e3] * self[e12]),
        );
    }
}
impl GeometricQuotient<Horizon> for Line {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        1
    //    simd4        0        4        0
    // Totals...
    // yes simd        0        5        1
    //  no simd        0       17        1
    fn geometric_quotient(self, other: Horizon) -> Self::Output {
        use crate::elements::*;
        let geometric_product = Horizon::from_groups(/* e321 */ 1.0 / other[e321] * -1.0);
        return Flector::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from([geometric_product[e321], geometric_product[e321], geometric_product[e321], 0.0]) * self.group1().with_w(0.0) * Simd32x4::from([1.0, 1.0, 1.0, 0.0]),
            // e423, e431, e412, e321
            Simd32x4::from([geometric_product[e321], geometric_product[e321], geometric_product[e321], 0.0]) * self.group0().with_w(0.0) * Simd32x4::from([-1.0, -1.0, -1.0, 0.0]),
        );
    }
}
impl GeometricQuotient<Line> for Line {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        9       18        0
    //    simd3        0        5        0
    //    simd4        3        0        0
    // Totals...
    // yes simd       12       23        0
    //  no simd       21       33        0
    fn geometric_quotient(self, other: Line) -> Self::Output {
        use crate::elements::*;
        let other = Scalar::from_groups(/* scalar */ -f32::powi(other[e23], 2) - f32::powi(other[e31], 2) - f32::powi(other[e12], 2));
        let geometric_product = Line::from_groups(
            // e41, e42, e43
            Simd32x3::from(other[scalar]) * other.group0(),
            // e23, e31, e12
            Simd32x3::from(other[scalar]) * other.group1(),
        );
        return Motor::from_groups(
            // e41, e42, e43, e1234
            Simd32x4::from([
                (geometric_product[e42] * self[e12]) + (geometric_product[e31] * self[e43]),
                (geometric_product[e43] * self[e23]) + (geometric_product[e12] * self[e41]),
                (geometric_product[e41] * self[e31]) + (geometric_product[e23] * self[e42]),
                -(geometric_product[e43] * self[e12]) - (geometric_product[e23] * self[e41]) - (geometric_product[e31] * self[e42]) - (geometric_product[e12] * self[e43]),
            ]) - (geometric_product.group0().zxy() * self.group1().yzx()).with_w(geometric_product[e41] * self[e23])
                - (geometric_product.group1().zxy() * self.group0().yzx()).with_w(geometric_product[e42] * self[e31]),
            // e23, e31, e12, scalar
            Simd32x4::from([
                geometric_product[e31] * self[e12],
                geometric_product[e12] * self[e23],
                geometric_product[e23] * self[e31],
                -(geometric_product[e31] * self[e31]) - (geometric_product[e12] * self[e12]),
            ]) - (geometric_product.group1().zxy() * self.group1().yzx()).with_w(geometric_product[e23] * self[e23]),
        );
    }
}
impl GeometricQuotient<Motor> for Line {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       19       25        0
    //    simd3        0        1        0
    //    simd4        3        4        0
    // Totals...
    // yes simd       22       30        0
    //  no simd       31       44        0
    fn geometric_quotient(self, other: Motor) -> Self::Output {
        use crate::elements::*;
        let other = Scalar::from_groups(
            // scalar
            f32::powi(other[scalar], 2) - f32::powi(other[e23], 2) - f32::powi(other[e31], 2) - f32::powi(other[e12], 2),
        );
        let geometric_product = Motor::from_groups(
            // e41, e42, e43, e1234
            Simd32x4::from(other[scalar]) * other.group0(),
            // e23, e31, e12, scalar
            Simd32x4::from(other[scalar]) * other.group1(),
        );
        return Motor::from_groups(
            // e41, e42, e43, e1234
            Simd32x4::from([
                (self[e41] * geometric_product[scalar]) + (self[e43] * geometric_product[e31]) + (self[e23] * geometric_product[e1234]) + (self[e12] * geometric_product[e42]),
                (self[e41] * geometric_product[e12]) + (self[e42] * geometric_product[scalar]) + (self[e23] * geometric_product[e43]) + (self[e31] * geometric_product[e1234]),
                (self[e42] * geometric_product[e23]) + (self[e43] * geometric_product[scalar]) + (self[e31] * geometric_product[e41]) + (self[e12] * geometric_product[e1234]),
                -(self[e43] * geometric_product[e12]) - (self[e23] * geometric_product[e41]) - (self[e31] * geometric_product[e42]) - (self[e12] * geometric_product[e43]),
            ]) - (geometric_product.group1().zxyx() * self.group0().yzx().with_w(self[e41]))
                - (self.group1().yzx() * geometric_product.group0().zxy()).with_w(self[e42] * geometric_product[e31]),
            // e23, e31, e12, scalar
            Simd32x4::from([
                (self[e23] * geometric_product[scalar]) + (self[e12] * geometric_product[e31]),
                (self[e23] * geometric_product[e12]) + (self[e31] * geometric_product[scalar]),
                (self[e31] * geometric_product[e23]) + (self[e12] * geometric_product[scalar]),
                -(self[e31] * geometric_product[e31]) - (self[e12] * geometric_product[e12]),
            ]) - (geometric_product.group1().zxyx() * self.group1().yzx().with_w(self[e23])),
        );
    }
}
impl GeometricQuotient<MultiVector> for Line {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       17       17        0
    //    simd2        3        4        0
    //    simd3        7       13        0
    //    simd4        6        6        0
    // Totals...
    // yes simd       33       40        0
    //  no simd       68       88        0
    fn geometric_quotient(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        let other = Scalar::from_groups(
            // scalar
            f32::powi(other[scalar], 2) + f32::powi(other[e1], 2) + f32::powi(other[e2], 2) + f32::powi(other[e3], 2)
                - f32::powi(other[e23], 2)
                - f32::powi(other[e31], 2)
                - f32::powi(other[e12], 2)
                - f32::powi(other[e321], 2),
        );
        let geometric_product = MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from(other[scalar]) * other.group0(),
            // e1, e2, e3, e4
            Simd32x4::from(other[scalar]) * other.group1(),
            // e41, e42, e43
            Simd32x3::from(other[scalar]) * other.group2(),
            // e23, e31, e12
            Simd32x3::from(other[scalar]) * other.group3(),
            // e423, e431, e412, e321
            Simd32x4::from(other[scalar]) * other.group4(),
        );
        return MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([0.0, -(self[e23] * geometric_product[e41]) - (self[e31] * geometric_product[e42]) - (self[e12] * geometric_product[e43])])
                - (Simd32x2::from(geometric_product[e23]) * Simd32x2::from([self[e23], self[e41]]))
                - (Simd32x2::from(geometric_product[e31]) * Simd32x2::from([self[e31], self[e42]]))
                - (Simd32x2::from(geometric_product[e12]) * Simd32x2::from([self[e12], self[e43]])),
            // e1, e2, e3, e4
            (Simd32x4::from([geometric_product[e2], geometric_product[e321], geometric_product[e321], geometric_product[e2]]) * self.group1().zyz().with_w(self[e42]))
                + (Simd32x4::from([geometric_product[e321], geometric_product[e3], geometric_product[e1], geometric_product[e1]]) * self.group1().xxy().with_w(self[e41]))
                + Simd32x3::from(0.0).with_w((self[e43] * geometric_product[e3]) - (self[e31] * geometric_product[e431]) - (self[e12] * geometric_product[e412]))
                - (self.group1().yzx() * geometric_product.group1().zxy()).with_w(self[e23] * geometric_product[e423]),
            // e41, e42, e43
            (Simd32x3::from(geometric_product[scalar]) * self.group0())
                + (Simd32x3::from(geometric_product[e1234]) * self.group1())
                + (self.group0().zxy() * geometric_product.group3().yzx())
                + (self.group1().zxy() * geometric_product.group2().yzx())
                - (self.group0().yzx() * geometric_product.group3().zxy())
                - (self.group1().yzx() * geometric_product.group2().zxy()),
            // e23, e31, e12
            (Simd32x3::from(geometric_product[scalar]) * self.group1()) + (self.group1().zxy() * geometric_product.group3().yzx())
                - (self.group1().yzx() * geometric_product.group3().zxy()),
            // e423, e431, e412, e321
            Simd32x4::from([
                (self[e42] * geometric_product[e3]) + (self[e23] * geometric_product[e4]) + (self[e12] * geometric_product[e431]),
                (self[e43] * geometric_product[e1]) + (self[e23] * geometric_product[e412]) + (self[e31] * geometric_product[e4]),
                (self[e41] * geometric_product[e2]) + (self[e31] * geometric_product[e423]) + (self[e12] * geometric_product[e4]),
                0.0,
            ]) - (Simd32x4::from([geometric_product[e2], geometric_product[e321], geometric_product[e321], geometric_product[e2]]) * self.group0().zyz().with_w(self[e31]))
                - (Simd32x4::from([geometric_product[e321], geometric_product[e3], geometric_product[e1], geometric_product[e1]]) * self.group0().xxy().with_w(self[e23]))
                - (self.group1().yzx() * geometric_product.group4().zxy()).with_w(self[e12] * geometric_product[e3]),
        );
    }
}
impl GeometricQuotient<Plane> for Line {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        4        0
    //    simd3        0        3        0
    //    simd4        2        2        0
    // Totals...
    // yes simd        4        9        0
    //  no simd       10       21        0
    fn geometric_quotient(self, other: Plane) -> Self::Output {
        use crate::elements::*;
        let geometric_product = Plane::from_groups(/* e423, e431, e412, e321 */ Simd32x4::from(f32::powi(other[e321], -2) * -1.0) * other.group0());
        return Flector::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from([geometric_product[e321], geometric_product[e321], geometric_product[e321], 1.0])
                * self
                    .group1()
                    .with_w(-(self[e23] * geometric_product[e423]) - (self[e31] * geometric_product[e431]) - (self[e12] * geometric_product[e412])),
            // e423, e431, e412, e321
            (self.group1().zxy() * geometric_product.group0().yzx()).with_w(0.0)
                - (Simd32x3::from(geometric_product[e321]) * self.group0()).with_w(0.0)
                - (self.group1().yzx() * geometric_product.group0().zxy()).with_w(0.0),
        );
    }
}
impl GeometricQuotient<Point> for Line {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        7       16        0
    //    simd4        2        3        0
    // Totals...
    // yes simd        9       19        0
    //  no simd       15       28        0
    fn geometric_quotient(self, other: Point) -> Self::Output {
        use crate::elements::*;
        let geometric_product = Point::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from(f32::powi(other[e1], 2) + f32::powi(other[e2], 2) + f32::powi(other[e3], 2)) * other.group0(),
        );
        return Flector::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from([
                self[e31] * geometric_product[e3] * -1.0,
                self[e12] * geometric_product[e1] * -1.0,
                self[e23] * geometric_product[e2] * -1.0,
                (self[e42] * geometric_product[e2]) + (self[e43] * geometric_product[e3]),
            ]) + (geometric_product.group0().yzxx() * self.group1().zxy().with_w(self[e41])),
            // e423, e431, e412, e321
            Simd32x4::from([
                (self[e42] * geometric_product[e3]) + (self[e23] * geometric_product[e4]),
                (self[e43] * geometric_product[e1]) + (self[e31] * geometric_product[e4]),
                (self[e41] * geometric_product[e2]) + (self[e12] * geometric_product[e4]),
                -(self[e31] * geometric_product[e2]) - (self[e12] * geometric_product[e3]),
            ]) - (geometric_product.group0().yzxx() * self.group0().zxy().with_w(self[e23])),
        );
    }
}
impl GeometricQuotient<Scalar> for Line {
    type Output = Line;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        0        1
    //    simd3        0        2        0
    // Totals...
    // yes simd        0        2        1
    //  no simd        0        6        1
    fn geometric_quotient(self, other: Scalar) -> Self::Output {
        use crate::elements::*;
        let geometric_product = Scalar::from_groups(/* scalar */ 1.0 / other[scalar]);
        return Line::from_groups(
            // e41, e42, e43
            Simd32x3::from(geometric_product[scalar]) * self.group0(),
            // e23, e31, e12
            Simd32x3::from(geometric_product[scalar]) * self.group1(),
        );
    }
}
impl std::ops::Div<geometric_quotient> for Motor {
    type Output = geometric_quotient_partial<Motor>;
    fn div(self, _rhs: geometric_quotient) -> Self::Output {
        geometric_quotient_partial(self)
    }
}
impl GeometricQuotient<DualNum> for Motor {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd2        0        1        0
    //    simd4        1        3        0
    // Totals...
    // yes simd        1        4        0
    //  no simd        4       14        0
    fn geometric_quotient(self, other: DualNum) -> Self::Output {
        use crate::elements::*;
        let geometric_product = DualNum::from_groups(/* scalar, e1234 */ Simd32x2::from(f32::powi(other[scalar], -2)) * other.group0());
        return Motor::from_groups(
            // e41, e42, e43, e1234
            (Simd32x4::from(geometric_product[scalar]) * self.group0()) + (Simd32x4::from(geometric_product[e1234]) * self.group1()),
            // e23, e31, e12, scalar
            Simd32x4::from(geometric_product[scalar]) * self.group1(),
        );
    }
}
impl GeometricQuotient<Flector> for Motor {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       15       18        0
    //    simd3        0        2        0
    //    simd4        8        8        0
    // Totals...
    // yes simd       23       28        0
    //  no simd       47       56        0
    fn geometric_quotient(self, other: Flector) -> Self::Output {
        use crate::elements::*;
        let other = Scalar::from_groups(
            // scalar
            f32::powi(other[e1], 2) + f32::powi(other[e2], 2) + f32::powi(other[e3], 2) - f32::powi(other[e321], 2),
        );
        let geometric_product = Flector::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from(other[scalar]) * other.group0(),
            // e423, e431, e412, e321
            Simd32x4::from(other[scalar]) * other.group1(),
        );
        return Flector::from_groups(
            // e1, e2, e3, e4
            (geometric_product.group0().xyxx() * self.group1().wwy().with_w(self[e41]))
                + (geometric_product.group0().yzzy() * self.group1().zxw().with_w(self[e42]))
                + Simd32x3::from(0.0).with_w(
                    (geometric_product[e4] * self[scalar])
                        - (geometric_product[e431] * self[e31])
                        - (geometric_product[e412] * self[e12])
                        - (geometric_product[e321] * self[e1234]),
                )
                + (geometric_product.group1().www() * self.group1().xyz()).with_w(geometric_product[e3] * self[e43])
                - (self.group1().yzxx() * geometric_product.group0().zxy().with_w(geometric_product[e423])),
            // e423, e431, e412, e321
            Simd32x4::from([
                (geometric_product[e4] * self[e23]) + (geometric_product[e423] * self[scalar]) + (geometric_product[e431] * self[e12]) - (geometric_product[e321] * self[e41]),
                (geometric_product[e4] * self[e31]) + (geometric_product[e431] * self[scalar]) + (geometric_product[e412] * self[e23]) - (geometric_product[e321] * self[e42]),
                (geometric_product[e4] * self[e12]) + (geometric_product[e423] * self[e31]) + (geometric_product[e412] * self[scalar]) - (geometric_product[e321] * self[e43]),
                0.0,
            ]) + (geometric_product.group0().zxy() * self.group0().yzx()).with_w(geometric_product[e321] * self[scalar])
                - (geometric_product.group0().xyxx() * self.group0().wwy().with_w(self[e23]))
                - (geometric_product.group0().yzzy() * self.group0().zxw().with_w(self[e31]))
                - (self.group1().yzxz() * geometric_product.group1().zxy().with_w(geometric_product[e3])),
        );
    }
}
impl GeometricQuotient<Horizon> for Motor {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        1
    //    simd4        0        4        0
    // Totals...
    // yes simd        0        5        1
    //  no simd        0       17        1
    fn geometric_quotient(self, other: Horizon) -> Self::Output {
        use crate::elements::*;
        let geometric_product = Horizon::from_groups(/* e321 */ 1.0 / other[e321] * -1.0);
        return Flector::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from(geometric_product[e321]) * self.group1().xyz().with_w(self[e1234]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e423, e431, e412, e321
            Simd32x4::from(geometric_product[e321]) * self.group0().xyz().with_w(self[scalar]) * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
        );
    }
}
impl GeometricQuotient<Line> for Motor {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       18       25        0
    //    simd3        0        3        0
    //    simd4        3        2        0
    // Totals...
    // yes simd       21       30        0
    //  no simd       30       42        0
    fn geometric_quotient(self, other: Line) -> Self::Output {
        use crate::elements::*;
        let other = Scalar::from_groups(/* scalar */ -f32::powi(other[e23], 2) - f32::powi(other[e31], 2) - f32::powi(other[e12], 2));
        let geometric_product = Line::from_groups(
            // e41, e42, e43
            Simd32x3::from(other[scalar]) * other.group0(),
            // e23, e31, e12
            Simd32x3::from(other[scalar]) * other.group1(),
        );
        return Motor::from_groups(
            // e41, e42, e43, e1234
            Simd32x4::from([
                (geometric_product[e41] * self[scalar]) + (geometric_product[e42] * self[e12]) + (geometric_product[e23] * self[e1234]) + (geometric_product[e31] * self[e43]),
                (geometric_product[e42] * self[scalar]) + (geometric_product[e43] * self[e23]) + (geometric_product[e31] * self[e1234]) + (geometric_product[e12] * self[e41]),
                (geometric_product[e41] * self[e31]) + (geometric_product[e43] * self[scalar]) + (geometric_product[e23] * self[e42]) + (geometric_product[e12] * self[e1234]),
                -(geometric_product[e43] * self[e12]) - (geometric_product[e23] * self[e41]) - (geometric_product[e31] * self[e42]) - (geometric_product[e12] * self[e43]),
            ]) - (self.group1().yzxx() * geometric_product.group0().zxy().with_w(geometric_product[e41]))
                - (geometric_product.group1().zxy() * self.group0().yzx()).with_w(geometric_product[e42] * self[e31]),
            // e23, e31, e12, scalar
            Simd32x4::from([
                (geometric_product[e23] * self[scalar]) + (geometric_product[e31] * self[e12]),
                (geometric_product[e31] * self[scalar]) + (geometric_product[e12] * self[e23]),
                (geometric_product[e23] * self[e31]) + (geometric_product[e12] * self[scalar]),
                -(geometric_product[e31] * self[e31]) - (geometric_product[e12] * self[e12]),
            ]) - (self.group1().yzxx() * geometric_product.group1().zxy().with_w(geometric_product[e23])),
        );
    }
}
impl GeometricQuotient<Motor> for Motor {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       19       26        0
    //    simd3        0        2        0
    //    simd4        6        6        0
    // Totals...
    // yes simd       25       34        0
    //  no simd       43       56        0
    fn geometric_quotient(self, other: Motor) -> Self::Output {
        use crate::elements::*;
        let other = Scalar::from_groups(
            // scalar
            f32::powi(other[scalar], 2) - f32::powi(other[e23], 2) - f32::powi(other[e31], 2) - f32::powi(other[e12], 2),
        );
        let geometric_product = Motor::from_groups(
            // e41, e42, e43, e1234
            Simd32x4::from(other[scalar]) * other.group0(),
            // e23, e31, e12, scalar
            Simd32x4::from(other[scalar]) * other.group1(),
        );
        return Motor::from_groups(
            // e41, e42, e43, e1234
            Simd32x4::from([
                (geometric_product[e1234] * self[e23]) + (geometric_product[e23] * self[e1234]) + (geometric_product[e31] * self[e43]) + (geometric_product[scalar] * self[e41]),
                (geometric_product[e1234] * self[e31]) + (geometric_product[e31] * self[e1234]) + (geometric_product[e12] * self[e41]) + (geometric_product[scalar] * self[e42]),
                (geometric_product[e1234] * self[e12]) + (geometric_product[e23] * self[e42]) + (geometric_product[e12] * self[e1234]) + (geometric_product[scalar] * self[e43]),
                -(geometric_product[e43] * self[e12]) - (geometric_product[e23] * self[e41]) - (geometric_product[e31] * self[e42]) - (geometric_product[e12] * self[e43]),
            ]) + (geometric_product.group0().xyxw() * self.group1().wwyw())
                + (geometric_product.group0().yzz() * self.group1().zxw()).with_w(geometric_product[scalar] * self[e1234])
                - (geometric_product.group0().zxyx() * self.group1().yzxx())
                - (geometric_product.group1().zxy() * self.group0().yzx()).with_w(geometric_product[e42] * self[e31]),
            // e23, e31, e12, scalar
            Simd32x4::from([
                (geometric_product[e31] * self[e12]) + (geometric_product[scalar] * self[e23]),
                (geometric_product[e12] * self[e23]) + (geometric_product[scalar] * self[e31]),
                (geometric_product[e12] * self[scalar]) + (geometric_product[scalar] * self[e12]),
                -(geometric_product[e31] * self[e31]) - (geometric_product[e12] * self[e12]),
            ]) + (geometric_product.group1().xyxw() * self.group1().wwyw())
                - (geometric_product.group1().zxyx() * self.group1().yzxx()),
        );
    }
}
impl GeometricQuotient<MultiVector> for Motor {
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
    fn geometric_quotient(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        let other = Scalar::from_groups(
            // scalar
            f32::powi(other[scalar], 2) + f32::powi(other[e1], 2) + f32::powi(other[e2], 2) + f32::powi(other[e3], 2)
                - f32::powi(other[e23], 2)
                - f32::powi(other[e31], 2)
                - f32::powi(other[e12], 2)
                - f32::powi(other[e321], 2),
        );
        let geometric_product = MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from(other[scalar]) * other.group0(),
            // e1, e2, e3, e4
            Simd32x4::from(other[scalar]) * other.group1(),
            // e41, e42, e43
            Simd32x3::from(other[scalar]) * other.group2(),
            // e23, e31, e12
            Simd32x3::from(other[scalar]) * other.group3(),
            // e423, e431, e412, e321
            Simd32x4::from(other[scalar]) * other.group4(),
        );
        return MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([
                0.0,
                (self[scalar] * geometric_product[e1234]) - (self[e41] * geometric_product[e23]) - (self[e42] * geometric_product[e31]) - (self[e43] * geometric_product[e12]),
            ]) + (Simd32x2::from(geometric_product[scalar]) * Simd32x2::from([self[scalar], self[e1234]]))
                - (Simd32x2::from(self[e23]) * Simd32x2::from([geometric_product[e23], geometric_product[e41]]))
                - (Simd32x2::from(self[e31]) * Simd32x2::from([geometric_product[e31], geometric_product[e42]]))
                - (Simd32x2::from(self[e12]) * Simd32x2::from([geometric_product[e12], geometric_product[e43]])),
            // e1, e2, e3, e4
            (Simd32x4::from([geometric_product[e2], geometric_product[e321], geometric_product[e321], geometric_product[e2]]) * self.group1().zyz().with_w(self[e42]))
                + (Simd32x4::from([geometric_product[e321], geometric_product[e3], geometric_product[e1], geometric_product[e1]]) * self.group1().xxy().with_w(self[e41]))
                + (geometric_product.group1().xyzz() * self.group1().www().with_w(self[e43]))
                + Simd32x3::from(0.0).with_w(
                    (self[scalar] * geometric_product[e4]) - (self[e23] * geometric_product[e423]) - (self[e31] * geometric_product[e431]) - (self[e12] * geometric_product[e412]),
                )
                - (self.group1().yzx() * geometric_product.group1().zxy()).with_w(self[e1234] * geometric_product[e321]),
            // e41, e42, e43
            (Simd32x3::from(geometric_product[scalar]) * self.group0().xyz())
                + (Simd32x3::from(geometric_product[e1234]) * self.group1().xyz())
                + (geometric_product.group2().xyx() * self.group1().wwy())
                + (geometric_product.group2().yzz() * self.group1().zxw())
                + (geometric_product.group3().xyx() * self.group0().wwy())
                + (geometric_product.group3().yzz() * self.group0().zxw())
                - (geometric_product.group2().zxy() * self.group1().yzx())
                - (geometric_product.group3().zxy() * self.group0().yzx()),
            // e23, e31, e12
            (Simd32x3::from(geometric_product[scalar]) * self.group1().xyz())
                + (geometric_product.group3().xyx() * self.group1().wwy())
                + (geometric_product.group3().yzz() * self.group1().zxw())
                - (geometric_product.group3().zxy() * self.group1().yzx()),
            // e423, e431, e412, e321
            Simd32x4::from([
                (self[e23] * geometric_product[e4]) + (self[e12] * geometric_product[e431]) + (self[scalar] * geometric_product[e423]) - (self[e31] * geometric_product[e412]),
                (self[e23] * geometric_product[e412]) + (self[e31] * geometric_product[e4]) + (self[scalar] * geometric_product[e431]) - (self[e12] * geometric_product[e423]),
                (self[e31] * geometric_product[e423]) + (self[e12] * geometric_product[e4]) + (self[scalar] * geometric_product[e412]) - (self[e23] * geometric_product[e431]),
                0.0,
            ]) + (self.group0().yzx() * geometric_product.group1().zxy()).with_w(self[scalar] * geometric_product[e321])
                - (Simd32x4::from([geometric_product[e2], geometric_product[e321], geometric_product[e321], geometric_product[e2]]) * self.group0().zyz().with_w(self[e31]))
                - (Simd32x4::from([geometric_product[e321], geometric_product[e3], geometric_product[e1], geometric_product[e1]]) * self.group0().xxy().with_w(self[e23]))
                - (geometric_product.group1().xyzz() * self.group0().www().with_w(self[e12])),
        );
    }
}
impl GeometricQuotient<Plane> for Motor {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        6        0
    //    simd3        3        4        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        6       12        0
    //  no simd       12       26        0
    fn geometric_quotient(self, other: Plane) -> Self::Output {
        use crate::elements::*;
        let geometric_product = Plane::from_groups(/* e423, e431, e412, e321 */ Simd32x4::from(f32::powi(other[e321], -2) * -1.0) * other.group0());
        return Flector::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from([geometric_product[e321], geometric_product[e321], geometric_product[e321], 1.0])
                * self.group1().xyz().with_w(
                    -(self[e1234] * geometric_product[e321])
                        - (self[e23] * geometric_product[e423])
                        - (self[e31] * geometric_product[e431])
                        - (self[e12] * geometric_product[e412]),
                ),
            // e423, e431, e412, e321
            ((Simd32x3::from(self[scalar]) * geometric_product.group0().xyz()) + (self.group1().zxy() * geometric_product.group0().yzx())
                - (Simd32x3::from(geometric_product[e321]) * self.group0().xyz())
                - (self.group1().yzx() * geometric_product.group0().zxy()))
            .with_w(self[scalar] * geometric_product[e321]),
        );
    }
}
impl GeometricQuotient<Point> for Motor {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        6       16        0
    //    simd4        4        5        0
    // Totals...
    // yes simd       10       21        0
    //  no simd       22       36        0
    fn geometric_quotient(self, other: Point) -> Self::Output {
        use crate::elements::*;
        let geometric_product = Point::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from(f32::powi(other[e1], 2) + f32::powi(other[e2], 2) + f32::powi(other[e3], 2)) * other.group0(),
        );
        return Flector::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from([
                self[e31] * geometric_product[e3] * -1.0,
                self[e12] * geometric_product[e1] * -1.0,
                self[e23] * geometric_product[e2] * -1.0,
                (self[e43] * geometric_product[e3]) + (self[scalar] * geometric_product[e4]),
            ]) + (geometric_product.group0().xyzy() * self.group1().www().with_w(self[e42]))
                + (geometric_product.group0().yzxx() * self.group1().zxy().with_w(self[e41])),
            // e423, e431, e412, e321
            Simd32x4::from([
                (self[e42] * geometric_product[e3]) + (self[e23] * geometric_product[e4]),
                (self[e43] * geometric_product[e1]) + (self[e31] * geometric_product[e4]),
                (self[e41] * geometric_product[e2]) + (self[e12] * geometric_product[e4]),
                self[e12] * geometric_product[e3] * -1.0,
            ]) - (geometric_product.group0().xyzy() * self.group0().www().with_w(self[e31]))
                - (geometric_product.group0().yzxx() * self.group0().zxy().with_w(self[e23])),
        );
    }
}
impl GeometricQuotient<Scalar> for Motor {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        0        1
    //    simd4        0        2        0
    // Totals...
    // yes simd        0        2        1
    //  no simd        0        8        1
    fn geometric_quotient(self, other: Scalar) -> Self::Output {
        use crate::elements::*;
        let geometric_product = Scalar::from_groups(/* scalar */ 1.0 / other[scalar]);
        return Motor::from_groups(
            // e41, e42, e43, e1234
            Simd32x4::from(geometric_product[scalar]) * self.group0(),
            // e23, e31, e12, scalar
            Simd32x4::from(geometric_product[scalar]) * self.group1(),
        );
    }
}
impl std::ops::Div<geometric_quotient> for MultiVector {
    type Output = geometric_quotient_partial<MultiVector>;
    fn div(self, _rhs: geometric_quotient) -> Self::Output {
        geometric_quotient_partial(self)
    }
}
impl GeometricQuotient<DualNum> for MultiVector {
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
    fn geometric_quotient(self, other: DualNum) -> Self::Output {
        use crate::elements::*;
        let geometric_product = DualNum::from_groups(/* scalar, e1234 */ Simd32x2::from(f32::powi(other[scalar], -2)) * other.group0());
        return MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([
                geometric_product[scalar] * self[scalar],
                (geometric_product[scalar] * self[e1234]) + (geometric_product[e1234] * self[scalar]),
            ]),
            // e1, e2, e3, e4
            Simd32x4::from([self[e1], self[e2], self[e3], 1.0])
                * geometric_product
                    .group0()
                    .xx()
                    .with_zw(geometric_product[scalar], (geometric_product[scalar] * self[e4]) + (geometric_product[e1234] * self[e321])),
            // e41, e42, e43
            (Simd32x3::from(geometric_product[scalar]) * self.group2()) + (Simd32x3::from(geometric_product[e1234]) * self.group3()),
            // e23, e31, e12
            Simd32x3::from(geometric_product[scalar]) * self.group3(),
            // e423, e431, e412, e321
            ((Simd32x3::from(geometric_product[scalar]) * self.group4().xyz()) + (Simd32x3::from(geometric_product[e1234]) * self.group1().xyz()))
                .with_w(geometric_product[scalar] * self[e321]),
        );
    }
}
impl GeometricQuotient<Flector> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       18       21        0
    //    simd2        4        4        0
    //    simd3       10       13        0
    //    simd4        8        9        0
    // Totals...
    // yes simd       40       47        0
    //  no simd       88      104        0
    fn geometric_quotient(self, other: Flector) -> Self::Output {
        use crate::elements::*;
        let other = Scalar::from_groups(
            // scalar
            f32::powi(other[e1], 2) + f32::powi(other[e2], 2) + f32::powi(other[e3], 2) - f32::powi(other[e321], 2),
        );
        let geometric_product = Flector::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from(other[scalar]) * other.group0(),
            // e423, e431, e412, e321
            Simd32x4::from(other[scalar]) * other.group1(),
        );
        return MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([
                0.0,
                (geometric_product[e321] * self[e4]) - (geometric_product[e2] * self[e431]) - (geometric_product[e3] * self[e412]) - (geometric_product[e4] * self[e321]),
            ]) + (Simd32x2::from(self[e1]) * Simd32x2::from([geometric_product[e1], geometric_product[e423]]))
                + (Simd32x2::from(self[e2]) * Simd32x2::from([geometric_product[e2], geometric_product[e431]]))
                + (Simd32x2::from(self[e3]) * Simd32x2::from([geometric_product[e3], geometric_product[e412]]))
                - (Simd32x2::from([geometric_product[e321], geometric_product[e1]]) * self.group4().wx()),
            // e1, e2, e3, e4
            (Simd32x4::from(self[scalar]) * geometric_product.group0())
                + (Simd32x4::from([geometric_product[e2], geometric_product[e321], geometric_product[e321], geometric_product[e2]]) * self.group3().zyz().with_w(self[e42]))
                + (Simd32x4::from([geometric_product[e321], geometric_product[e3], geometric_product[e1], geometric_product[e1]]) * self.group3().xxy().with_w(self[e41]))
                + Simd32x3::from(0.0).with_w(
                    (geometric_product[e3] * self[e43]) - (geometric_product[e423] * self[e23]) - (geometric_product[e431] * self[e31]) - (geometric_product[e412] * self[e12]),
                )
                - (self.group3().yzx() * geometric_product.group0().zxy()).with_w(geometric_product[e321] * self[e1234]),
            // e41, e42, e43
            (Simd32x3::from(geometric_product[e321]) * self.group4().xyz())
                + (Simd32x3::from([self[e4], self[e4], self[e431]]) * geometric_product.group0().xyx())
                + (Simd32x3::from([self[e412], self[e423], self[e4]]) * geometric_product.group0().yzz())
                + (geometric_product.group1().zxy() * self.group1().yzx())
                - (Simd32x3::from(geometric_product[e4]) * self.group1().xyz())
                - (Simd32x3::from([self[e3], self[e1], self[e321]]) * geometric_product.group1().yzz())
                - (Simd32x3::from([self[e321], self[e321], self[e2]]) * geometric_product.group1().xyx())
                - (geometric_product.group0().zxy() * self.group4().yzx()),
            // e23, e31, e12
            (geometric_product.group0().zxy() * self.group1().yzx())
                - (Simd32x3::from(geometric_product[e321]) * self.group1().xyz())
                - (Simd32x3::from([self[e3], self[e1], self[e321]]) * geometric_product.group0().yzz())
                - (Simd32x3::from([self[e321], self[e321], self[e2]]) * geometric_product.group0().xyx()),
            // e423, e431, e412, e321
            Simd32x4::from([
                (geometric_product[e3] * self[e42]) + (geometric_product[e4] * self[e23]) + (geometric_product[e431] * self[e12]) - (geometric_product[e412] * self[e31]),
                (geometric_product[e1] * self[e43]) + (geometric_product[e4] * self[e31]) + (geometric_product[e412] * self[e23]) - (geometric_product[e423] * self[e12]),
                (geometric_product[e2] * self[e41]) + (geometric_product[e4] * self[e12]) + (geometric_product[e423] * self[e31]) - (geometric_product[e431] * self[e23]),
                0.0,
            ]) + (Simd32x4::from(self[scalar]) * geometric_product.group1())
                - (Simd32x4::from([geometric_product[e2], geometric_product[e321], geometric_product[e321], geometric_product[e3]]) * self.group2().zyz().with_w(self[e12]))
                - (Simd32x4::from([geometric_product[e321], geometric_product[e3], geometric_product[e1], geometric_product[e2]]) * self.group2().xxy().with_w(self[e31]))
                - (geometric_product.group0().xyzx() * self.group0().yy().with_zw(self[e1234], self[e23])),
        );
    }
}
impl GeometricQuotient<Horizon> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        1
    //    simd2        0        2        0
    //    simd3        0        3        0
    //    simd4        0        4        0
    // Totals...
    // yes simd        0       10        1
    //  no simd        0       30        1
    fn geometric_quotient(self, other: Horizon) -> Self::Output {
        use crate::elements::*;
        let geometric_product = Horizon::from_groups(/* e321 */ 1.0 / other[e321] * -1.0);
        return MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from(geometric_product[e321]) * Simd32x2::from([self[e321], self[e4]]) * Simd32x2::from([-1.0, 1.0]),
            // e1, e2, e3, e4
            Simd32x4::from(geometric_product[e321]) * self.group3().with_w(self[e1234]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e41, e42, e43
            Simd32x3::from(geometric_product[e321]) * self.group4().xyz(),
            // e23, e31, e12
            Simd32x3::from(geometric_product[e321]) * self.group1().xyz() * Simd32x3::from(-1.0),
            // e423, e431, e412, e321
            Simd32x4::from(geometric_product[e321]) * self.group2().with_w(self[scalar]) * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
        );
    }
}
impl GeometricQuotient<Line> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       20       29        0
    //    simd2        3        3        0
    //    simd3        7       12        0
    //    simd4        3        2        0
    // Totals...
    // yes simd       33       46        0
    //  no simd       59       79        0
    fn geometric_quotient(self, other: Line) -> Self::Output {
        use crate::elements::*;
        let other = Scalar::from_groups(/* scalar */ -f32::powi(other[e23], 2) - f32::powi(other[e31], 2) - f32::powi(other[e12], 2));
        let geometric_product = Line::from_groups(
            // e41, e42, e43
            Simd32x3::from(other[scalar]) * other.group0(),
            // e23, e31, e12
            Simd32x3::from(other[scalar]) * other.group1(),
        );
        return MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([0.0, -(geometric_product[e23] * self[e41]) - (geometric_product[e31] * self[e42]) - (geometric_product[e12] * self[e43])])
                - (Simd32x2::from(self[e23]) * Simd32x2::from([geometric_product[e23], geometric_product[e41]]))
                - (Simd32x2::from(self[e31]) * Simd32x2::from([geometric_product[e31], geometric_product[e42]]))
                - (Simd32x2::from(self[e12]) * Simd32x2::from([geometric_product[e12], geometric_product[e43]])),
            // e1, e2, e3, e4
            Simd32x4::from([
                (geometric_product[e23] * self[e321]) + (geometric_product[e31] * self[e3]),
                (geometric_product[e31] * self[e321]) + (geometric_product[e12] * self[e1]),
                (geometric_product[e23] * self[e2]) + (geometric_product[e12] * self[e321]),
                -(geometric_product[e42] * self[e2])
                    - (geometric_product[e43] * self[e3])
                    - (geometric_product[e23] * self[e423])
                    - (geometric_product[e31] * self[e431])
                    - (geometric_product[e12] * self[e412]),
            ]) - (self.group1().yzxx() * geometric_product.group1().zxy().with_w(geometric_product[e41])),
            // e41, e42, e43
            (Simd32x3::from(self[scalar]) * geometric_product.group0())
                + (Simd32x3::from(self[e1234]) * geometric_product.group1())
                + (geometric_product.group0().yzx() * self.group3().zxy())
                + (geometric_product.group1().yzx() * self.group2().zxy())
                - (geometric_product.group0().zxy() * self.group3().yzx())
                - (geometric_product.group1().zxy() * self.group2().yzx()),
            // e23, e31, e12
            (Simd32x3::from(self[scalar]) * geometric_product.group1()) + (geometric_product.group1().yzx() * self.group3().zxy())
                - (geometric_product.group1().zxy() * self.group3().yzx()),
            // e423, e431, e412, e321
            Simd32x4::from([
                (geometric_product[e41] * self[e321]) + (geometric_product[e42] * self[e3]) + (geometric_product[e23] * self[e4]) + (geometric_product[e31] * self[e412]),
                (geometric_product[e42] * self[e321]) + (geometric_product[e43] * self[e1]) + (geometric_product[e31] * self[e4]) + (geometric_product[e12] * self[e423]),
                (geometric_product[e41] * self[e2]) + (geometric_product[e43] * self[e321]) + (geometric_product[e23] * self[e431]) + (geometric_product[e12] * self[e4]),
                geometric_product[e12] * self[e3] * -1.0,
            ]) - (self.group1().yzxx() * geometric_product.group0().zxy().with_w(geometric_product[e23]))
                - (geometric_product.group1().zxy() * self.group4().yzx()).with_w(geometric_product[e31] * self[e2]),
        );
    }
}
impl GeometricQuotient<Motor> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       22       29        0
    //    simd2        4        4        0
    //    simd3       10       12        0
    //    simd4        6        8        0
    // Totals...
    // yes simd       42       53        0
    //  no simd       84      105        0
    fn geometric_quotient(self, other: Motor) -> Self::Output {
        use crate::elements::*;
        let other = Scalar::from_groups(
            // scalar
            f32::powi(other[scalar], 2) - f32::powi(other[e23], 2) - f32::powi(other[e31], 2) - f32::powi(other[e12], 2),
        );
        let geometric_product = Motor::from_groups(
            // e41, e42, e43, e1234
            Simd32x4::from(other[scalar]) * other.group0(),
            // e23, e31, e12, scalar
            Simd32x4::from(other[scalar]) * other.group1(),
        );
        return MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([
                0.0,
                (geometric_product[scalar] * self[e1234]) - (geometric_product[e41] * self[e23]) - (geometric_product[e42] * self[e31]) - (geometric_product[e43] * self[e12]),
            ]) + (Simd32x2::from(self[scalar]) * Simd32x2::from([geometric_product[scalar], geometric_product[e1234]]))
                - (Simd32x2::from(geometric_product[e23]) * Simd32x2::from([self[e23], self[e41]]))
                - (Simd32x2::from(geometric_product[e31]) * Simd32x2::from([self[e31], self[e42]]))
                - (Simd32x2::from(geometric_product[e12]) * Simd32x2::from([self[e12], self[e43]])),
            // e1, e2, e3, e4
            Simd32x4::from([
                geometric_product[scalar] * self[e1],
                geometric_product[scalar] * self[e2],
                geometric_product[scalar] * self[e3],
                -(geometric_product[e42] * self[e2])
                    - (geometric_product[e43] * self[e3])
                    - (geometric_product[e23] * self[e423])
                    - (geometric_product[e31] * self[e431])
                    - (geometric_product[e12] * self[e412]),
            ]) + (geometric_product.group1().yzzw() * self.group1().zx().with_zw(self[e321], self[e4]))
                + (self.group4().ww().with_zw(self[e2], self[e321]) * geometric_product.group1().xyx().with_w(geometric_product[e1234]))
                - (self.group1().yzxx() * geometric_product.group1().zxy().with_w(geometric_product[e41])),
            // e41, e42, e43
            (Simd32x3::from(self[scalar]) * geometric_product.group0().xyz())
                + (Simd32x3::from(self[e1234]) * geometric_product.group1().xyz())
                + (self.group2().xxy() * geometric_product.group1().wzx())
                + (self.group2().zyz() * geometric_product.group1().yww())
                + (self.group3().xxy() * geometric_product.group0().wzx())
                + (self.group3().zyz() * geometric_product.group0().yww())
                - (self.group2().yzx() * geometric_product.group1().zxy())
                - (self.group3().yzx() * geometric_product.group0().zxy()),
            // e23, e31, e12
            (Simd32x3::from(self[scalar]) * geometric_product.group1().xyz())
                + (self.group3().xxy() * geometric_product.group1().wzx())
                + (self.group3().zyz() * geometric_product.group1().yww())
                - (self.group3().yzx() * geometric_product.group1().zxy()),
            // e423, e431, e412, e321
            Simd32x4::from([
                (geometric_product[e42] * self[e3])
                    + (geometric_product[e1234] * self[e1])
                    + (geometric_product[e23] * self[e4])
                    + (geometric_product[e31] * self[e412])
                    + (geometric_product[scalar] * self[e423]),
                (geometric_product[e43] * self[e1])
                    + (geometric_product[e1234] * self[e2])
                    + (geometric_product[e31] * self[e4])
                    + (geometric_product[e12] * self[e423])
                    + (geometric_product[scalar] * self[e431]),
                (geometric_product[e43] * self[e321])
                    + (geometric_product[e1234] * self[e3])
                    + (geometric_product[e23] * self[e431])
                    + (geometric_product[e12] * self[e4])
                    + (geometric_product[scalar] * self[e412]),
                geometric_product[e12] * self[e3] * -1.0,
            ]) + (self.group4().ww().with_zw(self[e2], self[e321]) * geometric_product.group0().xyx().with_w(geometric_product[scalar]))
                - (geometric_product.group1().zxyy() * self.group4().yzx().with_w(self[e2]))
                - (self.group1().yzxx() * geometric_product.group0().zxy().with_w(geometric_product[e23])),
        );
    }
}
impl GeometricQuotient<MultiVector> for MultiVector {
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
    fn geometric_quotient(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        let other = Scalar::from_groups(
            // scalar
            f32::powi(other[scalar], 2) + f32::powi(other[e1], 2) + f32::powi(other[e2], 2) + f32::powi(other[e3], 2)
                - f32::powi(other[e23], 2)
                - f32::powi(other[e31], 2)
                - f32::powi(other[e12], 2)
                - f32::powi(other[e321], 2),
        );
        let geometric_product = MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from(other[scalar]) * other.group0(),
            // e1, e2, e3, e4
            Simd32x4::from(other[scalar]) * other.group1(),
            // e41, e42, e43
            Simd32x3::from(other[scalar]) * other.group2(),
            // e23, e31, e12
            Simd32x3::from(other[scalar]) * other.group3(),
            // e423, e431, e412, e321
            Simd32x4::from(other[scalar]) * other.group4(),
        );
        return MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([
                0.0,
                (geometric_product[e1234] * self[scalar]) + (geometric_product[e321] * self[e4])
                    - (geometric_product[e2] * self[e431])
                    - (geometric_product[e3] * self[e412])
                    - (geometric_product[e4] * self[e321])
                    - (geometric_product[e23] * self[e41])
                    - (geometric_product[e31] * self[e42])
                    - (geometric_product[e12] * self[e43]),
            ]) + (Simd32x2::from(geometric_product[scalar]) * self.group0())
                + (Simd32x2::from(self[e1]) * Simd32x2::from([geometric_product[e1], geometric_product[e423]]))
                + (Simd32x2::from(self[e2]) * Simd32x2::from([geometric_product[e2], geometric_product[e431]]))
                + (Simd32x2::from(self[e3]) * Simd32x2::from([geometric_product[e3], geometric_product[e412]]))
                - (Simd32x2::from(self[e23]) * Simd32x2::from([geometric_product[e23], geometric_product[e41]]))
                - (Simd32x2::from(self[e31]) * Simd32x2::from([geometric_product[e31], geometric_product[e42]]))
                - (Simd32x2::from(self[e12]) * Simd32x2::from([geometric_product[e12], geometric_product[e43]]))
                - (Simd32x2::from([geometric_product[e321], geometric_product[e1]]) * self.group4().wx()),
            // e1, e2, e3, e4
            (Simd32x4::from(geometric_product[scalar]) * self.group1())
                + (Simd32x4::from([geometric_product[e2], geometric_product[e321], geometric_product[e321], geometric_product[e3]]) * self.group3().zyz().with_w(self[e43]))
                + (Simd32x4::from([geometric_product[e321], geometric_product[e3], geometric_product[e1], geometric_product[e2]]) * self.group3().xxy().with_w(self[e42]))
                + (self.group0().xx().with_zw(self[scalar], geometric_product[e1234]) * geometric_product.group1().xyz().with_w(self[e321]))
                + (self.group1().zx().with_zw(self[e321], geometric_product[e1]) * geometric_product.group3().yzz().with_w(self[e41]))
                + (self.group4().ww().with_zw(self[e2], geometric_product[e4]) * geometric_product.group3().xyx().with_w(self[scalar]))
                + Simd32x3::from(0.0).with_w(
                    -(geometric_product[e42] * self[e2])
                        - (geometric_product[e43] * self[e3])
                        - (geometric_product[e23] * self[e423])
                        - (geometric_product[e31] * self[e431])
                        - (geometric_product[e12] * self[e412])
                        - (geometric_product[e423] * self[e23])
                        - (geometric_product[e431] * self[e31])
                        - (geometric_product[e412] * self[e12]),
                )
                - (geometric_product.group3().zxy() * self.group1().yzx()).with_w(geometric_product[e321] * self[e1234])
                - (self.group3().yzx() * geometric_product.group1().zxy()).with_w(geometric_product[e41] * self[e1]),
            // e41, e42, e43
            (Simd32x3::from(geometric_product[scalar]) * self.group2())
                + (Simd32x3::from(geometric_product[e1234]) * self.group3())
                + (Simd32x3::from(geometric_product[e321]) * self.group4().xyz())
                + (Simd32x3::from(self[scalar]) * geometric_product.group2())
                + (Simd32x3::from(self[e1234]) * geometric_product.group3())
                + (Simd32x3::from([self[e4], self[e4], self[e431]]) * geometric_product.group1().xyx())
                + (Simd32x3::from([self[e412], self[e423], self[e4]]) * geometric_product.group1().yzz())
                + (geometric_product.group2().yzx() * self.group3().zxy())
                + (geometric_product.group3().yzx() * self.group2().zxy())
                + (geometric_product.group4().zxy() * self.group1().yzx())
                - (Simd32x3::from(geometric_product[e4]) * self.group1().xyz())
                - (Simd32x3::from([self[e3], self[e1], self[e321]]) * geometric_product.group4().yzz())
                - (Simd32x3::from([self[e321], self[e321], self[e2]]) * geometric_product.group4().xyx())
                - (geometric_product.group2().zxy() * self.group3().yzx())
                - (geometric_product.group3().zxy() * self.group2().yzx())
                - (geometric_product.group1().zxy() * self.group4().yzx()),
            // e23, e31, e12
            (Simd32x3::from(geometric_product[scalar]) * self.group3())
                + (Simd32x3::from(self[scalar]) * geometric_product.group3())
                + (geometric_product.group3().yzx() * self.group3().zxy())
                + (geometric_product.group1().zxy() * self.group1().yzx())
                - (Simd32x3::from(geometric_product[e321]) * self.group1().xyz())
                - (Simd32x3::from([self[e3], self[e1], self[e321]]) * geometric_product.group1().yzz())
                - (Simd32x3::from([self[e321], self[e321], self[e2]]) * geometric_product.group1().xyx())
                - (geometric_product.group3().zxy() * self.group3().yzx()),
            // e423, e431, e412, e321
            Simd32x4::from([
                (geometric_product[e3] * self[e42])
                    + (geometric_product[e4] * self[e23])
                    + (geometric_product[e41] * self[e321])
                    + (geometric_product[e42] * self[e3])
                    + (geometric_product[e23] * self[e4])
                    + (geometric_product[e31] * self[e412])
                    + (geometric_product[e423] * self[scalar])
                    + (geometric_product[e431] * self[e12]),
                (geometric_product[e1] * self[e43])
                    + (geometric_product[e4] * self[e31])
                    + (geometric_product[e42] * self[e321])
                    + (geometric_product[e43] * self[e1])
                    + (geometric_product[e31] * self[e4])
                    + (geometric_product[e12] * self[e423])
                    + (geometric_product[e431] * self[scalar])
                    + (geometric_product[e412] * self[e23]),
                (geometric_product[e2] * self[e41])
                    + (geometric_product[e4] * self[e12])
                    + (geometric_product[e41] * self[e2])
                    + (geometric_product[e43] * self[e321])
                    + (geometric_product[e23] * self[e431])
                    + (geometric_product[e12] * self[e4])
                    + (geometric_product[e423] * self[e31])
                    + (geometric_product[e412] * self[scalar]),
                0.0,
            ]) + (Simd32x4::from(geometric_product[scalar]) * self.group4())
                + (geometric_product.group0().yy().with_zw(geometric_product[e1234], self[scalar]) * self.group1().xyz().with_w(geometric_product[e321]))
                - (Simd32x4::from([geometric_product[e2], geometric_product[e321], geometric_product[e321], geometric_product[e2]]) * self.group2().zyz().with_w(self[e31]))
                - (Simd32x4::from([geometric_product[e321], geometric_product[e3], geometric_product[e1], geometric_product[e1]]) * self.group2().xxy().with_w(self[e23]))
                - (self.group1().yzxy() * geometric_product.group2().zxy().with_w(geometric_product[e31]))
                - (self.group0().yy().with_zw(self[e1234], geometric_product[e23]) * geometric_product.group1().xyz().with_w(self[e1]))
                - (geometric_product.group3().zxy() * self.group4().yzx()).with_w(geometric_product[e12] * self[e3])
                - (self.group3().yzx() * geometric_product.group4().zxy()).with_w(geometric_product[e3] * self[e12]),
        );
    }
}
impl GeometricQuotient<Plane> for MultiVector {
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
    fn geometric_quotient(self, other: Plane) -> Self::Output {
        use crate::elements::*;
        let geometric_product = Plane::from_groups(/* e423, e431, e412, e321 */ Simd32x4::from(f32::powi(other[e321], -2) * -1.0) * other.group0());
        return MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([
                self[e321] * geometric_product[e321],
                (self[e1] * geometric_product[e423]) + (self[e2] * geometric_product[e431]) + (self[e3] * geometric_product[e412]) + (self[e4] * geometric_product[e321]),
            ]) * Simd32x2::from([-1.0, 1.0]),
            // e1, e2, e3, e4
            Simd32x4::from([geometric_product[e321], geometric_product[e321], geometric_product[e321], 1.0])
                * self.group3().with_w(
                    -(self[e1234] * geometric_product[e321])
                        - (self[e23] * geometric_product[e423])
                        - (self[e31] * geometric_product[e431])
                        - (self[e12] * geometric_product[e412]),
                ),
            // e41, e42, e43
            (Simd32x3::from(geometric_product[e321]) * self.group4().xyz()) + (self.group1().yzx() * geometric_product.group0().zxy())
                - (Simd32x3::from(self[e321]) * geometric_product.group0().xyz())
                - (self.group1().zxy() * geometric_product.group0().yzx()),
            // e23, e31, e12
            Simd32x3::from(geometric_product[e321]) * self.group1().xyz() * Simd32x3::from(-1.0),
            // e423, e431, e412, e321
            ((Simd32x3::from(self[scalar]) * geometric_product.group0().xyz()) + (self.group3().zxy() * geometric_product.group0().yzx())
                - (Simd32x3::from(geometric_product[e321]) * self.group2())
                - (self.group3().yzx() * geometric_product.group0().zxy()))
            .with_w(self[scalar] * geometric_product[e321]),
        );
    }
}
impl GeometricQuotient<Point> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       11       23        0
    //    simd3        5        7        0
    //    simd4        4        5        0
    // Totals...
    // yes simd       20       35        0
    //  no simd       42       64        0
    fn geometric_quotient(self, other: Point) -> Self::Output {
        use crate::elements::*;
        let geometric_product = Point::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from(f32::powi(other[e1], 2) + f32::powi(other[e2], 2) + f32::powi(other[e3], 2)) * other.group0(),
        );
        return MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([
                (self[e1] * geometric_product[e1]) + (self[e2] * geometric_product[e2]) + (self[e3] * geometric_product[e3]),
                -(self[e423] * geometric_product[e1]) - (self[e431] * geometric_product[e2]) - (self[e412] * geometric_product[e3]) - (self[e321] * geometric_product[e4]),
            ]),
            // e1, e2, e3, e4
            Simd32x4::from([
                self[e31] * geometric_product[e3] * -1.0,
                self[e12] * geometric_product[e1] * -1.0,
                self[e23] * geometric_product[e2] * -1.0,
                (self[e42] * geometric_product[e2]) + (self[e43] * geometric_product[e3]),
            ]) + (Simd32x4::from(self[scalar]) * geometric_product.group0())
                + (geometric_product.group0().yzxx() * self.group3().zxy().with_w(self[e41])),
            // e41, e42, e43
            (Simd32x3::from(self[e4]) * geometric_product.group0().xyz()) + (self.group4().zxy() * geometric_product.group0().yzx())
                - (Simd32x3::from(geometric_product[e4]) * self.group1().xyz())
                - (self.group4().yzx() * geometric_product.group0().zxy()),
            // e23, e31, e12
            (self.group1().yzx() * geometric_product.group0().zxy())
                - (Simd32x3::from(self[e321]) * geometric_product.group0().xyz())
                - (self.group1().zxy() * geometric_product.group0().yzx()),
            // e423, e431, e412, e321
            Simd32x4::from([
                (self[e42] * geometric_product[e3]) + (self[e23] * geometric_product[e4]),
                (self[e43] * geometric_product[e1]) + (self[e31] * geometric_product[e4]),
                (self[e41] * geometric_product[e2]) + (self[e12] * geometric_product[e4]),
                self[e12] * geometric_product[e3] * -1.0,
            ]) - (geometric_product.group0().xyzx() * self.group0().yy().with_zw(self[e1234], self[e23]))
                - (geometric_product.group0().yzxy() * self.group2().zxy().with_w(self[e31])),
        );
    }
}
impl GeometricQuotient<Scalar> for MultiVector {
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
    fn geometric_quotient(self, other: Scalar) -> Self::Output {
        use crate::elements::*;
        let geometric_product = Scalar::from_groups(/* scalar */ 1.0 / other[scalar]);
        return MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from(geometric_product[scalar]) * self.group0(),
            // e1, e2, e3, e4
            Simd32x4::from(geometric_product[scalar]) * self.group1(),
            // e41, e42, e43
            Simd32x3::from(geometric_product[scalar]) * self.group2(),
            // e23, e31, e12
            Simd32x3::from(geometric_product[scalar]) * self.group3(),
            // e423, e431, e412, e321
            Simd32x4::from(geometric_product[scalar]) * self.group4(),
        );
    }
}
impl std::ops::Div<geometric_quotient> for Origin {
    type Output = geometric_quotient_partial<Origin>;
    fn div(self, _rhs: geometric_quotient) -> Self::Output {
        geometric_quotient_partial(self)
    }
}
impl GeometricQuotient<DualNum> for Origin {
    type Output = Origin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        2        0
    fn geometric_quotient(self, other: DualNum) -> Self::Output {
        use crate::elements::*;
        let other = Scalar::from_groups(/* scalar */ f32::powi(other[scalar], -2));
        return Origin::from_groups(/* e4 */ other[scalar] * self[e4] * other[scalar]);
    }
}
impl GeometricQuotient<Flector> for Origin {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        0        0
    //    simd4        0        3        0
    // Totals...
    // yes simd        3        3        0
    //  no simd        3       12        0
    fn geometric_quotient(self, other: Flector) -> Self::Output {
        use crate::elements::*;
        let other = Scalar::from_groups(
            // scalar
            f32::powi(other[e1], 2) + f32::powi(other[e2], 2) + f32::powi(other[e3], 2) - f32::powi(other[e321], 2),
        );
        let geometric_product = Flector::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from(other[scalar]) * other.group0(),
            // e423, e431, e412, e321
            Simd32x4::from(other[scalar]) * other.group1(),
        );
        return Motor::from_groups(
            // e41, e42, e43, e1234
            Simd32x4::from(self[e4]) * geometric_product.group0().xyz().with_w(geometric_product[e321]),
            // e23, e31, e12, scalar
            Simd32x4::from(0.0),
        );
    }
}
impl GeometricQuotient<Horizon> for Origin {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        3        0
    fn geometric_quotient(self, other: Horizon) -> Self::Output {
        use crate::elements::*;
        let other = Scalar::from_groups(/* scalar */ f32::powi(other[e321], -2) * -1.0);
        return AntiScalar::from_groups(/* e1234 */ other[e321] * self[e4] * other[scalar]);
    }
}
impl GeometricQuotient<Line> for Origin {
    type Output = Plane;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        3        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        2        5        0
    //  no simd        2       11        0
    fn geometric_quotient(self, other: Line) -> Self::Output {
        use crate::elements::*;
        let other = Scalar::from_groups(/* scalar */ -f32::powi(other[e23], 2) - f32::powi(other[e31], 2) - f32::powi(other[e12], 2));
        return Plane::from_groups(
            // e423, e431, e412, e321
            Simd32x4::from([self[e4], self[e4], self[e4], 0.0])
                * Simd32x3::from([other[e23] * other[scalar], other[e31] * other[scalar], other[e12] * other[scalar]]).with_w(0.0)
                * Simd32x4::from([1.0, 1.0, 1.0, 0.0]),
        );
    }
}
impl GeometricQuotient<Motor> for Origin {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        1        0
    //    simd4        0        4        0
    // Totals...
    // yes simd        3        5        0
    //  no simd        3       17        0
    fn geometric_quotient(self, other: Motor) -> Self::Output {
        use crate::elements::*;
        let other = Scalar::from_groups(
            // scalar
            f32::powi(other[scalar], 2) - f32::powi(other[e23], 2) - f32::powi(other[e31], 2) - f32::powi(other[e12], 2),
        );
        let geometric_product = Motor::from_groups(
            // e41, e42, e43, e1234
            Simd32x4::from(other[scalar]) * other.group0(),
            // e23, e31, e12, scalar
            Simd32x4::from(other[scalar]) * other.group1(),
        );
        return Flector::from_groups(
            // e1, e2, e3, e4
            Simd32x3::from(0.0).with_w(geometric_product[scalar] * self[e4]),
            // e423, e431, e412, e321
            Simd32x4::from([self[e4], self[e4], self[e4], 0.0]) * geometric_product.group1().xyz().with_w(0.0) * Simd32x4::from([1.0, 1.0, 1.0, 0.0]),
        );
    }
}
impl GeometricQuotient<MultiVector> for Origin {
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
    fn geometric_quotient(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        let other = Scalar::from_groups(
            // scalar
            f32::powi(other[scalar], 2) + f32::powi(other[e1], 2) + f32::powi(other[e2], 2) + f32::powi(other[e3], 2)
                - f32::powi(other[e23], 2)
                - f32::powi(other[e31], 2)
                - f32::powi(other[e12], 2)
                - f32::powi(other[e321], 2),
        );
        let geometric_product = MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from(other[scalar]) * other.group0(),
            // e1, e2, e3, e4
            Simd32x4::from(other[scalar]) * other.group1(),
            // e41, e42, e43
            Simd32x3::from(other[scalar]) * other.group2(),
            // e23, e31, e12
            Simd32x3::from(other[scalar]) * other.group3(),
            // e423, e431, e412, e321
            Simd32x4::from(other[scalar]) * other.group4(),
        );
        return MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([1.0, geometric_product[e321] * self[e4]]) * Simd32x2::from([0.0, 1.0]),
            // e1, e2, e3, e4
            Simd32x3::from(0.0).with_w(geometric_product[scalar] * self[e4]),
            // e41, e42, e43
            Simd32x3::from(self[e4]) * geometric_product.group1().xyz(),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e423, e431, e412, e321
            Simd32x4::from([self[e4], self[e4], self[e4], 0.0]) * geometric_product.group3().with_w(0.0) * Simd32x4::from([1.0, 1.0, 1.0, 0.0]),
        );
    }
}
impl GeometricQuotient<Plane> for Origin {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        3        0
    fn geometric_quotient(self, other: Plane) -> Self::Output {
        use crate::elements::*;
        let other = Scalar::from_groups(/* scalar */ f32::powi(other[e321], -2) * -1.0);
        return AntiScalar::from_groups(/* e1234 */ self[e4] * other[e321] * other[scalar]);
    }
}
impl GeometricQuotient<Point> for Origin {
    type Output = Line;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        4        0
    //    simd3        0        1        0
    // Totals...
    // yes simd        2        5        0
    //  no simd        2        7        0
    fn geometric_quotient(self, other: Point) -> Self::Output {
        use crate::elements::*;
        let other = Scalar::from_groups(/* scalar */ f32::powi(other[e1], 2) + f32::powi(other[e2], 2) + f32::powi(other[e3], 2));
        return Line::from_groups(
            // e41, e42, e43
            Simd32x3::from(self[e4]) * Simd32x4::from([other[e1] * other[scalar], other[e2] * other[scalar], other[e3] * other[scalar], other[e4] * other[scalar]]).xyz(),
            // e23, e31, e12
            Simd32x3::from(0.0),
        );
    }
}
impl GeometricQuotient<Scalar> for Origin {
    type Output = Origin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        2        0
    fn geometric_quotient(self, other: Scalar) -> Self::Output {
        use crate::elements::*;
        let other = Scalar::from_groups(/* scalar */ f32::powi(other[scalar], -2));
        return Origin::from_groups(/* e4 */ self[e4] * other[scalar] * other[scalar]);
    }
}
impl std::ops::Div<geometric_quotient> for Plane {
    type Output = geometric_quotient_partial<Plane>;
    fn div(self, _rhs: geometric_quotient) -> Self::Output {
        geometric_quotient_partial(self)
    }
}
impl GeometricQuotient<DualNum> for Plane {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd2        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        3        0
    //  no simd        0        7        0
    fn geometric_quotient(self, other: DualNum) -> Self::Output {
        use crate::elements::*;
        let geometric_product = DualNum::from_groups(/* scalar, e1234 */ Simd32x2::from(f32::powi(other[scalar], -2)) * other.group0());
        return Flector::from_groups(
            // e1, e2, e3, e4
            Simd32x3::from(0.0).with_w(geometric_product[e1234] * self[e321]),
            // e423, e431, e412, e321
            Simd32x4::from(geometric_product[scalar]) * self.group0(),
        );
    }
}
impl GeometricQuotient<Flector> for Plane {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        7        8        0
    //    simd4        2        6        0
    // Totals...
    // yes simd        9       14        0
    //  no simd       15       32        0
    fn geometric_quotient(self, other: Flector) -> Self::Output {
        use crate::elements::*;
        let other = Scalar::from_groups(
            // scalar
            f32::powi(other[e1], 2) + f32::powi(other[e2], 2) + f32::powi(other[e3], 2) - f32::powi(other[e321], 2),
        );
        let geometric_product = Flector::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from(other[scalar]) * other.group0(),
            // e423, e431, e412, e321
            Simd32x4::from(other[scalar]) * other.group1(),
        );
        return Motor::from_groups(
            // e41, e42, e43, e1234
            Simd32x4::from([
                (geometric_product[e2] * self[e412]) + (geometric_product[e321] * self[e423]),
                (geometric_product[e3] * self[e423]) + (geometric_product[e321] * self[e431]),
                (geometric_product[e1] * self[e431]) + (geometric_product[e321] * self[e412]),
                -(geometric_product[e3] * self[e412]) - (geometric_product[e4] * self[e321]),
            ]) - (geometric_product.group0().zxyx() * self.group0().yzxx())
                - (self.group0().wwwy() * geometric_product.group1().xyz().with_w(geometric_product[e2])),
            // e23, e31, e12, scalar
            Simd32x4::from(self[e321]) * geometric_product.group0().xyz().with_w(geometric_product[e321]) * Simd32x4::from(-1.0),
        );
    }
}
impl GeometricQuotient<Horizon> for Plane {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        3        1
    //    simd4        0        2        0
    // Totals...
    // yes simd        0        5        1
    //  no simd        0       11        1
    fn geometric_quotient(self, other: Horizon) -> Self::Output {
        use crate::elements::*;
        let geometric_product = Horizon::from_groups(/* e321 */ 1.0 / other[e321] * -1.0);
        return Motor::from_groups(
            // e41, e42, e43, e1234
            Simd32x4::from([geometric_product[e321], geometric_product[e321], geometric_product[e321], 0.0])
                * self.group0().xyz().with_w(0.0)
                * Simd32x4::from([1.0, 1.0, 1.0, 0.0]),
            // e23, e31, e12, scalar
            Simd32x3::from(0.0).with_w(geometric_product[e321] * self[e321] * -1.0),
        );
    }
}
impl GeometricQuotient<Line> for Plane {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        3        0
    //    simd3        0        5        0
    //    simd4        2        1        0
    // Totals...
    // yes simd        6        9        0
    //  no simd       12       22        0
    fn geometric_quotient(self, other: Line) -> Self::Output {
        use crate::elements::*;
        let other = Scalar::from_groups(/* scalar */ -f32::powi(other[e23], 2) - f32::powi(other[e31], 2) - f32::powi(other[e12], 2));
        let geometric_product = Line::from_groups(
            // e41, e42, e43
            Simd32x3::from(other[scalar]) * other.group0(),
            // e23, e31, e12
            Simd32x3::from(other[scalar]) * other.group1(),
        );
        return Flector::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from([self[e321], self[e321], self[e321], 1.0])
                * geometric_product
                    .group1()
                    .with_w(-(geometric_product[e23] * self[e423]) - (geometric_product[e31] * self[e431]) - (geometric_product[e12] * self[e412])),
            // e423, e431, e412, e321
            (Simd32x3::from(self[e321]) * geometric_product.group0()).with_w(0.0) + (geometric_product.group1().yzx() * self.group0().zxy()).with_w(0.0)
                - (geometric_product.group1().zxy() * self.group0().yzx()).with_w(0.0),
        );
    }
}
impl GeometricQuotient<Motor> for Plane {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        6        5        0
    //    simd3        3        4        0
    //    simd4        0        3        0
    // Totals...
    // yes simd        9       12        0
    //  no simd       15       29        0
    fn geometric_quotient(self, other: Motor) -> Self::Output {
        use crate::elements::*;
        let other = Scalar::from_groups(
            // scalar
            f32::powi(other[scalar], 2) - f32::powi(other[e23], 2) - f32::powi(other[e31], 2) - f32::powi(other[e12], 2),
        );
        let geometric_product = Motor::from_groups(
            // e41, e42, e43, e1234
            Simd32x4::from(other[scalar]) * other.group0(),
            // e23, e31, e12, scalar
            Simd32x4::from(other[scalar]) * other.group1(),
        );
        return Flector::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from([self[e321], self[e321], self[e321], 1.0])
                * geometric_product.group1().xyz().with_w(
                    (geometric_product[e1234] * self[e321]) - (geometric_product[e23] * self[e423]) - (geometric_product[e31] * self[e431]) - (geometric_product[e12] * self[e412]),
                ),
            // e423, e431, e412, e321
            ((Simd32x3::from(geometric_product[scalar]) * self.group0().xyz())
                + (Simd32x3::from(self[e321]) * geometric_product.group0().xyz())
                + (geometric_product.group1().yzx() * self.group0().zxy())
                - (geometric_product.group1().zxy() * self.group0().yzx()))
            .with_w(geometric_product[scalar] * self[e321]),
        );
    }
}
impl GeometricQuotient<MultiVector> for Plane {
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
    fn geometric_quotient(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        let other = Scalar::from_groups(
            // scalar
            f32::powi(other[scalar], 2) + f32::powi(other[e1], 2) + f32::powi(other[e2], 2) + f32::powi(other[e3], 2)
                - f32::powi(other[e23], 2)
                - f32::powi(other[e31], 2)
                - f32::powi(other[e12], 2)
                - f32::powi(other[e321], 2),
        );
        let geometric_product = MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from(other[scalar]) * other.group0(),
            // e1, e2, e3, e4
            Simd32x4::from(other[scalar]) * other.group1(),
            // e41, e42, e43
            Simd32x3::from(other[scalar]) * other.group2(),
            // e23, e31, e12
            Simd32x3::from(other[scalar]) * other.group3(),
            // e423, e431, e412, e321
            Simd32x4::from(other[scalar]) * other.group4(),
        );
        return MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([
                geometric_product[e321] * self[e321],
                -(geometric_product[e1] * self[e423]) - (geometric_product[e2] * self[e431]) - (geometric_product[e3] * self[e412]) - (geometric_product[e4] * self[e321]),
            ]) * Simd32x2::from([-1.0, 1.0]),
            // e1, e2, e3, e4
            Simd32x4::from([self[e321], self[e321], self[e321], 1.0])
                * geometric_product.group3().with_w(
                    (geometric_product[e1234] * self[e321]) - (geometric_product[e23] * self[e423]) - (geometric_product[e31] * self[e431]) - (geometric_product[e12] * self[e412]),
                ),
            // e41, e42, e43
            (Simd32x3::from(geometric_product[e321]) * self.group0().xyz()) + (geometric_product.group1().yzx() * self.group0().zxy())
                - (Simd32x3::from(self[e321]) * geometric_product.group4().xyz())
                - (geometric_product.group1().zxy() * self.group0().yzx()),
            // e23, e31, e12
            Simd32x3::from(self[e321]) * geometric_product.group1().xyz() * Simd32x3::from(-1.0),
            // e423, e431, e412, e321
            ((Simd32x3::from(geometric_product[scalar]) * self.group0().xyz())
                + (Simd32x3::from(self[e321]) * geometric_product.group2())
                + (geometric_product.group3().yzx() * self.group0().zxy())
                - (geometric_product.group3().zxy() * self.group0().yzx()))
            .with_w(geometric_product[scalar] * self[e321]),
        );
    }
}
impl GeometricQuotient<Plane> for Plane {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        3        0
    //    simd3        1        2        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        1        6        0
    //  no simd        3       13        0
    fn geometric_quotient(self, other: Plane) -> Self::Output {
        use crate::elements::*;
        let geometric_product = Plane::from_groups(/* e423, e431, e412, e321 */ Simd32x4::from(f32::powi(other[e321], -2) * -1.0) * other.group0());
        return Motor::from_groups(
            // e41, e42, e43, e1234
            ((Simd32x3::from(geometric_product[e321]) * self.group0().xyz()) - (Simd32x3::from(self[e321]) * geometric_product.group0().xyz())).with_w(0.0),
            // e23, e31, e12, scalar
            Simd32x3::from(0.0).with_w(geometric_product[e321] * self[e321] * -1.0),
        );
    }
}
impl GeometricQuotient<Point> for Plane {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        6        0
    //    simd4        1        5        0
    // Totals...
    // yes simd        5       11        0
    //  no simd        8       26        0
    fn geometric_quotient(self, other: Point) -> Self::Output {
        use crate::elements::*;
        let geometric_product = Point::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from(f32::powi(other[e1], 2) + f32::powi(other[e2], 2) + f32::powi(other[e3], 2)) * other.group0(),
        );
        return Motor::from_groups(
            // e41, e42, e43, e1234
            Simd32x4::from([
                self[e412] * geometric_product[e2],
                self[e423] * geometric_product[e3],
                self[e431] * geometric_product[e1],
                -(self[e431] * geometric_product[e2]) - (self[e412] * geometric_product[e3]) - (self[e321] * geometric_product[e4]),
            ]) - (self.group0().yzxx() * geometric_product.group0().zxyx()),
            // e23, e31, e12, scalar
            Simd32x3::from(1.0).with_w(0.0) * self.group0().www().with_w(0.0) * geometric_product.group0().xyz().with_w(0.0) * Simd32x4::from([-1.0, -1.0, -1.0, 0.0]),
        );
    }
}
impl GeometricQuotient<Scalar> for Plane {
    type Output = Plane;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        2        0
    //  no simd        0        5        0
    fn geometric_quotient(self, other: Scalar) -> Self::Output {
        use crate::elements::*;
        let other = Scalar::from_groups(/* scalar */ f32::powi(other[scalar], -2));
        return Plane::from_groups(/* e423, e431, e412, e321 */ Simd32x4::from(other[scalar] * other[scalar]) * self.group0());
    }
}
impl std::ops::Div<geometric_quotient> for Point {
    type Output = geometric_quotient_partial<Point>;
    fn div(self, _rhs: geometric_quotient) -> Self::Output {
        geometric_quotient_partial(self)
    }
}
impl GeometricQuotient<DualNum> for Point {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd2        0        1        0
    //    simd4        0        4        0
    // Totals...
    // yes simd        0        5        0
    //  no simd        0       18        0
    fn geometric_quotient(self, other: DualNum) -> Self::Output {
        use crate::elements::*;
        let geometric_product = DualNum::from_groups(/* scalar, e1234 */ Simd32x2::from(f32::powi(other[scalar], -2)) * other.group0());
        return Flector::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from(geometric_product[scalar]) * self.group0(),
            // e423, e431, e412, e321
            geometric_product.group0().yy().with_zw(geometric_product[e1234], 0.0)
                * Simd32x3::from(1.0).with_w(0.0)
                * self.group0().xyz().with_w(0.0)
                * Simd32x4::from([1.0, 1.0, 1.0, 0.0]),
        );
    }
}
impl GeometricQuotient<Flector> for Point {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       11       16        0
    //    simd4        3        5        0
    // Totals...
    // yes simd       14       21        0
    //  no simd       23       36        0
    fn geometric_quotient(self, other: Flector) -> Self::Output {
        use crate::elements::*;
        let other = Scalar::from_groups(
            // scalar
            f32::powi(other[e1], 2) + f32::powi(other[e2], 2) + f32::powi(other[e3], 2) - f32::powi(other[e321], 2),
        );
        let geometric_product = Flector::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from(other[scalar]) * other.group0(),
            // e423, e431, e412, e321
            Simd32x4::from(other[scalar]) * other.group1(),
        );
        return Motor::from_groups(
            // e41, e42, e43, e1234
            Simd32x4::from([
                -(geometric_product[e4] * self[e1]) - (geometric_product[e431] * self[e3]),
                -(geometric_product[e4] * self[e2]) - (geometric_product[e412] * self[e1]),
                -(geometric_product[e4] * self[e3]) - (geometric_product[e423] * self[e2]),
                (geometric_product[e412] * self[e3]) + (geometric_product[e321] * self[e4]),
            ]) + (geometric_product.group1().zxyy() * self.group0().yzxy())
                + (self.group0().wwwx() * geometric_product.group0().xyz().with_w(geometric_product[e423])),
            // e23, e31, e12, scalar
            Simd32x4::from([
                -(geometric_product[e2] * self[e3]) - (geometric_product[e321] * self[e1]),
                -(geometric_product[e3] * self[e1]) - (geometric_product[e321] * self[e2]),
                -(geometric_product[e1] * self[e2]) - (geometric_product[e321] * self[e3]),
                (geometric_product[e2] * self[e2]) + (geometric_product[e3] * self[e3]),
            ]) + (geometric_product.group0().zxyx() * self.group0().yzxx()),
        );
    }
}
impl GeometricQuotient<Horizon> for Point {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        2        1
    //    simd4        0        2        0
    // Totals...
    // yes simd        0        4        1
    //  no simd        0       10        1
    fn geometric_quotient(self, other: Horizon) -> Self::Output {
        use crate::elements::*;
        let geometric_product = Horizon::from_groups(/* e321 */ 1.0 / other[e321] * -1.0);
        return Motor::from_groups(
            // e41, e42, e43, e1234
            Simd32x3::from(0.0).with_w(geometric_product[e321] * self[e4]),
            // e23, e31, e12, scalar
            Simd32x4::from([geometric_product[e321], geometric_product[e321], geometric_product[e321], 0.0])
                * self.group0().xyz().with_w(0.0)
                * Simd32x4::from([-1.0, -1.0, -1.0, 0.0]),
        );
    }
}
impl GeometricQuotient<Line> for Point {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        7       13        0
    //    simd3        0        2        0
    //    simd4        2        2        0
    // Totals...
    // yes simd        9       17        0
    //  no simd       15       27        0
    fn geometric_quotient(self, other: Line) -> Self::Output {
        use crate::elements::*;
        let other = Scalar::from_groups(/* scalar */ -f32::powi(other[e23], 2) - f32::powi(other[e31], 2) - f32::powi(other[e12], 2));
        let geometric_product = Line::from_groups(
            // e41, e42, e43
            Simd32x3::from(other[scalar]) * other.group0(),
            // e23, e31, e12
            Simd32x3::from(other[scalar]) * other.group1(),
        );
        return Flector::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from([
                geometric_product[e31] * self[e3],
                geometric_product[e12] * self[e1],
                geometric_product[e23] * self[e2],
                -(geometric_product[e42] * self[e2]) - (geometric_product[e43] * self[e3]),
            ]) - (self.group0().yzxx() * geometric_product.group1().zxy().with_w(geometric_product[e41])),
            // e423, e431, e412, e321
            Simd32x4::from([
                (geometric_product[e42] * self[e3]) + (geometric_product[e23] * self[e4]),
                (geometric_product[e43] * self[e1]) + (geometric_product[e31] * self[e4]),
                (geometric_product[e41] * self[e2]) + (geometric_product[e12] * self[e4]),
                -(geometric_product[e31] * self[e2]) - (geometric_product[e12] * self[e3]),
            ]) - (self.group0().yzxx() * geometric_product.group0().zxy().with_w(geometric_product[e23])),
        );
    }
}
impl GeometricQuotient<Motor> for Point {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       11       16        0
    //    simd4        3        5        0
    // Totals...
    // yes simd       14       21        0
    //  no simd       23       36        0
    fn geometric_quotient(self, other: Motor) -> Self::Output {
        use crate::elements::*;
        let other = Scalar::from_groups(
            // scalar
            f32::powi(other[scalar], 2) - f32::powi(other[e23], 2) - f32::powi(other[e31], 2) - f32::powi(other[e12], 2),
        );
        let geometric_product = Motor::from_groups(
            // e41, e42, e43, e1234
            Simd32x4::from(other[scalar]) * other.group0(),
            // e23, e31, e12, scalar
            Simd32x4::from(other[scalar]) * other.group1(),
        );
        return Flector::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from([
                geometric_product[scalar] * self[e1],
                geometric_product[scalar] * self[e2],
                geometric_product[scalar] * self[e3],
                -(geometric_product[e42] * self[e2]) - (geometric_product[e43] * self[e3]),
            ]) + (geometric_product.group1().yzxw() * self.group0().zxyw())
                - (self.group0().yzxx() * geometric_product.group1().zxy().with_w(geometric_product[e41])),
            // e423, e431, e412, e321
            Simd32x4::from([
                (geometric_product[e42] * self[e3]) + (geometric_product[e1234] * self[e1]) + (geometric_product[e23] * self[e4]),
                (geometric_product[e43] * self[e1]) + (geometric_product[e1234] * self[e2]) + (geometric_product[e31] * self[e4]),
                (geometric_product[e41] * self[e2]) + (geometric_product[e1234] * self[e3]) + (geometric_product[e12] * self[e4]),
                -(geometric_product[e31] * self[e2]) - (geometric_product[e12] * self[e3]),
            ]) - (self.group0().yzxx() * geometric_product.group0().zxy().with_w(geometric_product[e23])),
        );
    }
}
impl GeometricQuotient<MultiVector> for Point {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       15       17        0
    //    simd2        3        4        0
    //    simd3        5        9        0
    //    simd4        3        5        0
    // Totals...
    // yes simd       26       35        0
    //  no simd       48       72        0
    fn geometric_quotient(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        let other = Scalar::from_groups(
            // scalar
            f32::powi(other[scalar], 2) + f32::powi(other[e1], 2) + f32::powi(other[e2], 2) + f32::powi(other[e3], 2)
                - f32::powi(other[e23], 2)
                - f32::powi(other[e31], 2)
                - f32::powi(other[e12], 2)
                - f32::powi(other[e321], 2),
        );
        let geometric_product = MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from(other[scalar]) * other.group0(),
            // e1, e2, e3, e4
            Simd32x4::from(other[scalar]) * other.group1(),
            // e41, e42, e43
            Simd32x3::from(other[scalar]) * other.group2(),
            // e23, e31, e12
            Simd32x3::from(other[scalar]) * other.group3(),
            // e423, e431, e412, e321
            Simd32x4::from(other[scalar]) * other.group4(),
        );
        return MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([0.0, geometric_product[e321] * self[e4]])
                + (Simd32x2::from(self[e1]) * Simd32x2::from([geometric_product[e1], geometric_product[e423]]))
                + (Simd32x2::from(self[e2]) * Simd32x2::from([geometric_product[e2], geometric_product[e431]]))
                + (Simd32x2::from(self[e3]) * Simd32x2::from([geometric_product[e3], geometric_product[e412]])),
            // e1, e2, e3, e4
            Simd32x4::from([
                geometric_product[e31] * self[e3],
                geometric_product[e12] * self[e1],
                geometric_product[e23] * self[e2],
                -(geometric_product[e42] * self[e2]) - (geometric_product[e43] * self[e3]),
            ]) + (Simd32x4::from(geometric_product[scalar]) * self.group0())
                - (self.group0().yzxx() * geometric_product.group3().zxy().with_w(geometric_product[e41])),
            // e41, e42, e43
            (Simd32x3::from(self[e4]) * geometric_product.group1().xyz()) + (geometric_product.group4().zxy() * self.group0().yzx())
                - (Simd32x3::from(geometric_product[e4]) * self.group0().xyz())
                - (geometric_product.group4().yzx() * self.group0().zxy()),
            // e23, e31, e12
            (geometric_product.group1().zxy() * self.group0().yzx())
                - (Simd32x3::from(geometric_product[e321]) * self.group0().xyz())
                - (geometric_product.group1().yzx() * self.group0().zxy()),
            // e423, e431, e412, e321
            Simd32x4::from([
                (geometric_product[e1234] * self[e1]) + (geometric_product[e42] * self[e3]) + (geometric_product[e23] * self[e4]),
                (geometric_product[e1234] * self[e2]) + (geometric_product[e43] * self[e1]) + (geometric_product[e31] * self[e4]),
                (geometric_product[e1234] * self[e3]) + (geometric_product[e41] * self[e2]) + (geometric_product[e12] * self[e4]),
                -(geometric_product[e31] * self[e2]) - (geometric_product[e12] * self[e3]),
            ]) - (self.group0().yzxx() * geometric_product.group2().zxy().with_w(geometric_product[e23])),
        );
    }
}
impl GeometricQuotient<Plane> for Point {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2       10        0
    //    simd4        1        5        0
    // Totals...
    // yes simd        3       15        0
    //  no simd        6       30        0
    fn geometric_quotient(self, other: Plane) -> Self::Output {
        use crate::elements::*;
        let geometric_product = Plane::from_groups(/* e423, e431, e412, e321 */ Simd32x4::from(f32::powi(other[e321], -2) * -1.0) * other.group0());
        return Motor::from_groups(
            // e41, e42, e43, e1234
            Simd32x4::from([
                geometric_product[e431] * self[e3] * -1.0,
                geometric_product[e412] * self[e1] * -1.0,
                geometric_product[e423] * self[e2] * -1.0,
                (geometric_product[e431] * self[e2]) + (geometric_product[e412] * self[e3]) + (geometric_product[e321] * self[e4]),
            ]) + (geometric_product.group0().zxyx() * self.group0().yzxx()),
            // e23, e31, e12, scalar
            Simd32x3::from(1.0).with_w(0.0) * geometric_product.group0().www().with_w(0.0) * self.group0().xyz().with_w(0.0) * Simd32x4::from([-1.0, -1.0, -1.0, 0.0]),
        );
    }
}
impl GeometricQuotient<Point> for Point {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        8        0
    //    simd3        1        2        0
    //    simd4        1        2        0
    // Totals...
    // yes simd        5       12        0
    //  no simd       10       22        0
    fn geometric_quotient(self, other: Point) -> Self::Output {
        use crate::elements::*;
        let geometric_product = Point::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from(f32::powi(other[e1], 2) + f32::powi(other[e2], 2) + f32::powi(other[e3], 2)) * other.group0(),
        );
        return Motor::from_groups(
            // e41, e42, e43, e1234
            ((Simd32x3::from(self[e4]) * geometric_product.group0().xyz()) - (Simd32x3::from(geometric_product[e4]) * self.group0().xyz())).with_w(0.0),
            // e23, e31, e12, scalar
            Simd32x4::from([
                geometric_product[e2] * self[e3] * -1.0,
                geometric_product[e3] * self[e1] * -1.0,
                geometric_product[e1] * self[e2] * -1.0,
                (geometric_product[e2] * self[e2]) + (geometric_product[e3] * self[e3]),
            ]) + (geometric_product.group0().zxyx() * self.group0().yzxx()),
        );
    }
}
impl GeometricQuotient<Scalar> for Point {
    type Output = Point;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        2        0
    //  no simd        0        5        0
    fn geometric_quotient(self, other: Scalar) -> Self::Output {
        use crate::elements::*;
        let other = Scalar::from_groups(/* scalar */ f32::powi(other[scalar], -2));
        return Point::from_groups(/* e1, e2, e3, e4 */ Simd32x4::from(other[scalar] * other[scalar]) * self.group0());
    }
}
impl std::ops::Div<geometric_quotient> for Scalar {
    type Output = geometric_quotient_partial<Scalar>;
    fn div(self, _rhs: geometric_quotient) -> Self::Output {
        geometric_quotient_partial(self)
    }
}
impl GeometricQuotient<DualNum> for Scalar {
    type Output = DualNum;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        2        0
    //    simd2        0        1        0
    // Totals...
    // yes simd        0        3        0
    //  no simd        0        4        0
    fn geometric_quotient(self, other: DualNum) -> Self::Output {
        use crate::elements::*;
        let other = Scalar::from_groups(/* scalar */ f32::powi(other[scalar], -2));
        return DualNum::from_groups(
            // scalar, e1234
            Simd32x2::from(self[scalar]) * Simd32x2::from([other[scalar] * other[scalar], other[e1234] * other[scalar]]),
        );
    }
}
impl GeometricQuotient<Flector> for Scalar {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        0        0
    //    simd4        0        4        0
    // Totals...
    // yes simd        3        4        0
    //  no simd        3       16        0
    fn geometric_quotient(self, other: Flector) -> Self::Output {
        use crate::elements::*;
        let other = Scalar::from_groups(
            // scalar
            f32::powi(other[e1], 2) + f32::powi(other[e2], 2) + f32::powi(other[e3], 2) - f32::powi(other[e321], 2),
        );
        let geometric_product = Flector::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from(other[scalar]) * other.group0(),
            // e423, e431, e412, e321
            Simd32x4::from(other[scalar]) * other.group1(),
        );
        return Flector::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from(self[scalar]) * geometric_product.group0(),
            // e423, e431, e412, e321
            Simd32x4::from(self[scalar]) * geometric_product.group1(),
        );
    }
}
impl GeometricQuotient<Horizon> for Scalar {
    type Output = Horizon;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        3        0
    fn geometric_quotient(self, other: Horizon) -> Self::Output {
        use crate::elements::*;
        let other = Scalar::from_groups(/* scalar */ f32::powi(other[e321], -2) * -1.0);
        return Horizon::from_groups(/* e321 */ other[e321] * other[scalar] * self[scalar]);
    }
}
impl GeometricQuotient<Line> for Scalar {
    type Output = Line;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        0        0
    //    simd3        0        4        0
    // Totals...
    // yes simd        2        4        0
    //  no simd        2       12        0
    fn geometric_quotient(self, other: Line) -> Self::Output {
        use crate::elements::*;
        let other = Scalar::from_groups(/* scalar */ -f32::powi(other[e23], 2) - f32::powi(other[e31], 2) - f32::powi(other[e12], 2));
        let geometric_product = Line::from_groups(
            // e41, e42, e43
            Simd32x3::from(other[scalar]) * other.group0(),
            // e23, e31, e12
            Simd32x3::from(other[scalar]) * other.group1(),
        );
        return Line::from_groups(
            // e41, e42, e43
            Simd32x3::from(self[scalar]) * geometric_product.group0(),
            // e23, e31, e12
            Simd32x3::from(self[scalar]) * geometric_product.group1(),
        );
    }
}
impl GeometricQuotient<Motor> for Scalar {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        0        0
    //    simd4        0        4        0
    // Totals...
    // yes simd        3        4        0
    //  no simd        3       16        0
    fn geometric_quotient(self, other: Motor) -> Self::Output {
        use crate::elements::*;
        let other = Scalar::from_groups(
            // scalar
            f32::powi(other[scalar], 2) - f32::powi(other[e23], 2) - f32::powi(other[e31], 2) - f32::powi(other[e12], 2),
        );
        let geometric_product = Motor::from_groups(
            // e41, e42, e43, e1234
            Simd32x4::from(other[scalar]) * other.group0(),
            // e23, e31, e12, scalar
            Simd32x4::from(other[scalar]) * other.group1(),
        );
        return Motor::from_groups(
            // e41, e42, e43, e1234
            Simd32x4::from(self[scalar]) * geometric_product.group0(),
            // e23, e31, e12, scalar
            Simd32x4::from(self[scalar]) * geometric_product.group1(),
        );
    }
}
impl GeometricQuotient<MultiVector> for Scalar {
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
    fn geometric_quotient(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        let other = Scalar::from_groups(
            // scalar
            f32::powi(other[scalar], 2) + f32::powi(other[e1], 2) + f32::powi(other[e2], 2) + f32::powi(other[e3], 2)
                - f32::powi(other[e23], 2)
                - f32::powi(other[e31], 2)
                - f32::powi(other[e12], 2)
                - f32::powi(other[e321], 2),
        );
        let geometric_product = MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from(other[scalar]) * other.group0(),
            // e1, e2, e3, e4
            Simd32x4::from(other[scalar]) * other.group1(),
            // e41, e42, e43
            Simd32x3::from(other[scalar]) * other.group2(),
            // e23, e31, e12
            Simd32x3::from(other[scalar]) * other.group3(),
            // e423, e431, e412, e321
            Simd32x4::from(other[scalar]) * other.group4(),
        );
        return MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from(self[scalar]) * geometric_product.group0(),
            // e1, e2, e3, e4
            Simd32x4::from(self[scalar]) * geometric_product.group1(),
            // e41, e42, e43
            Simd32x3::from(self[scalar]) * geometric_product.group2(),
            // e23, e31, e12
            Simd32x3::from(self[scalar]) * geometric_product.group3(),
            // e423, e431, e412, e321
            Simd32x4::from(self[scalar]) * geometric_product.group4(),
        );
    }
}
impl GeometricQuotient<Plane> for Scalar {
    type Output = Plane;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        5        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        6        0
    //  no simd        0        9        0
    fn geometric_quotient(self, other: Plane) -> Self::Output {
        use crate::elements::*;
        let other = Scalar::from_groups(/* scalar */ f32::powi(other[e321], -2) * -1.0);
        return Plane::from_groups(
            // e423, e431, e412, e321
            Simd32x4::from(self[scalar]) * Simd32x4::from([other[e423] * other[scalar], other[e431] * other[scalar], other[e412] * other[scalar], other[e321] * other[scalar]]),
        );
    }
}
impl GeometricQuotient<Point> for Scalar {
    type Output = Point;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        4        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        2        5        0
    //  no simd        2        8        0
    fn geometric_quotient(self, other: Point) -> Self::Output {
        use crate::elements::*;
        let other = Scalar::from_groups(/* scalar */ f32::powi(other[e1], 2) + f32::powi(other[e2], 2) + f32::powi(other[e3], 2));
        return Point::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from(self[scalar]) * Simd32x4::from([other[e1] * other[scalar], other[e2] * other[scalar], other[e3] * other[scalar], other[e4] * other[scalar]]),
        );
    }
}
impl GeometricQuotient<Scalar> for Scalar {
    type Output = Scalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        2        0
    fn geometric_quotient(self, other: Scalar) -> Self::Output {
        use crate::elements::*;
        let other = Scalar::from_groups(/* scalar */ f32::powi(other[scalar], -2));
        return Scalar::from_groups(/* scalar */ other[scalar] * other[scalar] * self[scalar]);
    }
}
