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
impl std::ops::Div<geometric_quotient> for AntiScalar {
    type Output = geometric_quotient_partial<AntiScalar>;
    fn div(self, _rhs: geometric_quotient) -> Self::Output {
        geometric_quotient_partial(self)
    }
}
impl GeometricQuotient<DualNum> for AntiScalar {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        1
    fn geometric_quotient(self, other: DualNum) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(/* scalar */ f32::powi(other[scalar], 2));
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return AntiScalar::from_groups(/* e1234 */ self[e1234] * inverse[scalar]);
    }
}
impl GeometricQuotient<Flector> for AntiScalar {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        1        1
    fn geometric_quotient(self, other: Flector) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(
            // scalar
            f32::powi(other[e1], 2) + f32::powi(other[e2], 2) + f32::powi(other[e3], 2) - f32::powi(other[e321], 2),
        );
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return AntiScalar::from_groups(/* e1234 */ self[e1234] * inverse[scalar]);
    }
}
impl GeometricQuotient<Horizon> for AntiScalar {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        2        1
    fn geometric_quotient(self, other: Horizon) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(/* scalar */ f32::powi(other[e321], 2) * -1.0);
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return AntiScalar::from_groups(/* e1234 */ self[e1234] * inverse[scalar]);
    }
}
impl GeometricQuotient<Line> for AntiScalar {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        1        1
    fn geometric_quotient(self, other: Line) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(/* scalar */ -f32::powi(other[e23], 2) - f32::powi(other[e31], 2) - f32::powi(other[e12], 2));
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return AntiScalar::from_groups(/* e1234 */ self[e1234] * inverse[scalar]);
    }
}
impl GeometricQuotient<Motor> for AntiScalar {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        1        1
    fn geometric_quotient(self, other: Motor) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(
            // scalar
            f32::powi(other[scalar], 2) - f32::powi(other[e23], 2) - f32::powi(other[e31], 2) - f32::powi(other[e12], 2),
        );
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return AntiScalar::from_groups(/* e1234 */ self[e1234] * inverse[scalar]);
    }
}
impl GeometricQuotient<MultiVector> for AntiScalar {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        7        1        1
    fn geometric_quotient(self, other: MultiVector) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(
            // scalar
            f32::powi(other[scalar], 2) + f32::powi(other[e1], 2) + f32::powi(other[e2], 2) + f32::powi(other[e3], 2)
                - f32::powi(other[e23], 2)
                - f32::powi(other[e31], 2)
                - f32::powi(other[e12], 2)
                - f32::powi(other[e321], 2),
        );
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return AntiScalar::from_groups(/* e1234 */ self[e1234] * inverse[scalar]);
    }
}
impl GeometricQuotient<Plane> for AntiScalar {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        2        1
    fn geometric_quotient(self, other: Plane) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(/* scalar */ f32::powi(other[e321], 2) * -1.0);
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return AntiScalar::from_groups(/* e1234 */ self[e1234] * inverse[scalar]);
    }
}
impl GeometricQuotient<Point> for AntiScalar {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        1        1
    fn geometric_quotient(self, other: Point) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(/* scalar */ f32::powi(other[e1], 2) + f32::powi(other[e2], 2) + f32::powi(other[e3], 2));
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return AntiScalar::from_groups(/* e1234 */ self[e1234] * inverse[scalar]);
    }
}
impl GeometricQuotient<Scalar> for AntiScalar {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        1
    fn geometric_quotient(self, other: Scalar) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(/* scalar */ f32::powi(other[scalar], 2));
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return AntiScalar::from_groups(/* e1234 */ self[e1234] * inverse[scalar]);
    }
}
impl std::ops::Div<geometric_quotient> for DualNum {
    type Output = geometric_quotient_partial<DualNum>;
    fn div(self, _rhs: geometric_quotient) -> Self::Output {
        geometric_quotient_partial(self)
    }
}
impl GeometricQuotient<DualNum> for DualNum {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        0        1
    //    simd2        0        1        0
    // Totals...
    // yes simd        0        1        1
    //  no simd        0        2        1
    fn geometric_quotient(self, other: DualNum) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(/* scalar */ f32::powi(other[scalar], 2));
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return DualNum::from_groups(/* scalar, e1234 */ Simd32x2::from(inverse[scalar]) * self.group0());
    }
}
impl GeometricQuotient<Flector> for DualNum {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        0        1
    //    simd2        0        1        0
    // Totals...
    // yes simd        3        1        1
    //  no simd        3        2        1
    fn geometric_quotient(self, other: Flector) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(
            // scalar
            f32::powi(other[e1], 2) + f32::powi(other[e2], 2) + f32::powi(other[e3], 2) - f32::powi(other[e321], 2),
        );
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return DualNum::from_groups(/* scalar, e1234 */ Simd32x2::from(inverse[scalar]) * self.group0());
    }
}
impl GeometricQuotient<Horizon> for DualNum {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        1
    //    simd2        0        1        0
    // Totals...
    // yes simd        0        2        1
    //  no simd        0        3        1
    fn geometric_quotient(self, other: Horizon) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(/* scalar */ f32::powi(other[e321], 2) * -1.0);
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return DualNum::from_groups(/* scalar, e1234 */ Simd32x2::from(inverse[scalar]) * self.group0());
    }
}
impl GeometricQuotient<Line> for DualNum {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        0        1
    //    simd2        0        1        0
    // Totals...
    // yes simd        2        1        1
    //  no simd        2        2        1
    fn geometric_quotient(self, other: Line) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(/* scalar */ -f32::powi(other[e23], 2) - f32::powi(other[e31], 2) - f32::powi(other[e12], 2));
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return DualNum::from_groups(/* scalar, e1234 */ Simd32x2::from(inverse[scalar]) * self.group0());
    }
}
impl GeometricQuotient<Motor> for DualNum {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        0        1
    //    simd2        0        1        0
    // Totals...
    // yes simd        3        1        1
    //  no simd        3        2        1
    fn geometric_quotient(self, other: Motor) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(
            // scalar
            f32::powi(other[scalar], 2) - f32::powi(other[e23], 2) - f32::powi(other[e31], 2) - f32::powi(other[e12], 2),
        );
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return DualNum::from_groups(/* scalar, e1234 */ Simd32x2::from(inverse[scalar]) * self.group0());
    }
}
impl GeometricQuotient<MultiVector> for DualNum {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        7        0        1
    //    simd2        0        1        0
    // Totals...
    // yes simd        7        1        1
    //  no simd        7        2        1
    fn geometric_quotient(self, other: MultiVector) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(
            // scalar
            f32::powi(other[scalar], 2) + f32::powi(other[e1], 2) + f32::powi(other[e2], 2) + f32::powi(other[e3], 2)
                - f32::powi(other[e23], 2)
                - f32::powi(other[e31], 2)
                - f32::powi(other[e12], 2)
                - f32::powi(other[e321], 2),
        );
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return DualNum::from_groups(/* scalar, e1234 */ Simd32x2::from(inverse[scalar]) * self.group0());
    }
}
impl GeometricQuotient<Plane> for DualNum {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        1
    //    simd2        0        1        0
    // Totals...
    // yes simd        0        2        1
    //  no simd        0        3        1
    fn geometric_quotient(self, other: Plane) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(/* scalar */ f32::powi(other[e321], 2) * -1.0);
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return DualNum::from_groups(/* scalar, e1234 */ Simd32x2::from(inverse[scalar]) * self.group0());
    }
}
impl GeometricQuotient<Point> for DualNum {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        0        1
    //    simd2        0        1        0
    // Totals...
    // yes simd        2        1        1
    //  no simd        2        2        1
    fn geometric_quotient(self, other: Point) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(/* scalar */ f32::powi(other[e1], 2) + f32::powi(other[e2], 2) + f32::powi(other[e3], 2));
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return DualNum::from_groups(/* scalar, e1234 */ Simd32x2::from(inverse[scalar]) * self.group0());
    }
}
impl GeometricQuotient<Scalar> for DualNum {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        0        1
    //    simd2        0        1        0
    // Totals...
    // yes simd        0        1        1
    //  no simd        0        2        1
    fn geometric_quotient(self, other: Scalar) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(/* scalar */ f32::powi(other[scalar], 2));
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return DualNum::from_groups(/* scalar, e1234 */ Simd32x2::from(inverse[scalar]) * self.group0());
    }
}
impl std::ops::Div<geometric_quotient> for Flector {
    type Output = geometric_quotient_partial<Flector>;
    fn div(self, _rhs: geometric_quotient) -> Self::Output {
        geometric_quotient_partial(self)
    }
}
impl GeometricQuotient<DualNum> for Flector {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        0        1
    //    simd4        0        2        0
    // Totals...
    // yes simd        0        2        1
    //  no simd        0        8        1
    fn geometric_quotient(self, other: DualNum) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(/* scalar */ f32::powi(other[scalar], 2));
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return Flector::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from(inverse[scalar]) * self.group0(),
            // e423, e431, e412, e321
            Simd32x4::from(inverse[scalar]) * self.group1(),
        );
    }
}
impl GeometricQuotient<Flector> for Flector {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        0        1
    //    simd4        0        2        0
    // Totals...
    // yes simd        3        2        1
    //  no simd        3        8        1
    fn geometric_quotient(self, other: Flector) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(
            // scalar
            f32::powi(other[e1], 2) + f32::powi(other[e2], 2) + f32::powi(other[e3], 2) - f32::powi(other[e321], 2),
        );
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return Flector::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from(inverse[scalar]) * self.group0(),
            // e423, e431, e412, e321
            Simd32x4::from(inverse[scalar]) * self.group1(),
        );
    }
}
impl GeometricQuotient<Horizon> for Flector {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        1
    //    simd4        0        2        0
    // Totals...
    // yes simd        0        3        1
    //  no simd        0        9        1
    fn geometric_quotient(self, other: Horizon) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(/* scalar */ f32::powi(other[e321], 2) * -1.0);
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return Flector::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from(inverse[scalar]) * self.group0(),
            // e423, e431, e412, e321
            Simd32x4::from(inverse[scalar]) * self.group1(),
        );
    }
}
impl GeometricQuotient<Line> for Flector {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        0        1
    //    simd4        0        2        0
    // Totals...
    // yes simd        2        2        1
    //  no simd        2        8        1
    fn geometric_quotient(self, other: Line) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(/* scalar */ -f32::powi(other[e23], 2) - f32::powi(other[e31], 2) - f32::powi(other[e12], 2));
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return Flector::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from(inverse[scalar]) * self.group0(),
            // e423, e431, e412, e321
            Simd32x4::from(inverse[scalar]) * self.group1(),
        );
    }
}
impl GeometricQuotient<Motor> for Flector {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        0        1
    //    simd4        0        2        0
    // Totals...
    // yes simd        3        2        1
    //  no simd        3        8        1
    fn geometric_quotient(self, other: Motor) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(
            // scalar
            f32::powi(other[scalar], 2) - f32::powi(other[e23], 2) - f32::powi(other[e31], 2) - f32::powi(other[e12], 2),
        );
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return Flector::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from(inverse[scalar]) * self.group0(),
            // e423, e431, e412, e321
            Simd32x4::from(inverse[scalar]) * self.group1(),
        );
    }
}
impl GeometricQuotient<MultiVector> for Flector {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        7        0        1
    //    simd4        0        2        0
    // Totals...
    // yes simd        7        2        1
    //  no simd        7        8        1
    fn geometric_quotient(self, other: MultiVector) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(
            // scalar
            f32::powi(other[scalar], 2) + f32::powi(other[e1], 2) + f32::powi(other[e2], 2) + f32::powi(other[e3], 2)
                - f32::powi(other[e23], 2)
                - f32::powi(other[e31], 2)
                - f32::powi(other[e12], 2)
                - f32::powi(other[e321], 2),
        );
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return Flector::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from(inverse[scalar]) * self.group0(),
            // e423, e431, e412, e321
            Simd32x4::from(inverse[scalar]) * self.group1(),
        );
    }
}
impl GeometricQuotient<Plane> for Flector {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        1
    //    simd4        0        2        0
    // Totals...
    // yes simd        0        3        1
    //  no simd        0        9        1
    fn geometric_quotient(self, other: Plane) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(/* scalar */ f32::powi(other[e321], 2) * -1.0);
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return Flector::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from(inverse[scalar]) * self.group0(),
            // e423, e431, e412, e321
            Simd32x4::from(inverse[scalar]) * self.group1(),
        );
    }
}
impl GeometricQuotient<Point> for Flector {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        0        1
    //    simd4        0        2        0
    // Totals...
    // yes simd        2        2        1
    //  no simd        2        8        1
    fn geometric_quotient(self, other: Point) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(/* scalar */ f32::powi(other[e1], 2) + f32::powi(other[e2], 2) + f32::powi(other[e3], 2));
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return Flector::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from(inverse[scalar]) * self.group0(),
            // e423, e431, e412, e321
            Simd32x4::from(inverse[scalar]) * self.group1(),
        );
    }
}
impl GeometricQuotient<Scalar> for Flector {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        0        1
    //    simd4        0        2        0
    // Totals...
    // yes simd        0        2        1
    //  no simd        0        8        1
    fn geometric_quotient(self, other: Scalar) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(/* scalar */ f32::powi(other[scalar], 2));
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return Flector::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from(inverse[scalar]) * self.group0(),
            // e423, e431, e412, e321
            Simd32x4::from(inverse[scalar]) * self.group1(),
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
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        1
    fn geometric_quotient(self, other: DualNum) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(/* scalar */ f32::powi(other[scalar], 2));
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return Horizon::from_groups(/* e321 */ self[e321] * inverse[scalar]);
    }
}
impl GeometricQuotient<Flector> for Horizon {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        1        1
    fn geometric_quotient(self, other: Flector) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(
            // scalar
            f32::powi(other[e1], 2) + f32::powi(other[e2], 2) + f32::powi(other[e3], 2) - f32::powi(other[e321], 2),
        );
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return Horizon::from_groups(/* e321 */ self[e321] * inverse[scalar]);
    }
}
impl GeometricQuotient<Horizon> for Horizon {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        2        1
    fn geometric_quotient(self, other: Horizon) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(/* scalar */ f32::powi(other[e321], 2) * -1.0);
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return Horizon::from_groups(/* e321 */ self[e321] * inverse[scalar]);
    }
}
impl GeometricQuotient<Line> for Horizon {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        1        1
    fn geometric_quotient(self, other: Line) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(/* scalar */ -f32::powi(other[e23], 2) - f32::powi(other[e31], 2) - f32::powi(other[e12], 2));
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return Horizon::from_groups(/* e321 */ self[e321] * inverse[scalar]);
    }
}
impl GeometricQuotient<Motor> for Horizon {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        1        1
    fn geometric_quotient(self, other: Motor) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(
            // scalar
            f32::powi(other[scalar], 2) - f32::powi(other[e23], 2) - f32::powi(other[e31], 2) - f32::powi(other[e12], 2),
        );
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return Horizon::from_groups(/* e321 */ self[e321] * inverse[scalar]);
    }
}
impl GeometricQuotient<MultiVector> for Horizon {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        7        1        1
    fn geometric_quotient(self, other: MultiVector) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(
            // scalar
            f32::powi(other[scalar], 2) + f32::powi(other[e1], 2) + f32::powi(other[e2], 2) + f32::powi(other[e3], 2)
                - f32::powi(other[e23], 2)
                - f32::powi(other[e31], 2)
                - f32::powi(other[e12], 2)
                - f32::powi(other[e321], 2),
        );
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return Horizon::from_groups(/* e321 */ self[e321] * inverse[scalar]);
    }
}
impl GeometricQuotient<Plane> for Horizon {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        2        1
    fn geometric_quotient(self, other: Plane) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(/* scalar */ f32::powi(other[e321], 2) * -1.0);
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return Horizon::from_groups(/* e321 */ self[e321] * inverse[scalar]);
    }
}
impl GeometricQuotient<Point> for Horizon {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        1        1
    fn geometric_quotient(self, other: Point) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(/* scalar */ f32::powi(other[e1], 2) + f32::powi(other[e2], 2) + f32::powi(other[e3], 2));
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return Horizon::from_groups(/* e321 */ self[e321] * inverse[scalar]);
    }
}
impl GeometricQuotient<Scalar> for Horizon {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        1
    fn geometric_quotient(self, other: Scalar) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(/* scalar */ f32::powi(other[scalar], 2));
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return Horizon::from_groups(/* e321 */ self[e321] * inverse[scalar]);
    }
}
impl std::ops::Div<geometric_quotient> for Line {
    type Output = geometric_quotient_partial<Line>;
    fn div(self, _rhs: geometric_quotient) -> Self::Output {
        geometric_quotient_partial(self)
    }
}
impl GeometricQuotient<DualNum> for Line {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        0        1
    //    simd3        0        2        0
    // Totals...
    // yes simd        0        2        1
    //  no simd        0        6        1
    fn geometric_quotient(self, other: DualNum) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(/* scalar */ f32::powi(other[scalar], 2));
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return Line::from_groups(
            // e41, e42, e43
            Simd32x3::from(inverse[scalar]) * self.group0(),
            // e23, e31, e12
            Simd32x3::from(inverse[scalar]) * self.group1(),
        );
    }
}
impl GeometricQuotient<Flector> for Line {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        0        1
    //    simd3        0        2        0
    // Totals...
    // yes simd        3        2        1
    //  no simd        3        6        1
    fn geometric_quotient(self, other: Flector) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(
            // scalar
            f32::powi(other[e1], 2) + f32::powi(other[e2], 2) + f32::powi(other[e3], 2) - f32::powi(other[e321], 2),
        );
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return Line::from_groups(
            // e41, e42, e43
            Simd32x3::from(inverse[scalar]) * self.group0(),
            // e23, e31, e12
            Simd32x3::from(inverse[scalar]) * self.group1(),
        );
    }
}
impl GeometricQuotient<Horizon> for Line {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        1
    //    simd3        0        2        0
    // Totals...
    // yes simd        0        3        1
    //  no simd        0        7        1
    fn geometric_quotient(self, other: Horizon) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(/* scalar */ f32::powi(other[e321], 2) * -1.0);
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return Line::from_groups(
            // e41, e42, e43
            Simd32x3::from(inverse[scalar]) * self.group0(),
            // e23, e31, e12
            Simd32x3::from(inverse[scalar]) * self.group1(),
        );
    }
}
impl GeometricQuotient<Line> for Line {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        0        1
    //    simd3        0        2        0
    // Totals...
    // yes simd        2        2        1
    //  no simd        2        6        1
    fn geometric_quotient(self, other: Line) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(/* scalar */ -f32::powi(other[e23], 2) - f32::powi(other[e31], 2) - f32::powi(other[e12], 2));
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return Line::from_groups(
            // e41, e42, e43
            Simd32x3::from(inverse[scalar]) * self.group0(),
            // e23, e31, e12
            Simd32x3::from(inverse[scalar]) * self.group1(),
        );
    }
}
impl GeometricQuotient<Motor> for Line {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        0        1
    //    simd3        0        2        0
    // Totals...
    // yes simd        3        2        1
    //  no simd        3        6        1
    fn geometric_quotient(self, other: Motor) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(
            // scalar
            f32::powi(other[scalar], 2) - f32::powi(other[e23], 2) - f32::powi(other[e31], 2) - f32::powi(other[e12], 2),
        );
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return Line::from_groups(
            // e41, e42, e43
            Simd32x3::from(inverse[scalar]) * self.group0(),
            // e23, e31, e12
            Simd32x3::from(inverse[scalar]) * self.group1(),
        );
    }
}
impl GeometricQuotient<MultiVector> for Line {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        7        0        1
    //    simd3        0        2        0
    // Totals...
    // yes simd        7        2        1
    //  no simd        7        6        1
    fn geometric_quotient(self, other: MultiVector) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(
            // scalar
            f32::powi(other[scalar], 2) + f32::powi(other[e1], 2) + f32::powi(other[e2], 2) + f32::powi(other[e3], 2)
                - f32::powi(other[e23], 2)
                - f32::powi(other[e31], 2)
                - f32::powi(other[e12], 2)
                - f32::powi(other[e321], 2),
        );
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return Line::from_groups(
            // e41, e42, e43
            Simd32x3::from(inverse[scalar]) * self.group0(),
            // e23, e31, e12
            Simd32x3::from(inverse[scalar]) * self.group1(),
        );
    }
}
impl GeometricQuotient<Plane> for Line {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        1
    //    simd3        0        2        0
    // Totals...
    // yes simd        0        3        1
    //  no simd        0        7        1
    fn geometric_quotient(self, other: Plane) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(/* scalar */ f32::powi(other[e321], 2) * -1.0);
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return Line::from_groups(
            // e41, e42, e43
            Simd32x3::from(inverse[scalar]) * self.group0(),
            // e23, e31, e12
            Simd32x3::from(inverse[scalar]) * self.group1(),
        );
    }
}
impl GeometricQuotient<Point> for Line {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        0        1
    //    simd3        0        2        0
    // Totals...
    // yes simd        2        2        1
    //  no simd        2        6        1
    fn geometric_quotient(self, other: Point) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(/* scalar */ f32::powi(other[e1], 2) + f32::powi(other[e2], 2) + f32::powi(other[e3], 2));
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return Line::from_groups(
            // e41, e42, e43
            Simd32x3::from(inverse[scalar]) * self.group0(),
            // e23, e31, e12
            Simd32x3::from(inverse[scalar]) * self.group1(),
        );
    }
}
impl GeometricQuotient<Scalar> for Line {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        0        1
    //    simd3        0        2        0
    // Totals...
    // yes simd        0        2        1
    //  no simd        0        6        1
    fn geometric_quotient(self, other: Scalar) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(/* scalar */ f32::powi(other[scalar], 2));
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return Line::from_groups(
            // e41, e42, e43
            Simd32x3::from(inverse[scalar]) * self.group0(),
            // e23, e31, e12
            Simd32x3::from(inverse[scalar]) * self.group1(),
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
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        0        1
    //    simd4        0        2        0
    // Totals...
    // yes simd        0        2        1
    //  no simd        0        8        1
    fn geometric_quotient(self, other: DualNum) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(/* scalar */ f32::powi(other[scalar], 2));
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return Motor::from_groups(
            // e41, e42, e43, e1234
            Simd32x4::from(inverse[scalar]) * self.group0(),
            // e23, e31, e12, scalar
            Simd32x4::from(inverse[scalar]) * self.group1(),
        );
    }
}
impl GeometricQuotient<Flector> for Motor {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        0        1
    //    simd4        0        2        0
    // Totals...
    // yes simd        3        2        1
    //  no simd        3        8        1
    fn geometric_quotient(self, other: Flector) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(
            // scalar
            f32::powi(other[e1], 2) + f32::powi(other[e2], 2) + f32::powi(other[e3], 2) - f32::powi(other[e321], 2),
        );
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return Motor::from_groups(
            // e41, e42, e43, e1234
            Simd32x4::from(inverse[scalar]) * self.group0(),
            // e23, e31, e12, scalar
            Simd32x4::from(inverse[scalar]) * self.group1(),
        );
    }
}
impl GeometricQuotient<Horizon> for Motor {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        1
    //    simd4        0        2        0
    // Totals...
    // yes simd        0        3        1
    //  no simd        0        9        1
    fn geometric_quotient(self, other: Horizon) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(/* scalar */ f32::powi(other[e321], 2) * -1.0);
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return Motor::from_groups(
            // e41, e42, e43, e1234
            Simd32x4::from(inverse[scalar]) * self.group0(),
            // e23, e31, e12, scalar
            Simd32x4::from(inverse[scalar]) * self.group1(),
        );
    }
}
impl GeometricQuotient<Line> for Motor {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        0        1
    //    simd4        0        2        0
    // Totals...
    // yes simd        2        2        1
    //  no simd        2        8        1
    fn geometric_quotient(self, other: Line) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(/* scalar */ -f32::powi(other[e23], 2) - f32::powi(other[e31], 2) - f32::powi(other[e12], 2));
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return Motor::from_groups(
            // e41, e42, e43, e1234
            Simd32x4::from(inverse[scalar]) * self.group0(),
            // e23, e31, e12, scalar
            Simd32x4::from(inverse[scalar]) * self.group1(),
        );
    }
}
impl GeometricQuotient<Motor> for Motor {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        0        1
    //    simd4        0        2        0
    // Totals...
    // yes simd        3        2        1
    //  no simd        3        8        1
    fn geometric_quotient(self, other: Motor) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(
            // scalar
            f32::powi(other[scalar], 2) - f32::powi(other[e23], 2) - f32::powi(other[e31], 2) - f32::powi(other[e12], 2),
        );
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return Motor::from_groups(
            // e41, e42, e43, e1234
            Simd32x4::from(inverse[scalar]) * self.group0(),
            // e23, e31, e12, scalar
            Simd32x4::from(inverse[scalar]) * self.group1(),
        );
    }
}
impl GeometricQuotient<MultiVector> for Motor {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        7        0        1
    //    simd4        0        2        0
    // Totals...
    // yes simd        7        2        1
    //  no simd        7        8        1
    fn geometric_quotient(self, other: MultiVector) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(
            // scalar
            f32::powi(other[scalar], 2) + f32::powi(other[e1], 2) + f32::powi(other[e2], 2) + f32::powi(other[e3], 2)
                - f32::powi(other[e23], 2)
                - f32::powi(other[e31], 2)
                - f32::powi(other[e12], 2)
                - f32::powi(other[e321], 2),
        );
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return Motor::from_groups(
            // e41, e42, e43, e1234
            Simd32x4::from(inverse[scalar]) * self.group0(),
            // e23, e31, e12, scalar
            Simd32x4::from(inverse[scalar]) * self.group1(),
        );
    }
}
impl GeometricQuotient<Plane> for Motor {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        1
    //    simd4        0        2        0
    // Totals...
    // yes simd        0        3        1
    //  no simd        0        9        1
    fn geometric_quotient(self, other: Plane) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(/* scalar */ f32::powi(other[e321], 2) * -1.0);
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return Motor::from_groups(
            // e41, e42, e43, e1234
            Simd32x4::from(inverse[scalar]) * self.group0(),
            // e23, e31, e12, scalar
            Simd32x4::from(inverse[scalar]) * self.group1(),
        );
    }
}
impl GeometricQuotient<Point> for Motor {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        0        1
    //    simd4        0        2        0
    // Totals...
    // yes simd        2        2        1
    //  no simd        2        8        1
    fn geometric_quotient(self, other: Point) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(/* scalar */ f32::powi(other[e1], 2) + f32::powi(other[e2], 2) + f32::powi(other[e3], 2));
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return Motor::from_groups(
            // e41, e42, e43, e1234
            Simd32x4::from(inverse[scalar]) * self.group0(),
            // e23, e31, e12, scalar
            Simd32x4::from(inverse[scalar]) * self.group1(),
        );
    }
}
impl GeometricQuotient<Scalar> for Motor {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        0        1
    //    simd4        0        2        0
    // Totals...
    // yes simd        0        2        1
    //  no simd        0        8        1
    fn geometric_quotient(self, other: Scalar) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(/* scalar */ f32::powi(other[scalar], 2));
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return Motor::from_groups(
            // e41, e42, e43, e1234
            Simd32x4::from(inverse[scalar]) * self.group0(),
            // e23, e31, e12, scalar
            Simd32x4::from(inverse[scalar]) * self.group1(),
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
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        0        1
    //    simd2        0        1        0
    //    simd3        0        2        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        0        5        1
    //  no simd        0       16        1
    fn geometric_quotient(self, other: DualNum) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(/* scalar */ f32::powi(other[scalar], 2));
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from(inverse[scalar]) * self.group0(),
            // e1, e2, e3, e4
            Simd32x4::from(inverse[scalar]) * self.group1(),
            // e41, e42, e43
            Simd32x3::from(inverse[scalar]) * self.group2(),
            // e23, e31, e12
            Simd32x3::from(inverse[scalar]) * self.group3(),
            // e423, e431, e412, e321
            Simd32x4::from(inverse[scalar]) * self.group4(),
        );
    }
}
impl GeometricQuotient<Flector> for MultiVector {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        0        1
    //    simd2        0        1        0
    //    simd3        0        2        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        3        5        1
    //  no simd        3       16        1
    fn geometric_quotient(self, other: Flector) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(
            // scalar
            f32::powi(other[e1], 2) + f32::powi(other[e2], 2) + f32::powi(other[e3], 2) - f32::powi(other[e321], 2),
        );
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from(inverse[scalar]) * self.group0(),
            // e1, e2, e3, e4
            Simd32x4::from(inverse[scalar]) * self.group1(),
            // e41, e42, e43
            Simd32x3::from(inverse[scalar]) * self.group2(),
            // e23, e31, e12
            Simd32x3::from(inverse[scalar]) * self.group3(),
            // e423, e431, e412, e321
            Simd32x4::from(inverse[scalar]) * self.group4(),
        );
    }
}
impl GeometricQuotient<Horizon> for MultiVector {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        1
    //    simd2        0        1        0
    //    simd3        0        2        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        0        6        1
    //  no simd        0       17        1
    fn geometric_quotient(self, other: Horizon) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(/* scalar */ f32::powi(other[e321], 2) * -1.0);
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from(inverse[scalar]) * self.group0(),
            // e1, e2, e3, e4
            Simd32x4::from(inverse[scalar]) * self.group1(),
            // e41, e42, e43
            Simd32x3::from(inverse[scalar]) * self.group2(),
            // e23, e31, e12
            Simd32x3::from(inverse[scalar]) * self.group3(),
            // e423, e431, e412, e321
            Simd32x4::from(inverse[scalar]) * self.group4(),
        );
    }
}
impl GeometricQuotient<Line> for MultiVector {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        0        1
    //    simd2        0        1        0
    //    simd3        0        2        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        2        5        1
    //  no simd        2       16        1
    fn geometric_quotient(self, other: Line) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(/* scalar */ -f32::powi(other[e23], 2) - f32::powi(other[e31], 2) - f32::powi(other[e12], 2));
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from(inverse[scalar]) * self.group0(),
            // e1, e2, e3, e4
            Simd32x4::from(inverse[scalar]) * self.group1(),
            // e41, e42, e43
            Simd32x3::from(inverse[scalar]) * self.group2(),
            // e23, e31, e12
            Simd32x3::from(inverse[scalar]) * self.group3(),
            // e423, e431, e412, e321
            Simd32x4::from(inverse[scalar]) * self.group4(),
        );
    }
}
impl GeometricQuotient<Motor> for MultiVector {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        0        1
    //    simd2        0        1        0
    //    simd3        0        2        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        3        5        1
    //  no simd        3       16        1
    fn geometric_quotient(self, other: Motor) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(
            // scalar
            f32::powi(other[scalar], 2) - f32::powi(other[e23], 2) - f32::powi(other[e31], 2) - f32::powi(other[e12], 2),
        );
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from(inverse[scalar]) * self.group0(),
            // e1, e2, e3, e4
            Simd32x4::from(inverse[scalar]) * self.group1(),
            // e41, e42, e43
            Simd32x3::from(inverse[scalar]) * self.group2(),
            // e23, e31, e12
            Simd32x3::from(inverse[scalar]) * self.group3(),
            // e423, e431, e412, e321
            Simd32x4::from(inverse[scalar]) * self.group4(),
        );
    }
}
impl GeometricQuotient<MultiVector> for MultiVector {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        7        0        1
    //    simd2        0        1        0
    //    simd3        0        2        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        7        5        1
    //  no simd        7       16        1
    fn geometric_quotient(self, other: MultiVector) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(
            // scalar
            f32::powi(other[scalar], 2) + f32::powi(other[e1], 2) + f32::powi(other[e2], 2) + f32::powi(other[e3], 2)
                - f32::powi(other[e23], 2)
                - f32::powi(other[e31], 2)
                - f32::powi(other[e12], 2)
                - f32::powi(other[e321], 2),
        );
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from(inverse[scalar]) * self.group0(),
            // e1, e2, e3, e4
            Simd32x4::from(inverse[scalar]) * self.group1(),
            // e41, e42, e43
            Simd32x3::from(inverse[scalar]) * self.group2(),
            // e23, e31, e12
            Simd32x3::from(inverse[scalar]) * self.group3(),
            // e423, e431, e412, e321
            Simd32x4::from(inverse[scalar]) * self.group4(),
        );
    }
}
impl GeometricQuotient<Plane> for MultiVector {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        1
    //    simd2        0        1        0
    //    simd3        0        2        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        0        6        1
    //  no simd        0       17        1
    fn geometric_quotient(self, other: Plane) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(/* scalar */ f32::powi(other[e321], 2) * -1.0);
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from(inverse[scalar]) * self.group0(),
            // e1, e2, e3, e4
            Simd32x4::from(inverse[scalar]) * self.group1(),
            // e41, e42, e43
            Simd32x3::from(inverse[scalar]) * self.group2(),
            // e23, e31, e12
            Simd32x3::from(inverse[scalar]) * self.group3(),
            // e423, e431, e412, e321
            Simd32x4::from(inverse[scalar]) * self.group4(),
        );
    }
}
impl GeometricQuotient<Point> for MultiVector {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        0        1
    //    simd2        0        1        0
    //    simd3        0        2        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        2        5        1
    //  no simd        2       16        1
    fn geometric_quotient(self, other: Point) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(/* scalar */ f32::powi(other[e1], 2) + f32::powi(other[e2], 2) + f32::powi(other[e3], 2));
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from(inverse[scalar]) * self.group0(),
            // e1, e2, e3, e4
            Simd32x4::from(inverse[scalar]) * self.group1(),
            // e41, e42, e43
            Simd32x3::from(inverse[scalar]) * self.group2(),
            // e23, e31, e12
            Simd32x3::from(inverse[scalar]) * self.group3(),
            // e423, e431, e412, e321
            Simd32x4::from(inverse[scalar]) * self.group4(),
        );
    }
}
impl GeometricQuotient<Scalar> for MultiVector {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        0        1
    //    simd2        0        1        0
    //    simd3        0        2        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        0        5        1
    //  no simd        0       16        1
    fn geometric_quotient(self, other: Scalar) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(/* scalar */ f32::powi(other[scalar], 2));
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from(inverse[scalar]) * self.group0(),
            // e1, e2, e3, e4
            Simd32x4::from(inverse[scalar]) * self.group1(),
            // e41, e42, e43
            Simd32x3::from(inverse[scalar]) * self.group2(),
            // e23, e31, e12
            Simd32x3::from(inverse[scalar]) * self.group3(),
            // e423, e431, e412, e321
            Simd32x4::from(inverse[scalar]) * self.group4(),
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
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        1
    fn geometric_quotient(self, other: DualNum) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(/* scalar */ f32::powi(other[scalar], 2));
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return Origin::from_groups(/* e4 */ self[e4] * inverse[scalar]);
    }
}
impl GeometricQuotient<Flector> for Origin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        1        1
    fn geometric_quotient(self, other: Flector) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(
            // scalar
            f32::powi(other[e1], 2) + f32::powi(other[e2], 2) + f32::powi(other[e3], 2) - f32::powi(other[e321], 2),
        );
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return Origin::from_groups(/* e4 */ self[e4] * inverse[scalar]);
    }
}
impl GeometricQuotient<Horizon> for Origin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        2        1
    fn geometric_quotient(self, other: Horizon) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(/* scalar */ f32::powi(other[e321], 2) * -1.0);
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return Origin::from_groups(/* e4 */ self[e4] * inverse[scalar]);
    }
}
impl GeometricQuotient<Line> for Origin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        1        1
    fn geometric_quotient(self, other: Line) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(/* scalar */ -f32::powi(other[e23], 2) - f32::powi(other[e31], 2) - f32::powi(other[e12], 2));
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return Origin::from_groups(/* e4 */ self[e4] * inverse[scalar]);
    }
}
impl GeometricQuotient<Motor> for Origin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        1        1
    fn geometric_quotient(self, other: Motor) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(
            // scalar
            f32::powi(other[scalar], 2) - f32::powi(other[e23], 2) - f32::powi(other[e31], 2) - f32::powi(other[e12], 2),
        );
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return Origin::from_groups(/* e4 */ self[e4] * inverse[scalar]);
    }
}
impl GeometricQuotient<MultiVector> for Origin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        7        1        1
    fn geometric_quotient(self, other: MultiVector) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(
            // scalar
            f32::powi(other[scalar], 2) + f32::powi(other[e1], 2) + f32::powi(other[e2], 2) + f32::powi(other[e3], 2)
                - f32::powi(other[e23], 2)
                - f32::powi(other[e31], 2)
                - f32::powi(other[e12], 2)
                - f32::powi(other[e321], 2),
        );
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return Origin::from_groups(/* e4 */ self[e4] * inverse[scalar]);
    }
}
impl GeometricQuotient<Plane> for Origin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        2        1
    fn geometric_quotient(self, other: Plane) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(/* scalar */ f32::powi(other[e321], 2) * -1.0);
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return Origin::from_groups(/* e4 */ self[e4] * inverse[scalar]);
    }
}
impl GeometricQuotient<Point> for Origin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        1        1
    fn geometric_quotient(self, other: Point) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(/* scalar */ f32::powi(other[e1], 2) + f32::powi(other[e2], 2) + f32::powi(other[e3], 2));
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return Origin::from_groups(/* e4 */ self[e4] * inverse[scalar]);
    }
}
impl GeometricQuotient<Scalar> for Origin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        1
    fn geometric_quotient(self, other: Scalar) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(/* scalar */ f32::powi(other[scalar], 2));
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return Origin::from_groups(/* e4 */ self[e4] * inverse[scalar]);
    }
}
impl std::ops::Div<geometric_quotient> for Plane {
    type Output = geometric_quotient_partial<Plane>;
    fn div(self, _rhs: geometric_quotient) -> Self::Output {
        geometric_quotient_partial(self)
    }
}
impl GeometricQuotient<DualNum> for Plane {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        0        1
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        1        1
    //  no simd        0        4        1
    fn geometric_quotient(self, other: DualNum) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(/* scalar */ f32::powi(other[scalar], 2));
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return Plane::from_groups(/* e423, e431, e412, e321 */ Simd32x4::from(inverse[scalar]) * self.group0());
    }
}
impl GeometricQuotient<Flector> for Plane {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        0        1
    //    simd4        0        1        0
    // Totals...
    // yes simd        3        1        1
    //  no simd        3        4        1
    fn geometric_quotient(self, other: Flector) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(
            // scalar
            f32::powi(other[e1], 2) + f32::powi(other[e2], 2) + f32::powi(other[e3], 2) - f32::powi(other[e321], 2),
        );
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return Plane::from_groups(/* e423, e431, e412, e321 */ Simd32x4::from(inverse[scalar]) * self.group0());
    }
}
impl GeometricQuotient<Horizon> for Plane {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        1
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        2        1
    //  no simd        0        5        1
    fn geometric_quotient(self, other: Horizon) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(/* scalar */ f32::powi(other[e321], 2) * -1.0);
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return Plane::from_groups(/* e423, e431, e412, e321 */ Simd32x4::from(inverse[scalar]) * self.group0());
    }
}
impl GeometricQuotient<Line> for Plane {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        0        1
    //    simd4        0        1        0
    // Totals...
    // yes simd        2        1        1
    //  no simd        2        4        1
    fn geometric_quotient(self, other: Line) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(/* scalar */ -f32::powi(other[e23], 2) - f32::powi(other[e31], 2) - f32::powi(other[e12], 2));
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return Plane::from_groups(/* e423, e431, e412, e321 */ Simd32x4::from(inverse[scalar]) * self.group0());
    }
}
impl GeometricQuotient<Motor> for Plane {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        0        1
    //    simd4        0        1        0
    // Totals...
    // yes simd        3        1        1
    //  no simd        3        4        1
    fn geometric_quotient(self, other: Motor) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(
            // scalar
            f32::powi(other[scalar], 2) - f32::powi(other[e23], 2) - f32::powi(other[e31], 2) - f32::powi(other[e12], 2),
        );
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return Plane::from_groups(/* e423, e431, e412, e321 */ Simd32x4::from(inverse[scalar]) * self.group0());
    }
}
impl GeometricQuotient<MultiVector> for Plane {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        7        0        1
    //    simd4        0        1        0
    // Totals...
    // yes simd        7        1        1
    //  no simd        7        4        1
    fn geometric_quotient(self, other: MultiVector) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(
            // scalar
            f32::powi(other[scalar], 2) + f32::powi(other[e1], 2) + f32::powi(other[e2], 2) + f32::powi(other[e3], 2)
                - f32::powi(other[e23], 2)
                - f32::powi(other[e31], 2)
                - f32::powi(other[e12], 2)
                - f32::powi(other[e321], 2),
        );
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return Plane::from_groups(/* e423, e431, e412, e321 */ Simd32x4::from(inverse[scalar]) * self.group0());
    }
}
impl GeometricQuotient<Plane> for Plane {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        1
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        2        1
    //  no simd        0        5        1
    fn geometric_quotient(self, other: Plane) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(/* scalar */ f32::powi(other[e321], 2) * -1.0);
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return Plane::from_groups(/* e423, e431, e412, e321 */ Simd32x4::from(inverse[scalar]) * self.group0());
    }
}
impl GeometricQuotient<Point> for Plane {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        0        1
    //    simd4        0        1        0
    // Totals...
    // yes simd        2        1        1
    //  no simd        2        4        1
    fn geometric_quotient(self, other: Point) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(/* scalar */ f32::powi(other[e1], 2) + f32::powi(other[e2], 2) + f32::powi(other[e3], 2));
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return Plane::from_groups(/* e423, e431, e412, e321 */ Simd32x4::from(inverse[scalar]) * self.group0());
    }
}
impl GeometricQuotient<Scalar> for Plane {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        0        1
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        1        1
    //  no simd        0        4        1
    fn geometric_quotient(self, other: Scalar) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(/* scalar */ f32::powi(other[scalar], 2));
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return Plane::from_groups(/* e423, e431, e412, e321 */ Simd32x4::from(inverse[scalar]) * self.group0());
    }
}
impl std::ops::Div<geometric_quotient> for Point {
    type Output = geometric_quotient_partial<Point>;
    fn div(self, _rhs: geometric_quotient) -> Self::Output {
        geometric_quotient_partial(self)
    }
}
impl GeometricQuotient<DualNum> for Point {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        0        1
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        1        1
    //  no simd        0        4        1
    fn geometric_quotient(self, other: DualNum) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(/* scalar */ f32::powi(other[scalar], 2));
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return Point::from_groups(/* e1, e2, e3, e4 */ Simd32x4::from(inverse[scalar]) * self.group0());
    }
}
impl GeometricQuotient<Flector> for Point {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        0        1
    //    simd4        0        1        0
    // Totals...
    // yes simd        3        1        1
    //  no simd        3        4        1
    fn geometric_quotient(self, other: Flector) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(
            // scalar
            f32::powi(other[e1], 2) + f32::powi(other[e2], 2) + f32::powi(other[e3], 2) - f32::powi(other[e321], 2),
        );
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return Point::from_groups(/* e1, e2, e3, e4 */ Simd32x4::from(inverse[scalar]) * self.group0());
    }
}
impl GeometricQuotient<Horizon> for Point {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        1
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        2        1
    //  no simd        0        5        1
    fn geometric_quotient(self, other: Horizon) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(/* scalar */ f32::powi(other[e321], 2) * -1.0);
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return Point::from_groups(/* e1, e2, e3, e4 */ Simd32x4::from(inverse[scalar]) * self.group0());
    }
}
impl GeometricQuotient<Line> for Point {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        0        1
    //    simd4        0        1        0
    // Totals...
    // yes simd        2        1        1
    //  no simd        2        4        1
    fn geometric_quotient(self, other: Line) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(/* scalar */ -f32::powi(other[e23], 2) - f32::powi(other[e31], 2) - f32::powi(other[e12], 2));
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return Point::from_groups(/* e1, e2, e3, e4 */ Simd32x4::from(inverse[scalar]) * self.group0());
    }
}
impl GeometricQuotient<Motor> for Point {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        0        1
    //    simd4        0        1        0
    // Totals...
    // yes simd        3        1        1
    //  no simd        3        4        1
    fn geometric_quotient(self, other: Motor) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(
            // scalar
            f32::powi(other[scalar], 2) - f32::powi(other[e23], 2) - f32::powi(other[e31], 2) - f32::powi(other[e12], 2),
        );
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return Point::from_groups(/* e1, e2, e3, e4 */ Simd32x4::from(inverse[scalar]) * self.group0());
    }
}
impl GeometricQuotient<MultiVector> for Point {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        7        0        1
    //    simd4        0        1        0
    // Totals...
    // yes simd        7        1        1
    //  no simd        7        4        1
    fn geometric_quotient(self, other: MultiVector) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(
            // scalar
            f32::powi(other[scalar], 2) + f32::powi(other[e1], 2) + f32::powi(other[e2], 2) + f32::powi(other[e3], 2)
                - f32::powi(other[e23], 2)
                - f32::powi(other[e31], 2)
                - f32::powi(other[e12], 2)
                - f32::powi(other[e321], 2),
        );
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return Point::from_groups(/* e1, e2, e3, e4 */ Simd32x4::from(inverse[scalar]) * self.group0());
    }
}
impl GeometricQuotient<Plane> for Point {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        1
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        2        1
    //  no simd        0        5        1
    fn geometric_quotient(self, other: Plane) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(/* scalar */ f32::powi(other[e321], 2) * -1.0);
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return Point::from_groups(/* e1, e2, e3, e4 */ Simd32x4::from(inverse[scalar]) * self.group0());
    }
}
impl GeometricQuotient<Point> for Point {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        0        1
    //    simd4        0        1        0
    // Totals...
    // yes simd        2        1        1
    //  no simd        2        4        1
    fn geometric_quotient(self, other: Point) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(/* scalar */ f32::powi(other[e1], 2) + f32::powi(other[e2], 2) + f32::powi(other[e3], 2));
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return Point::from_groups(/* e1, e2, e3, e4 */ Simd32x4::from(inverse[scalar]) * self.group0());
    }
}
impl GeometricQuotient<Scalar> for Point {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        0        1
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        1        1
    //  no simd        0        4        1
    fn geometric_quotient(self, other: Scalar) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(/* scalar */ f32::powi(other[scalar], 2));
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return Point::from_groups(/* e1, e2, e3, e4 */ Simd32x4::from(inverse[scalar]) * self.group0());
    }
}
impl std::ops::Div<geometric_quotient> for Scalar {
    type Output = geometric_quotient_partial<Scalar>;
    fn div(self, _rhs: geometric_quotient) -> Self::Output {
        geometric_quotient_partial(self)
    }
}
impl GeometricQuotient<DualNum> for Scalar {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        1
    fn geometric_quotient(self, other: DualNum) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(/* scalar */ f32::powi(other[scalar], 2));
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return Scalar::from_groups(/* scalar */ inverse[scalar] * self[scalar]);
    }
}
impl GeometricQuotient<Flector> for Scalar {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        1        1
    fn geometric_quotient(self, other: Flector) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(
            // scalar
            f32::powi(other[e1], 2) + f32::powi(other[e2], 2) + f32::powi(other[e3], 2) - f32::powi(other[e321], 2),
        );
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return Scalar::from_groups(/* scalar */ inverse[scalar] * self[scalar]);
    }
}
impl GeometricQuotient<Horizon> for Scalar {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        2        1
    fn geometric_quotient(self, other: Horizon) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(/* scalar */ f32::powi(other[e321], 2) * -1.0);
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return Scalar::from_groups(/* scalar */ inverse[scalar] * self[scalar]);
    }
}
impl GeometricQuotient<Line> for Scalar {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        1        1
    fn geometric_quotient(self, other: Line) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(/* scalar */ -f32::powi(other[e23], 2) - f32::powi(other[e31], 2) - f32::powi(other[e12], 2));
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return Scalar::from_groups(/* scalar */ inverse[scalar] * self[scalar]);
    }
}
impl GeometricQuotient<Motor> for Scalar {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        1        1
    fn geometric_quotient(self, other: Motor) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(
            // scalar
            f32::powi(other[scalar], 2) - f32::powi(other[e23], 2) - f32::powi(other[e31], 2) - f32::powi(other[e12], 2),
        );
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return Scalar::from_groups(/* scalar */ inverse[scalar] * self[scalar]);
    }
}
impl GeometricQuotient<MultiVector> for Scalar {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        7        1        1
    fn geometric_quotient(self, other: MultiVector) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(
            // scalar
            f32::powi(other[scalar], 2) + f32::powi(other[e1], 2) + f32::powi(other[e2], 2) + f32::powi(other[e3], 2)
                - f32::powi(other[e23], 2)
                - f32::powi(other[e31], 2)
                - f32::powi(other[e12], 2)
                - f32::powi(other[e321], 2),
        );
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return Scalar::from_groups(/* scalar */ inverse[scalar] * self[scalar]);
    }
}
impl GeometricQuotient<Plane> for Scalar {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        2        1
    fn geometric_quotient(self, other: Plane) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(/* scalar */ f32::powi(other[e321], 2) * -1.0);
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return Scalar::from_groups(/* scalar */ inverse[scalar] * self[scalar]);
    }
}
impl GeometricQuotient<Point> for Scalar {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        1        1
    fn geometric_quotient(self, other: Point) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(/* scalar */ f32::powi(other[e1], 2) + f32::powi(other[e2], 2) + f32::powi(other[e3], 2));
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return Scalar::from_groups(/* scalar */ inverse[scalar] * self[scalar]);
    }
}
impl GeometricQuotient<Scalar> for Scalar {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        1
    fn geometric_quotient(self, other: Scalar) -> Self {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(/* scalar */ f32::powi(other[scalar], 2));
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return Scalar::from_groups(/* scalar */ inverse[scalar] * self[scalar]);
    }
}
