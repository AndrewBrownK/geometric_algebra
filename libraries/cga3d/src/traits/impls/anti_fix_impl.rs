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
//   Median:         2       4       1
//  Average:         1       2       1
//  Maximum:         3       4       1
//
//  No SIMD:   add/sub     mul     div
//  Minimum:         0       1       1
//   Median:         2       5       1
//  Average:         1       4       1
//  Maximum:         3       7       1
impl AntiFixImpl for AntiDualNum321 {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        3        1
    //    simd2        0        1        0
    // Totals...
    // yes simd        1        4        1
    //  no simd        1        5        1
    fn anti_fix_impl(self) -> Self {
        use crate::elements::*;
        let anti_reverse = AntiDualNum321::from_groups(/* e45, scalar */ Simd32x2::from([(self.group0()[0] * -1.0), self.group0()[1]]));
        let geometric_anti_product = AntiScalar::from_groups(/* e12345 */ (-(anti_reverse.group0()[0] * self.group0()[0]) - (anti_reverse.group0()[1] * self.group0()[1])));
        let anti_square_root = AntiScalar::from_groups(/* e12345 */ f32::powf(geometric_anti_product[e12345], 0.5));
        let anti_scalar_product = AntiScalar::from_groups(/* e12345 */ f32::powi(anti_square_root[e12345], 2));
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ (1.0 / anti_scalar_product[e12345]));
        let geometric_anti_product_2 = AntiDualNum321::from_groups(/* e45, scalar */ (Simd32x2::from(anti_inverse[e12345]) * self.group0()));
        return geometric_anti_product_2;
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
        let geometric_anti_product_2 = AntiPlane::from_groups(/* e1, e2, e3, e5 */ (Simd32x4::from(anti_inverse[e12345]) * self.group0()));
        return geometric_anti_product_2;
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
        let geometric_anti_product_2 = AntiScalar::from_groups(/* e12345 */ (anti_inverse[e12345] * self[e12345]));
        return geometric_anti_product_2;
    }
}
impl AntiFixImpl for DualNum321 {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        3        1
    //    simd2        0        1        0
    // Totals...
    // yes simd        1        4        1
    //  no simd        1        5        1
    fn anti_fix_impl(self) -> Self {
        use crate::elements::*;
        let anti_reverse = DualNum321::from_groups(/* e321, e12345 */ Simd32x2::from([(self.group0()[0] * -1.0), self.group0()[1]]));
        let geometric_anti_product = AntiScalar::from_groups(/* e12345 */ ((anti_reverse.group0()[0] * self.group0()[0]) + (anti_reverse.group0()[1] * self.group0()[1])));
        let anti_square_root = AntiScalar::from_groups(/* e12345 */ f32::powf(geometric_anti_product[e12345], 0.5));
        let anti_scalar_product = AntiScalar::from_groups(/* e12345 */ f32::powi(anti_square_root[e12345], 2));
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ (1.0 / anti_scalar_product[e12345]));
        let geometric_anti_product_2 = DualNum321::from_groups(/* e321, e12345 */ (Simd32x2::from(anti_inverse[e12345]) * self.group0()));
        return geometric_anti_product_2;
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
        let geometric_anti_product_2 = Plane::from_groups(/* e4235, e4315, e4125, e3215 */ (Simd32x4::from(anti_inverse[e12345]) * self.group0()));
        return geometric_anti_product_2;
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
        let geometric_anti_product_2 = RoundPoint::from_groups(
            // e1, e2, e3, e4
            (Simd32x4::from(anti_inverse[e12345]) * self.group0()),
            // e5
            (anti_inverse[e12345] * self[e2]),
        );
        return geometric_anti_product_2;
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
        let geometric_anti_product_2 = Scalar::from_groups(/* scalar */ (anti_inverse[e12345] * self[scalar]));
        return geometric_anti_product_2;
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
        let geometric_anti_product_2 = Sphere::from_groups(
            // e4235, e4315, e4125, e3215
            (Simd32x4::from(anti_inverse[e12345]) * self.group0()),
            // e1234
            (anti_inverse[e12345] * self[e4315]),
        );
        return geometric_anti_product_2;
    }
}
