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
impl std::ops::Div<fix> for AntiDipoleOnOrigin {
    type Output = AntiDipoleOnOrigin;
    fn div(self, _rhs: fix) -> Self::Output {
        self.fix()
    }
}
impl std::ops::DivAssign<fix> for AntiDipoleOnOrigin {
    fn div_assign(&mut self, _rhs: fix) {
        *self = self.fix()
    }
}
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
        let reverse = AntiDipoleOnOrigin::from_groups(
            // e423, e431, e412, e321
            Simd32x4::from([self[e423], self[e431], self[e412], self[e321]]) * Simd32x4::from(-1.0),
        );
        let geometric_product = Scalar::from_groups(/* scalar */ reverse[e321] * self[e321] * -1.0);
        let square_root = Scalar::from_groups(/* scalar */ f32::powf(geometric_product[scalar], 0.5));
        let dot_product = Scalar::from_groups(/* scalar */ f32::powi(square_root[scalar], 2));
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return AntiDipoleOnOrigin::from_groups(
            // e423, e431, e412, e321
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e423], self[e431], self[e412], self[e321]]),
        );
    }
}
impl std::ops::Div<fix> for AntiFlatOrigin {
    type Output = AntiFlatOrigin;
    fn div(self, _rhs: fix) -> Self::Output {
        self.fix()
    }
}
impl std::ops::DivAssign<fix> for AntiFlatOrigin {
    fn div_assign(&mut self, _rhs: fix) {
        *self = self.fix()
    }
}
impl Fix for AntiFlatOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        4        1
    fn fix(self) -> Self {
        use crate::elements::*;
        let reverse = AntiFlatOrigin::from_groups(/* e321 */ self[e321] * -1.0);
        let geometric_product = Scalar::from_groups(/* scalar */ reverse[e321] * self[e321] * -1.0);
        let square_root = Scalar::from_groups(/* scalar */ f32::powf(geometric_product[scalar], 0.5));
        let dot_product = Scalar::from_groups(/* scalar */ f32::powi(square_root[scalar], 2));
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return AntiFlatOrigin::from_groups(/* e321 */ self[e321] * inverse[scalar]);
    }
}
impl std::ops::Div<fix> for AntiFlatPoint {
    type Output = AntiFlatPoint;
    fn div(self, _rhs: fix) -> Self::Output {
        self.fix()
    }
}
impl std::ops::DivAssign<fix> for AntiFlatPoint {
    fn div_assign(&mut self, _rhs: fix) {
        *self = self.fix()
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
        let reverse = AntiFlatPoint::from_groups(
            // e235, e315, e125, e321
            Simd32x4::from([self[e235], self[e315], self[e125], self[e321]]) * Simd32x4::from(-1.0),
        );
        let geometric_product = Scalar::from_groups(/* scalar */ reverse[e321] * self[e321] * -1.0);
        let square_root = Scalar::from_groups(/* scalar */ f32::powf(geometric_product[scalar], 0.5));
        let dot_product = Scalar::from_groups(/* scalar */ f32::powi(square_root[scalar], 2));
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return AntiFlatPoint::from_groups(
            // e235, e315, e125, e321
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e235], self[e315], self[e125], self[e321]]),
        );
    }
}
impl std::ops::Div<fix> for AntiFlectorOnOrigin {
    type Output = AntiFlectorOnOrigin;
    fn div(self, _rhs: fix) -> Self::Output {
        self.fix()
    }
}
impl std::ops::DivAssign<fix> for AntiFlectorOnOrigin {
    fn div_assign(&mut self, _rhs: fix) {
        *self = self.fix()
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
        let reverse = AntiFlectorOnOrigin::from_groups(/* e321, e1, e2, e3 */ Simd32x4::from([self[e321] * -1.0, self[e1], self[e2], self[e3]]));
        let geometric_product = Scalar::from_groups(
            // scalar
            (reverse[e1] * self[e1]) + (reverse[e2] * self[e2]) + (reverse[e3] * self[e3]) - (reverse[e321] * self[e321]),
        );
        let square_root = Scalar::from_groups(/* scalar */ f32::powf(geometric_product[scalar], 0.5));
        let dot_product = Scalar::from_groups(/* scalar */ f32::powi(square_root[scalar], 2));
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return AntiFlectorOnOrigin::from_groups(/* e321, e1, e2, e3 */ Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e321], self[e1], self[e2], self[e3]]));
    }
}
impl std::ops::Div<fix> for AntiLineOnOrigin {
    type Output = AntiLineOnOrigin;
    fn div(self, _rhs: fix) -> Self::Output {
        self.fix()
    }
}
impl std::ops::DivAssign<fix> for AntiLineOnOrigin {
    fn div_assign(&mut self, _rhs: fix) {
        *self = self.fix()
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
        let reverse = AntiLineOnOrigin::from_groups(/* e23, e31, e12 */ Simd32x3::from([self[e23], self[e31], self[e12]]) * Simd32x3::from(-1.0));
        let geometric_product = Scalar::from_groups(/* scalar */ -(reverse[e23] * self[e23]) - (reverse[e31] * self[e31]) - (reverse[e12] * self[e12]));
        let square_root = Scalar::from_groups(/* scalar */ f32::powf(geometric_product[scalar], 0.5));
        let dot_product = Scalar::from_groups(/* scalar */ f32::powi(square_root[scalar], 2));
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return AntiLineOnOrigin::from_groups(/* e23, e31, e12 */ Simd32x3::from(inverse[scalar]) * Simd32x3::from([self[e23], self[e31], self[e12]]));
    }
}
impl std::ops::Div<fix> for AntiMotorOnOrigin {
    type Output = AntiMotorOnOrigin;
    fn div(self, _rhs: fix) -> Self::Output {
        self.fix()
    }
}
impl std::ops::DivAssign<fix> for AntiMotorOnOrigin {
    fn div_assign(&mut self, _rhs: fix) {
        *self = self.fix()
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
        let reverse = AntiMotorOnOrigin::from_groups(/* e23, e31, e12, scalar */ Simd32x4::from([self[e23] * -1.0, self[e31] * -1.0, self[e12] * -1.0, self[scalar]]));
        let geometric_product = Scalar::from_groups(
            // scalar
            (reverse[scalar] * self[scalar]) - (reverse[e23] * self[e23]) - (reverse[e31] * self[e31]) - (reverse[e12] * self[e12]),
        );
        let square_root = Scalar::from_groups(/* scalar */ f32::powf(geometric_product[scalar], 0.5));
        let dot_product = Scalar::from_groups(/* scalar */ f32::powi(square_root[scalar], 2));
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return AntiMotorOnOrigin::from_groups(
            // e23, e31, e12, scalar
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e23], self[e31], self[e12], self[scalar]]),
        );
    }
}
impl std::ops::Div<fix> for AntiPlane {
    type Output = AntiPlane;
    fn div(self, _rhs: fix) -> Self::Output {
        self.fix()
    }
}
impl std::ops::DivAssign<fix> for AntiPlane {
    fn div_assign(&mut self, _rhs: fix) {
        *self = self.fix()
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
        let geometric_product = Scalar::from_groups(/* scalar */ f32::powi(self[e1], 2) + f32::powi(self[e2], 2) + f32::powi(self[e3], 2));
        let square_root = Scalar::from_groups(/* scalar */ f32::powf(geometric_product[scalar], 0.5));
        let dot_product = Scalar::from_groups(/* scalar */ f32::powi(square_root[scalar], 2));
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return AntiPlane::from_groups(/* e1, e2, e3, e5 */ Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e1], self[e2], self[e3], self[e5]]));
    }
}
impl std::ops::Div<fix> for AntiPlaneOnOrigin {
    type Output = AntiPlaneOnOrigin;
    fn div(self, _rhs: fix) -> Self::Output {
        self.fix()
    }
}
impl std::ops::DivAssign<fix> for AntiPlaneOnOrigin {
    fn div_assign(&mut self, _rhs: fix) {
        *self = self.fix()
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
        let geometric_product = Scalar::from_groups(/* scalar */ f32::powi(self[e1], 2) + f32::powi(self[e2], 2) + f32::powi(self[e3], 2));
        let square_root = Scalar::from_groups(/* scalar */ f32::powf(geometric_product[scalar], 0.5));
        let dot_product = Scalar::from_groups(/* scalar */ f32::powi(square_root[scalar], 2));
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return AntiPlaneOnOrigin::from_groups(/* e1, e2, e3 */ Simd32x3::from(inverse[scalar]) * Simd32x3::from([self[e1], self[e2], self[e3]]));
    }
}
impl std::ops::Div<fix> for AntiScalar {
    type Output = AntiScalar;
    fn div(self, _rhs: fix) -> Self::Output {
        self.fix()
    }
}
impl std::ops::DivAssign<fix> for AntiScalar {
    fn div_assign(&mut self, _rhs: fix) {
        *self = self.fix()
    }
}
impl Fix for AntiScalar {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        2        1
    fn fix(self) -> Self {
        use crate::elements::*;
        let geometric_product = Scalar::from_groups(/* scalar */ f32::powi(self[e12345], 2) * -1.0);
        let square_root = Scalar::from_groups(/* scalar */ f32::powf(geometric_product[scalar], 0.5));
        let dot_product = Scalar::from_groups(/* scalar */ f32::powi(square_root[scalar], 2));
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return AntiScalar::from_groups(/* e12345 */ self[e12345] * inverse[scalar]);
    }
}
impl std::ops::Div<fix> for AntiSphereOnOrigin {
    type Output = AntiSphereOnOrigin;
    fn div(self, _rhs: fix) -> Self::Output {
        self.fix()
    }
}
impl std::ops::DivAssign<fix> for AntiSphereOnOrigin {
    fn div_assign(&mut self, _rhs: fix) {
        *self = self.fix()
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
        let geometric_product = Scalar::from_groups(/* scalar */ f32::powi(self[e1], 2) + f32::powi(self[e2], 2) + f32::powi(self[e3], 2));
        let square_root = Scalar::from_groups(/* scalar */ f32::powf(geometric_product[scalar], 0.5));
        let dot_product = Scalar::from_groups(/* scalar */ f32::powi(square_root[scalar], 2));
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return AntiSphereOnOrigin::from_groups(/* e1, e2, e3, e4 */ Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e1], self[e2], self[e3], self[e4]]));
    }
}
impl std::ops::Div<fix> for DipoleOnOrigin {
    type Output = DipoleOnOrigin;
    fn div(self, _rhs: fix) -> Self::Output {
        self.fix()
    }
}
impl std::ops::DivAssign<fix> for DipoleOnOrigin {
    fn div_assign(&mut self, _rhs: fix) {
        *self = self.fix()
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
        let reverse = DipoleOnOrigin::from_groups(/* e41, e42, e43, e45 */ Simd32x4::from([self[e41], self[e42], self[e43], self[e45]]) * Simd32x4::from(-1.0));
        let geometric_product = Scalar::from_groups(/* scalar */ reverse[e45] * self[e45]);
        let square_root = Scalar::from_groups(/* scalar */ f32::powf(geometric_product[scalar], 0.5));
        let dot_product = Scalar::from_groups(/* scalar */ f32::powi(square_root[scalar], 2));
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return DipoleOnOrigin::from_groups(
            // e41, e42, e43, e45
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e41], self[e42], self[e43], self[e45]]),
        );
    }
}
impl std::ops::Div<fix> for FlatOrigin {
    type Output = FlatOrigin;
    fn div(self, _rhs: fix) -> Self::Output {
        self.fix()
    }
}
impl std::ops::DivAssign<fix> for FlatOrigin {
    fn div_assign(&mut self, _rhs: fix) {
        *self = self.fix()
    }
}
impl Fix for FlatOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        3        1
    fn fix(self) -> Self {
        use crate::elements::*;
        let reverse = FlatOrigin::from_groups(/* e45 */ self[e45] * -1.0);
        let geometric_product = Scalar::from_groups(/* scalar */ reverse[e45] * self[e45]);
        let square_root = Scalar::from_groups(/* scalar */ f32::powf(geometric_product[scalar], 0.5));
        let dot_product = Scalar::from_groups(/* scalar */ f32::powi(square_root[scalar], 2));
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return FlatOrigin::from_groups(/* e45 */ self[e45] * inverse[scalar]);
    }
}
impl std::ops::Div<fix> for FlatPoint {
    type Output = FlatPoint;
    fn div(self, _rhs: fix) -> Self::Output {
        self.fix()
    }
}
impl std::ops::DivAssign<fix> for FlatPoint {
    fn div_assign(&mut self, _rhs: fix) {
        *self = self.fix()
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
        let reverse = FlatPoint::from_groups(/* e15, e25, e35, e45 */ Simd32x4::from([self[e15], self[e25], self[e35], self[e45]]) * Simd32x4::from(-1.0));
        let geometric_product = Scalar::from_groups(/* scalar */ reverse[e45] * self[e45]);
        let square_root = Scalar::from_groups(/* scalar */ f32::powf(geometric_product[scalar], 0.5));
        let dot_product = Scalar::from_groups(/* scalar */ f32::powi(square_root[scalar], 2));
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return FlatPoint::from_groups(
            // e15, e25, e35, e45
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e15], self[e25], self[e35], self[e45]]),
        );
    }
}
impl std::ops::Div<fix> for FlectorOnOrigin {
    type Output = FlectorOnOrigin;
    fn div(self, _rhs: fix) -> Self::Output {
        self.fix()
    }
}
impl std::ops::DivAssign<fix> for FlectorOnOrigin {
    fn div_assign(&mut self, _rhs: fix) {
        *self = self.fix()
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
        let reverse = FlectorOnOrigin::from_groups(/* e45, e4235, e4315, e4125 */ Simd32x4::from([self[e45] * -1.0, self[e4235], self[e4315], self[e4125]]));
        let geometric_product = Scalar::from_groups(
            // scalar
            (reverse[e45] * self[e45]) - (reverse[e4235] * self[e4235]) - (reverse[e4315] * self[e4315]) - (reverse[e4125] * self[e4125]),
        );
        let square_root = Scalar::from_groups(/* scalar */ f32::powf(geometric_product[scalar], 0.5));
        let dot_product = Scalar::from_groups(/* scalar */ f32::powi(square_root[scalar], 2));
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return FlectorOnOrigin::from_groups(
            // e45, e4235, e4315, e4125
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e45], self[e4235], self[e4315], self[e4125]]),
        );
    }
}
impl std::ops::Div<fix> for LineOnOrigin {
    type Output = LineOnOrigin;
    fn div(self, _rhs: fix) -> Self::Output {
        self.fix()
    }
}
impl std::ops::DivAssign<fix> for LineOnOrigin {
    fn div_assign(&mut self, _rhs: fix) {
        *self = self.fix()
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
        let reverse = LineOnOrigin::from_groups(/* e415, e425, e435 */ Simd32x3::from([self[e415], self[e425], self[e435]]) * Simd32x3::from(-1.0));
        let geometric_product = Scalar::from_groups(/* scalar */ (reverse[e415] * self[e415]) + (reverse[e425] * self[e425]) + (reverse[e435] * self[e435]));
        let square_root = Scalar::from_groups(/* scalar */ f32::powf(geometric_product[scalar], 0.5));
        let dot_product = Scalar::from_groups(/* scalar */ f32::powi(square_root[scalar], 2));
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return LineOnOrigin::from_groups(/* e415, e425, e435 */ Simd32x3::from(inverse[scalar]) * Simd32x3::from([self[e415], self[e425], self[e435]]));
    }
}
impl std::ops::Div<fix> for MotorOnOrigin {
    type Output = MotorOnOrigin;
    fn div(self, _rhs: fix) -> Self::Output {
        self.fix()
    }
}
impl std::ops::DivAssign<fix> for MotorOnOrigin {
    fn div_assign(&mut self, _rhs: fix) {
        *self = self.fix()
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
            Simd32x4::from([self[e415] * -1.0, self[e425] * -1.0, self[e435] * -1.0, self[e12345]]),
        );
        let geometric_product = Scalar::from_groups(
            // scalar
            (reverse[e415] * self[e415]) + (reverse[e425] * self[e425]) + (reverse[e435] * self[e435]) - (reverse[e12345] * self[e12345]),
        );
        let square_root = Scalar::from_groups(/* scalar */ f32::powf(geometric_product[scalar], 0.5));
        let dot_product = Scalar::from_groups(/* scalar */ f32::powi(square_root[scalar], 2));
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return MotorOnOrigin::from_groups(
            // e415, e425, e435, e12345
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e415], self[e425], self[e435], self[e12345]]),
        );
    }
}
impl std::ops::Div<fix> for Plane {
    type Output = Plane;
    fn div(self, _rhs: fix) -> Self::Output {
        self.fix()
    }
}
impl std::ops::DivAssign<fix> for Plane {
    fn div_assign(&mut self, _rhs: fix) {
        *self = self.fix()
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
        let geometric_product = Scalar::from_groups(/* scalar */ -f32::powi(self[e4235], 2) - f32::powi(self[e4315], 2) - f32::powi(self[e4125], 2));
        let square_root = Scalar::from_groups(/* scalar */ f32::powf(geometric_product[scalar], 0.5));
        let dot_product = Scalar::from_groups(/* scalar */ f32::powi(square_root[scalar], 2));
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return Plane::from_groups(
            // e4235, e4315, e4125, e3215
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e4235], self[e4315], self[e4125], self[e3215]]),
        );
    }
}
impl std::ops::Div<fix> for PlaneOnOrigin {
    type Output = PlaneOnOrigin;
    fn div(self, _rhs: fix) -> Self::Output {
        self.fix()
    }
}
impl std::ops::DivAssign<fix> for PlaneOnOrigin {
    fn div_assign(&mut self, _rhs: fix) {
        *self = self.fix()
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
        let geometric_product = Scalar::from_groups(/* scalar */ -f32::powi(self[e4235], 2) - f32::powi(self[e4315], 2) - f32::powi(self[e4125], 2));
        let square_root = Scalar::from_groups(/* scalar */ f32::powf(geometric_product[scalar], 0.5));
        let dot_product = Scalar::from_groups(/* scalar */ f32::powi(square_root[scalar], 2));
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return PlaneOnOrigin::from_groups(/* e4235, e4315, e4125 */ Simd32x3::from(inverse[scalar]) * Simd32x3::from([self[e4235], self[e4315], self[e4125]]));
    }
}
impl std::ops::Div<fix> for RoundPoint {
    type Output = RoundPoint;
    fn div(self, _rhs: fix) -> Self::Output {
        self.fix()
    }
}
impl std::ops::DivAssign<fix> for RoundPoint {
    fn div_assign(&mut self, _rhs: fix) {
        *self = self.fix()
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
        let geometric_product = Scalar::from_groups(/* scalar */ f32::powi(self[e1], 2) + f32::powi(self[e2], 2) + f32::powi(self[e3], 2) - 2.0 * (self[e4] * self[e5]));
        let square_root = Scalar::from_groups(/* scalar */ f32::powf(geometric_product[scalar], 0.5));
        let dot_product = Scalar::from_groups(/* scalar */ f32::powi(square_root[scalar], 2));
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return RoundPoint::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e1], self[e2], self[e3], self[e4]]),
            // e5
            self[e5] * inverse[scalar],
        );
    }
}
impl std::ops::Div<fix> for RoundPointAtOrigin {
    type Output = RoundPointAtOrigin;
    fn div(self, _rhs: fix) -> Self::Output {
        self.fix()
    }
}
impl std::ops::DivAssign<fix> for RoundPointAtOrigin {
    fn div_assign(&mut self, _rhs: fix) {
        *self = self.fix()
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
        let geometric_product = Scalar::from_groups(/* scalar */ self[e4] * self[e5] * -2.0);
        let square_root = Scalar::from_groups(/* scalar */ f32::powf(geometric_product[scalar], 0.5));
        let dot_product = Scalar::from_groups(/* scalar */ f32::powi(square_root[scalar], 2));
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return RoundPointAtOrigin::from_groups(/* e4, e5 */ Simd32x2::from(inverse[scalar]) * Simd32x2::from([self[e4], self[e5]]));
    }
}
impl std::ops::Div<fix> for Scalar {
    type Output = Scalar;
    fn div(self, _rhs: fix) -> Self::Output {
        self.fix()
    }
}
impl std::ops::DivAssign<fix> for Scalar {
    fn div_assign(&mut self, _rhs: fix) {
        *self = self.fix()
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
        let dot_product = Scalar::from_groups(/* scalar */ f32::powi(square_root[scalar], 2));
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return Scalar::from_groups(/* scalar */ inverse[scalar] * self[scalar]);
    }
}
impl std::ops::Div<fix> for Sphere {
    type Output = Sphere;
    fn div(self, _rhs: fix) -> Self::Output {
        self.fix()
    }
}
impl std::ops::DivAssign<fix> for Sphere {
    fn div_assign(&mut self, _rhs: fix) {
        *self = self.fix()
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
            2.0 * (self[e3215] * self[e1234]) - f32::powi(self[e4235], 2) - f32::powi(self[e4315], 2) - f32::powi(self[e4125], 2),
        );
        let square_root = Scalar::from_groups(/* scalar */ f32::powf(geometric_product[scalar], 0.5));
        let dot_product = Scalar::from_groups(/* scalar */ f32::powi(square_root[scalar], 2));
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return Sphere::from_groups(
            // e4235, e4315, e4125, e3215
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e4235], self[e4315], self[e4125], self[e3215]]),
            // e1234
            inverse[scalar] * self[e1234],
        );
    }
}
impl std::ops::Div<fix> for SphereAtOrigin {
    type Output = SphereAtOrigin;
    fn div(self, _rhs: fix) -> Self::Output {
        self.fix()
    }
}
impl std::ops::DivAssign<fix> for SphereAtOrigin {
    fn div_assign(&mut self, _rhs: fix) {
        *self = self.fix()
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
        let geometric_product = Scalar::from_groups(/* scalar */ self[e3215] * self[e1234] * 2.0);
        let square_root = Scalar::from_groups(/* scalar */ f32::powf(geometric_product[scalar], 0.5));
        let dot_product = Scalar::from_groups(/* scalar */ f32::powi(square_root[scalar], 2));
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return SphereAtOrigin::from_groups(/* e3215, e1234 */ Simd32x2::from(inverse[scalar]) * Simd32x2::from([self[e3215], self[e1234]]));
    }
}
impl std::ops::Div<fix> for SphereOnOrigin {
    type Output = SphereOnOrigin;
    fn div(self, _rhs: fix) -> Self::Output {
        self.fix()
    }
}
impl std::ops::DivAssign<fix> for SphereOnOrigin {
    fn div_assign(&mut self, _rhs: fix) {
        *self = self.fix()
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
        let geometric_product = Scalar::from_groups(/* scalar */ -f32::powi(self[e4235], 2) - f32::powi(self[e4315], 2) - f32::powi(self[e4125], 2));
        let square_root = Scalar::from_groups(/* scalar */ f32::powf(geometric_product[scalar], 0.5));
        let dot_product = Scalar::from_groups(/* scalar */ f32::powi(square_root[scalar], 2));
        let inverse = Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
        return SphereOnOrigin::from_groups(
            // e4235, e4315, e4125, e1234
            Simd32x4::from(inverse[scalar]) * Simd32x4::from([self[e4235], self[e4315], self[e4125], self[e1234]]),
        );
    }
}
