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
//  Minimum:         0       2       1
//   Median:         0       4       1
//  Average:         0       3       1
//  Maximum:         0       4       1
//
//  No SIMD:   add/sub     mul     div
//  Minimum:         0       2       1
//   Median:         0       4       1
//  Average:         0       3       1
//  Maximum:         0       4       1
impl Fix for Horizon {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        4        1
    fn fix(self) -> Self {
        use crate::elements::*;
        let reverse = Horizon::from_groups(/* e321 */ (self[e321] * -1.0));
        let geometric_product = Scalar::from_groups(/* scalar */ (self[e321] * reverse[e321] * -1.0));
        let square_root = Scalar::from_groups(/* scalar */ f32::powf(geometric_product[scalar], 0.5));
        let scalar_product = Scalar::from_groups(/* scalar */ f32::powi(square_root[scalar], 2));
        let inverse = Scalar::from_groups(/* scalar */ (1.0 / scalar_product[scalar]));
        let geometric_product_2 = Horizon::from_groups(/* e321 */ (self[e321] * inverse[scalar]));
        return geometric_product_2;
    }
}
impl Fix for Scalar {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        2        1
    fn fix(self) -> Self {
        use crate::elements::*;
        let reverse = self;
        let geometric_product = Scalar::from_groups(/* scalar */ (self[scalar] * reverse[scalar]));
        let square_root = Scalar::from_groups(/* scalar */ f32::powf(geometric_product[scalar], 0.5));
        let scalar_product = Scalar::from_groups(/* scalar */ f32::powi(square_root[scalar], 2));
        let inverse = Scalar::from_groups(/* scalar */ (1.0 / scalar_product[scalar]));
        let geometric_product_2 = Scalar::from_groups(/* scalar */ (self[scalar] * inverse[scalar]));
        return geometric_product_2;
    }
}
