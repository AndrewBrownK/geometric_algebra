use crate::traits::FlatWeightNormSquared;
use crate::traits::RightAntiDual;
use crate::traits::RoundBulkNormSquared;
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
//  Minimum:         3       6       0
//   Median:         6       8       0
//  Average:        11      13       0
//  Maximum:        63      65       0
//
//  No SIMD:   add/sub     mul     div
//  Minimum:         3       6       0
//   Median:         6       8       0
//  Average:        11      13       0
//  Maximum:        63      65       0
impl std::ops::Div<center_norm_squared> for AntiCircleRotor {
    type Output = Scalar;
    fn div(self, _rhs: center_norm_squared) -> Self::Output {
        self.center_norm_squared()
    }
}
impl CenterNormSquared for AntiCircleRotor {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        4        7        0
    fn center_norm_squared(self) -> Scalar {
        use crate::elements::*;
        return Scalar::from_groups(/* scalar */ self.flat_weight_norm_squared().right_anti_dual()[scalar] + self.round_bulk_norm_squared()[scalar]);
    }
}
impl std::ops::Div<center_norm_squared> for AntiDipoleInversion {
    type Output = Scalar;
    fn div(self, _rhs: center_norm_squared) -> Self::Output {
        self.center_norm_squared()
    }
}
impl CenterNormSquared for AntiDipoleInversion {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        6        8        0
    fn center_norm_squared(self) -> Scalar {
        use crate::elements::*;
        return Scalar::from_groups(/* scalar */ self.flat_weight_norm_squared().right_anti_dual()[scalar] + self.round_bulk_norm_squared()[scalar]);
    }
}
impl std::ops::Div<center_norm_squared> for Circle {
    type Output = Scalar;
    fn div(self, _rhs: center_norm_squared) -> Self::Output {
        self.center_norm_squared()
    }
}
impl CenterNormSquared for Circle {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        6        0
    fn center_norm_squared(self) -> Scalar {
        use crate::elements::*;
        return Scalar::from_groups(/* scalar */ self.flat_weight_norm_squared().right_anti_dual()[scalar] + self.round_bulk_norm_squared()[scalar]);
    }
}
impl std::ops::Div<center_norm_squared> for CircleRotor {
    type Output = Scalar;
    fn div(self, _rhs: center_norm_squared) -> Self::Output {
        self.center_norm_squared()
    }
}
impl CenterNormSquared for CircleRotor {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        4        7        0
    fn center_norm_squared(self) -> Scalar {
        use crate::elements::*;
        return Scalar::from_groups(/* scalar */ self.flat_weight_norm_squared().right_anti_dual()[scalar] + self.round_bulk_norm_squared()[scalar]);
    }
}
impl std::ops::Div<center_norm_squared> for Dipole {
    type Output = Scalar;
    fn div(self, _rhs: center_norm_squared) -> Self::Output {
        self.center_norm_squared()
    }
}
impl CenterNormSquared for Dipole {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        6        0
    fn center_norm_squared(self) -> Scalar {
        use crate::elements::*;
        return Scalar::from_groups(/* scalar */ self.flat_weight_norm_squared().right_anti_dual()[scalar] + self.round_bulk_norm_squared()[scalar]);
    }
}
impl std::ops::Div<center_norm_squared> for DipoleInversion {
    type Output = Scalar;
    fn div(self, _rhs: center_norm_squared) -> Self::Output {
        self.center_norm_squared()
    }
}
impl CenterNormSquared for DipoleInversion {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        6        8        0
    fn center_norm_squared(self) -> Scalar {
        use crate::elements::*;
        return Scalar::from_groups(/* scalar */ self.flat_weight_norm_squared().right_anti_dual()[scalar] + self.round_bulk_norm_squared()[scalar]);
    }
}
impl std::ops::Div<center_norm_squared> for MultiVector {
    type Output = Scalar;
    fn div(self, _rhs: center_norm_squared) -> Self::Output {
        self.center_norm_squared()
    }
}
impl CenterNormSquared for MultiVector {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       63       65        0
    fn center_norm_squared(self) -> Scalar {
        use crate::elements::*;
        return Scalar::from_groups(/* scalar */ self.flat_weight_norm_squared().right_anti_dual()[scalar] + self.round_bulk_norm_squared()[scalar]);
    }
}
impl std::ops::Div<center_norm_squared> for VersorEven {
    type Output = Scalar;
    fn div(self, _rhs: center_norm_squared) -> Self::Output {
        self.center_norm_squared()
    }
}
impl CenterNormSquared for VersorEven {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        7        9        0
    fn center_norm_squared(self) -> Scalar {
        use crate::elements::*;
        return Scalar::from_groups(/* scalar */ self.flat_weight_norm_squared().right_anti_dual()[scalar] + self.round_bulk_norm_squared()[scalar]);
    }
}
impl std::ops::Div<center_norm_squared> for VersorOdd {
    type Output = Scalar;
    fn div(self, _rhs: center_norm_squared) -> Self::Output {
        self.center_norm_squared()
    }
}
impl CenterNormSquared for VersorOdd {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        7        9        0
    fn center_norm_squared(self) -> Scalar {
        use crate::elements::*;
        return Scalar::from_groups(/* scalar */ self.flat_weight_norm_squared().right_anti_dual()[scalar] + self.round_bulk_norm_squared()[scalar]);
    }
}
