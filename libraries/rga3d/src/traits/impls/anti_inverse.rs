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
        let anti_dot_product = AntiScalar::from_groups(/* e1234 */ f32::powi(self[e1234], 2));
        return AntiScalar::from_groups(/* e1234 */ 1.0 / anti_dot_product[e1234]);
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
        let anti_dot_product = AntiScalar::from_groups(/* e1234 */ f32::powi(self[e1234], 2));
        return AntiScalar::from_groups(/* e1234 */ 1.0 / anti_dot_product[e1234]);
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
        let anti_dot_product = AntiScalar::from_groups(/* e1234 */ f32::powi(self[e423], 2) + f32::powi(self[e431], 2) + f32::powi(self[e412], 2) - f32::powi(self[e4], 2));
        return AntiScalar::from_groups(/* e1234 */ 1.0 / anti_dot_product[e1234]);
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
        let anti_dot_product = AntiScalar::from_groups(/* e1234 */ -f32::powi(self[e41], 2) - f32::powi(self[e42], 2) - f32::powi(self[e43], 2));
        return AntiScalar::from_groups(/* e1234 */ 1.0 / anti_dot_product[e1234]);
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
        let anti_dot_product = AntiScalar::from_groups(/* e1234 */ f32::powi(self[e1234], 2) - f32::powi(self[e41], 2) - f32::powi(self[e42], 2) - f32::powi(self[e43], 2));
        return AntiScalar::from_groups(/* e1234 */ 1.0 / anti_dot_product[e1234]);
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
    // f32        7        0        1
    fn anti_inverse(self) -> AntiScalar {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(
            // e1234
            f32::powi(self[e1234], 2) + f32::powi(self[e423], 2) + f32::powi(self[e431], 2) + f32::powi(self[e412], 2)
                - f32::powi(self[e4], 2)
                - f32::powi(self[e41], 2)
                - f32::powi(self[e42], 2)
                - f32::powi(self[e43], 2),
        );
        return AntiScalar::from_groups(/* e1234 */ 1.0 / anti_dot_product[e1234]);
    }
}
impl std::ops::Div<anti_inverse> for Origin {
    type Output = AntiScalar;
    fn div(self, _rhs: anti_inverse) -> Self::Output {
        self.anti_inverse()
    }
}
impl AntiInverse for Origin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        1
    fn anti_inverse(self) -> AntiScalar {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(/* e1234 */ f32::powi(self[e4], 2) * -1.0);
        return AntiScalar::from_groups(/* e1234 */ 1.0 / anti_dot_product[e1234]);
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
        let anti_dot_product = AntiScalar::from_groups(/* e1234 */ f32::powi(self[e423], 2) + f32::powi(self[e431], 2) + f32::powi(self[e412], 2));
        return AntiScalar::from_groups(/* e1234 */ 1.0 / anti_dot_product[e1234]);
    }
}
impl std::ops::Div<anti_inverse> for Point {
    type Output = AntiScalar;
    fn div(self, _rhs: anti_inverse) -> Self::Output {
        self.anti_inverse()
    }
}
impl AntiInverse for Point {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        1
    fn anti_inverse(self) -> AntiScalar {
        use crate::elements::*;
        let anti_dot_product = AntiScalar::from_groups(/* e1234 */ f32::powi(self[e4], 2) * -1.0);
        return AntiScalar::from_groups(/* e1234 */ 1.0 / anti_dot_product[e1234]);
    }
}
