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
impl Inverse for DualNum {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        0        1
    fn inverse(self) -> Scalar {
        use crate::elements::*;
        let scalar_product = Scalar::from_groups(/* scalar */ f32::powi(self.group0()[0], 2));
        return Scalar::from_groups(/* scalar */ (1.0 / scalar_product[scalar]));
    }
}
impl Inverse for Flector {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        0        1
    fn inverse(self) -> Scalar {
        use crate::elements::*;
        let scalar_product = Scalar::from_groups(
            // scalar
            f32::powi(self.group0()[0], 2) + f32::powi(self.group0()[1], 2) + f32::powi(self.group0()[2], 2) - f32::powi(self.group1()[3], 2),
        );
        return Scalar::from_groups(/* scalar */ (1.0 / scalar_product[scalar]));
    }
}
impl Inverse for Horizon {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        1
    fn inverse(self) -> Scalar {
        use crate::elements::*;
        let scalar_product = Scalar::from_groups(/* scalar */ f32::powi(self[e321], 2) * -1.0);
        return Scalar::from_groups(/* scalar */ (1.0 / scalar_product[scalar]));
    }
}
impl Inverse for Line {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        0        1
    fn inverse(self) -> Scalar {
        use crate::elements::*;
        let scalar_product = Scalar::from_groups(/* scalar */ -f32::powi(self.group1()[0], 2) - f32::powi(self.group1()[1], 2) - f32::powi(self.group1()[2], 2));
        return Scalar::from_groups(/* scalar */ (1.0 / scalar_product[scalar]));
    }
}
impl Inverse for Motor {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        0        1
    fn inverse(self) -> Scalar {
        use crate::elements::*;
        let scalar_product = Scalar::from_groups(
            // scalar
            -f32::powi(self.group1()[0], 2) - f32::powi(self.group1()[1], 2) - f32::powi(self.group1()[2], 2) + f32::powi(self.group1()[3], 2),
        );
        return Scalar::from_groups(/* scalar */ (1.0 / scalar_product[scalar]));
    }
}
impl Inverse for MultiVector {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        7        0        1
    fn inverse(self) -> Scalar {
        use crate::elements::*;
        let scalar_product = Scalar::from_groups(
            // scalar
            f32::powi(self.group0()[0], 2) - f32::powi(self.group3()[0], 2) - f32::powi(self.group3()[1], 2) - f32::powi(self.group3()[2], 2)
                + f32::powi(self.group1()[0], 2)
                + f32::powi(self.group1()[1], 2)
                + f32::powi(self.group1()[2], 2)
                - f32::powi(self.group4()[3], 2),
        );
        return Scalar::from_groups(/* scalar */ (1.0 / scalar_product[scalar]));
    }
}
impl Inverse for Plane {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        1
    fn inverse(self) -> Scalar {
        use crate::elements::*;
        let scalar_product = Scalar::from_groups(/* scalar */ f32::powi(self.group0()[3], 2) * -1.0);
        return Scalar::from_groups(/* scalar */ (1.0 / scalar_product[scalar]));
    }
}
impl Inverse for Point {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        0        1
    fn inverse(self) -> Scalar {
        use crate::elements::*;
        let scalar_product = Scalar::from_groups(/* scalar */ f32::powi(self.group0()[0], 2) + f32::powi(self.group0()[1], 2) + f32::powi(self.group0()[2], 2));
        return Scalar::from_groups(/* scalar */ (1.0 / scalar_product[scalar]));
    }
}
impl Inverse for Scalar {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        0        1
    fn inverse(self) -> Scalar {
        use crate::elements::*;
        let scalar_product = Scalar::from_groups(/* scalar */ f32::powi(self[scalar], 2));
        return Scalar::from_groups(/* scalar */ (1.0 / scalar_product[scalar]));
    }
}
