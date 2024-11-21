// Note on Operative Statistics:
// Operative Statistics are not a precise predictor of performance or performance comparisons.
// This is due to varying hardware capabilities and compiler optimizations.
// As always, where performance is a concern, there is no substitute for
// real measurements on real work-loads on real hardware.
// Disclaimer aside, enjoy the fun information =)
//
// Total Implementations: 25
//
// Yes SIMD:   add/sub     mul     div
//  Minimum:         0       0       1
//   Median:         3       1       1
//  Average:         4       3       1
//  Maximum:        23      16       1
//
//  No SIMD:   add/sub     mul     div
//  Minimum:         0       0       1
//   Median:         3       1       1
//  Average:         4       3       1
//  Maximum:        23      16       1
impl std::ops::Div<anti_inverse> for AntiCircleRotor {
    type Output = AntiScalar;
    fn div(self, _rhs: anti_inverse) -> Self::Output {
        self.anti_inverse()
    }
}
impl AntiInverse for AntiCircleRotor {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        7        6        1
    fn anti_inverse(self) -> AntiScalar {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(
            // e12345
            2.0 * (self[e41] * self[e15])
                + 2.0 * (self[e42] * self[e25])
                + 2.0 * (self[e43] * self[e35])
                + f32::powi(self[e23], 2)
                + f32::powi(self[e31], 2)
                + f32::powi(self[e12], 2)
                - f32::powi(self[e45], 2)
                - f32::powi(self[scalar], 2),
        );
        return AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
    }
}
impl std::ops::Div<anti_inverse> for AntiDipoleInversion {
    type Output = AntiScalar;
    fn div(self, _rhs: anti_inverse) -> Self::Output {
        self.anti_inverse()
    }
}
impl AntiInverse for AntiDipoleInversion {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       10        8        1
    fn anti_inverse(self) -> AntiScalar {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(
            // e12345
            2.0 * (self[e4] * self[e5]) + f32::powi(self[e321], 2)
                - f32::powi(self[e415], 2)
                - f32::powi(self[e425], 2)
                - f32::powi(self[e435], 2)
                - f32::powi(self[e1], 2)
                - f32::powi(self[e2], 2)
                - f32::powi(self[e3], 2)
                - 2.0 * (self[e423] * self[e235])
                - 2.0 * (self[e431] * self[e315])
                - 2.0 * (self[e412] * self[e125]),
        );
        return AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
    }
}
impl std::ops::Div<anti_inverse> for AntiDualNum {
    type Output = AntiScalar;
    fn div(self, _rhs: anti_inverse) -> Self::Output {
        self.anti_inverse()
    }
}
impl AntiInverse for AntiDualNum {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        1
    fn anti_inverse(self) -> AntiScalar {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(/* e12345 */ f32::powi(self[scalar], 2) * -1.0);
        return AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
    }
}
impl std::ops::Div<anti_inverse> for AntiFlatPoint {
    type Output = AntiScalar;
    fn div(self, _rhs: anti_inverse) -> Self::Output {
        self.anti_inverse()
    }
}
impl AntiInverse for AntiFlatPoint {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        0        1
    fn anti_inverse(self) -> AntiScalar {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(/* e12345 */ f32::powi(self[e321], 2));
        return AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
    }
}
impl std::ops::Div<anti_inverse> for AntiFlector {
    type Output = AntiScalar;
    fn div(self, _rhs: anti_inverse) -> Self::Output {
        self.anti_inverse()
    }
}
impl AntiInverse for AntiFlector {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        0        1
    fn anti_inverse(self) -> AntiScalar {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(/* e12345 */ f32::powi(self[e321], 2) - f32::powi(self[e1], 2) - f32::powi(self[e2], 2) - f32::powi(self[e3], 2));
        return AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
    }
}
impl std::ops::Div<anti_inverse> for AntiLine {
    type Output = AntiScalar;
    fn div(self, _rhs: anti_inverse) -> Self::Output {
        self.anti_inverse()
    }
}
impl AntiInverse for AntiLine {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        0        1
    fn anti_inverse(self) -> AntiScalar {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(/* e12345 */ f32::powi(self[e23], 2) + f32::powi(self[e31], 2) + f32::powi(self[e12], 2));
        return AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
    }
}
impl std::ops::Div<anti_inverse> for AntiMotor {
    type Output = AntiScalar;
    fn div(self, _rhs: anti_inverse) -> Self::Output {
        self.anti_inverse()
    }
}
impl AntiInverse for AntiMotor {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        0        1
    fn anti_inverse(self) -> AntiScalar {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(
            // e12345
            f32::powi(self[e23], 2) + f32::powi(self[e31], 2) + f32::powi(self[e12], 2) - f32::powi(self[scalar], 2),
        );
        return AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
    }
}
impl std::ops::Div<anti_inverse> for AntiPlane {
    type Output = AntiScalar;
    fn div(self, _rhs: anti_inverse) -> Self::Output {
        self.anti_inverse()
    }
}
impl AntiInverse for AntiPlane {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        0        1
    fn anti_inverse(self) -> AntiScalar {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(/* e12345 */ -f32::powi(self[e1], 2) - f32::powi(self[e2], 2) - f32::powi(self[e3], 2));
        return AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
    }
}
impl std::ops::Div<anti_inverse> for AntiScalar {
    type Output = AntiScalar;
    fn div(self, _rhs: anti_inverse) -> Self::Output {
        self.anti_inverse()
    }
}
impl std::ops::DivAssign<anti_inverse> for AntiScalar {
    fn div_assign(&mut self, _rhs: anti_inverse) {
        *self = self.anti_inverse()
    }
}
impl AntiInverse for AntiScalar {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        0        1
    fn anti_inverse(self) -> AntiScalar {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(/* e12345 */ f32::powi(self[e12345], 2));
        return AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
    }
}
impl std::ops::Div<anti_inverse> for Circle {
    type Output = AntiScalar;
    fn div(self, _rhs: anti_inverse) -> Self::Output {
        self.anti_inverse()
    }
}
impl AntiInverse for Circle {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        6        6        1
    fn anti_inverse(self) -> AntiScalar {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(
            // e12345
            f32::powi(self[e321], 2)
                - f32::powi(self[e415], 2)
                - f32::powi(self[e425], 2)
                - f32::powi(self[e435], 2)
                - 2.0 * (self[e423] * self[e235])
                - 2.0 * (self[e431] * self[e315])
                - 2.0 * (self[e412] * self[e125]),
        );
        return AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
    }
}
impl std::ops::Div<anti_inverse> for CircleRotor {
    type Output = AntiScalar;
    fn div(self, _rhs: anti_inverse) -> Self::Output {
        self.anti_inverse()
    }
}
impl AntiInverse for CircleRotor {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        7        6        1
    fn anti_inverse(self) -> AntiScalar {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(
            // e12345
            f32::powi(self[e321], 2) + f32::powi(self[e12345], 2)
                - f32::powi(self[e415], 2)
                - f32::powi(self[e425], 2)
                - f32::powi(self[e435], 2)
                - 2.0 * (self[e423] * self[e235])
                - 2.0 * (self[e431] * self[e315])
                - 2.0 * (self[e412] * self[e125]),
        );
        return AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
    }
}
impl std::ops::Div<anti_inverse> for Dipole {
    type Output = AntiScalar;
    fn div(self, _rhs: anti_inverse) -> Self::Output {
        self.anti_inverse()
    }
}
impl AntiInverse for Dipole {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        6        6        1
    fn anti_inverse(self) -> AntiScalar {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(
            // e12345
            2.0 * (self[e41] * self[e15])
                + 2.0 * (self[e42] * self[e25])
                + 2.0 * (self[e43] * self[e35])
                + f32::powi(self[e23], 2)
                + f32::powi(self[e31], 2)
                + f32::powi(self[e12], 2)
                - f32::powi(self[e45], 2),
        );
        return AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
    }
}
impl std::ops::Div<anti_inverse> for DipoleInversion {
    type Output = AntiScalar;
    fn div(self, _rhs: anti_inverse) -> Self::Output {
        self.anti_inverse()
    }
}
impl AntiInverse for DipoleInversion {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       10        8        1
    fn anti_inverse(self) -> AntiScalar {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(
            // e12345
            2.0 * (self[e41] * self[e15])
                + 2.0 * (self[e42] * self[e25])
                + 2.0 * (self[e43] * self[e35])
                + f32::powi(self[e23], 2)
                + f32::powi(self[e31], 2)
                + f32::powi(self[e12], 2)
                + f32::powi(self[e4235], 2)
                + f32::powi(self[e4315], 2)
                + f32::powi(self[e4125], 2)
                - f32::powi(self[e45], 2)
                - 2.0 * (self[e1234] * self[e3215]),
        );
        return AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
    }
}
impl std::ops::Div<anti_inverse> for DualNum {
    type Output = AntiScalar;
    fn div(self, _rhs: anti_inverse) -> Self::Output {
        self.anti_inverse()
    }
}
impl AntiInverse for DualNum {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        0        1
    fn anti_inverse(self) -> AntiScalar {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(/* e12345 */ f32::powi(self[e12345], 2));
        return AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
    }
}
impl std::ops::Div<anti_inverse> for FlatPoint {
    type Output = AntiScalar;
    fn div(self, _rhs: anti_inverse) -> Self::Output {
        self.anti_inverse()
    }
}
impl AntiInverse for FlatPoint {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        1
    fn anti_inverse(self) -> AntiScalar {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(/* e12345 */ f32::powi(self[e45], 2) * -1.0);
        return AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
    }
}
impl std::ops::Div<anti_inverse> for Flector {
    type Output = AntiScalar;
    fn div(self, _rhs: anti_inverse) -> Self::Output {
        self.anti_inverse()
    }
}
impl AntiInverse for Flector {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        0        1
    fn anti_inverse(self) -> AntiScalar {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(
            // e12345
            f32::powi(self[e4235], 2) + f32::powi(self[e4315], 2) + f32::powi(self[e4125], 2) - f32::powi(self[e45], 2),
        );
        return AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
    }
}
impl std::ops::Div<anti_inverse> for Line {
    type Output = AntiScalar;
    fn div(self, _rhs: anti_inverse) -> Self::Output {
        self.anti_inverse()
    }
}
impl AntiInverse for Line {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        0        1
    fn anti_inverse(self) -> AntiScalar {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(/* e12345 */ -f32::powi(self[e415], 2) - f32::powi(self[e425], 2) - f32::powi(self[e435], 2));
        return AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
    }
}
impl std::ops::Div<anti_inverse> for Motor {
    type Output = AntiScalar;
    fn div(self, _rhs: anti_inverse) -> Self::Output {
        self.anti_inverse()
    }
}
impl AntiInverse for Motor {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        0        1
    fn anti_inverse(self) -> AntiScalar {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(
            // e12345
            f32::powi(self[e12345], 2) - f32::powi(self[e415], 2) - f32::powi(self[e425], 2) - f32::powi(self[e435], 2),
        );
        return AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
    }
}
impl std::ops::Div<anti_inverse> for MultiVector {
    type Output = AntiScalar;
    fn div(self, _rhs: anti_inverse) -> Self::Output {
        self.anti_inverse()
    }
}
impl AntiInverse for MultiVector {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       23       16        1
    fn anti_inverse(self) -> AntiScalar {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(
            // e12345
            2.0 * (self[e4] * self[e5])
                + 2.0 * (self[e15] * self[e41])
                + 2.0 * (self[e25] * self[e42])
                + 2.0 * (self[e35] * self[e43])
                + f32::powi(self[e12345], 2)
                + f32::powi(self[e23], 2)
                + f32::powi(self[e31], 2)
                + f32::powi(self[e12], 2)
                + f32::powi(self[e321], 2)
                + f32::powi(self[e4235], 2)
                + f32::powi(self[e4315], 2)
                + f32::powi(self[e4125], 2)
                - f32::powi(self[scalar], 2)
                - f32::powi(self[e1], 2)
                - f32::powi(self[e2], 2)
                - f32::powi(self[e3], 2)
                - f32::powi(self[e45], 2)
                - f32::powi(self[e415], 2)
                - f32::powi(self[e425], 2)
                - f32::powi(self[e435], 2)
                - 2.0 * (self[e423] * self[e235])
                - 2.0 * (self[e431] * self[e315])
                - 2.0 * (self[e412] * self[e125])
                - 2.0 * (self[e3215] * self[e1234]),
        );
        return AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
    }
}
impl std::ops::Div<anti_inverse> for Plane {
    type Output = AntiScalar;
    fn div(self, _rhs: anti_inverse) -> Self::Output {
        self.anti_inverse()
    }
}
impl AntiInverse for Plane {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        0        1
    fn anti_inverse(self) -> AntiScalar {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(/* e12345 */ f32::powi(self[e4235], 2) + f32::powi(self[e4315], 2) + f32::powi(self[e4125], 2));
        return AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
    }
}
impl std::ops::Div<anti_inverse> for RoundPoint {
    type Output = AntiScalar;
    fn div(self, _rhs: anti_inverse) -> Self::Output {
        self.anti_inverse()
    }
}
impl AntiInverse for RoundPoint {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        2        1
    fn anti_inverse(self) -> AntiScalar {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(/* e12345 */ 2.0 * (self[e4] * self[e5]) - f32::powi(self[e1], 2) - f32::powi(self[e2], 2) - f32::powi(self[e3], 2));
        return AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
    }
}
impl std::ops::Div<anti_inverse> for Scalar {
    type Output = AntiScalar;
    fn div(self, _rhs: anti_inverse) -> Self::Output {
        self.anti_inverse()
    }
}
impl AntiInverse for Scalar {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        1
    fn anti_inverse(self) -> AntiScalar {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(/* e12345 */ f32::powi(self[scalar], 2) * -1.0);
        return AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
    }
}
impl std::ops::Div<anti_inverse> for Sphere {
    type Output = AntiScalar;
    fn div(self, _rhs: anti_inverse) -> Self::Output {
        self.anti_inverse()
    }
}
impl AntiInverse for Sphere {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        2        1
    fn anti_inverse(self) -> AntiScalar {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(
            // e12345
            f32::powi(self[e4235], 2) + f32::powi(self[e4315], 2) + f32::powi(self[e4125], 2) - 2.0 * (self[e3215] * self[e1234]),
        );
        return AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
    }
}
impl std::ops::Div<anti_inverse> for VersorEven {
    type Output = AntiScalar;
    fn div(self, _rhs: anti_inverse) -> Self::Output {
        self.anti_inverse()
    }
}
impl AntiInverse for VersorEven {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       11        8        1
    fn anti_inverse(self) -> AntiScalar {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(
            // e12345
            2.0 * (self[e5] * self[e4]) + f32::powi(self[e12345], 2) + f32::powi(self[e321], 2)
                - f32::powi(self[e415], 2)
                - f32::powi(self[e425], 2)
                - f32::powi(self[e435], 2)
                - f32::powi(self[e1], 2)
                - f32::powi(self[e2], 2)
                - f32::powi(self[e3], 2)
                - 2.0 * (self[e423] * self[e235])
                - 2.0 * (self[e431] * self[e315])
                - 2.0 * (self[e412] * self[e125]),
        );
        return AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
    }
}
impl std::ops::Div<anti_inverse> for VersorOdd {
    type Output = AntiScalar;
    fn div(self, _rhs: anti_inverse) -> Self::Output {
        self.anti_inverse()
    }
}
impl AntiInverse for VersorOdd {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       11        8        1
    fn anti_inverse(self) -> AntiScalar {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(
            // e12345
            2.0 * (self[e41] * self[e15])
                + 2.0 * (self[e42] * self[e25])
                + 2.0 * (self[e43] * self[e35])
                + f32::powi(self[e23], 2)
                + f32::powi(self[e31], 2)
                + f32::powi(self[e12], 2)
                + f32::powi(self[e4235], 2)
                + f32::powi(self[e4315], 2)
                + f32::powi(self[e4125], 2)
                - f32::powi(self[scalar], 2)
                - f32::powi(self[e45], 2)
                - 2.0 * (self[e1234] * self[e3215]),
        );
        return AntiScalar::from_groups(/* e12345 */ 1.0 / anti_dot_product[e12345]);
    }
}
