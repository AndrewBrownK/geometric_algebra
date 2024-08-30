// Note on Operative Statistics:
// Operative Statistics are not a precise predictor of performance or performance comparisons.
// This is due to varying hardware capabilities and compiler optimizations.
// As always, where performance is a concern, there is no substitute for
// real measurements on real work-loads on real hardware.
// Disclaimer aside, enjoy the fun information =)
//
// Total Implementations: 20
//
// Yes SIMD:   add/sub     mul     div
//  Minimum:         0       1       1
//   Median:         2       3       1
//  Average:         1       3       1
//  Maximum:         3       8       1
//
//  No SIMD:   add/sub     mul     div
//  Minimum:         0       1       1
//   Median:         2       4       1
//  Average:         1       5       1
//  Maximum:         3      11       1
impl AntiFixImpl for AntiFlatOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        3        1
    fn anti_fix_impl(self) -> Self {
        use crate::elements::*;
        let anti_reverse = AntiFlatOrigin::from_groups(/* e321 */ (self[e321] * -1.0));
        let geometric_anti_product = AntiScalar::from_groups(/* e12345 */ (anti_reverse[e321] * self[e321]));
        let anti_square_root = AntiScalar::from_groups(/* e12345 */ f32::powf(geometric_anti_product[e12345], 0.5));
        let anti_scalar_product = AntiScalar::from_groups(/* e12345 */ f32::powi(anti_square_root[e12345], 2));
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ (1.0 / anti_scalar_product[e12345]));
        let geometric_anti_product_2 = AntiFlatOrigin::from_groups(/* e321 */ (self[e321] * anti_inverse[e12345]));
        return geometric_anti_product_2;
    }
}
impl AntiFixImpl for AntiFlectorOnOrigin {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        5        1
    //    simd4        0        1        0
    // Totals...
    // yes simd        3        6        1
    //  no simd        3        9        1
    fn anti_fix_impl(self) -> Self {
        use crate::elements::*;
        let anti_reverse = AntiFlectorOnOrigin::from_groups(
            // e321, e1, e2, e3
            Simd32x4::from([(self.group0()[0] * -1.0), self.group0()[1], self.group0()[2], self.group0()[3]]),
        );
        let geometric_anti_product = AntiScalar::from_groups(
            // e12345
            ((anti_reverse.group0()[0] * self.group0()[0])
                - (anti_reverse.group0()[1] * self.group0()[1])
                - (anti_reverse.group0()[2] * self.group0()[2])
                - (anti_reverse.group0()[3] * self.group0()[3])),
        );
        let anti_square_root = AntiScalar::from_groups(/* e12345 */ f32::powf(geometric_anti_product[e12345], 0.5));
        let anti_scalar_product = AntiScalar::from_groups(/* e12345 */ f32::powi(anti_square_root[e12345], 2));
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ (1.0 / anti_scalar_product[e12345]));
        let geometric_anti_product_2 = AntiFlectorOnOrigin::from_groups(/* e321, e1, e2, e3 */ (Simd32x4::from(anti_inverse[e12345]) * self.group0()));
        return geometric_anti_product_2;
    }
}
impl AntiFixImpl for AntiMotorOnOrigin {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        7        1
    //    simd4        0        1        0
    // Totals...
    // yes simd        3        8        1
    //  no simd        3       11        1
    fn anti_fix_impl(self) -> Self {
        use crate::elements::*;
        let anti_reverse = AntiMotorOnOrigin::from_groups(
            // e23, e31, e12, scalar
            Simd32x4::from([(self.group0()[0] * -1.0), (self.group0()[1] * -1.0), (self.group0()[2] * -1.0), self.group0()[3]]),
        );
        let geometric_anti_product = AntiScalar::from_groups(
            // e12345
            ((anti_reverse.group0()[0] * self.group0()[0]) + (anti_reverse.group0()[1] * self.group0()[1]) + (anti_reverse.group0()[2] * self.group0()[2])
                - (anti_reverse.group0()[3] * self.group0()[3])),
        );
        let anti_square_root = AntiScalar::from_groups(/* e12345 */ f32::powf(geometric_anti_product[e12345], 0.5));
        let anti_scalar_product = AntiScalar::from_groups(/* e12345 */ f32::powi(anti_square_root[e12345], 2));
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ (1.0 / anti_scalar_product[e12345]));
        let geometric_anti_product_2 = AntiMotorOnOrigin::from_groups(/* e23, e31, e12, scalar */ (Simd32x4::from(anti_inverse[e12345]) * self.group0()));
        return geometric_anti_product_2;
    }
}
impl AntiFixImpl for AntiMysteryQuadNum {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        3        1
    //    simd2        0        1        0
    // Totals...
    // yes simd        1        4        1
    //  no simd        1        5        1
    fn anti_fix_impl(self) -> Self {
        use crate::elements::*;
        let anti_reverse = AntiMysteryQuadNum::from_groups(/* e45, scalar */ Simd32x2::from([(self.group0()[0] * -1.0), self.group0()[1]]));
        let geometric_anti_product = AntiScalar::from_groups(/* e12345 */ (-(anti_reverse.group0()[0] * self.group0()[0]) - (anti_reverse.group0()[1] * self.group0()[1])));
        let anti_square_root = AntiScalar::from_groups(/* e12345 */ f32::powf(geometric_anti_product[e12345], 0.5));
        let anti_scalar_product = AntiScalar::from_groups(/* e12345 */ f32::powi(anti_square_root[e12345], 2));
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ (1.0 / anti_scalar_product[e12345]));
        let geometric_anti_product_2 = AntiMysteryQuadNum::from_groups(/* e45, scalar */ (Simd32x2::from(anti_inverse[e12345]) * self.group0()));
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
impl AntiFixImpl for AntiPlaneOnOrigin {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        0        1
    //    simd3        0        1        0
    // Totals...
    // yes simd        2        1        1
    //  no simd        2        3        1
    fn anti_fix_impl(self) -> Self {
        use crate::elements::*;
        let geometric_anti_product = AntiScalar::from_groups(/* e12345 */ (-f32::powi(self.group0()[0], 2) - f32::powi(self.group0()[1], 2) - f32::powi(self.group0()[2], 2)));
        let anti_square_root = AntiScalar::from_groups(/* e12345 */ f32::powf(geometric_anti_product[e12345], 0.5));
        let anti_scalar_product = AntiScalar::from_groups(/* e12345 */ f32::powi(anti_square_root[e12345], 2));
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ (1.0 / anti_scalar_product[e12345]));
        let geometric_anti_product_2 = AntiPlaneOnOrigin::from_groups(/* e1, e2, e3 */ (Simd32x3::from(anti_inverse[e12345]) * self.group0()));
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
impl AntiFixImpl for AntiSphereOnOrigin {
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
        let geometric_anti_product_2 = AntiSphereOnOrigin::from_groups(/* e1, e2, e3, e4 */ (Simd32x4::from(anti_inverse[e12345]) * self.group0()));
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
        let geometric_anti_product = AntiScalar::from_groups(/* e12345 */ (anti_reverse[e45] * self[e45] * -1.0));
        let anti_square_root = AntiScalar::from_groups(/* e12345 */ f32::powf(geometric_anti_product[e12345], 0.5));
        let anti_scalar_product = AntiScalar::from_groups(/* e12345 */ f32::powi(anti_square_root[e12345], 2));
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ (1.0 / anti_scalar_product[e12345]));
        let geometric_anti_product_2 = FlatOrigin::from_groups(/* e45 */ (anti_inverse[e12345] * self[e45]));
        return geometric_anti_product_2;
    }
}
impl AntiFixImpl for FlectorOnOrigin {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        5        1
    //    simd4        0        1        0
    // Totals...
    // yes simd        3        6        1
    //  no simd        3        9        1
    fn anti_fix_impl(self) -> Self {
        use crate::elements::*;
        let anti_reverse = FlectorOnOrigin::from_groups(
            // e45, e4235, e4315, e4125
            Simd32x4::from([(self.group0()[0] * -1.0), self.group0()[1], self.group0()[2], self.group0()[3]]),
        );
        let geometric_anti_product = AntiScalar::from_groups(
            // e12345
            (-(anti_reverse.group0()[0] * self.group0()[0])
                + (anti_reverse.group0()[1] * self.group0()[1])
                + (anti_reverse.group0()[2] * self.group0()[2])
                + (anti_reverse.group0()[3] * self.group0()[3])),
        );
        let anti_square_root = AntiScalar::from_groups(/* e12345 */ f32::powf(geometric_anti_product[e12345], 0.5));
        let anti_scalar_product = AntiScalar::from_groups(/* e12345 */ f32::powi(anti_square_root[e12345], 2));
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ (1.0 / anti_scalar_product[e12345]));
        let geometric_anti_product_2 = FlectorOnOrigin::from_groups(/* e45, e4235, e4315, e4125 */ (Simd32x4::from(anti_inverse[e12345]) * self.group0()));
        return geometric_anti_product_2;
    }
}
impl AntiFixImpl for MotorOnOrigin {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        7        1
    //    simd4        0        1        0
    // Totals...
    // yes simd        3        8        1
    //  no simd        3       11        1
    fn anti_fix_impl(self) -> Self {
        use crate::elements::*;
        let anti_reverse = MotorOnOrigin::from_groups(
            // e415, e425, e435, e12345
            Simd32x4::from([(self.group0()[0] * -1.0), (self.group0()[1] * -1.0), (self.group0()[2] * -1.0), self.group0()[3]]),
        );
        let geometric_anti_product = AntiScalar::from_groups(
            // e12345
            (-(anti_reverse.group0()[0] * self.group0()[0]) - (anti_reverse.group0()[1] * self.group0()[1]) - (anti_reverse.group0()[2] * self.group0()[2])
                + (anti_reverse.group0()[3] * self.group0()[3])),
        );
        let anti_square_root = AntiScalar::from_groups(/* e12345 */ f32::powf(geometric_anti_product[e12345], 0.5));
        let anti_scalar_product = AntiScalar::from_groups(/* e12345 */ f32::powi(anti_square_root[e12345], 2));
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ (1.0 / anti_scalar_product[e12345]));
        let geometric_anti_product_2 = MotorOnOrigin::from_groups(/* e415, e425, e435, e12345 */ (Simd32x4::from(anti_inverse[e12345]) * self.group0()));
        return geometric_anti_product_2;
    }
}
impl AntiFixImpl for MysteryQuadNum {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        3        1
    //    simd2        0        1        0
    // Totals...
    // yes simd        1        4        1
    //  no simd        1        5        1
    fn anti_fix_impl(self) -> Self {
        use crate::elements::*;
        let anti_reverse = MysteryQuadNum::from_groups(/* e321, e12345 */ Simd32x2::from([(self.group0()[0] * -1.0), self.group0()[1]]));
        let geometric_anti_product = AntiScalar::from_groups(/* e12345 */ ((anti_reverse.group0()[0] * self.group0()[0]) + (anti_reverse.group0()[1] * self.group0()[1])));
        let anti_square_root = AntiScalar::from_groups(/* e12345 */ f32::powf(geometric_anti_product[e12345], 0.5));
        let anti_scalar_product = AntiScalar::from_groups(/* e12345 */ f32::powi(anti_square_root[e12345], 2));
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ (1.0 / anti_scalar_product[e12345]));
        let geometric_anti_product_2 = MysteryQuadNum::from_groups(/* e321, e12345 */ (Simd32x2::from(anti_inverse[e12345]) * self.group0()));
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
impl AntiFixImpl for PlaneOnOrigin {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        0        1
    //    simd3        0        1        0
    // Totals...
    // yes simd        2        1        1
    //  no simd        2        3        1
    fn anti_fix_impl(self) -> Self {
        use crate::elements::*;
        let geometric_anti_product = AntiScalar::from_groups(/* e12345 */ (f32::powi(self.group0()[0], 2) + f32::powi(self.group0()[1], 2) + f32::powi(self.group0()[2], 2)));
        let anti_square_root = AntiScalar::from_groups(/* e12345 */ f32::powf(geometric_anti_product[e12345], 0.5));
        let anti_scalar_product = AntiScalar::from_groups(/* e12345 */ f32::powi(anti_square_root[e12345], 2));
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ (1.0 / anti_scalar_product[e12345]));
        let geometric_anti_product_2 = PlaneOnOrigin::from_groups(/* e4235, e4315, e4125 */ (Simd32x3::from(anti_inverse[e12345]) * self.group0()));
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
            (-f32::powi(self.group0()[0], 2) - f32::powi(self.group0()[1], 2) - f32::powi(self.group0()[2], 2) + *2.0(self.group0()[3] * self[e2])),
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
impl AntiFixImpl for RoundPointAtOrigin {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        2        1
    //    simd2        0        1        0
    // Totals...
    // yes simd        0        3        1
    //  no simd        0        4        1
    fn anti_fix_impl(self) -> Self {
        use crate::elements::*;
        let geometric_anti_product = AntiScalar::from_groups(/* e12345 */ (self.group0()[0] * self.group0()[1] * 2.0));
        let anti_square_root = AntiScalar::from_groups(/* e12345 */ f32::powf(geometric_anti_product[e12345], 0.5));
        let anti_scalar_product = AntiScalar::from_groups(/* e12345 */ f32::powi(anti_square_root[e12345], 2));
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ (1.0 / anti_scalar_product[e12345]));
        let geometric_anti_product_2 = RoundPointAtOrigin::from_groups(/* e4, e5 */ (Simd32x2::from(anti_inverse[e12345]) * self.group0()));
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
impl AntiFixImpl for SphereAtOrigin {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        2        1
    //    simd2        0        1        0
    // Totals...
    // yes simd        0        3        1
    //  no simd        0        4        1
    fn anti_fix_impl(self) -> Self {
        use crate::elements::*;
        let geometric_anti_product = AntiScalar::from_groups(/* e12345 */ (self.group0()[0] * self.group0()[1] * -2.0));
        let anti_square_root = AntiScalar::from_groups(/* e12345 */ f32::powf(geometric_anti_product[e12345], 0.5));
        let anti_scalar_product = AntiScalar::from_groups(/* e12345 */ f32::powi(anti_square_root[e12345], 2));
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ (1.0 / anti_scalar_product[e12345]));
        let geometric_anti_product_2 = SphereAtOrigin::from_groups(/* e3215, e1234 */ (Simd32x2::from(anti_inverse[e12345]) * self.group0()));
        return geometric_anti_product_2;
    }
}
impl AntiFixImpl for SphereOnOrigin {
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
        let geometric_anti_product_2 = SphereOnOrigin::from_groups(/* e4235, e4315, e4125, e1234 */ (Simd32x4::from(anti_inverse[e12345]) * self.group0()));
        return geometric_anti_product_2;
    }
}
