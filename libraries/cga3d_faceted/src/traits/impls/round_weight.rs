// Note on Operative Statistics:
// Operative Statistics are not a precise predictor of performance or performance comparisons.
// This is due to varying hardware capabilities and compiler optimizations.
// As always, where performance is a concern, there is no substitute for
// real measurements on real work-loads on real hardware.
// Disclaimer aside, enjoy the fun information =)
//
// Total Implementations: 49
//
// Yes SIMD:   add/sub     mul     div
//  Minimum:         0       0       0
//   Median:         0       0       0
//  Average:         0       0       0
//  Maximum:         0       0       0
//
//  No SIMD:   add/sub     mul     div
//  Minimum:         0       0       0
//   Median:         0       0       0
//  Average:         0       0       0
//  Maximum:         0       0       0
impl std::ops::Div<RoundWeightPrefixOrPostfix> for AntiCircleOnOrigin {
    type Output = NullDipoleAtOrigin;
    fn div(self, _rhs: RoundWeightPrefixOrPostfix) -> Self::Output {
        self.round_weight()
    }
}
impl RoundWeight for AntiCircleOnOrigin {
    type Output = NullDipoleAtOrigin;
    fn round_weight(self) -> Self::Output {
        return NullDipoleAtOrigin::from_groups(/* e41, e42, e43 */ self.group0());
    }
}
impl std::ops::Div<RoundWeightPrefixOrPostfix> for AntiCircleRotor {
    type Output = NullDipoleAtOrigin;
    fn div(self, _rhs: RoundWeightPrefixOrPostfix) -> Self::Output {
        self.round_weight()
    }
}
impl RoundWeight for AntiCircleRotor {
    type Output = NullDipoleAtOrigin;
    fn round_weight(self) -> Self::Output {
        return NullDipoleAtOrigin::from_groups(/* e41, e42, e43 */ self.group0());
    }
}
impl std::ops::Div<RoundWeightPrefixOrPostfix> for AntiCircleRotorAligningOrigin {
    type Output = NullDipoleAtOrigin;
    fn div(self, _rhs: RoundWeightPrefixOrPostfix) -> Self::Output {
        self.round_weight()
    }
}
impl RoundWeight for AntiCircleRotorAligningOrigin {
    type Output = NullDipoleAtOrigin;
    fn round_weight(self) -> Self::Output {
        return NullDipoleAtOrigin::from_groups(/* e41, e42, e43 */ self.group0());
    }
}
impl std::ops::Div<RoundWeightPrefixOrPostfix> for AntiCircleRotorOnOrigin {
    type Output = NullDipoleAtOrigin;
    fn div(self, _rhs: RoundWeightPrefixOrPostfix) -> Self::Output {
        self.round_weight()
    }
}
impl RoundWeight for AntiCircleRotorOnOrigin {
    type Output = NullDipoleAtOrigin;
    fn round_weight(self) -> Self::Output {
        return NullDipoleAtOrigin::from_groups(/* e41, e42, e43 */ self.group0().xyz());
    }
}
impl std::ops::Div<RoundWeightPrefixOrPostfix> for AntiDipoleInversion {
    type Output = NullVersorEvenAtOrigin;
    fn div(self, _rhs: RoundWeightPrefixOrPostfix) -> Self::Output {
        self.round_weight()
    }
}
impl RoundWeight for AntiDipoleInversion {
    type Output = NullVersorEvenAtOrigin;
    fn round_weight(self) -> Self::Output {
        use crate::elements::*;
        return NullVersorEvenAtOrigin::from_groups(/* e423, e431, e412, e4 */ self.group0().with_w(self[e4]));
    }
}
impl std::ops::Div<RoundWeightPrefixOrPostfix> for AntiDipoleInversionOnOrigin {
    type Output = NullVersorEvenAtOrigin;
    fn div(self, _rhs: RoundWeightPrefixOrPostfix) -> Self::Output {
        self.round_weight()
    }
}
impl RoundWeight for AntiDipoleInversionOnOrigin {
    type Output = NullVersorEvenAtOrigin;
    fn round_weight(self) -> Self::Output {
        use crate::elements::*;
        return NullVersorEvenAtOrigin::from_groups(/* e423, e431, e412, e4 */ Simd32x4::from([self[e423], self[e431], self[e412], self[e4]]));
    }
}
impl std::ops::Div<RoundWeightPrefixOrPostfix> for AntiDipoleInversionOrthogonalOrigin {
    type Output = NullVersorEvenAtOrigin;
    fn div(self, _rhs: RoundWeightPrefixOrPostfix) -> Self::Output {
        self.round_weight()
    }
}
impl RoundWeight for AntiDipoleInversionOrthogonalOrigin {
    type Output = NullVersorEvenAtOrigin;
    fn round_weight(self) -> Self::Output {
        use crate::elements::*;
        return NullVersorEvenAtOrigin::from_groups(/* e423, e431, e412, e4 */ Simd32x4::from([self[e423], self[e431], self[e412], self[e4]]));
    }
}
impl std::ops::Div<RoundWeightPrefixOrPostfix> for AntiDipoleOnOrigin {
    type Output = NullCircleAtOrigin;
    fn div(self, _rhs: RoundWeightPrefixOrPostfix) -> Self::Output {
        self.round_weight()
    }
}
impl RoundWeight for AntiDipoleOnOrigin {
    type Output = NullCircleAtOrigin;
    fn round_weight(self) -> Self::Output {
        return NullCircleAtOrigin::from_groups(/* e423, e431, e412 */ self.group0().xyz());
    }
}
impl std::ops::Div<RoundWeightPrefixOrPostfix> for AntiDualNum {
    type Output = NullSphereAtOrigin;
    fn div(self, _rhs: RoundWeightPrefixOrPostfix) -> Self::Output {
        self.round_weight()
    }
}
impl RoundWeight for AntiDualNum {
    type Output = NullSphereAtOrigin;
    fn round_weight(self) -> Self::Output {
        use crate::elements::*;
        return NullSphereAtOrigin::from_groups(/* e1234 */ self[e1234]);
    }
}
impl std::ops::Div<RoundWeightPrefixOrPostfix> for AntiSphereOnOrigin {
    type Output = Origin;
    fn div(self, _rhs: RoundWeightPrefixOrPostfix) -> Self::Output {
        self.round_weight()
    }
}
impl RoundWeight for AntiSphereOnOrigin {
    type Output = Origin;
    fn round_weight(self) -> Self::Output {
        use crate::elements::*;
        return Origin::from_groups(/* e4 */ self[e4]);
    }
}
impl std::ops::Div<RoundWeightPrefixOrPostfix> for AntiVersorEvenOnOrigin {
    type Output = NullDipoleInversionAtOrigin;
    fn div(self, _rhs: RoundWeightPrefixOrPostfix) -> Self::Output {
        self.round_weight()
    }
}
impl RoundWeight for AntiVersorEvenOnOrigin {
    type Output = NullDipoleInversionAtOrigin;
    fn round_weight(self) -> Self::Output {
        use crate::elements::*;
        return NullDipoleInversionAtOrigin::from_groups(/* e41, e42, e43, e1234 */ Simd32x4::from([self[e41], self[e42], self[e43], self[e1234]]));
    }
}
impl std::ops::Div<RoundWeightPrefixOrPostfix> for Circle {
    type Output = NullCircleAtOrigin;
    fn div(self, _rhs: RoundWeightPrefixOrPostfix) -> Self::Output {
        self.round_weight()
    }
}
impl RoundWeight for Circle {
    type Output = NullCircleAtOrigin;
    fn round_weight(self) -> Self::Output {
        return NullCircleAtOrigin::from_groups(/* e423, e431, e412 */ self.group0());
    }
}
impl std::ops::Div<RoundWeightPrefixOrPostfix> for CircleAligningOrigin {
    type Output = NullCircleAtOrigin;
    fn div(self, _rhs: RoundWeightPrefixOrPostfix) -> Self::Output {
        self.round_weight()
    }
}
impl RoundWeight for CircleAligningOrigin {
    type Output = NullCircleAtOrigin;
    fn round_weight(self) -> Self::Output {
        return NullCircleAtOrigin::from_groups(/* e423, e431, e412 */ self.group0());
    }
}
impl std::ops::Div<RoundWeightPrefixOrPostfix> for CircleAtOrigin {
    type Output = NullCircleAtOrigin;
    fn div(self, _rhs: RoundWeightPrefixOrPostfix) -> Self::Output {
        self.round_weight()
    }
}
impl RoundWeight for CircleAtOrigin {
    type Output = NullCircleAtOrigin;
    fn round_weight(self) -> Self::Output {
        return NullCircleAtOrigin::from_groups(/* e423, e431, e412 */ self.group0());
    }
}
impl std::ops::Div<RoundWeightPrefixOrPostfix> for CircleOnOrigin {
    type Output = NullCircleAtOrigin;
    fn div(self, _rhs: RoundWeightPrefixOrPostfix) -> Self::Output {
        self.round_weight()
    }
}
impl RoundWeight for CircleOnOrigin {
    type Output = NullCircleAtOrigin;
    fn round_weight(self) -> Self::Output {
        return NullCircleAtOrigin::from_groups(/* e423, e431, e412 */ self.group0());
    }
}
impl std::ops::Div<RoundWeightPrefixOrPostfix> for CircleOrthogonalOrigin {
    type Output = NullCircleAtOrigin;
    fn div(self, _rhs: RoundWeightPrefixOrPostfix) -> Self::Output {
        self.round_weight()
    }
}
impl RoundWeight for CircleOrthogonalOrigin {
    type Output = NullCircleAtOrigin;
    fn round_weight(self) -> Self::Output {
        return NullCircleAtOrigin::from_groups(/* e423, e431, e412 */ self.group0().xyz());
    }
}
impl std::ops::Div<RoundWeightPrefixOrPostfix> for CircleRotor {
    type Output = NullCircleAtOrigin;
    fn div(self, _rhs: RoundWeightPrefixOrPostfix) -> Self::Output {
        self.round_weight()
    }
}
impl RoundWeight for CircleRotor {
    type Output = NullCircleAtOrigin;
    fn round_weight(self) -> Self::Output {
        return NullCircleAtOrigin::from_groups(/* e423, e431, e412 */ self.group0());
    }
}
impl std::ops::Div<RoundWeightPrefixOrPostfix> for CircleRotorAligningOrigin {
    type Output = NullCircleAtOrigin;
    fn div(self, _rhs: RoundWeightPrefixOrPostfix) -> Self::Output {
        self.round_weight()
    }
}
impl RoundWeight for CircleRotorAligningOrigin {
    type Output = NullCircleAtOrigin;
    fn round_weight(self) -> Self::Output {
        return NullCircleAtOrigin::from_groups(/* e423, e431, e412 */ self.group0());
    }
}
impl std::ops::Div<RoundWeightPrefixOrPostfix> for CircleRotorOnOrigin {
    type Output = NullCircleAtOrigin;
    fn div(self, _rhs: RoundWeightPrefixOrPostfix) -> Self::Output {
        self.round_weight()
    }
}
impl RoundWeight for CircleRotorOnOrigin {
    type Output = NullCircleAtOrigin;
    fn round_weight(self) -> Self::Output {
        return NullCircleAtOrigin::from_groups(/* e423, e431, e412 */ self.group0().xyz());
    }
}
impl std::ops::Div<RoundWeightPrefixOrPostfix> for Dipole {
    type Output = NullDipoleAtOrigin;
    fn div(self, _rhs: RoundWeightPrefixOrPostfix) -> Self::Output {
        self.round_weight()
    }
}
impl RoundWeight for Dipole {
    type Output = NullDipoleAtOrigin;
    fn round_weight(self) -> Self::Output {
        return NullDipoleAtOrigin::from_groups(/* e41, e42, e43 */ self.group0());
    }
}
impl std::ops::Div<RoundWeightPrefixOrPostfix> for DipoleAligningOrigin {
    type Output = NullDipoleAtOrigin;
    fn div(self, _rhs: RoundWeightPrefixOrPostfix) -> Self::Output {
        self.round_weight()
    }
}
impl RoundWeight for DipoleAligningOrigin {
    type Output = NullDipoleAtOrigin;
    fn round_weight(self) -> Self::Output {
        return NullDipoleAtOrigin::from_groups(/* e41, e42, e43 */ self.group0().xyz());
    }
}
impl std::ops::Div<RoundWeightPrefixOrPostfix> for DipoleAtOrigin {
    type Output = NullDipoleAtOrigin;
    fn div(self, _rhs: RoundWeightPrefixOrPostfix) -> Self::Output {
        self.round_weight()
    }
}
impl RoundWeight for DipoleAtOrigin {
    type Output = NullDipoleAtOrigin;
    fn round_weight(self) -> Self::Output {
        return NullDipoleAtOrigin::from_groups(/* e41, e42, e43 */ self.group0());
    }
}
impl std::ops::Div<RoundWeightPrefixOrPostfix> for DipoleInversion {
    type Output = NullDipoleInversionAtOrigin;
    fn div(self, _rhs: RoundWeightPrefixOrPostfix) -> Self::Output {
        self.round_weight()
    }
}
impl RoundWeight for DipoleInversion {
    type Output = NullDipoleInversionAtOrigin;
    fn round_weight(self) -> Self::Output {
        use crate::elements::*;
        return NullDipoleInversionAtOrigin::from_groups(/* e41, e42, e43, e1234 */ self.group0().with_w(self[e1234]));
    }
}
impl std::ops::Div<RoundWeightPrefixOrPostfix> for DipoleInversionAligningOrigin {
    type Output = NullDipoleInversionAtOrigin;
    fn div(self, _rhs: RoundWeightPrefixOrPostfix) -> Self::Output {
        self.round_weight()
    }
}
impl RoundWeight for DipoleInversionAligningOrigin {
    type Output = NullDipoleInversionAtOrigin;
    fn round_weight(self) -> Self::Output {
        use crate::elements::*;
        return NullDipoleInversionAtOrigin::from_groups(/* e41, e42, e43, e1234 */ Simd32x4::from([self[e41], self[e42], self[e43], self[e1234]]));
    }
}
impl std::ops::Div<RoundWeightPrefixOrPostfix> for DipoleInversionAtOrigin {
    type Output = NullDipoleInversionAtOrigin;
    fn div(self, _rhs: RoundWeightPrefixOrPostfix) -> Self::Output {
        self.round_weight()
    }
}
impl RoundWeight for DipoleInversionAtOrigin {
    type Output = NullDipoleInversionAtOrigin;
    fn round_weight(self) -> Self::Output {
        use crate::elements::*;
        return NullDipoleInversionAtOrigin::from_groups(/* e41, e42, e43, e1234 */ Simd32x4::from([self[e41], self[e42], self[e43], self[e1234]]));
    }
}
impl std::ops::Div<RoundWeightPrefixOrPostfix> for DipoleInversionOnOrigin {
    type Output = NullDipoleInversionAtOrigin;
    fn div(self, _rhs: RoundWeightPrefixOrPostfix) -> Self::Output {
        self.round_weight()
    }
}
impl RoundWeight for DipoleInversionOnOrigin {
    type Output = NullDipoleInversionAtOrigin;
    fn round_weight(self) -> Self::Output {
        use crate::elements::*;
        return NullDipoleInversionAtOrigin::from_groups(/* e41, e42, e43, e1234 */ Simd32x4::from([self[e41], self[e42], self[e43], self[e1234]]));
    }
}
impl std::ops::Div<RoundWeightPrefixOrPostfix> for DipoleInversionOrthogonalOrigin {
    type Output = NullDipoleInversionAtOrigin;
    fn div(self, _rhs: RoundWeightPrefixOrPostfix) -> Self::Output {
        self.round_weight()
    }
}
impl RoundWeight for DipoleInversionOrthogonalOrigin {
    type Output = NullDipoleInversionAtOrigin;
    fn round_weight(self) -> Self::Output {
        use crate::elements::*;
        return NullDipoleInversionAtOrigin::from_groups(/* e41, e42, e43, e1234 */ Simd32x4::from([self[e41], self[e42], self[e43], self[e1234]]));
    }
}
impl std::ops::Div<RoundWeightPrefixOrPostfix> for DipoleOnOrigin {
    type Output = NullDipoleAtOrigin;
    fn div(self, _rhs: RoundWeightPrefixOrPostfix) -> Self::Output {
        self.round_weight()
    }
}
impl RoundWeight for DipoleOnOrigin {
    type Output = NullDipoleAtOrigin;
    fn round_weight(self) -> Self::Output {
        return NullDipoleAtOrigin::from_groups(/* e41, e42, e43 */ self.group0().xyz());
    }
}
impl std::ops::Div<RoundWeightPrefixOrPostfix> for DipoleOrthogonalOrigin {
    type Output = NullDipoleAtOrigin;
    fn div(self, _rhs: RoundWeightPrefixOrPostfix) -> Self::Output {
        self.round_weight()
    }
}
impl RoundWeight for DipoleOrthogonalOrigin {
    type Output = NullDipoleAtOrigin;
    fn round_weight(self) -> Self::Output {
        return NullDipoleAtOrigin::from_groups(/* e41, e42, e43 */ self.group0());
    }
}
impl std::ops::Div<RoundWeightPrefixOrPostfix> for DualNum {
    type Output = Origin;
    fn div(self, _rhs: RoundWeightPrefixOrPostfix) -> Self::Output {
        self.round_weight()
    }
}
impl RoundWeight for DualNum {
    type Output = Origin;
    fn round_weight(self) -> Self::Output {
        use crate::elements::*;
        return Origin::from_groups(/* e4 */ self[e4]);
    }
}
impl std::ops::Div<RoundWeightPrefixOrPostfix> for MultiVector {
    type Output = MultiVector;
    fn div(self, _rhs: RoundWeightPrefixOrPostfix) -> Self::Output {
        self.round_weight()
    }
}
impl std::ops::DivAssign<RoundWeightPrefixOrPostfix> for MultiVector {
    fn div_assign(&mut self, _rhs: RoundWeightPrefixOrPostfix) {
        *self = self.round_weight()
    }
}
impl RoundWeight for MultiVector {
    type Output = MultiVector;
    fn round_weight(self) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x3::from(0.0).with_w(self[e4]),
            // e5
            0.0,
            // e41, e42, e43, e45
            self.group3().xyz().with_w(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([self[e1234], 0.0, 0.0, 0.0]),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Div<RoundWeightPrefixOrPostfix> for NullCircleAtOrigin {
    type Output = NullCircleAtOrigin;
    fn div(self, _rhs: RoundWeightPrefixOrPostfix) -> Self::Output {
        self.round_weight()
    }
}
impl std::ops::DivAssign<RoundWeightPrefixOrPostfix> for NullCircleAtOrigin {
    fn div_assign(&mut self, _rhs: RoundWeightPrefixOrPostfix) {
        *self = self.round_weight()
    }
}
impl RoundWeight for NullCircleAtOrigin {
    type Output = NullCircleAtOrigin;
    fn round_weight(self) -> Self::Output {
        return self;
    }
}
impl std::ops::Div<RoundWeightPrefixOrPostfix> for NullDipoleAtOrigin {
    type Output = NullDipoleAtOrigin;
    fn div(self, _rhs: RoundWeightPrefixOrPostfix) -> Self::Output {
        self.round_weight()
    }
}
impl std::ops::DivAssign<RoundWeightPrefixOrPostfix> for NullDipoleAtOrigin {
    fn div_assign(&mut self, _rhs: RoundWeightPrefixOrPostfix) {
        *self = self.round_weight()
    }
}
impl RoundWeight for NullDipoleAtOrigin {
    type Output = NullDipoleAtOrigin;
    fn round_weight(self) -> Self::Output {
        return self;
    }
}
impl std::ops::Div<RoundWeightPrefixOrPostfix> for NullDipoleInversionAtOrigin {
    type Output = NullDipoleInversionAtOrigin;
    fn div(self, _rhs: RoundWeightPrefixOrPostfix) -> Self::Output {
        self.round_weight()
    }
}
impl std::ops::DivAssign<RoundWeightPrefixOrPostfix> for NullDipoleInversionAtOrigin {
    fn div_assign(&mut self, _rhs: RoundWeightPrefixOrPostfix) {
        *self = self.round_weight()
    }
}
impl RoundWeight for NullDipoleInversionAtOrigin {
    type Output = NullDipoleInversionAtOrigin;
    fn round_weight(self) -> Self::Output {
        return self;
    }
}
impl std::ops::Div<RoundWeightPrefixOrPostfix> for NullSphereAtOrigin {
    type Output = NullSphereAtOrigin;
    fn div(self, _rhs: RoundWeightPrefixOrPostfix) -> Self::Output {
        self.round_weight()
    }
}
impl std::ops::DivAssign<RoundWeightPrefixOrPostfix> for NullSphereAtOrigin {
    fn div_assign(&mut self, _rhs: RoundWeightPrefixOrPostfix) {
        *self = self.round_weight()
    }
}
impl RoundWeight for NullSphereAtOrigin {
    type Output = NullSphereAtOrigin;
    fn round_weight(self) -> Self::Output {
        return self;
    }
}
impl std::ops::Div<RoundWeightPrefixOrPostfix> for NullVersorEvenAtOrigin {
    type Output = NullVersorEvenAtOrigin;
    fn div(self, _rhs: RoundWeightPrefixOrPostfix) -> Self::Output {
        self.round_weight()
    }
}
impl std::ops::DivAssign<RoundWeightPrefixOrPostfix> for NullVersorEvenAtOrigin {
    fn div_assign(&mut self, _rhs: RoundWeightPrefixOrPostfix) {
        *self = self.round_weight()
    }
}
impl RoundWeight for NullVersorEvenAtOrigin {
    type Output = NullVersorEvenAtOrigin;
    fn round_weight(self) -> Self::Output {
        return self;
    }
}
impl std::ops::Div<RoundWeightPrefixOrPostfix> for Origin {
    type Output = Origin;
    fn div(self, _rhs: RoundWeightPrefixOrPostfix) -> Self::Output {
        self.round_weight()
    }
}
impl std::ops::DivAssign<RoundWeightPrefixOrPostfix> for Origin {
    fn div_assign(&mut self, _rhs: RoundWeightPrefixOrPostfix) {
        *self = self.round_weight()
    }
}
impl RoundWeight for Origin {
    type Output = Origin;
    fn round_weight(self) -> Self::Output {
        return self;
    }
}
impl std::ops::Div<RoundWeightPrefixOrPostfix> for RoundPoint {
    type Output = Origin;
    fn div(self, _rhs: RoundWeightPrefixOrPostfix) -> Self::Output {
        self.round_weight()
    }
}
impl RoundWeight for RoundPoint {
    type Output = Origin;
    fn round_weight(self) -> Self::Output {
        use crate::elements::*;
        return Origin::from_groups(/* e4 */ self[e4]);
    }
}
impl std::ops::Div<RoundWeightPrefixOrPostfix> for RoundPointAtOrigin {
    type Output = Origin;
    fn div(self, _rhs: RoundWeightPrefixOrPostfix) -> Self::Output {
        self.round_weight()
    }
}
impl RoundWeight for RoundPointAtOrigin {
    type Output = Origin;
    fn round_weight(self) -> Self::Output {
        use crate::elements::*;
        return Origin::from_groups(/* e4 */ self[e4]);
    }
}
impl std::ops::Div<RoundWeightPrefixOrPostfix> for Sphere {
    type Output = NullSphereAtOrigin;
    fn div(self, _rhs: RoundWeightPrefixOrPostfix) -> Self::Output {
        self.round_weight()
    }
}
impl RoundWeight for Sphere {
    type Output = NullSphereAtOrigin;
    fn round_weight(self) -> Self::Output {
        use crate::elements::*;
        return NullSphereAtOrigin::from_groups(/* e1234 */ self[e1234]);
    }
}
impl std::ops::Div<RoundWeightPrefixOrPostfix> for SphereAtOrigin {
    type Output = NullSphereAtOrigin;
    fn div(self, _rhs: RoundWeightPrefixOrPostfix) -> Self::Output {
        self.round_weight()
    }
}
impl RoundWeight for SphereAtOrigin {
    type Output = NullSphereAtOrigin;
    fn round_weight(self) -> Self::Output {
        use crate::elements::*;
        return NullSphereAtOrigin::from_groups(/* e1234 */ self[e1234]);
    }
}
impl std::ops::Div<RoundWeightPrefixOrPostfix> for SphereOnOrigin {
    type Output = NullSphereAtOrigin;
    fn div(self, _rhs: RoundWeightPrefixOrPostfix) -> Self::Output {
        self.round_weight()
    }
}
impl RoundWeight for SphereOnOrigin {
    type Output = NullSphereAtOrigin;
    fn round_weight(self) -> Self::Output {
        use crate::elements::*;
        return NullSphereAtOrigin::from_groups(/* e1234 */ self[e1234]);
    }
}
impl std::ops::Div<RoundWeightPrefixOrPostfix> for VersorEven {
    type Output = NullVersorEvenAtOrigin;
    fn div(self, _rhs: RoundWeightPrefixOrPostfix) -> Self::Output {
        self.round_weight()
    }
}
impl RoundWeight for VersorEven {
    type Output = NullVersorEvenAtOrigin;
    fn round_weight(self) -> Self::Output {
        use crate::elements::*;
        return NullVersorEvenAtOrigin::from_groups(/* e423, e431, e412, e4 */ Simd32x4::from([self[e423], self[e431], self[e412], self[e4]]));
    }
}
impl std::ops::Div<RoundWeightPrefixOrPostfix> for VersorEvenAligningOrigin {
    type Output = NullVersorEvenAtOrigin;
    fn div(self, _rhs: RoundWeightPrefixOrPostfix) -> Self::Output {
        self.round_weight()
    }
}
impl RoundWeight for VersorEvenAligningOrigin {
    type Output = NullVersorEvenAtOrigin;
    fn round_weight(self) -> Self::Output {
        use crate::elements::*;
        return NullVersorEvenAtOrigin::from_groups(/* e423, e431, e412, e4 */ Simd32x4::from([self[e423], self[e431], self[e412], self[e4]]));
    }
}
impl std::ops::Div<RoundWeightPrefixOrPostfix> for VersorEvenAtOrigin {
    type Output = NullVersorEvenAtOrigin;
    fn div(self, _rhs: RoundWeightPrefixOrPostfix) -> Self::Output {
        self.round_weight()
    }
}
impl RoundWeight for VersorEvenAtOrigin {
    type Output = NullVersorEvenAtOrigin;
    fn round_weight(self) -> Self::Output {
        return NullVersorEvenAtOrigin::from_groups(/* e423, e431, e412, e4 */ self.group0());
    }
}
impl std::ops::Div<RoundWeightPrefixOrPostfix> for VersorEvenOnOrigin {
    type Output = NullVersorEvenAtOrigin;
    fn div(self, _rhs: RoundWeightPrefixOrPostfix) -> Self::Output {
        self.round_weight()
    }
}
impl RoundWeight for VersorEvenOnOrigin {
    type Output = NullVersorEvenAtOrigin;
    fn round_weight(self) -> Self::Output {
        use crate::elements::*;
        return NullVersorEvenAtOrigin::from_groups(/* e423, e431, e412, e4 */ Simd32x4::from([self[e423], self[e431], self[e412], self[e4]]));
    }
}
impl std::ops::Div<RoundWeightPrefixOrPostfix> for VersorEvenOrthogonalOrigin {
    type Output = NullVersorEvenAtOrigin;
    fn div(self, _rhs: RoundWeightPrefixOrPostfix) -> Self::Output {
        self.round_weight()
    }
}
impl RoundWeight for VersorEvenOrthogonalOrigin {
    type Output = NullVersorEvenAtOrigin;
    fn round_weight(self) -> Self::Output {
        use crate::elements::*;
        return NullVersorEvenAtOrigin::from_groups(/* e423, e431, e412, e4 */ Simd32x4::from([self[e423], self[e431], self[e412], self[e4]]));
    }
}
impl std::ops::Div<RoundWeightPrefixOrPostfix> for VersorOdd {
    type Output = NullDipoleInversionAtOrigin;
    fn div(self, _rhs: RoundWeightPrefixOrPostfix) -> Self::Output {
        self.round_weight()
    }
}
impl RoundWeight for VersorOdd {
    type Output = NullDipoleInversionAtOrigin;
    fn round_weight(self) -> Self::Output {
        use crate::elements::*;
        return NullDipoleInversionAtOrigin::from_groups(/* e41, e42, e43, e1234 */ Simd32x4::from([self[e41], self[e42], self[e43], self[e1234]]));
    }
}
impl std::ops::Div<RoundWeightPrefixOrPostfix> for VersorOddOrthogonalOrigin {
    type Output = NullDipoleInversionAtOrigin;
    fn div(self, _rhs: RoundWeightPrefixOrPostfix) -> Self::Output {
        self.round_weight()
    }
}
impl RoundWeight for VersorOddOrthogonalOrigin {
    type Output = NullDipoleInversionAtOrigin;
    fn round_weight(self) -> Self::Output {
        use crate::elements::*;
        return NullDipoleInversionAtOrigin::from_groups(/* e41, e42, e43, e1234 */ Simd32x4::from([self[e41], self[e42], self[e43], self[e1234]]));
    }
}
