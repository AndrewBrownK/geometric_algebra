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
impl std::ops::Div<inverse> for DualNum {
    type Output = Scalar;
    fn div(self, _rhs: inverse) -> Self::Output {
        self.inverse()
    }
}
impl Inverse for DualNum {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        0        1
    fn inverse(self) -> Scalar {
        use crate::elements::*;
        let scalar_product = Scalar::from_groups(/* scalar */ f32::powi(self[scalar], 2));
        return Scalar::from_groups(/* scalar */ 1.0 / scalar_product[scalar]);
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
        let scalar_product = Scalar::from_groups(/* scalar */ f32::powi(self[e1], 2) + f32::powi(self[e2], 2) + f32::powi(self[e3], 2) - f32::powi(self[e321], 2));
        return Scalar::from_groups(/* scalar */ 1.0 / scalar_product[scalar]);
    }
}
impl std::ops::Div<inverse> for Horizon {
    type Output = Scalar;
    fn div(self, _rhs: inverse) -> Self::Output {
        self.inverse()
    }
}
impl Inverse for Horizon {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        1
    fn inverse(self) -> Scalar {
        use crate::elements::*;
        let scalar_product = Scalar::from_groups(/* scalar */ f32::powi(self[e321], 2) * -1.0);
        return Scalar::from_groups(/* scalar */ 1.0 / scalar_product[scalar]);
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
        let scalar_product = Scalar::from_groups(/* scalar */ -f32::powi(self[e23], 2) - f32::powi(self[e31], 2) - f32::powi(self[e12], 2));
        return Scalar::from_groups(/* scalar */ 1.0 / scalar_product[scalar]);
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
        let scalar_product = Scalar::from_groups(
            // scalar
            f32::powi(self[scalar], 2) - f32::powi(self[e23], 2) - f32::powi(self[e31], 2) - f32::powi(self[e12], 2),
        );
        return Scalar::from_groups(/* scalar */ 1.0 / scalar_product[scalar]);
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
    // f32        7        0        1
    fn inverse(self) -> Scalar {
        use crate::elements::*;
        let scalar_product = Scalar::from_groups(
            // scalar
            f32::powi(self[scalar], 2) + f32::powi(self[e1], 2) + f32::powi(self[e2], 2) + f32::powi(self[e3], 2)
                - f32::powi(self[e23], 2)
                - f32::powi(self[e31], 2)
                - f32::powi(self[e12], 2)
                - f32::powi(self[e321], 2),
        );
        return Scalar::from_groups(/* scalar */ 1.0 / scalar_product[scalar]);
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
    // f32        0        1        1
    fn inverse(self) -> Scalar {
        use crate::elements::*;
        let scalar_product = Scalar::from_groups(/* scalar */ f32::powi(self[e321], 2) * -1.0);
        return Scalar::from_groups(/* scalar */ 1.0 / scalar_product[scalar]);
    }
}
impl std::ops::Div<inverse> for Point {
    type Output = Scalar;
    fn div(self, _rhs: inverse) -> Self::Output {
        self.inverse()
    }
}
impl Inverse for Point {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        0        1
    fn inverse(self) -> Scalar {
        use crate::elements::*;
        let scalar_product = Scalar::from_groups(/* scalar */ f32::powi(self[e1], 2) + f32::powi(self[e2], 2) + f32::powi(self[e3], 2));
        return Scalar::from_groups(/* scalar */ 1.0 / scalar_product[scalar]);
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
        let scalar_product = Scalar::from_groups(/* scalar */ f32::powi(self[scalar], 2));
        return Scalar::from_groups(/* scalar */ 1.0 / scalar_product[scalar]);
    }
}
