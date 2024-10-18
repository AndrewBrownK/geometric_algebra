// Note on Operative Statistics:
// Operative Statistics are not a precise predictor of performance or performance comparisons.
// This is due to varying hardware capabilities and compiler optimizations.
// As always, where performance is a concern, there is no substitute for
// real measurements on real work-loads on real hardware.
// Disclaimer aside, enjoy the fun information =)
//
// Total Implementations: 24
//
// Yes SIMD:   add/sub     mul     div
//  Minimum:         0       1       1
//   Median:         2       3       1
//  Average:         1       3       1
//  Maximum:         3       8       1
//
//  No SIMD:   add/sub     mul     div
//  Minimum:         0       1       1
//   Median:         2       7       1
//  Average:         1       6       1
//  Maximum:         3      11       1
impl AntiFixImpl for AntiDipoleOnOrigin {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        1
    //    simd4        0        2        0
    // Totals...
    // yes simd        0        3        1
    //  no simd        0        9        1
    fn anti_fix_impl(self) -> Self {
        use crate::elements::*;
        let anti_reverse = AntiDipoleOnOrigin::from_groups(/* e423, e431, e412, e321 */ (self.group0() * Simd32x4::from(-1.0)));
        let geometric_anti_product = AntiScalar::from_groups(/* e12345 */ (anti_reverse.group0()[3] * self.group0()[3]));
        let anti_square_root = AntiScalar::from_groups(/* e12345 */ f32::powf(geometric_anti_product[e12345], 0.5));
        let anti_scalar_product = AntiScalar::from_groups(/* e12345 */ f32::powi(anti_square_root[e12345], 2));
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ (1.0 / anti_scalar_product[e12345]));
        return AntiDipoleOnOrigin::from_groups(/* e423, e431, e412, e321 */ (Simd32x4::from(anti_inverse[e12345]) * self.group0()));
    }
}
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
        return AntiFlatOrigin::from_groups(/* e321 */ (self[e321] * anti_inverse[e12345]));
    }
}
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
        return AntiFlectorOnOrigin::from_groups(/* e321, e1, e2, e3 */ (Simd32x4::from(anti_inverse[e12345]) * self.group0()));
    }
}
impl AntiFixImpl for AntiLineOnOrigin {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        3        1
    //    simd3        0        2        0
    // Totals...
    // yes simd        2        5        1
    //  no simd        2        9        1
    fn anti_fix_impl(self) -> Self {
        use crate::elements::*;
        let anti_reverse = AntiLineOnOrigin::from_groups(/* e23, e31, e12 */ (self.group0() * Simd32x3::from(-1.0)));
        let geometric_anti_product = AntiScalar::from_groups(
            // e12345
            ((anti_reverse.group0()[0] * self.group0()[0]) + (anti_reverse.group0()[1] * self.group0()[1]) + (anti_reverse.group0()[2] * self.group0()[2])),
        );
        let anti_square_root = AntiScalar::from_groups(/* e12345 */ f32::powf(geometric_anti_product[e12345], 0.5));
        let anti_scalar_product = AntiScalar::from_groups(/* e12345 */ f32::powi(anti_square_root[e12345], 2));
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ (1.0 / anti_scalar_product[e12345]));
        return AntiLineOnOrigin::from_groups(/* e23, e31, e12 */ (Simd32x3::from(anti_inverse[e12345]) * self.group0()));
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
        return AntiMotorOnOrigin::from_groups(/* e23, e31, e12, scalar */ (Simd32x4::from(anti_inverse[e12345]) * self.group0()));
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
        return AntiPlaneOnOrigin::from_groups(/* e1, e2, e3 */ (Simd32x3::from(anti_inverse[e12345]) * self.group0()));
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
        return AntiSphereOnOrigin::from_groups(/* e1, e2, e3, e4 */ (Simd32x4::from(anti_inverse[e12345]) * self.group0()));
    }
}
impl AntiFixImpl for DipoleOnOrigin {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        2        1
    //    simd4        0        2        0
    // Totals...
    // yes simd        0        4        1
    //  no simd        0       10        1
    fn anti_fix_impl(self) -> Self {
        use crate::elements::*;
        let anti_reverse = DipoleOnOrigin::from_groups(/* e41, e42, e43, e45 */ (self.group0() * Simd32x4::from(-1.0)));
        let geometric_anti_product = AntiScalar::from_groups(/* e12345 */ (anti_reverse.group0()[3] * self.group0()[3] * -1.0));
        let anti_square_root = AntiScalar::from_groups(/* e12345 */ f32::powf(geometric_anti_product[e12345], 0.5));
        let anti_scalar_product = AntiScalar::from_groups(/* e12345 */ f32::powi(anti_square_root[e12345], 2));
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ (1.0 / anti_scalar_product[e12345]));
        return DipoleOnOrigin::from_groups(/* e41, e42, e43, e45 */ (Simd32x4::from(anti_inverse[e12345]) * self.group0()));
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
        return FlatOrigin::from_groups(/* e45 */ (anti_inverse[e12345] * self[e45]));
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
        return FlectorOnOrigin::from_groups(/* e45, e4235, e4315, e4125 */ (Simd32x4::from(anti_inverse[e12345]) * self.group0()));
    }
}
impl AntiFixImpl for LineOnOrigin {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        3        1
    //    simd3        0        2        0
    // Totals...
    // yes simd        2        5        1
    //  no simd        2        9        1
    fn anti_fix_impl(self) -> Self {
        use crate::elements::*;
        let anti_reverse = LineOnOrigin::from_groups(/* e415, e425, e435 */ (self.group0() * Simd32x3::from(-1.0)));
        let geometric_anti_product = AntiScalar::from_groups(
            // e12345
            (-(anti_reverse.group0()[0] * self.group0()[0]) - (anti_reverse.group0()[1] * self.group0()[1]) - (anti_reverse.group0()[2] * self.group0()[2])),
        );
        let anti_square_root = AntiScalar::from_groups(/* e12345 */ f32::powf(geometric_anti_product[e12345], 0.5));
        let anti_scalar_product = AntiScalar::from_groups(/* e12345 */ f32::powi(anti_square_root[e12345], 2));
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ (1.0 / anti_scalar_product[e12345]));
        return LineOnOrigin::from_groups(/* e415, e425, e435 */ (Simd32x3::from(anti_inverse[e12345]) * self.group0()));
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
        return MotorOnOrigin::from_groups(/* e415, e425, e435, e12345 */ (Simd32x4::from(anti_inverse[e12345]) * self.group0()));
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
        return PlaneOnOrigin::from_groups(/* e4235, e4315, e4125 */ (Simd32x3::from(anti_inverse[e12345]) * self.group0()));
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
        return RoundPointAtOrigin::from_groups(/* e4, e5 */ (Simd32x2::from(anti_inverse[e12345]) * self.group0()));
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
        return SphereAtOrigin::from_groups(/* e3215, e1234 */ (Simd32x2::from(anti_inverse[e12345]) * self.group0()));
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
        return SphereOnOrigin::from_groups(/* e4235, e4315, e4125, e1234 */ (Simd32x4::from(anti_inverse[e12345]) * self.group0()));
    }
}
