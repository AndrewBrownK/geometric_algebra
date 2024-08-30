// Note on Operative Statistics:
// Operative Statistics are not a precise predictor of performance or performance comparisons.
// This is due to varying hardware capabilities and compiler optimizations.
// As always, where performance is a concern, there is no substitute for
// real measurements on real work-loads on real hardware.
// Disclaimer aside, enjoy the fun information =)
//
// Total Implementations: 4
//
// Yes SIMD:   add/sub     mul     div
//  Minimum:         0       1       1
//   Median:         0       3       1
//  Average:         0       2       1
//  Maximum:         0       4       1
//
//  No SIMD:   add/sub     mul     div
//  Minimum:         0       1       1
//   Median:         0       3       1
//  Average:         0       2       1
//  Maximum:         0       4       1
impl AntiFixImpl for AntiFlatOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        3        1
    fn anti_fix_impl(self) -> Self {
        use crate::elements::*;
        let anti_reverse = AntiFlatOrigin::from_groups(/* e321 */ (self[e321] * -1.0));
        let geometric_anti_product = AntiScalar::from_groups(/* e12345 */ (self[e321] * anti_reverse[e321]));
        let anti_square_root = AntiScalar::from_groups(/* e12345 */ f32::powf(geometric_anti_product[e12345], 0.5));
        let anti_scalar_product = AntiScalar::from_groups(/* e12345 */ f32::powi(anti_square_root[e12345], 2));
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ (1.0 / anti_scalar_product[e12345]));
        let geometric_anti_product_2 = AntiFlatOrigin::from_groups(/* e321 */ (self[e321] * anti_inverse[e12345]));
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
        let geometric_anti_product_2 = AntiScalar::from_groups(/* e12345 */ (self[e12345] * anti_inverse[e12345]));
        return geometric_anti_product_2;
    }
}
impl AntiFixImpl for FlatOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        4        1
    fn anti_fix_impl(self) -> Self {
        use crate::elements::*;
        let anti_reverse = FlatOrigin::from_groups(/* e45 */ (self[e45] * -1.0));
        let geometric_anti_product = AntiScalar::from_groups(/* e12345 */ (self[e45] * anti_reverse[e45] * -1.0));
        let anti_square_root = AntiScalar::from_groups(/* e12345 */ f32::powf(geometric_anti_product[e12345], 0.5));
        let anti_scalar_product = AntiScalar::from_groups(/* e12345 */ f32::powi(anti_square_root[e12345], 2));
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ (1.0 / anti_scalar_product[e12345]));
        let geometric_anti_product_2 = FlatOrigin::from_groups(/* e45 */ (self[e45] * anti_inverse[e12345]));
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
        let geometric_anti_product_2 = Scalar::from_groups(/* scalar */ (self[scalar] * anti_inverse[e12345]));
        return geometric_anti_product_2;
    }
}
