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
impl std::ops::Div<anti_fix> for AntiDipoleOnOrigin {
    type Output = AntiDipoleOnOrigin;
    fn div(self, _rhs: anti_fix) -> Self::Output {
        self.anti_fix()
    }
}
impl std::ops::DivAssign<anti_fix> for AntiDipoleOnOrigin {
    fn div_assign(&mut self, _rhs: anti_fix) {
        *self = self.anti_fix()
    }
}
impl AntiFix for AntiDipoleOnOrigin {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        1
    //    simd4        0        2        0
    // Totals...
    // yes simd        0        3        1
    //  no simd        0        9        1
    fn anti_fix(self) -> Self {
        use crate::elements::*;
        let anti_reverse = AntiDipoleOnOrigin::from_groups(
            // e423, e431, e412, e321
            Simd32x4::from([self[e423], self[e431], self[e412], self[e321]]) * Simd32x4::from(-1.0),
        );
        let geometric_anti_product = AntiScalar::from_groups(/* e12345 */ anti_reverse[e321] * self[e321]);
        let anti_square_root = AntiScalar::from_groups(/* e12345 */ f32::powf(geometric_anti_product[e12345], 0.5));
        let anti_dot_product = AntiScalar::from_groups(/* e12345 */ f32::powi(anti_square_root[e12345], 2));
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return AntiDipoleOnOrigin::from_groups(
            // e423, e431, e412, e321
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e423], self[e431], self[e412], self[e321]]),
        );
    }
}
impl std::ops::Div<anti_fix> for AntiFlatOrigin {
    type Output = AntiFlatOrigin;
    fn div(self, _rhs: anti_fix) -> Self::Output {
        self.anti_fix()
    }
}
impl std::ops::DivAssign<anti_fix> for AntiFlatOrigin {
    fn div_assign(&mut self, _rhs: anti_fix) {
        *self = self.anti_fix()
    }
}
impl AntiFix for AntiFlatOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        3        1
    fn anti_fix(self) -> Self {
        use crate::elements::*;
        let anti_reverse = AntiFlatOrigin::from_groups(/* e321 */ self[e321] * -1.0);
        let geometric_anti_product = AntiScalar::from_groups(/* e12345 */ anti_reverse[e321] * self[e321]);
        let anti_square_root = AntiScalar::from_groups(/* e12345 */ f32::powf(geometric_anti_product[e12345], 0.5));
        let anti_dot_product = AntiScalar::from_groups(/* e12345 */ f32::powi(anti_square_root[e12345], 2));
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return AntiFlatOrigin::from_groups(/* e321 */ self[e321] * anti_inverse[e12345]);
    }
}
impl std::ops::Div<anti_fix> for AntiFlatPoint {
    type Output = AntiFlatPoint;
    fn div(self, _rhs: anti_fix) -> Self::Output {
        self.anti_fix()
    }
}
impl std::ops::DivAssign<anti_fix> for AntiFlatPoint {
    fn div_assign(&mut self, _rhs: anti_fix) {
        *self = self.anti_fix()
    }
}
impl AntiFix for AntiFlatPoint {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        1
    //    simd4        0        2        0
    // Totals...
    // yes simd        0        3        1
    //  no simd        0        9        1
    fn anti_fix(self) -> Self {
        use crate::elements::*;
        let anti_reverse = AntiFlatPoint::from_groups(
            // e235, e315, e125, e321
            Simd32x4::from([self[e235], self[e315], self[e125], self[e321]]) * Simd32x4::from(-1.0),
        );
        let geometric_anti_product = AntiScalar::from_groups(/* e12345 */ anti_reverse[e321] * self[e321]);
        let anti_square_root = AntiScalar::from_groups(/* e12345 */ f32::powf(geometric_anti_product[e12345], 0.5));
        let anti_dot_product = AntiScalar::from_groups(/* e12345 */ f32::powi(anti_square_root[e12345], 2));
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return AntiFlatPoint::from_groups(
            // e235, e315, e125, e321
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e235], self[e315], self[e125], self[e321]]),
        );
    }
}
impl std::ops::Div<anti_fix> for AntiFlectorOnOrigin {
    type Output = AntiFlectorOnOrigin;
    fn div(self, _rhs: anti_fix) -> Self::Output {
        self.anti_fix()
    }
}
impl std::ops::DivAssign<anti_fix> for AntiFlectorOnOrigin {
    fn div_assign(&mut self, _rhs: anti_fix) {
        *self = self.anti_fix()
    }
}
impl AntiFix for AntiFlectorOnOrigin {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        5        1
    //    simd4        0        1        0
    // Totals...
    // yes simd        3        6        1
    //  no simd        3        9        1
    fn anti_fix(self) -> Self {
        use crate::elements::*;
        let anti_reverse = AntiFlectorOnOrigin::from_groups(/* e321, e1, e2, e3 */ Simd32x4::from([self[e321] * -1.0, self[e1], self[e2], self[e3]]));
        let geometric_anti_product = AntiScalar::from_groups(
            // e12345
            (anti_reverse[e321] * self[e321]) - (anti_reverse[e1] * self[e1]) - (anti_reverse[e2] * self[e2]) - (anti_reverse[e3] * self[e3]),
        );
        let anti_square_root = AntiScalar::from_groups(/* e12345 */ f32::powf(geometric_anti_product[e12345], 0.5));
        let anti_dot_product = AntiScalar::from_groups(/* e12345 */ f32::powi(anti_square_root[e12345], 2));
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return AntiFlectorOnOrigin::from_groups(
            // e321, e1, e2, e3
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e321], self[e1], self[e2], self[e3]]),
        );
    }
}
impl std::ops::Div<anti_fix> for AntiLineOnOrigin {
    type Output = AntiLineOnOrigin;
    fn div(self, _rhs: anti_fix) -> Self::Output {
        self.anti_fix()
    }
}
impl std::ops::DivAssign<anti_fix> for AntiLineOnOrigin {
    fn div_assign(&mut self, _rhs: anti_fix) {
        *self = self.anti_fix()
    }
}
impl AntiFix for AntiLineOnOrigin {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        3        1
    //    simd3        0        2        0
    // Totals...
    // yes simd        2        5        1
    //  no simd        2        9        1
    fn anti_fix(self) -> Self {
        use crate::elements::*;
        let anti_reverse = AntiLineOnOrigin::from_groups(/* e23, e31, e12 */ Simd32x3::from([self[e23], self[e31], self[e12]]) * Simd32x3::from(-1.0));
        let geometric_anti_product = AntiScalar::from_groups(/* e12345 */ (anti_reverse[e23] * self[e23]) + (anti_reverse[e31] * self[e31]) + (anti_reverse[e12] * self[e12]));
        let anti_square_root = AntiScalar::from_groups(/* e12345 */ f32::powf(geometric_anti_product[e12345], 0.5));
        let anti_dot_product = AntiScalar::from_groups(/* e12345 */ f32::powi(anti_square_root[e12345], 2));
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return AntiLineOnOrigin::from_groups(/* e23, e31, e12 */ Simd32x3::from(anti_inverse[e12345]) * Simd32x3::from([self[e23], self[e31], self[e12]]));
    }
}
impl std::ops::Div<anti_fix> for AntiMotorOnOrigin {
    type Output = AntiMotorOnOrigin;
    fn div(self, _rhs: anti_fix) -> Self::Output {
        self.anti_fix()
    }
}
impl std::ops::DivAssign<anti_fix> for AntiMotorOnOrigin {
    fn div_assign(&mut self, _rhs: anti_fix) {
        *self = self.anti_fix()
    }
}
impl AntiFix for AntiMotorOnOrigin {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        7        1
    //    simd4        0        1        0
    // Totals...
    // yes simd        3        8        1
    //  no simd        3       11        1
    fn anti_fix(self) -> Self {
        use crate::elements::*;
        let anti_reverse = AntiMotorOnOrigin::from_groups(/* e23, e31, e12, scalar */ Simd32x4::from([self[e23] * -1.0, self[e31] * -1.0, self[e12] * -1.0, self[scalar]]));
        let geometric_anti_product = AntiScalar::from_groups(
            // e12345
            (anti_reverse[e23] * self[e23]) + (anti_reverse[e31] * self[e31]) + (anti_reverse[e12] * self[e12]) - (anti_reverse[scalar] * self[scalar]),
        );
        let anti_square_root = AntiScalar::from_groups(/* e12345 */ f32::powf(geometric_anti_product[e12345], 0.5));
        let anti_dot_product = AntiScalar::from_groups(/* e12345 */ f32::powi(anti_square_root[e12345], 2));
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return AntiMotorOnOrigin::from_groups(
            // e23, e31, e12, scalar
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e23], self[e31], self[e12], self[scalar]]),
        );
    }
}
impl std::ops::Div<anti_fix> for AntiPlane {
    type Output = AntiPlane;
    fn div(self, _rhs: anti_fix) -> Self::Output {
        self.anti_fix()
    }
}
impl std::ops::DivAssign<anti_fix> for AntiPlane {
    fn div_assign(&mut self, _rhs: anti_fix) {
        *self = self.anti_fix()
    }
}
impl AntiFix for AntiPlane {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        0        1
    //    simd4        0        1        0
    // Totals...
    // yes simd        2        1        1
    //  no simd        2        4        1
    fn anti_fix(self) -> Self {
        use crate::elements::*;
        let geometric_anti_product = AntiScalar::from_groups(/* e12345 */ -f32::powi(self[e1], 2) - f32::powi(self[e2], 2) - f32::powi(self[e3], 2));
        let anti_square_root = AntiScalar::from_groups(/* e12345 */ f32::powf(geometric_anti_product[e12345], 0.5));
        let anti_dot_product = AntiScalar::from_groups(/* e12345 */ f32::powi(anti_square_root[e12345], 2));
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return AntiPlane::from_groups(
            // e1, e2, e3, e5
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e1], self[e2], self[e3], self[e5]]),
        );
    }
}
impl std::ops::Div<anti_fix> for AntiPlaneOnOrigin {
    type Output = AntiPlaneOnOrigin;
    fn div(self, _rhs: anti_fix) -> Self::Output {
        self.anti_fix()
    }
}
impl std::ops::DivAssign<anti_fix> for AntiPlaneOnOrigin {
    fn div_assign(&mut self, _rhs: anti_fix) {
        *self = self.anti_fix()
    }
}
impl AntiFix for AntiPlaneOnOrigin {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        0        1
    //    simd3        0        1        0
    // Totals...
    // yes simd        2        1        1
    //  no simd        2        3        1
    fn anti_fix(self) -> Self {
        use crate::elements::*;
        let geometric_anti_product = AntiScalar::from_groups(/* e12345 */ -f32::powi(self[e1], 2) - f32::powi(self[e2], 2) - f32::powi(self[e3], 2));
        let anti_square_root = AntiScalar::from_groups(/* e12345 */ f32::powf(geometric_anti_product[e12345], 0.5));
        let anti_dot_product = AntiScalar::from_groups(/* e12345 */ f32::powi(anti_square_root[e12345], 2));
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return AntiPlaneOnOrigin::from_groups(/* e1, e2, e3 */ Simd32x3::from(anti_inverse[e12345]) * Simd32x3::from([self[e1], self[e2], self[e3]]));
    }
}
impl std::ops::Div<anti_fix> for AntiScalar {
    type Output = AntiScalar;
    fn div(self, _rhs: anti_fix) -> Self::Output {
        self.anti_fix()
    }
}
impl std::ops::DivAssign<anti_fix> for AntiScalar {
    fn div_assign(&mut self, _rhs: anti_fix) {
        *self = self.anti_fix()
    }
}
impl AntiFix for AntiScalar {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        1
    fn anti_fix(self) -> Self {
        use crate::elements::*;
        let geometric_anti_product = AntiScalar::from_groups(/* e12345 */ f32::powi(self[e12345], 2));
        let anti_square_root = AntiScalar::from_groups(/* e12345 */ f32::powf(geometric_anti_product[e12345], 0.5));
        let anti_dot_product = AntiScalar::from_groups(/* e12345 */ f32::powi(anti_square_root[e12345], 2));
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return AntiScalar::from_groups(/* e12345 */ anti_inverse[e12345] * self[e12345]);
    }
}
impl std::ops::Div<anti_fix> for AntiSphereOnOrigin {
    type Output = AntiSphereOnOrigin;
    fn div(self, _rhs: anti_fix) -> Self::Output {
        self.anti_fix()
    }
}
impl std::ops::DivAssign<anti_fix> for AntiSphereOnOrigin {
    fn div_assign(&mut self, _rhs: anti_fix) {
        *self = self.anti_fix()
    }
}
impl AntiFix for AntiSphereOnOrigin {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        0        1
    //    simd4        0        1        0
    // Totals...
    // yes simd        2        1        1
    //  no simd        2        4        1
    fn anti_fix(self) -> Self {
        use crate::elements::*;
        let geometric_anti_product = AntiScalar::from_groups(/* e12345 */ -f32::powi(self[e1], 2) - f32::powi(self[e2], 2) - f32::powi(self[e3], 2));
        let anti_square_root = AntiScalar::from_groups(/* e12345 */ f32::powf(geometric_anti_product[e12345], 0.5));
        let anti_dot_product = AntiScalar::from_groups(/* e12345 */ f32::powi(anti_square_root[e12345], 2));
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return AntiSphereOnOrigin::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e1], self[e2], self[e3], self[e4]]),
        );
    }
}
impl std::ops::Div<anti_fix> for DipoleOnOrigin {
    type Output = DipoleOnOrigin;
    fn div(self, _rhs: anti_fix) -> Self::Output {
        self.anti_fix()
    }
}
impl std::ops::DivAssign<anti_fix> for DipoleOnOrigin {
    fn div_assign(&mut self, _rhs: anti_fix) {
        *self = self.anti_fix()
    }
}
impl AntiFix for DipoleOnOrigin {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        2        1
    //    simd4        0        2        0
    // Totals...
    // yes simd        0        4        1
    //  no simd        0       10        1
    fn anti_fix(self) -> Self {
        use crate::elements::*;
        let anti_reverse = DipoleOnOrigin::from_groups(/* e41, e42, e43, e45 */ Simd32x4::from([self[e41], self[e42], self[e43], self[e45]]) * Simd32x4::from(-1.0));
        let geometric_anti_product = AntiScalar::from_groups(/* e12345 */ anti_reverse[e45] * self[e45] * -1.0);
        let anti_square_root = AntiScalar::from_groups(/* e12345 */ f32::powf(geometric_anti_product[e12345], 0.5));
        let anti_dot_product = AntiScalar::from_groups(/* e12345 */ f32::powi(anti_square_root[e12345], 2));
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return DipoleOnOrigin::from_groups(
            // e41, e42, e43, e45
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e41], self[e42], self[e43], self[e45]]),
        );
    }
}
impl std::ops::Div<anti_fix> for FlatOrigin {
    type Output = FlatOrigin;
    fn div(self, _rhs: anti_fix) -> Self::Output {
        self.anti_fix()
    }
}
impl std::ops::DivAssign<anti_fix> for FlatOrigin {
    fn div_assign(&mut self, _rhs: anti_fix) {
        *self = self.anti_fix()
    }
}
impl AntiFix for FlatOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        4        1
    fn anti_fix(self) -> Self {
        use crate::elements::*;
        let anti_reverse = FlatOrigin::from_groups(/* e45 */ self[e45] * -1.0);
        let geometric_anti_product = AntiScalar::from_groups(/* e12345 */ anti_reverse[e45] * self[e45] * -1.0);
        let anti_square_root = AntiScalar::from_groups(/* e12345 */ f32::powf(geometric_anti_product[e12345], 0.5));
        let anti_dot_product = AntiScalar::from_groups(/* e12345 */ f32::powi(anti_square_root[e12345], 2));
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return FlatOrigin::from_groups(/* e45 */ anti_inverse[e12345] * self[e45]);
    }
}
impl std::ops::Div<anti_fix> for FlatPoint {
    type Output = FlatPoint;
    fn div(self, _rhs: anti_fix) -> Self::Output {
        self.anti_fix()
    }
}
impl std::ops::DivAssign<anti_fix> for FlatPoint {
    fn div_assign(&mut self, _rhs: anti_fix) {
        *self = self.anti_fix()
    }
}
impl AntiFix for FlatPoint {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        2        1
    //    simd4        0        2        0
    // Totals...
    // yes simd        0        4        1
    //  no simd        0       10        1
    fn anti_fix(self) -> Self {
        use crate::elements::*;
        let anti_reverse = FlatPoint::from_groups(/* e15, e25, e35, e45 */ Simd32x4::from([self[e15], self[e25], self[e35], self[e45]]) * Simd32x4::from(-1.0));
        let geometric_anti_product = AntiScalar::from_groups(/* e12345 */ anti_reverse[e45] * self[e45] * -1.0);
        let anti_square_root = AntiScalar::from_groups(/* e12345 */ f32::powf(geometric_anti_product[e12345], 0.5));
        let anti_dot_product = AntiScalar::from_groups(/* e12345 */ f32::powi(anti_square_root[e12345], 2));
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return FlatPoint::from_groups(
            // e15, e25, e35, e45
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e15], self[e25], self[e35], self[e45]]),
        );
    }
}
impl std::ops::Div<anti_fix> for FlectorOnOrigin {
    type Output = FlectorOnOrigin;
    fn div(self, _rhs: anti_fix) -> Self::Output {
        self.anti_fix()
    }
}
impl std::ops::DivAssign<anti_fix> for FlectorOnOrigin {
    fn div_assign(&mut self, _rhs: anti_fix) {
        *self = self.anti_fix()
    }
}
impl AntiFix for FlectorOnOrigin {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        5        1
    //    simd4        0        1        0
    // Totals...
    // yes simd        3        6        1
    //  no simd        3        9        1
    fn anti_fix(self) -> Self {
        use crate::elements::*;
        let anti_reverse = FlectorOnOrigin::from_groups(/* e45, e4235, e4315, e4125 */ Simd32x4::from([self[e45] * -1.0, self[e4235], self[e4315], self[e4125]]));
        let geometric_anti_product = AntiScalar::from_groups(
            // e12345
            (anti_reverse[e4235] * self[e4235]) + (anti_reverse[e4315] * self[e4315]) + (anti_reverse[e4125] * self[e4125]) - (anti_reverse[e45] * self[e45]),
        );
        let anti_square_root = AntiScalar::from_groups(/* e12345 */ f32::powf(geometric_anti_product[e12345], 0.5));
        let anti_dot_product = AntiScalar::from_groups(/* e12345 */ f32::powi(anti_square_root[e12345], 2));
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return FlectorOnOrigin::from_groups(
            // e45, e4235, e4315, e4125
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e45], self[e4235], self[e4315], self[e4125]]),
        );
    }
}
impl std::ops::Div<anti_fix> for LineOnOrigin {
    type Output = LineOnOrigin;
    fn div(self, _rhs: anti_fix) -> Self::Output {
        self.anti_fix()
    }
}
impl std::ops::DivAssign<anti_fix> for LineOnOrigin {
    fn div_assign(&mut self, _rhs: anti_fix) {
        *self = self.anti_fix()
    }
}
impl AntiFix for LineOnOrigin {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        3        1
    //    simd3        0        2        0
    // Totals...
    // yes simd        2        5        1
    //  no simd        2        9        1
    fn anti_fix(self) -> Self {
        use crate::elements::*;
        let anti_reverse = LineOnOrigin::from_groups(/* e415, e425, e435 */ Simd32x3::from([self[e415], self[e425], self[e435]]) * Simd32x3::from(-1.0));
        let geometric_anti_product = AntiScalar::from_groups(
            // e12345
            -(anti_reverse[e415] * self[e415]) - (anti_reverse[e425] * self[e425]) - (anti_reverse[e435] * self[e435]),
        );
        let anti_square_root = AntiScalar::from_groups(/* e12345 */ f32::powf(geometric_anti_product[e12345], 0.5));
        let anti_dot_product = AntiScalar::from_groups(/* e12345 */ f32::powi(anti_square_root[e12345], 2));
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return LineOnOrigin::from_groups(/* e415, e425, e435 */ Simd32x3::from(anti_inverse[e12345]) * Simd32x3::from([self[e415], self[e425], self[e435]]));
    }
}
impl std::ops::Div<anti_fix> for MotorOnOrigin {
    type Output = MotorOnOrigin;
    fn div(self, _rhs: anti_fix) -> Self::Output {
        self.anti_fix()
    }
}
impl std::ops::DivAssign<anti_fix> for MotorOnOrigin {
    fn div_assign(&mut self, _rhs: anti_fix) {
        *self = self.anti_fix()
    }
}
impl AntiFix for MotorOnOrigin {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        7        1
    //    simd4        0        1        0
    // Totals...
    // yes simd        3        8        1
    //  no simd        3       11        1
    fn anti_fix(self) -> Self {
        use crate::elements::*;
        let anti_reverse = MotorOnOrigin::from_groups(
            // e415, e425, e435, e12345
            Simd32x4::from([self[e415] * -1.0, self[e425] * -1.0, self[e435] * -1.0, self[e12345]]),
        );
        let geometric_anti_product = AntiScalar::from_groups(
            // e12345
            (anti_reverse[e12345] * self[e12345]) - (anti_reverse[e415] * self[e415]) - (anti_reverse[e425] * self[e425]) - (anti_reverse[e435] * self[e435]),
        );
        let anti_square_root = AntiScalar::from_groups(/* e12345 */ f32::powf(geometric_anti_product[e12345], 0.5));
        let anti_dot_product = AntiScalar::from_groups(/* e12345 */ f32::powi(anti_square_root[e12345], 2));
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return MotorOnOrigin::from_groups(
            // e415, e425, e435, e12345
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e415], self[e425], self[e435], self[e12345]]),
        );
    }
}
impl std::ops::Div<anti_fix> for Plane {
    type Output = Plane;
    fn div(self, _rhs: anti_fix) -> Self::Output {
        self.anti_fix()
    }
}
impl std::ops::DivAssign<anti_fix> for Plane {
    fn div_assign(&mut self, _rhs: anti_fix) {
        *self = self.anti_fix()
    }
}
impl AntiFix for Plane {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        0        1
    //    simd4        0        1        0
    // Totals...
    // yes simd        2        1        1
    //  no simd        2        4        1
    fn anti_fix(self) -> Self {
        use crate::elements::*;
        let geometric_anti_product = AntiScalar::from_groups(/* e12345 */ f32::powi(self[e4235], 2) + f32::powi(self[e4315], 2) + f32::powi(self[e4125], 2));
        let anti_square_root = AntiScalar::from_groups(/* e12345 */ f32::powf(geometric_anti_product[e12345], 0.5));
        let anti_dot_product = AntiScalar::from_groups(/* e12345 */ f32::powi(anti_square_root[e12345], 2));
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return Plane::from_groups(
            // e4235, e4315, e4125, e3215
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e4235], self[e4315], self[e4125], self[e3215]]),
        );
    }
}
impl std::ops::Div<anti_fix> for PlaneOnOrigin {
    type Output = PlaneOnOrigin;
    fn div(self, _rhs: anti_fix) -> Self::Output {
        self.anti_fix()
    }
}
impl std::ops::DivAssign<anti_fix> for PlaneOnOrigin {
    fn div_assign(&mut self, _rhs: anti_fix) {
        *self = self.anti_fix()
    }
}
impl AntiFix for PlaneOnOrigin {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        0        1
    //    simd3        0        1        0
    // Totals...
    // yes simd        2        1        1
    //  no simd        2        3        1
    fn anti_fix(self) -> Self {
        use crate::elements::*;
        let geometric_anti_product = AntiScalar::from_groups(/* e12345 */ f32::powi(self[e4235], 2) + f32::powi(self[e4315], 2) + f32::powi(self[e4125], 2));
        let anti_square_root = AntiScalar::from_groups(/* e12345 */ f32::powf(geometric_anti_product[e12345], 0.5));
        let anti_dot_product = AntiScalar::from_groups(/* e12345 */ f32::powi(anti_square_root[e12345], 2));
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return PlaneOnOrigin::from_groups(
            // e4235, e4315, e4125
            Simd32x3::from(anti_inverse[e12345]) * Simd32x3::from([self[e4235], self[e4315], self[e4125]]),
        );
    }
}
impl std::ops::Div<anti_fix> for RoundPoint {
    type Output = RoundPoint;
    fn div(self, _rhs: anti_fix) -> Self::Output {
        self.anti_fix()
    }
}
impl std::ops::DivAssign<anti_fix> for RoundPoint {
    fn div_assign(&mut self, _rhs: anti_fix) {
        *self = self.anti_fix()
    }
}
impl AntiFix for RoundPoint {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        3        1
    //    simd4        0        1        0
    // Totals...
    // yes simd        3        4        1
    //  no simd        3        7        1
    fn anti_fix(self) -> Self {
        use crate::elements::*;
        let geometric_anti_product =
            AntiScalar::from_groups(/* e12345 */ 2.0 * (self[e4] * self[e5]) - f32::powi(self[e1], 2) - f32::powi(self[e2], 2) - f32::powi(self[e3], 2));
        let anti_square_root = AntiScalar::from_groups(/* e12345 */ f32::powf(geometric_anti_product[e12345], 0.5));
        let anti_dot_product = AntiScalar::from_groups(/* e12345 */ f32::powi(anti_square_root[e12345], 2));
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return RoundPoint::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e1], self[e2], self[e3], self[e4]]),
            // e5
            anti_inverse[e12345] * self[e5],
        );
    }
}
impl std::ops::Div<anti_fix> for RoundPointAtOrigin {
    type Output = RoundPointAtOrigin;
    fn div(self, _rhs: anti_fix) -> Self::Output {
        self.anti_fix()
    }
}
impl std::ops::DivAssign<anti_fix> for RoundPointAtOrigin {
    fn div_assign(&mut self, _rhs: anti_fix) {
        *self = self.anti_fix()
    }
}
impl AntiFix for RoundPointAtOrigin {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        2        1
    //    simd2        0        1        0
    // Totals...
    // yes simd        0        3        1
    //  no simd        0        4        1
    fn anti_fix(self) -> Self {
        use crate::elements::*;
        let geometric_anti_product = AntiScalar::from_groups(/* e12345 */ self[e4] * self[e5] * 2.0);
        let anti_square_root = AntiScalar::from_groups(/* e12345 */ f32::powf(geometric_anti_product[e12345], 0.5));
        let anti_dot_product = AntiScalar::from_groups(/* e12345 */ f32::powi(anti_square_root[e12345], 2));
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return RoundPointAtOrigin::from_groups(/* e4, e5 */ Simd32x2::from(anti_inverse[e12345]) * Simd32x2::from([self[e4], self[e5]]));
    }
}
impl std::ops::Div<anti_fix> for Scalar {
    type Output = Scalar;
    fn div(self, _rhs: anti_fix) -> Self::Output {
        self.anti_fix()
    }
}
impl std::ops::DivAssign<anti_fix> for Scalar {
    fn div_assign(&mut self, _rhs: anti_fix) {
        *self = self.anti_fix()
    }
}
impl AntiFix for Scalar {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        2        1
    fn anti_fix(self) -> Self {
        use crate::elements::*;
        let geometric_anti_product = AntiScalar::from_groups(/* e12345 */ f32::powi(self[scalar], 2) * -1.0);
        let anti_square_root = AntiScalar::from_groups(/* e12345 */ f32::powf(geometric_anti_product[e12345], 0.5));
        let anti_dot_product = AntiScalar::from_groups(/* e12345 */ f32::powi(anti_square_root[e12345], 2));
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return Scalar::from_groups(/* scalar */ anti_inverse[e12345] * self[scalar]);
    }
}
impl std::ops::Div<anti_fix> for Sphere {
    type Output = Sphere;
    fn div(self, _rhs: anti_fix) -> Self::Output {
        self.anti_fix()
    }
}
impl std::ops::DivAssign<anti_fix> for Sphere {
    fn div_assign(&mut self, _rhs: anti_fix) {
        *self = self.anti_fix()
    }
}
impl AntiFix for Sphere {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        3        1
    //    simd4        0        1        0
    // Totals...
    // yes simd        3        4        1
    //  no simd        3        7        1
    fn anti_fix(self) -> Self {
        use crate::elements::*;
        let geometric_anti_product = AntiScalar::from_groups(
            // e12345
            f32::powi(self[e4235], 2) + f32::powi(self[e4315], 2) + f32::powi(self[e4125], 2) - 2.0 * (self[e3215] * self[e1234]),
        );
        let anti_square_root = AntiScalar::from_groups(/* e12345 */ f32::powf(geometric_anti_product[e12345], 0.5));
        let anti_dot_product = AntiScalar::from_groups(/* e12345 */ f32::powi(anti_square_root[e12345], 2));
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return Sphere::from_groups(
            // e4235, e4315, e4125, e3215
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e4235], self[e4315], self[e4125], self[e3215]]),
            // e1234
            anti_inverse[e12345] * self[e1234],
        );
    }
}
impl std::ops::Div<anti_fix> for SphereAtOrigin {
    type Output = SphereAtOrigin;
    fn div(self, _rhs: anti_fix) -> Self::Output {
        self.anti_fix()
    }
}
impl std::ops::DivAssign<anti_fix> for SphereAtOrigin {
    fn div_assign(&mut self, _rhs: anti_fix) {
        *self = self.anti_fix()
    }
}
impl AntiFix for SphereAtOrigin {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        2        1
    //    simd2        0        1        0
    // Totals...
    // yes simd        0        3        1
    //  no simd        0        4        1
    fn anti_fix(self) -> Self {
        use crate::elements::*;
        let geometric_anti_product = AntiScalar::from_groups(/* e12345 */ self[e3215] * self[e1234] * -2.0);
        let anti_square_root = AntiScalar::from_groups(/* e12345 */ f32::powf(geometric_anti_product[e12345], 0.5));
        let anti_dot_product = AntiScalar::from_groups(/* e12345 */ f32::powi(anti_square_root[e12345], 2));
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return SphereAtOrigin::from_groups(/* e3215, e1234 */ Simd32x2::from(anti_inverse[e12345]) * Simd32x2::from([self[e3215], self[e1234]]));
    }
}
impl std::ops::Div<anti_fix> for SphereOnOrigin {
    type Output = SphereOnOrigin;
    fn div(self, _rhs: anti_fix) -> Self::Output {
        self.anti_fix()
    }
}
impl std::ops::DivAssign<anti_fix> for SphereOnOrigin {
    fn div_assign(&mut self, _rhs: anti_fix) {
        *self = self.anti_fix()
    }
}
impl AntiFix for SphereOnOrigin {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        0        1
    //    simd4        0        1        0
    // Totals...
    // yes simd        2        1        1
    //  no simd        2        4        1
    fn anti_fix(self) -> Self {
        use crate::elements::*;
        let geometric_anti_product = AntiScalar::from_groups(/* e12345 */ f32::powi(self[e4235], 2) + f32::powi(self[e4315], 2) + f32::powi(self[e4125], 2));
        let anti_square_root = AntiScalar::from_groups(/* e12345 */ f32::powf(geometric_anti_product[e12345], 0.5));
        let anti_dot_product = AntiScalar::from_groups(/* e12345 */ f32::powi(anti_square_root[e12345], 2));
        let anti_inverse = AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
        return SphereOnOrigin::from_groups(
            // e4235, e4315, e4125, e1234
            Simd32x4::from(anti_inverse[e12345]) * Simd32x4::from([self[e4235], self[e4315], self[e4125], self[e1234]]),
        );
    }
}
