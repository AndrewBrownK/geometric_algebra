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
// Total Implementations: 25
//
// Yes SIMD:   add/sub     mul     div
//  Minimum:         3       5       0
//   Median:         6       8       0
//  Average:         7       9       0
//  Maximum:        63      65       0
//
//  No SIMD:   add/sub     mul     div
//  Minimum:         3       5       0
//   Median:         6       8       0
//  Average:         7       9       0
//  Maximum:        63      65       0
impl std::ops::Div<CenterNormSquaredPrefixOrPostfix> for AntiCircleRotor {
    type Output = Scalar;
    fn div(self, _rhs: CenterNormSquaredPrefixOrPostfix) -> Self::Output {
        self.center_norm_squared()
    }
}
impl CenterNormSquared for AntiCircleRotor {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        4        6        0
    fn center_norm_squared(self) -> Scalar {
        use crate::elements::*;
        return Scalar::from_groups(/* scalar */ self.flat_weight_norm_squared().right_anti_dual()[scalar] + self.round_bulk_norm_squared()[scalar]);
    }
}
impl std::ops::Div<CenterNormSquaredPrefixOrPostfix> for AntiCircleRotorAtInfinity {
    type Output = Scalar;
    fn div(self, _rhs: CenterNormSquaredPrefixOrPostfix) -> Self::Output {
        self.center_norm_squared()
    }
}
impl CenterNormSquared for AntiCircleRotorAtInfinity {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        4        6        0
    fn center_norm_squared(self) -> Scalar {
        use crate::elements::*;
        return Scalar::from_groups(/* scalar */ self.flat_weight_norm_squared().right_anti_dual()[scalar] + self.round_bulk_norm_squared()[scalar]);
    }
}
impl std::ops::Div<CenterNormSquaredPrefixOrPostfix> for AntiDipoleInversion {
    type Output = Scalar;
    fn div(self, _rhs: CenterNormSquaredPrefixOrPostfix) -> Self::Output {
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
impl std::ops::Div<CenterNormSquaredPrefixOrPostfix> for AntiDipoleInversionAtInfinity {
    type Output = Scalar;
    fn div(self, _rhs: CenterNormSquaredPrefixOrPostfix) -> Self::Output {
        self.center_norm_squared()
    }
}
impl CenterNormSquared for AntiDipoleInversionAtInfinity {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        6        8        0
    fn center_norm_squared(self) -> Scalar {
        use crate::elements::*;
        return Scalar::from_groups(/* scalar */ self.flat_weight_norm_squared().right_anti_dual()[scalar] + self.round_bulk_norm_squared()[scalar]);
    }
}
impl std::ops::Div<CenterNormSquaredPrefixOrPostfix> for AntiMysteryCircleRotor {
    type Output = Scalar;
    fn div(self, _rhs: CenterNormSquaredPrefixOrPostfix) -> Self::Output {
        self.center_norm_squared()
    }
}
impl CenterNormSquared for AntiMysteryCircleRotor {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        4        6        0
    fn center_norm_squared(self) -> Scalar {
        use crate::elements::*;
        return Scalar::from_groups(/* scalar */ self.flat_weight_norm_squared().right_anti_dual()[scalar] + self.round_bulk_norm_squared()[scalar]);
    }
}
impl std::ops::Div<CenterNormSquaredPrefixOrPostfix> for AntiMysteryDipoleInversion {
    type Output = Scalar;
    fn div(self, _rhs: CenterNormSquaredPrefixOrPostfix) -> Self::Output {
        self.center_norm_squared()
    }
}
impl CenterNormSquared for AntiMysteryDipoleInversion {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        6        8        0
    fn center_norm_squared(self) -> Scalar {
        use crate::elements::*;
        return Scalar::from_groups(/* scalar */ self.flat_weight_norm_squared().right_anti_dual()[scalar] + self.round_bulk_norm_squared()[scalar]);
    }
}
impl std::ops::Div<CenterNormSquaredPrefixOrPostfix> for Circle {
    type Output = Scalar;
    fn div(self, _rhs: CenterNormSquaredPrefixOrPostfix) -> Self::Output {
        self.center_norm_squared()
    }
}
impl CenterNormSquared for Circle {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        5        0
    fn center_norm_squared(self) -> Scalar {
        use crate::elements::*;
        return Scalar::from_groups(/* scalar */ self.flat_weight_norm_squared().right_anti_dual()[scalar] + self.round_bulk_norm_squared()[scalar]);
    }
}
impl std::ops::Div<CenterNormSquaredPrefixOrPostfix> for CircleAtInfinity {
    type Output = Scalar;
    fn div(self, _rhs: CenterNormSquaredPrefixOrPostfix) -> Self::Output {
        self.center_norm_squared()
    }
}
impl CenterNormSquared for CircleAtInfinity {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        5        0
    fn center_norm_squared(self) -> Scalar {
        use crate::elements::*;
        return Scalar::from_groups(/* scalar */ self.flat_weight_norm_squared().right_anti_dual()[scalar] + self.round_bulk_norm_squared()[scalar]);
    }
}
impl std::ops::Div<CenterNormSquaredPrefixOrPostfix> for CircleRotor {
    type Output = Scalar;
    fn div(self, _rhs: CenterNormSquaredPrefixOrPostfix) -> Self::Output {
        self.center_norm_squared()
    }
}
impl CenterNormSquared for CircleRotor {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        4        6        0
    fn center_norm_squared(self) -> Scalar {
        use crate::elements::*;
        return Scalar::from_groups(/* scalar */ self.flat_weight_norm_squared().right_anti_dual()[scalar] + self.round_bulk_norm_squared()[scalar]);
    }
}
impl std::ops::Div<CenterNormSquaredPrefixOrPostfix> for CircleRotorAtInfinity {
    type Output = Scalar;
    fn div(self, _rhs: CenterNormSquaredPrefixOrPostfix) -> Self::Output {
        self.center_norm_squared()
    }
}
impl CenterNormSquared for CircleRotorAtInfinity {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        4        6        0
    fn center_norm_squared(self) -> Scalar {
        use crate::elements::*;
        return Scalar::from_groups(/* scalar */ self.flat_weight_norm_squared().right_anti_dual()[scalar] + self.round_bulk_norm_squared()[scalar]);
    }
}
impl std::ops::Div<CenterNormSquaredPrefixOrPostfix> for Dipole {
    type Output = Scalar;
    fn div(self, _rhs: CenterNormSquaredPrefixOrPostfix) -> Self::Output {
        self.center_norm_squared()
    }
}
impl CenterNormSquared for Dipole {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        5        0
    fn center_norm_squared(self) -> Scalar {
        use crate::elements::*;
        return Scalar::from_groups(/* scalar */ self.flat_weight_norm_squared().right_anti_dual()[scalar] + self.round_bulk_norm_squared()[scalar]);
    }
}
impl std::ops::Div<CenterNormSquaredPrefixOrPostfix> for DipoleAtInfinity {
    type Output = Scalar;
    fn div(self, _rhs: CenterNormSquaredPrefixOrPostfix) -> Self::Output {
        self.center_norm_squared()
    }
}
impl CenterNormSquared for DipoleAtInfinity {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        5        0
    fn center_norm_squared(self) -> Scalar {
        use crate::elements::*;
        return Scalar::from_groups(/* scalar */ self.flat_weight_norm_squared().right_anti_dual()[scalar] + self.round_bulk_norm_squared()[scalar]);
    }
}
impl std::ops::Div<CenterNormSquaredPrefixOrPostfix> for DipoleInversion {
    type Output = Scalar;
    fn div(self, _rhs: CenterNormSquaredPrefixOrPostfix) -> Self::Output {
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
impl std::ops::Div<CenterNormSquaredPrefixOrPostfix> for DipoleInversionAtInfinity {
    type Output = Scalar;
    fn div(self, _rhs: CenterNormSquaredPrefixOrPostfix) -> Self::Output {
        self.center_norm_squared()
    }
}
impl CenterNormSquared for DipoleInversionAtInfinity {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        6        8        0
    fn center_norm_squared(self) -> Scalar {
        use crate::elements::*;
        return Scalar::from_groups(/* scalar */ self.flat_weight_norm_squared().right_anti_dual()[scalar] + self.round_bulk_norm_squared()[scalar]);
    }
}
impl std::ops::Div<CenterNormSquaredPrefixOrPostfix> for MultiVector {
    type Output = Scalar;
    fn div(self, _rhs: CenterNormSquaredPrefixOrPostfix) -> Self::Output {
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
impl std::ops::Div<CenterNormSquaredPrefixOrPostfix> for MysteryCircle {
    type Output = Scalar;
    fn div(self, _rhs: CenterNormSquaredPrefixOrPostfix) -> Self::Output {
        self.center_norm_squared()
    }
}
impl CenterNormSquared for MysteryCircle {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        5        0
    fn center_norm_squared(self) -> Scalar {
        use crate::elements::*;
        return Scalar::from_groups(/* scalar */ self.flat_weight_norm_squared().right_anti_dual()[scalar] + self.round_bulk_norm_squared()[scalar]);
    }
}
impl std::ops::Div<CenterNormSquaredPrefixOrPostfix> for MysteryCircleRotor {
    type Output = Scalar;
    fn div(self, _rhs: CenterNormSquaredPrefixOrPostfix) -> Self::Output {
        self.center_norm_squared()
    }
}
impl CenterNormSquared for MysteryCircleRotor {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        4        6        0
    fn center_norm_squared(self) -> Scalar {
        use crate::elements::*;
        return Scalar::from_groups(/* scalar */ self.flat_weight_norm_squared().right_anti_dual()[scalar] + self.round_bulk_norm_squared()[scalar]);
    }
}
impl std::ops::Div<CenterNormSquaredPrefixOrPostfix> for MysteryDipole {
    type Output = Scalar;
    fn div(self, _rhs: CenterNormSquaredPrefixOrPostfix) -> Self::Output {
        self.center_norm_squared()
    }
}
impl CenterNormSquared for MysteryDipole {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        5        0
    fn center_norm_squared(self) -> Scalar {
        use crate::elements::*;
        return Scalar::from_groups(/* scalar */ self.flat_weight_norm_squared().right_anti_dual()[scalar] + self.round_bulk_norm_squared()[scalar]);
    }
}
impl std::ops::Div<CenterNormSquaredPrefixOrPostfix> for MysteryDipoleInversion {
    type Output = Scalar;
    fn div(self, _rhs: CenterNormSquaredPrefixOrPostfix) -> Self::Output {
        self.center_norm_squared()
    }
}
impl CenterNormSquared for MysteryDipoleInversion {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        6        8        0
    fn center_norm_squared(self) -> Scalar {
        use crate::elements::*;
        return Scalar::from_groups(/* scalar */ self.flat_weight_norm_squared().right_anti_dual()[scalar] + self.round_bulk_norm_squared()[scalar]);
    }
}
impl std::ops::Div<CenterNormSquaredPrefixOrPostfix> for MysteryVersorEven {
    type Output = Scalar;
    fn div(self, _rhs: CenterNormSquaredPrefixOrPostfix) -> Self::Output {
        self.center_norm_squared()
    }
}
impl CenterNormSquared for MysteryVersorEven {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        7        9        0
    fn center_norm_squared(self) -> Scalar {
        use crate::elements::*;
        return Scalar::from_groups(/* scalar */ self.flat_weight_norm_squared().right_anti_dual()[scalar] + self.round_bulk_norm_squared()[scalar]);
    }
}
impl std::ops::Div<CenterNormSquaredPrefixOrPostfix> for MysteryVersorOdd {
    type Output = Scalar;
    fn div(self, _rhs: CenterNormSquaredPrefixOrPostfix) -> Self::Output {
        self.center_norm_squared()
    }
}
impl CenterNormSquared for MysteryVersorOdd {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        7        9        0
    fn center_norm_squared(self) -> Scalar {
        use crate::elements::*;
        return Scalar::from_groups(/* scalar */ self.flat_weight_norm_squared().right_anti_dual()[scalar] + self.round_bulk_norm_squared()[scalar]);
    }
}
impl std::ops::Div<CenterNormSquaredPrefixOrPostfix> for VersorEven {
    type Output = Scalar;
    fn div(self, _rhs: CenterNormSquaredPrefixOrPostfix) -> Self::Output {
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
impl std::ops::Div<CenterNormSquaredPrefixOrPostfix> for VersorEvenAtInfinity {
    type Output = Scalar;
    fn div(self, _rhs: CenterNormSquaredPrefixOrPostfix) -> Self::Output {
        self.center_norm_squared()
    }
}
impl CenterNormSquared for VersorEvenAtInfinity {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        7        9        0
    fn center_norm_squared(self) -> Scalar {
        use crate::elements::*;
        return Scalar::from_groups(/* scalar */ self.flat_weight_norm_squared().right_anti_dual()[scalar] + self.round_bulk_norm_squared()[scalar]);
    }
}
impl std::ops::Div<CenterNormSquaredPrefixOrPostfix> for VersorOdd {
    type Output = Scalar;
    fn div(self, _rhs: CenterNormSquaredPrefixOrPostfix) -> Self::Output {
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
impl std::ops::Div<CenterNormSquaredPrefixOrPostfix> for VersorOddAtInfinity {
    type Output = Scalar;
    fn div(self, _rhs: CenterNormSquaredPrefixOrPostfix) -> Self::Output {
        self.center_norm_squared()
    }
}
impl CenterNormSquared for VersorOddAtInfinity {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        7        9        0
    fn center_norm_squared(self) -> Scalar {
        use crate::elements::*;
        return Scalar::from_groups(/* scalar */ self.flat_weight_norm_squared().right_anti_dual()[scalar] + self.round_bulk_norm_squared()[scalar]);
    }
}
