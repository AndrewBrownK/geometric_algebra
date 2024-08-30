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
//  Minimum:         0       1       1
//   Median:         0       4       1
//  Average:         0       2       1
//  Maximum:         0       4       1
//
//  No SIMD:   add/sub     mul     div
//  Minimum:         0       1       1
//   Median:         0       4       1
//  Average:         0       2       1
//  Maximum:         0       4       1
impl AntiFixImpl for AntiScalar {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        1
    fn anti_fix_impl(self) -> Self {
        use crate::elements::*;
        let geometric_anti_product = AntiScalar::from_groups(/* e1234 */ f32::powi(self[e1234], 2));
        let anti_square_root = AntiScalar::from_groups(/* e1234 */ f32::powf(geometric_anti_product[e1234], 0.5));
        let anti_scalar_product = AntiScalar::from_groups(/* e1234 */ f32::powi(anti_square_root[e1234], 2));
        let anti_inverse = AntiScalar::from_groups(/* e1234 */ (1.0 / anti_scalar_product[e1234]));
        let geometric_anti_product_2 = AntiScalar::from_groups(/* e1234 */ (self[e1234] * anti_inverse[e1234]));
        return geometric_anti_product_2;
    }
}
impl AntiFixImpl for Origin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        4        1
    fn anti_fix_impl(self) -> Self {
        use crate::elements::*;
        let anti_reverse = Origin::from_groups(/* e4 */ (self[e4] * -1.0));
        let geometric_anti_product = AntiScalar::from_groups(/* e1234 */ (self[e4] * anti_reverse[e4] * -1.0));
        let anti_square_root = AntiScalar::from_groups(/* e1234 */ f32::powf(geometric_anti_product[e1234], 0.5));
        let anti_scalar_product = AntiScalar::from_groups(/* e1234 */ f32::powi(anti_square_root[e1234], 2));
        let anti_inverse = AntiScalar::from_groups(/* e1234 */ (1.0 / anti_scalar_product[e1234]));
        let geometric_anti_product_2 = Origin::from_groups(/* e4 */ (self[e4] * anti_inverse[e1234]));
        return geometric_anti_product_2;
    }
}
