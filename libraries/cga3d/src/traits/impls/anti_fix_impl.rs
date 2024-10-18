// Note on Operative Statistics:
// Operative Statistics are not a precise predictor of performance or performance comparisons.
// This is due to varying hardware capabilities and compiler optimizations.
// As always, where performance is a concern, there is no substitute for
// real measurements on real work-loads on real hardware.
// Disclaimer aside, enjoy the fun information =)
//
// Total Implementations: 8
//
// Yes SIMD:   add/sub     mul     div
//  Minimum:         0       1       1
//   Median:         2       3       1
//  Average:         1       2       1
//  Maximum:         3       4       1
//
//  No SIMD:   add/sub     mul     div
//  Minimum:         0       1       1
//   Median:         2       7       1
//  Average:         1       5       1
//  Maximum:         3      10       1
impl AntiFixImpl for AntiFlatPoint {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        1
    //    simd4        0        2        0
    // Totals...
    // yes simd        0        3        1
    //  no simd        0        9        1
    fn anti_fix_impl(self) -> Self {
        use crate::elements::*;
        let anti_reverse = AntiFlatPoint::from_groups(/* e235, e315, e125, e321 */ (self.group0() * Simd32x4::from(-1.0)));
        let geometric_anti_product = AntiScalar::from_groups(/* e12345 */ (anti_reverse.group0()[3] * self.group0()[3]));
        let anti_square_root = AntiScalar::from_groups(/* e12345 */ f32::powf(geometric_anti_product[e12345], 0.5));
        let anti_scalar_product = AntiScalar::from_groups(/* e12345 */ f32::powi(anti_square_root[e12345], 2));
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ (1.0 / anti_scalar_product[e12345]));
        return AntiFlatPoint::from_groups(/* e235, e315, e125, e321 */ (Simd32x4::from(anti_inverse[e12345]) * self.group0()));
    }
}
impl AntiFixImpl for AntiPlane {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        0        1
    //    simd4        0        1        0
    // Totals...
    // yes simd        2        1        1
    //  no simd        2        4        1
    fn anti_fix_impl(self) -> Self {
        use crate::elements::*;
        let geometric_anti_product = AntiScalar::from_groups(/* e12345 */ (-f32::powi(self.group0()[0], 2) - f32::powi(self.group0()[1], 2) - f32::powi(self.group0()[2], 2)));
        let anti_square_root = AntiScalar::from_groups(/* e12345 */ f32::powf(geometric_anti_product[e12345], 0.5));
        let anti_scalar_product = AntiScalar::from_groups(/* e12345 */ f32::powi(anti_square_root[e12345], 2));
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ (1.0 / anti_scalar_product[e12345]));
        return AntiPlane::from_groups(/* e1, e2, e3, e5 */ (Simd32x4::from(anti_inverse[e12345]) * self.group0()));
    }
}
impl AntiFixImpl for AntiScalar {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        1
    fn anti_fix_impl(self) -> Self {
        use crate::elements::*;
        let geometric_anti_product = AntiScalar::from_groups(/* e12345 */ f32::powi(self[e12345], 2));
        let anti_square_root = AntiScalar::from_groups(/* e12345 */ f32::powf(geometric_anti_product[e12345], 0.5));
        let anti_scalar_product = AntiScalar::from_groups(/* e12345 */ f32::powi(anti_square_root[e12345], 2));
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ (1.0 / anti_scalar_product[e12345]));
        return AntiScalar::from_groups(/* e12345 */ (anti_inverse[e12345] * self[e12345]));
    }
}
impl AntiFixImpl for FlatPoint {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        2        1
    //    simd4        0        2        0
    // Totals...
    // yes simd        0        4        1
    //  no simd        0       10        1
    fn anti_fix_impl(self) -> Self {
        use crate::elements::*;
        let anti_reverse = FlatPoint::from_groups(/* e15, e25, e35, e45 */ (self.group0() * Simd32x4::from(-1.0)));
        let geometric_anti_product = AntiScalar::from_groups(/* e12345 */ (anti_reverse.group0()[3] * self.group0()[3] * -1.0));
        let anti_square_root = AntiScalar::from_groups(/* e12345 */ f32::powf(geometric_anti_product[e12345], 0.5));
        let anti_scalar_product = AntiScalar::from_groups(/* e12345 */ f32::powi(anti_square_root[e12345], 2));
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ (1.0 / anti_scalar_product[e12345]));
        return FlatPoint::from_groups(/* e15, e25, e35, e45 */ (Simd32x4::from(anti_inverse[e12345]) * self.group0()));
    }
}
impl AntiFixImpl for Plane {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        0        1
    //    simd4        0        1        0
    // Totals...
    // yes simd        2        1        1
    //  no simd        2        4        1
    fn anti_fix_impl(self) -> Self {
        use crate::elements::*;
        let geometric_anti_product = AntiScalar::from_groups(/* e12345 */ (f32::powi(self.group0()[0], 2) + f32::powi(self.group0()[1], 2) + f32::powi(self.group0()[2], 2)));
        let anti_square_root = AntiScalar::from_groups(/* e12345 */ f32::powf(geometric_anti_product[e12345], 0.5));
        let anti_scalar_product = AntiScalar::from_groups(/* e12345 */ f32::powi(anti_square_root[e12345], 2));
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ (1.0 / anti_scalar_product[e12345]));
        return Plane::from_groups(/* e4235, e4315, e4125, e3215 */ (Simd32x4::from(anti_inverse[e12345]) * self.group0()));
    }
}
impl AntiFixImpl for RoundPoint {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        3        1
    //    simd4        0        1        0
    // Totals...
    // yes simd        3        4        1
    //  no simd        3        7        1
    fn anti_fix_impl(self) -> Self {
        use crate::elements::*;
        let geometric_anti_product = AntiScalar::from_groups(
            // e12345
            (-f32::powi(self.group0()[0], 2) - f32::powi(self.group0()[1], 2) - f32::powi(self.group0()[2], 2) + 2.0 * (self.group0()[3] * self[e2])),
        );
        let anti_square_root = AntiScalar::from_groups(/* e12345 */ f32::powf(geometric_anti_product[e12345], 0.5));
        let anti_scalar_product = AntiScalar::from_groups(/* e12345 */ f32::powi(anti_square_root[e12345], 2));
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ (1.0 / anti_scalar_product[e12345]));
        return RoundPoint::from_groups(
            // e1, e2, e3, e4
            (Simd32x4::from(anti_inverse[e12345]) * self.group0()),
            // e5
            (anti_inverse[e12345] * self[e2]),
        );
    }
}
impl AntiFixImpl for Scalar {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        2        1
    fn anti_fix_impl(self) -> Self {
        use crate::elements::*;
        let geometric_anti_product = AntiScalar::from_groups(/* e12345 */ (f32::powi(self[scalar], 2) * -1.0));
        let anti_square_root = AntiScalar::from_groups(/* e12345 */ f32::powf(geometric_anti_product[e12345], 0.5));
        let anti_scalar_product = AntiScalar::from_groups(/* e12345 */ f32::powi(anti_square_root[e12345], 2));
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ (1.0 / anti_scalar_product[e12345]));
        return Scalar::from_groups(/* scalar */ (anti_inverse[e12345] * self[scalar]));
    }
}
impl AntiFixImpl for Sphere {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        3        1
    //    simd4        0        1        0
    // Totals...
    // yes simd        3        4        1
    //  no simd        3        7        1
    fn anti_fix_impl(self) -> Self {
        use crate::elements::*;
        let geometric_anti_product = AntiScalar::from_groups(
            // e12345
            (f32::powi(self.group0()[0], 2) + f32::powi(self.group0()[1], 2) + f32::powi(self.group0()[2], 2) - 2.0 * (self.group0()[3] * self[e4315])),
        );
        let anti_square_root = AntiScalar::from_groups(/* e12345 */ f32::powf(geometric_anti_product[e12345], 0.5));
        let anti_scalar_product = AntiScalar::from_groups(/* e12345 */ f32::powi(anti_square_root[e12345], 2));
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ (1.0 / anti_scalar_product[e12345]));
        return Sphere::from_groups(
            // e4235, e4315, e4125, e3215
            (Simd32x4::from(anti_inverse[e12345]) * self.group0()),
            // e1234
            (anti_inverse[e12345] * self[e4315]),
        );
    }
}
