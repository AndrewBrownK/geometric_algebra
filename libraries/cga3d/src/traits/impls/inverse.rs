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
impl std::ops::Div<inverse> for AntiCircleRotor {
    type Output = Scalar;
    fn div(self, _rhs: inverse) -> Self::Output {
        self.inverse()
    }
}
impl Inverse for AntiCircleRotor {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        7        6        1
    fn inverse(self) -> Scalar {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(
            // scalar
            f32::powi(self[e45], 2) + f32::powi(self[scalar], 2)
                - f32::powi(self[e23], 2)
                - f32::powi(self[e31], 2)
                - f32::powi(self[e12], 2)
                - 2.0 * (self[e41] * self[e15])
                - 2.0 * (self[e42] * self[e25])
                - 2.0 * (self[e43] * self[e35]),
        );
        return Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
    }
}
impl std::ops::Div<inverse> for AntiDipoleInversion {
    type Output = Scalar;
    fn div(self, _rhs: inverse) -> Self::Output {
        self.inverse()
    }
}
impl Inverse for AntiDipoleInversion {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       10        8        1
    fn inverse(self) -> Scalar {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(
            // scalar
            2.0 * (self[e423] * self[e235])
                + 2.0 * (self[e431] * self[e315])
                + 2.0 * (self[e412] * self[e125])
                + f32::powi(self[e415], 2)
                + f32::powi(self[e425], 2)
                + f32::powi(self[e435], 2)
                + f32::powi(self[e1], 2)
                + f32::powi(self[e2], 2)
                + f32::powi(self[e3], 2)
                - f32::powi(self[e321], 2)
                - 2.0 * (self[e4] * self[e5]),
        );
        return Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
    }
}
impl std::ops::Div<inverse> for AntiDualNum {
    type Output = Scalar;
    fn div(self, _rhs: inverse) -> Self::Output {
        self.inverse()
    }
}
impl Inverse for AntiDualNum {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        0        1
    fn inverse(self) -> Scalar {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(/* scalar */ f32::powi(self[scalar], 2));
        return Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
    }
}
impl std::ops::Div<inverse> for AntiFlatPoint {
    type Output = Scalar;
    fn div(self, _rhs: inverse) -> Self::Output {
        self.inverse()
    }
}
impl Inverse for AntiFlatPoint {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        1
    fn inverse(self) -> Scalar {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(/* scalar */ f32::powi(self[e321], 2) * -1.0);
        return Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
    }
}
impl std::ops::Div<inverse> for AntiFlector {
    type Output = Scalar;
    fn div(self, _rhs: inverse) -> Self::Output {
        self.inverse()
    }
}
impl Inverse for AntiFlector {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        0        1
    fn inverse(self) -> Scalar {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(/* scalar */ f32::powi(self[e1], 2) + f32::powi(self[e2], 2) + f32::powi(self[e3], 2) - f32::powi(self[e321], 2));
        return Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
    }
}
impl std::ops::Div<inverse> for AntiLine {
    type Output = Scalar;
    fn div(self, _rhs: inverse) -> Self::Output {
        self.inverse()
    }
}
impl Inverse for AntiLine {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        0        1
    fn inverse(self) -> Scalar {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(/* scalar */ -f32::powi(self[e23], 2) - f32::powi(self[e31], 2) - f32::powi(self[e12], 2));
        return Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
    }
}
impl std::ops::Div<inverse> for AntiMotor {
    type Output = Scalar;
    fn div(self, _rhs: inverse) -> Self::Output {
        self.inverse()
    }
}
impl Inverse for AntiMotor {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        0        1
    fn inverse(self) -> Scalar {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(
            // scalar
            f32::powi(self[scalar], 2) - f32::powi(self[e23], 2) - f32::powi(self[e31], 2) - f32::powi(self[e12], 2),
        );
        return Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
    }
}
impl std::ops::Div<inverse> for AntiPlane {
    type Output = Scalar;
    fn div(self, _rhs: inverse) -> Self::Output {
        self.inverse()
    }
}
impl Inverse for AntiPlane {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        0        1
    fn inverse(self) -> Scalar {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(/* scalar */ f32::powi(self[e1], 2) + f32::powi(self[e2], 2) + f32::powi(self[e3], 2));
        return Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
    }
}
impl std::ops::Div<inverse> for AntiScalar {
    type Output = Scalar;
    fn div(self, _rhs: inverse) -> Self::Output {
        self.inverse()
    }
}
impl Inverse for AntiScalar {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        1
    fn inverse(self) -> Scalar {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(/* scalar */ f32::powi(self[e12345], 2) * -1.0);
        return Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
    }
}
impl std::ops::Div<inverse> for Circle {
    type Output = Scalar;
    fn div(self, _rhs: inverse) -> Self::Output {
        self.inverse()
    }
}
impl Inverse for Circle {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        6        6        1
    fn inverse(self) -> Scalar {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(
            // scalar
            2.0 * (self[e423] * self[e235])
                + 2.0 * (self[e431] * self[e315])
                + 2.0 * (self[e412] * self[e125])
                + f32::powi(self[e415], 2)
                + f32::powi(self[e425], 2)
                + f32::powi(self[e435], 2)
                - f32::powi(self[e321], 2),
        );
        return Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
    }
}
impl std::ops::Div<inverse> for CircleRotor {
    type Output = Scalar;
    fn div(self, _rhs: inverse) -> Self::Output {
        self.inverse()
    }
}
impl Inverse for CircleRotor {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        7        6        1
    fn inverse(self) -> Scalar {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(
            // scalar
            2.0 * (self[e423] * self[e235])
                + 2.0 * (self[e431] * self[e315])
                + 2.0 * (self[e412] * self[e125])
                + f32::powi(self[e415], 2)
                + f32::powi(self[e425], 2)
                + f32::powi(self[e435], 2)
                - f32::powi(self[e321], 2)
                - f32::powi(self[e12345], 2),
        );
        return Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
    }
}
impl std::ops::Div<inverse> for Dipole {
    type Output = Scalar;
    fn div(self, _rhs: inverse) -> Self::Output {
        self.inverse()
    }
}
impl Inverse for Dipole {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        6        6        1
    fn inverse(self) -> Scalar {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(
            // scalar
            f32::powi(self[e45], 2)
                - f32::powi(self[e23], 2)
                - f32::powi(self[e31], 2)
                - f32::powi(self[e12], 2)
                - 2.0 * (self[e41] * self[e15])
                - 2.0 * (self[e42] * self[e25])
                - 2.0 * (self[e43] * self[e35]),
        );
        return Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
    }
}
impl std::ops::Div<inverse> for DipoleInversion {
    type Output = Scalar;
    fn div(self, _rhs: inverse) -> Self::Output {
        self.inverse()
    }
}
impl Inverse for DipoleInversion {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       10        8        1
    fn inverse(self) -> Scalar {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(
            // scalar
            2.0 * (self[e1234] * self[e3215]) + f32::powi(self[e45], 2)
                - f32::powi(self[e23], 2)
                - f32::powi(self[e31], 2)
                - f32::powi(self[e12], 2)
                - f32::powi(self[e4235], 2)
                - f32::powi(self[e4315], 2)
                - f32::powi(self[e4125], 2)
                - 2.0 * (self[e41] * self[e15])
                - 2.0 * (self[e42] * self[e25])
                - 2.0 * (self[e43] * self[e35]),
        );
        return Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
    }
}
impl std::ops::Div<inverse> for DualNum {
    type Output = Scalar;
    fn div(self, _rhs: inverse) -> Self::Output {
        self.inverse()
    }
}
impl Inverse for DualNum {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        1
    fn inverse(self) -> Scalar {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(/* scalar */ f32::powi(self[e12345], 2) * -1.0);
        return Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
    }
}
impl std::ops::Div<inverse> for FlatPoint {
    type Output = Scalar;
    fn div(self, _rhs: inverse) -> Self::Output {
        self.inverse()
    }
}
impl Inverse for FlatPoint {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        0        1
    fn inverse(self) -> Scalar {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(/* scalar */ f32::powi(self[e45], 2));
        return Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
    }
}
impl std::ops::Div<inverse> for Flector {
    type Output = Scalar;
    fn div(self, _rhs: inverse) -> Self::Output {
        self.inverse()
    }
}
impl Inverse for Flector {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        0        1
    fn inverse(self) -> Scalar {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(
            // scalar
            f32::powi(self[e45], 2) - f32::powi(self[e4235], 2) - f32::powi(self[e4315], 2) - f32::powi(self[e4125], 2),
        );
        return Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
    }
}
impl std::ops::Div<inverse> for Line {
    type Output = Scalar;
    fn div(self, _rhs: inverse) -> Self::Output {
        self.inverse()
    }
}
impl Inverse for Line {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        0        1
    fn inverse(self) -> Scalar {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(/* scalar */ f32::powi(self[e415], 2) + f32::powi(self[e425], 2) + f32::powi(self[e435], 2));
        return Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
    }
}
impl std::ops::Div<inverse> for Motor {
    type Output = Scalar;
    fn div(self, _rhs: inverse) -> Self::Output {
        self.inverse()
    }
}
impl Inverse for Motor {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        0        1
    fn inverse(self) -> Scalar {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(
            // scalar
            f32::powi(self[e415], 2) + f32::powi(self[e425], 2) + f32::powi(self[e435], 2) - f32::powi(self[e12345], 2),
        );
        return Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
    }
}
impl std::ops::Div<inverse> for MultiVector {
    type Output = Scalar;
    fn div(self, _rhs: inverse) -> Self::Output {
        self.inverse()
    }
}
impl Inverse for MultiVector {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       23       16        1
    fn inverse(self) -> Scalar {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(
            // scalar
            2.0 * (self[e423] * self[e235])
                + 2.0 * (self[e431] * self[e315])
                + 2.0 * (self[e412] * self[e125])
                + 2.0 * (self[e3215] * self[e1234])
                + f32::powi(self[scalar], 2)
                + f32::powi(self[e1], 2)
                + f32::powi(self[e2], 2)
                + f32::powi(self[e3], 2)
                + f32::powi(self[e45], 2)
                + f32::powi(self[e415], 2)
                + f32::powi(self[e425], 2)
                + f32::powi(self[e435], 2)
                - f32::powi(self[e12345], 2)
                - f32::powi(self[e23], 2)
                - f32::powi(self[e31], 2)
                - f32::powi(self[e12], 2)
                - f32::powi(self[e321], 2)
                - f32::powi(self[e4235], 2)
                - f32::powi(self[e4315], 2)
                - f32::powi(self[e4125], 2)
                - 2.0 * (self[e4] * self[e5])
                - 2.0 * (self[e15] * self[e41])
                - 2.0 * (self[e25] * self[e42])
                - 2.0 * (self[e35] * self[e43]),
        );
        return Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
    }
}
impl std::ops::Div<inverse> for Plane {
    type Output = Scalar;
    fn div(self, _rhs: inverse) -> Self::Output {
        self.inverse()
    }
}
impl Inverse for Plane {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        0        1
    fn inverse(self) -> Scalar {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(/* scalar */ -f32::powi(self[e4235], 2) - f32::powi(self[e4315], 2) - f32::powi(self[e4125], 2));
        return Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
    }
}
impl std::ops::Div<inverse> for RoundPoint {
    type Output = Scalar;
    fn div(self, _rhs: inverse) -> Self::Output {
        self.inverse()
    }
}
impl Inverse for RoundPoint {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        2        1
    fn inverse(self) -> Scalar {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(/* scalar */ f32::powi(self[e1], 2) + f32::powi(self[e2], 2) + f32::powi(self[e3], 2) - 2.0 * (self[e4] * self[e5]));
        return Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
    }
}
impl std::ops::Div<inverse> for Scalar {
    type Output = Scalar;
    fn div(self, _rhs: inverse) -> Self::Output {
        self.inverse()
    }
}
impl std::ops::DivAssign<inverse> for Scalar {
    fn div_assign(&mut self, _rhs: inverse) {
        *self = self.inverse()
    }
}
impl Inverse for Scalar {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        0        1
    fn inverse(self) -> Scalar {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(/* scalar */ f32::powi(self[scalar], 2));
        return Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
    }
}
impl std::ops::Div<inverse> for Sphere {
    type Output = Scalar;
    fn div(self, _rhs: inverse) -> Self::Output {
        self.inverse()
    }
}
impl Inverse for Sphere {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        2        1
    fn inverse(self) -> Scalar {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(
            // scalar
            2.0 * (self[e3215] * self[e1234]) - f32::powi(self[e4235], 2) - f32::powi(self[e4315], 2) - f32::powi(self[e4125], 2),
        );
        return Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
    }
}
impl std::ops::Div<inverse> for VersorEven {
    type Output = Scalar;
    fn div(self, _rhs: inverse) -> Self::Output {
        self.inverse()
    }
}
impl Inverse for VersorEven {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       11        8        1
    fn inverse(self) -> Scalar {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(
            // scalar
            2.0 * (self[e423] * self[e235])
                + 2.0 * (self[e431] * self[e315])
                + 2.0 * (self[e412] * self[e125])
                + f32::powi(self[e415], 2)
                + f32::powi(self[e425], 2)
                + f32::powi(self[e435], 2)
                + f32::powi(self[e1], 2)
                + f32::powi(self[e2], 2)
                + f32::powi(self[e3], 2)
                - f32::powi(self[e12345], 2)
                - f32::powi(self[e321], 2)
                - 2.0 * (self[e5] * self[e4]),
        );
        return Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
    }
}
impl std::ops::Div<inverse> for VersorOdd {
    type Output = Scalar;
    fn div(self, _rhs: inverse) -> Self::Output {
        self.inverse()
    }
}
impl Inverse for VersorOdd {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       11        8        1
    fn inverse(self) -> Scalar {
        use crate::elements::*;
        let dot_product = Scalar::from_groups(
            // scalar
            2.0 * (self[e1234] * self[e3215]) + f32::powi(self[scalar], 2) + f32::powi(self[e45], 2)
                - f32::powi(self[e23], 2)
                - f32::powi(self[e31], 2)
                - f32::powi(self[e12], 2)
                - f32::powi(self[e4235], 2)
                - f32::powi(self[e4315], 2)
                - f32::powi(self[e4125], 2)
                - 2.0 * (self[e41] * self[e15])
                - 2.0 * (self[e42] * self[e25])
                - 2.0 * (self[e43] * self[e35]),
        );
        return Scalar::from_groups(/* scalar */ 1.0 / dot_product[scalar]);
    }
}
