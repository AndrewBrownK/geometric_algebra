// Note on Operative Statistics:
// Operative Statistics are not a precise predictor of performance or performance comparisons.
// This is due to varying hardware capabilities and compiler optimizations.
// As always, where performance is a concern, there is no substitute for
// real measurements on real work-loads on real hardware.
// Disclaimer aside, enjoy the fun information =)
//
// Total Implementations: 9
//
// Yes SIMD:   add/sub     mul     div
//  Minimum:         0       0       1
//   Median:         2       0       1
//  Average:         1       0       1
//  Maximum:         7       1       1
//
//  No SIMD:   add/sub     mul     div
//  Minimum:         0       0       1
//   Median:         2       0       1
//  Average:         1       0       1
//  Maximum:         7       1       1
impl AntiInverse for AntiScalar {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        0        1
    fn anti_inverse(self) -> AntiScalar {
        use crate::elements::*;
        let anti_scalar_product = AntiScalar::from_groups(/* e1234 */ f32::powi(self[e1234], 2));
        return AntiScalar::from_groups(/* e1234 */ (1.0 / anti_scalar_product[e1234]));
    }
}
impl AntiInverse for DualNum {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        0        1
    fn anti_inverse(self) -> AntiScalar {
        use crate::elements::*;
        let anti_scalar_product = AntiScalar::from_groups(/* e1234 */ f32::powi(self.group0()[1], 2));
        return AntiScalar::from_groups(/* e1234 */ (1.0 / anti_scalar_product[e1234]));
    }
}
impl AntiInverse for Flector {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        0        1
    fn anti_inverse(self) -> AntiScalar {
        use crate::elements::*;
        let anti_scalar_product = AntiScalar::from_groups(
            // e1234
            (-f32::powi(self.group0()[3], 2) + f32::powi(self.group1()[0], 2) + f32::powi(self.group1()[1], 2) + f32::powi(self.group1()[2], 2)),
        );
        return AntiScalar::from_groups(/* e1234 */ (1.0 / anti_scalar_product[e1234]));
    }
}
impl AntiInverse for Line {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        0        1
    fn anti_inverse(self) -> AntiScalar {
        use crate::elements::*;
        let anti_scalar_product = AntiScalar::from_groups(/* e1234 */ (-f32::powi(self.group0()[0], 2) - f32::powi(self.group0()[1], 2) - f32::powi(self.group0()[2], 2)));
        return AntiScalar::from_groups(/* e1234 */ (1.0 / anti_scalar_product[e1234]));
    }
}
impl AntiInverse for Motor {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        0        1
    fn anti_inverse(self) -> AntiScalar {
        use crate::elements::*;
        let anti_scalar_product = AntiScalar::from_groups(
            // e1234
            (-f32::powi(self.group0()[0], 2) - f32::powi(self.group0()[1], 2) - f32::powi(self.group0()[2], 2) + f32::powi(self.group0()[3], 2)),
        );
        return AntiScalar::from_groups(/* e1234 */ (1.0 / anti_scalar_product[e1234]));
    }
}
impl AntiInverse for MultiVector {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        7        0        1
    fn anti_inverse(self) -> AntiScalar {
        use crate::elements::*;
        let anti_scalar_product = AntiScalar::from_groups(
            // e1234
            (f32::powi(self.group0()[1], 2) - f32::powi(self.group2()[0], 2) - f32::powi(self.group2()[1], 2) - f32::powi(self.group2()[2], 2) - f32::powi(self.group1()[3], 2)
                + f32::powi(self.group4()[0], 2)
                + f32::powi(self.group4()[1], 2)
                + f32::powi(self.group4()[2], 2)),
        );
        return AntiScalar::from_groups(/* e1234 */ (1.0 / anti_scalar_product[e1234]));
    }
}
impl AntiInverse for Origin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        1
    fn anti_inverse(self) -> AntiScalar {
        use crate::elements::*;
        let anti_scalar_product = AntiScalar::from_groups(/* e1234 */ (f32::powi(self[e4], 2) * -1.0));
        return AntiScalar::from_groups(/* e1234 */ (1.0 / anti_scalar_product[e1234]));
    }
}
impl AntiInverse for Plane {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        0        1
    fn anti_inverse(self) -> AntiScalar {
        use crate::elements::*;
        let anti_scalar_product = AntiScalar::from_groups(/* e1234 */ (f32::powi(self.group0()[0], 2) + f32::powi(self.group0()[1], 2) + f32::powi(self.group0()[2], 2)));
        return AntiScalar::from_groups(/* e1234 */ (1.0 / anti_scalar_product[e1234]));
    }
}
impl AntiInverse for Point {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        1
    fn anti_inverse(self) -> AntiScalar {
        use crate::elements::*;
        let anti_scalar_product = AntiScalar::from_groups(/* e1234 */ (f32::powi(self.group0()[3], 2) * -1.0));
        return AntiScalar::from_groups(/* e1234 */ (1.0 / anti_scalar_product[e1234]));
    }
}
