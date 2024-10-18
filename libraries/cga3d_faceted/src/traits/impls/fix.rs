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
impl Fix for AntiDipoleOnOrigin {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        2        1
    //    simd4        0        2        0
    // Totals...
    // yes simd        0        4        1
    //  no simd        0       10        1
    fn fix(self) -> Self {
        use crate::elements::*;
        let reverse = AntiDipoleOnOrigin::from_groups(/* e423, e431, e412, e321 */ (self.group0() * Simd32x4::from(-1.0)));
        let geometric_product = Scalar::from_groups(/* scalar */ (reverse.group0()[3] * self.group0()[3] * -1.0));
        let square_root = Scalar::from_groups(/* scalar */ f32::powf(geometric_product[scalar], 0.5));
        let scalar_product = Scalar::from_groups(/* scalar */ f32::powi(square_root[scalar], 2));
        let inverse = Scalar::from_groups(/* scalar */ (1.0 / scalar_product[scalar]));
        return AntiDipoleOnOrigin::from_groups(/* e423, e431, e412, e321 */ (Simd32x4::from(inverse[scalar]) * self.group0()));
    }
}
impl Fix for AntiFlatOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        4        1
    fn fix(self) -> Self {
        use crate::elements::*;
        let reverse = AntiFlatOrigin::from_groups(/* e321 */ (self[e321] * -1.0));
        let geometric_product = Scalar::from_groups(/* scalar */ (reverse[e321] * self[e321] * -1.0));
        let square_root = Scalar::from_groups(/* scalar */ f32::powf(geometric_product[scalar], 0.5));
        let scalar_product = Scalar::from_groups(/* scalar */ f32::powi(square_root[scalar], 2));
        let inverse = Scalar::from_groups(/* scalar */ (1.0 / scalar_product[scalar]));
        return AntiFlatOrigin::from_groups(/* e321 */ (self[e321] * inverse[scalar]));
    }
}
impl Fix for AntiFlatPoint {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        2        1
    //    simd4        0        2        0
    // Totals...
    // yes simd        0        4        1
    //  no simd        0       10        1
    fn fix(self) -> Self {
        use crate::elements::*;
        let reverse = AntiFlatPoint::from_groups(/* e235, e315, e125, e321 */ (self.group0() * Simd32x4::from(-1.0)));
        let geometric_product = Scalar::from_groups(/* scalar */ (reverse.group0()[3] * self.group0()[3] * -1.0));
        let square_root = Scalar::from_groups(/* scalar */ f32::powf(geometric_product[scalar], 0.5));
        let scalar_product = Scalar::from_groups(/* scalar */ f32::powi(square_root[scalar], 2));
        let inverse = Scalar::from_groups(/* scalar */ (1.0 / scalar_product[scalar]));
        return AntiFlatPoint::from_groups(/* e235, e315, e125, e321 */ (Simd32x4::from(inverse[scalar]) * self.group0()));
    }
}
impl Fix for AntiFlectorOnOrigin {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        5        1
    //    simd4        0        1        0
    // Totals...
    // yes simd        3        6        1
    //  no simd        3        9        1
    fn fix(self) -> Self {
        use crate::elements::*;
        let reverse = AntiFlectorOnOrigin::from_groups(
            // e321, e1, e2, e3
            Simd32x4::from([(self.group0()[0] * -1.0), self.group0()[1], self.group0()[2], self.group0()[3]]),
        );
        let geometric_product = Scalar::from_groups(
            // scalar
            (-(reverse.group0()[0] * self.group0()[0])
                + (reverse.group0()[1] * self.group0()[1])
                + (reverse.group0()[2] * self.group0()[2])
                + (reverse.group0()[3] * self.group0()[3])),
        );
        let square_root = Scalar::from_groups(/* scalar */ f32::powf(geometric_product[scalar], 0.5));
        let scalar_product = Scalar::from_groups(/* scalar */ f32::powi(square_root[scalar], 2));
        let inverse = Scalar::from_groups(/* scalar */ (1.0 / scalar_product[scalar]));
        return AntiFlectorOnOrigin::from_groups(/* e321, e1, e2, e3 */ (Simd32x4::from(inverse[scalar]) * self.group0()));
    }
}
impl Fix for AntiLineOnOrigin {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        3        1
    //    simd3        0        2        0
    // Totals...
    // yes simd        2        5        1
    //  no simd        2        9        1
    fn fix(self) -> Self {
        use crate::elements::*;
        let reverse = AntiLineOnOrigin::from_groups(/* e23, e31, e12 */ (self.group0() * Simd32x3::from(-1.0)));
        let geometric_product = Scalar::from_groups(
            // scalar
            (-(reverse.group0()[0] * self.group0()[0]) - (reverse.group0()[1] * self.group0()[1]) - (reverse.group0()[2] * self.group0()[2])),
        );
        let square_root = Scalar::from_groups(/* scalar */ f32::powf(geometric_product[scalar], 0.5));
        let scalar_product = Scalar::from_groups(/* scalar */ f32::powi(square_root[scalar], 2));
        let inverse = Scalar::from_groups(/* scalar */ (1.0 / scalar_product[scalar]));
        return AntiLineOnOrigin::from_groups(/* e23, e31, e12 */ (Simd32x3::from(inverse[scalar]) * self.group0()));
    }
}
impl Fix for AntiMotorOnOrigin {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        7        1
    //    simd4        0        1        0
    // Totals...
    // yes simd        3        8        1
    //  no simd        3       11        1
    fn fix(self) -> Self {
        use crate::elements::*;
        let reverse = AntiMotorOnOrigin::from_groups(
            // e23, e31, e12, scalar
            Simd32x4::from([(self.group0()[0] * -1.0), (self.group0()[1] * -1.0), (self.group0()[2] * -1.0), self.group0()[3]]),
        );
        let geometric_product = Scalar::from_groups(
            // scalar
            (-(reverse.group0()[0] * self.group0()[0]) - (reverse.group0()[1] * self.group0()[1]) - (reverse.group0()[2] * self.group0()[2])
                + (reverse.group0()[3] * self.group0()[3])),
        );
        let square_root = Scalar::from_groups(/* scalar */ f32::powf(geometric_product[scalar], 0.5));
        let scalar_product = Scalar::from_groups(/* scalar */ f32::powi(square_root[scalar], 2));
        let inverse = Scalar::from_groups(/* scalar */ (1.0 / scalar_product[scalar]));
        return AntiMotorOnOrigin::from_groups(/* e23, e31, e12, scalar */ (Simd32x4::from(inverse[scalar]) * self.group0()));
    }
}
impl Fix for AntiPlane {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        0        1
    //    simd4        0        1        0
    // Totals...
    // yes simd        2        1        1
    //  no simd        2        4        1
    fn fix(self) -> Self {
        use crate::elements::*;
        let geometric_product = Scalar::from_groups(/* scalar */ (f32::powi(self.group0()[0], 2) + f32::powi(self.group0()[1], 2) + f32::powi(self.group0()[2], 2)));
        let square_root = Scalar::from_groups(/* scalar */ f32::powf(geometric_product[scalar], 0.5));
        let scalar_product = Scalar::from_groups(/* scalar */ f32::powi(square_root[scalar], 2));
        let inverse = Scalar::from_groups(/* scalar */ (1.0 / scalar_product[scalar]));
        return AntiPlane::from_groups(/* e1, e2, e3, e5 */ (Simd32x4::from(inverse[scalar]) * self.group0()));
    }
}
impl Fix for AntiPlaneOnOrigin {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        0        1
    //    simd3        0        1        0
    // Totals...
    // yes simd        2        1        1
    //  no simd        2        3        1
    fn fix(self) -> Self {
        use crate::elements::*;
        let geometric_product = Scalar::from_groups(/* scalar */ (f32::powi(self.group0()[0], 2) + f32::powi(self.group0()[1], 2) + f32::powi(self.group0()[2], 2)));
        let square_root = Scalar::from_groups(/* scalar */ f32::powf(geometric_product[scalar], 0.5));
        let scalar_product = Scalar::from_groups(/* scalar */ f32::powi(square_root[scalar], 2));
        let inverse = Scalar::from_groups(/* scalar */ (1.0 / scalar_product[scalar]));
        return AntiPlaneOnOrigin::from_groups(/* e1, e2, e3 */ (Simd32x3::from(inverse[scalar]) * self.group0()));
    }
}
impl Fix for AntiScalar {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        2        1
    fn fix(self) -> Self {
        use crate::elements::*;
        let geometric_product = Scalar::from_groups(/* scalar */ (f32::powi(self[e12345], 2) * -1.0));
        let square_root = Scalar::from_groups(/* scalar */ f32::powf(geometric_product[scalar], 0.5));
        let scalar_product = Scalar::from_groups(/* scalar */ f32::powi(square_root[scalar], 2));
        let inverse = Scalar::from_groups(/* scalar */ (1.0 / scalar_product[scalar]));
        return AntiScalar::from_groups(/* e12345 */ (self[e12345] * inverse[scalar]));
    }
}
impl Fix for AntiSphereOnOrigin {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        0        1
    //    simd4        0        1        0
    // Totals...
    // yes simd        2        1        1
    //  no simd        2        4        1
    fn fix(self) -> Self {
        use crate::elements::*;
        let geometric_product = Scalar::from_groups(/* scalar */ (f32::powi(self.group0()[0], 2) + f32::powi(self.group0()[1], 2) + f32::powi(self.group0()[2], 2)));
        let square_root = Scalar::from_groups(/* scalar */ f32::powf(geometric_product[scalar], 0.5));
        let scalar_product = Scalar::from_groups(/* scalar */ f32::powi(square_root[scalar], 2));
        let inverse = Scalar::from_groups(/* scalar */ (1.0 / scalar_product[scalar]));
        return AntiSphereOnOrigin::from_groups(/* e1, e2, e3, e4 */ (Simd32x4::from(inverse[scalar]) * self.group0()));
    }
}
impl Fix for DipoleOnOrigin {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        1
    //    simd4        0        2        0
    // Totals...
    // yes simd        0        3        1
    //  no simd        0        9        1
    fn fix(self) -> Self {
        use crate::elements::*;
        let reverse = DipoleOnOrigin::from_groups(/* e41, e42, e43, e45 */ (self.group0() * Simd32x4::from(-1.0)));
        let geometric_product = Scalar::from_groups(/* scalar */ (reverse.group0()[3] * self.group0()[3]));
        let square_root = Scalar::from_groups(/* scalar */ f32::powf(geometric_product[scalar], 0.5));
        let scalar_product = Scalar::from_groups(/* scalar */ f32::powi(square_root[scalar], 2));
        let inverse = Scalar::from_groups(/* scalar */ (1.0 / scalar_product[scalar]));
        return DipoleOnOrigin::from_groups(/* e41, e42, e43, e45 */ (Simd32x4::from(inverse[scalar]) * self.group0()));
    }
}
impl Fix for FlatOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        3        1
    fn fix(self) -> Self {
        use crate::elements::*;
        let reverse = FlatOrigin::from_groups(/* e45 */ (self[e45] * -1.0));
        let geometric_product = Scalar::from_groups(/* scalar */ (reverse[e45] * self[e45]));
        let square_root = Scalar::from_groups(/* scalar */ f32::powf(geometric_product[scalar], 0.5));
        let scalar_product = Scalar::from_groups(/* scalar */ f32::powi(square_root[scalar], 2));
        let inverse = Scalar::from_groups(/* scalar */ (1.0 / scalar_product[scalar]));
        return FlatOrigin::from_groups(/* e45 */ (self[e45] * inverse[scalar]));
    }
}
impl Fix for FlatPoint {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        1
    //    simd4        0        2        0
    // Totals...
    // yes simd        0        3        1
    //  no simd        0        9        1
    fn fix(self) -> Self {
        use crate::elements::*;
        let reverse = FlatPoint::from_groups(/* e15, e25, e35, e45 */ (self.group0() * Simd32x4::from(-1.0)));
        let geometric_product = Scalar::from_groups(/* scalar */ (reverse.group0()[3] * self.group0()[3]));
        let square_root = Scalar::from_groups(/* scalar */ f32::powf(geometric_product[scalar], 0.5));
        let scalar_product = Scalar::from_groups(/* scalar */ f32::powi(square_root[scalar], 2));
        let inverse = Scalar::from_groups(/* scalar */ (1.0 / scalar_product[scalar]));
        return FlatPoint::from_groups(/* e15, e25, e35, e45 */ (Simd32x4::from(inverse[scalar]) * self.group0()));
    }
}
impl Fix for FlectorOnOrigin {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        5        1
    //    simd4        0        1        0
    // Totals...
    // yes simd        3        6        1
    //  no simd        3        9        1
    fn fix(self) -> Self {
        use crate::elements::*;
        let reverse = FlectorOnOrigin::from_groups(
            // e45, e4235, e4315, e4125
            Simd32x4::from([(self.group0()[0] * -1.0), self.group0()[1], self.group0()[2], self.group0()[3]]),
        );
        let geometric_product = Scalar::from_groups(
            // scalar
            ((reverse.group0()[0] * self.group0()[0])
                - (reverse.group0()[1] * self.group0()[1])
                - (reverse.group0()[2] * self.group0()[2])
                - (reverse.group0()[3] * self.group0()[3])),
        );
        let square_root = Scalar::from_groups(/* scalar */ f32::powf(geometric_product[scalar], 0.5));
        let scalar_product = Scalar::from_groups(/* scalar */ f32::powi(square_root[scalar], 2));
        let inverse = Scalar::from_groups(/* scalar */ (1.0 / scalar_product[scalar]));
        return FlectorOnOrigin::from_groups(/* e45, e4235, e4315, e4125 */ (Simd32x4::from(inverse[scalar]) * self.group0()));
    }
}
impl Fix for LineOnOrigin {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        3        1
    //    simd3        0        2        0
    // Totals...
    // yes simd        2        5        1
    //  no simd        2        9        1
    fn fix(self) -> Self {
        use crate::elements::*;
        let reverse = LineOnOrigin::from_groups(/* e415, e425, e435 */ (self.group0() * Simd32x3::from(-1.0)));
        let geometric_product = Scalar::from_groups(
            // scalar
            ((reverse.group0()[0] * self.group0()[0]) + (reverse.group0()[1] * self.group0()[1]) + (reverse.group0()[2] * self.group0()[2])),
        );
        let square_root = Scalar::from_groups(/* scalar */ f32::powf(geometric_product[scalar], 0.5));
        let scalar_product = Scalar::from_groups(/* scalar */ f32::powi(square_root[scalar], 2));
        let inverse = Scalar::from_groups(/* scalar */ (1.0 / scalar_product[scalar]));
        return LineOnOrigin::from_groups(/* e415, e425, e435 */ (Simd32x3::from(inverse[scalar]) * self.group0()));
    }
}
impl Fix for MotorOnOrigin {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        7        1
    //    simd4        0        1        0
    // Totals...
    // yes simd        3        8        1
    //  no simd        3       11        1
    fn fix(self) -> Self {
        use crate::elements::*;
        let reverse = MotorOnOrigin::from_groups(
            // e415, e425, e435, e12345
            Simd32x4::from([(self.group0()[0] * -1.0), (self.group0()[1] * -1.0), (self.group0()[2] * -1.0), self.group0()[3]]),
        );
        let geometric_product = Scalar::from_groups(
            // scalar
            ((reverse.group0()[0] * self.group0()[0]) + (reverse.group0()[1] * self.group0()[1]) + (reverse.group0()[2] * self.group0()[2])
                - (reverse.group0()[3] * self.group0()[3])),
        );
        let square_root = Scalar::from_groups(/* scalar */ f32::powf(geometric_product[scalar], 0.5));
        let scalar_product = Scalar::from_groups(/* scalar */ f32::powi(square_root[scalar], 2));
        let inverse = Scalar::from_groups(/* scalar */ (1.0 / scalar_product[scalar]));
        return MotorOnOrigin::from_groups(/* e415, e425, e435, e12345 */ (Simd32x4::from(inverse[scalar]) * self.group0()));
    }
}
impl Fix for Plane {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        0        1
    //    simd4        0        1        0
    // Totals...
    // yes simd        2        1        1
    //  no simd        2        4        1
    fn fix(self) -> Self {
        use crate::elements::*;
        let geometric_product = Scalar::from_groups(/* scalar */ (-f32::powi(self.group0()[0], 2) - f32::powi(self.group0()[1], 2) - f32::powi(self.group0()[2], 2)));
        let square_root = Scalar::from_groups(/* scalar */ f32::powf(geometric_product[scalar], 0.5));
        let scalar_product = Scalar::from_groups(/* scalar */ f32::powi(square_root[scalar], 2));
        let inverse = Scalar::from_groups(/* scalar */ (1.0 / scalar_product[scalar]));
        return Plane::from_groups(/* e4235, e4315, e4125, e3215 */ (Simd32x4::from(inverse[scalar]) * self.group0()));
    }
}
impl Fix for PlaneOnOrigin {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        0        1
    //    simd3        0        1        0
    // Totals...
    // yes simd        2        1        1
    //  no simd        2        3        1
    fn fix(self) -> Self {
        use crate::elements::*;
        let geometric_product = Scalar::from_groups(/* scalar */ (-f32::powi(self.group0()[0], 2) - f32::powi(self.group0()[1], 2) - f32::powi(self.group0()[2], 2)));
        let square_root = Scalar::from_groups(/* scalar */ f32::powf(geometric_product[scalar], 0.5));
        let scalar_product = Scalar::from_groups(/* scalar */ f32::powi(square_root[scalar], 2));
        let inverse = Scalar::from_groups(/* scalar */ (1.0 / scalar_product[scalar]));
        return PlaneOnOrigin::from_groups(/* e4235, e4315, e4125 */ (Simd32x3::from(inverse[scalar]) * self.group0()));
    }
}
impl Fix for RoundPoint {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        3        1
    //    simd4        0        1        0
    // Totals...
    // yes simd        3        4        1
    //  no simd        3        7        1
    fn fix(self) -> Self {
        use crate::elements::*;
        let geometric_product = Scalar::from_groups(
            // scalar
            (f32::powi(self.group0()[0], 2) + f32::powi(self.group0()[1], 2) + f32::powi(self.group0()[2], 2) - 2.0 * (self.group0()[3] * self[e2])),
        );
        let square_root = Scalar::from_groups(/* scalar */ f32::powf(geometric_product[scalar], 0.5));
        let scalar_product = Scalar::from_groups(/* scalar */ f32::powi(square_root[scalar], 2));
        let inverse = Scalar::from_groups(/* scalar */ (1.0 / scalar_product[scalar]));
        return RoundPoint::from_groups(/* e1, e2, e3, e4 */ (Simd32x4::from(inverse[scalar]) * self.group0()), /* e5 */ (self[e2] * inverse[scalar]));
    }
}
impl Fix for RoundPointAtOrigin {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        2        1
    //    simd2        0        1        0
    // Totals...
    // yes simd        0        3        1
    //  no simd        0        4        1
    fn fix(self) -> Self {
        use crate::elements::*;
        let geometric_product = Scalar::from_groups(/* scalar */ (self.group0()[0] * self.group0()[1] * -2.0));
        let square_root = Scalar::from_groups(/* scalar */ f32::powf(geometric_product[scalar], 0.5));
        let scalar_product = Scalar::from_groups(/* scalar */ f32::powi(square_root[scalar], 2));
        let inverse = Scalar::from_groups(/* scalar */ (1.0 / scalar_product[scalar]));
        return RoundPointAtOrigin::from_groups(/* e4, e5 */ (Simd32x2::from(inverse[scalar]) * self.group0()));
    }
}
impl Fix for Scalar {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        1
    fn fix(self) -> Self {
        use crate::elements::*;
        let geometric_product = Scalar::from_groups(/* scalar */ f32::powi(self[scalar], 2));
        let square_root = Scalar::from_groups(/* scalar */ f32::powf(geometric_product[scalar], 0.5));
        let scalar_product = Scalar::from_groups(/* scalar */ f32::powi(square_root[scalar], 2));
        let inverse = Scalar::from_groups(/* scalar */ (1.0 / scalar_product[scalar]));
        return Scalar::from_groups(/* scalar */ (inverse[scalar] * self[scalar]));
    }
}
impl Fix for Sphere {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        3        1
    //    simd4        0        1        0
    // Totals...
    // yes simd        3        4        1
    //  no simd        3        7        1
    fn fix(self) -> Self {
        use crate::elements::*;
        let geometric_product = Scalar::from_groups(
            // scalar
            (-f32::powi(self.group0()[0], 2) - f32::powi(self.group0()[1], 2) - f32::powi(self.group0()[2], 2) + 2.0 * (self.group0()[3] * self[e4315])),
        );
        let square_root = Scalar::from_groups(/* scalar */ f32::powf(geometric_product[scalar], 0.5));
        let scalar_product = Scalar::from_groups(/* scalar */ f32::powi(square_root[scalar], 2));
        let inverse = Scalar::from_groups(/* scalar */ (1.0 / scalar_product[scalar]));
        return Sphere::from_groups(
            // e4235, e4315, e4125, e3215
            (Simd32x4::from(inverse[scalar]) * self.group0()),
            // e1234
            (inverse[scalar] * self[e4315]),
        );
    }
}
impl Fix for SphereAtOrigin {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        2        1
    //    simd2        0        1        0
    // Totals...
    // yes simd        0        3        1
    //  no simd        0        4        1
    fn fix(self) -> Self {
        use crate::elements::*;
        let geometric_product = Scalar::from_groups(/* scalar */ (self.group0()[0] * self.group0()[1] * 2.0));
        let square_root = Scalar::from_groups(/* scalar */ f32::powf(geometric_product[scalar], 0.5));
        let scalar_product = Scalar::from_groups(/* scalar */ f32::powi(square_root[scalar], 2));
        let inverse = Scalar::from_groups(/* scalar */ (1.0 / scalar_product[scalar]));
        return SphereAtOrigin::from_groups(/* e3215, e1234 */ (Simd32x2::from(inverse[scalar]) * self.group0()));
    }
}
impl Fix for SphereOnOrigin {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        0        1
    //    simd4        0        1        0
    // Totals...
    // yes simd        2        1        1
    //  no simd        2        4        1
    fn fix(self) -> Self {
        use crate::elements::*;
        let geometric_product = Scalar::from_groups(/* scalar */ (-f32::powi(self.group0()[0], 2) - f32::powi(self.group0()[1], 2) - f32::powi(self.group0()[2], 2)));
        let square_root = Scalar::from_groups(/* scalar */ f32::powf(geometric_product[scalar], 0.5));
        let scalar_product = Scalar::from_groups(/* scalar */ f32::powi(square_root[scalar], 2));
        let inverse = Scalar::from_groups(/* scalar */ (1.0 / scalar_product[scalar]));
        return SphereOnOrigin::from_groups(/* e4235, e4315, e4125, e1234 */ (Simd32x4::from(inverse[scalar]) * self.group0()));
    }
}
