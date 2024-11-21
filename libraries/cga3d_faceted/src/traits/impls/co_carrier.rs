use crate::traits::RightAntiDual;
// Note on Operative Statistics:
// Operative Statistics are not a precise predictor of performance or performance comparisons.
// This is due to varying hardware capabilities and compiler optimizations.
// As always, where performance is a concern, there is no substitute for
// real measurements on real work-loads on real hardware.
// Disclaimer aside, enjoy the fun information =)
//
// Total Implementations: 76
//
// Yes SIMD:   add/sub     mul     div
//  Minimum:         0       0       0
//   Median:         0       4       0
//  Average:         0      11       0
//  Maximum:         0     168       0
//
//  No SIMD:   add/sub     mul     div
//  Minimum:         0       0       0
//   Median:         0       6       0
//  Average:         0      13       0
//  Maximum:         0     224       0
impl std::ops::Div<co_carrier> for AntiCircleOnOrigin {
    type Output = CircleOnOrigin;
    fn div(self, _rhs: co_carrier) -> Self::Output {
        self.co_carrier()
    }
}
impl CoCarrier for AntiCircleOnOrigin {
    type Output = CircleOnOrigin;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        2        0
    // no simd        0        6        0
    fn co_carrier(self) -> Self::Output {
        return self.right_anti_dual();
    }
}
impl std::ops::Div<co_carrier> for AntiCircleRotor {
    type Output = Plane;
    fn div(self, _rhs: co_carrier) -> Self::Output {
        self.co_carrier()
    }
}
impl CoCarrier for AntiCircleRotor {
    type Output = Plane;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0       12        0
    //    simd3        0        2        0
    // Totals...
    // yes simd        0       14        0
    //  no simd        0       18        0
    fn co_carrier(self) -> Self::Output {
        use crate::elements::*;
        return Plane::from_groups(
            // e4235, e4315, e4125, e3215
            crate::swizzle!(self.right_anti_dual().group0(), 0, 1, 2).extend_to_4(self.right_anti_dual()[e321]),
        );
    }
}
impl std::ops::Div<co_carrier> for AntiCircleRotorAligningOrigin {
    type Output = CircleRotorAligningOrigin;
    fn div(self, _rhs: co_carrier) -> Self::Output {
        self.co_carrier()
    }
}
impl CoCarrier for AntiCircleRotorAligningOrigin {
    type Output = CircleRotorAligningOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        3        0
    //    simd3        0        2        0
    // Totals...
    // yes simd        0        5        0
    //  no simd        0        9        0
    fn co_carrier(self) -> Self::Output {
        return self.right_anti_dual();
    }
}
impl std::ops::Div<co_carrier> for AntiCircleRotorAtInfinity {
    type Output = Horizon;
    fn div(self, _rhs: co_carrier) -> Self::Output {
        self.co_carrier()
    }
}
impl CoCarrier for AntiCircleRotorAtInfinity {
    type Output = Horizon;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        6        0
    fn co_carrier(self) -> Self::Output {
        use crate::elements::*;
        return Horizon::from_groups(/* e3215 */ self.right_anti_dual()[e321]);
    }
}
impl std::ops::Div<co_carrier> for AntiCircleRotorOnOrigin {
    type Output = PlaneOnOrigin;
    fn div(self, _rhs: co_carrier) -> Self::Output {
        self.co_carrier()
    }
}
impl CoCarrier for AntiCircleRotorOnOrigin {
    type Output = PlaneOnOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        9        0
    //    simd3        0        3        0
    // Totals...
    // yes simd        0       12        0
    //  no simd        0       18        0
    fn co_carrier(self) -> Self::Output {
        use crate::elements::*;
        return PlaneOnOrigin::from_groups(
            // e4235, e4315, e4125
            Simd32x3::from([self.right_anti_dual()[e423], self.right_anti_dual()[e431], self.right_anti_dual()[e412]]),
        );
    }
}
impl std::ops::Div<co_carrier> for AntiDipoleInversion {
    type Output = CircleRotorAligningOriginAtInfinity;
    fn div(self, _rhs: co_carrier) -> Self::Output {
        self.co_carrier()
    }
}
impl CoCarrier for AntiDipoleInversion {
    type Output = CircleRotorAligningOriginAtInfinity;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0       15        0
    fn co_carrier(self) -> Self::Output {
        use crate::elements::*;
        return CircleRotorAligningOriginAtInfinity::from_groups(
            // e415, e425, e435
            self.right_anti_dual().group0(),
            // e235, e315, e125, e12345
            Simd32x4::from([self.right_anti_dual()[e23], self.right_anti_dual()[e31], self.right_anti_dual()[e12], self.right_anti_dual()[e1234]]),
        );
    }
}
impl std::ops::Div<co_carrier> for AntiDipoleInversionAtInfinity {
    type Output = LineAtInfinity;
    fn div(self, _rhs: co_carrier) -> Self::Output {
        self.co_carrier()
    }
}
impl CoCarrier for AntiDipoleInversionAtInfinity {
    type Output = LineAtInfinity;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        6        0
    fn co_carrier(self) -> Self::Output {
        use crate::elements::*;
        return LineAtInfinity::from_groups(
            // e235, e315, e125
            Simd32x3::from([self.right_anti_dual()[e23], self.right_anti_dual()[e31], self.right_anti_dual()[e12]]),
        );
    }
}
impl std::ops::Div<co_carrier> for AntiDipoleInversionOnOrigin {
    type Output = MotorOnOrigin;
    fn div(self, _rhs: co_carrier) -> Self::Output {
        self.co_carrier()
    }
}
impl CoCarrier for AntiDipoleInversionOnOrigin {
    type Output = MotorOnOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        8        0
    fn co_carrier(self) -> Self::Output {
        use crate::elements::*;
        return MotorOnOrigin::from_groups(/* e415, e425, e435, e12345 */ Simd32x4::from([
            self.right_anti_dual()[e41],
            self.right_anti_dual()[e42],
            self.right_anti_dual()[e43],
            self.right_anti_dual()[e1234],
        ]));
    }
}
impl std::ops::Div<co_carrier> for AntiDipoleInversionOrthogonalOrigin {
    type Output = CircleRotorAligningOriginAtInfinity;
    fn div(self, _rhs: co_carrier) -> Self::Output {
        self.co_carrier()
    }
}
impl CoCarrier for AntiDipoleInversionOrthogonalOrigin {
    type Output = CircleRotorAligningOriginAtInfinity;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0       10        0
    fn co_carrier(self) -> Self::Output {
        use crate::elements::*;
        return CircleRotorAligningOriginAtInfinity::from_groups(
            // e415, e425, e435
            Simd32x3::from([self.right_anti_dual()[e41], self.right_anti_dual()[e42], self.right_anti_dual()[e43]]),
            // e235, e315, e125, e12345
            crate::swizzle!(self.right_anti_dual().group1(), 0, 1, 2).extend_to_4(self.right_anti_dual()[e1234]),
        );
    }
}
impl std::ops::Div<co_carrier> for AntiDipoleOnOrigin {
    type Output = LineOnOrigin;
    fn div(self, _rhs: co_carrier) -> Self::Output {
        self.co_carrier()
    }
}
impl CoCarrier for AntiDipoleOnOrigin {
    type Output = LineOnOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        3        0
    fn co_carrier(self) -> Self::Output {
        use crate::elements::*;
        return LineOnOrigin::from_groups(
            // e415, e425, e435
            Simd32x3::from([self.right_anti_dual()[e41], self.right_anti_dual()[e42], self.right_anti_dual()[e43]]),
        );
    }
}
impl std::ops::Div<co_carrier> for AntiDualNum {
    type Output = Motor;
    fn div(self, _rhs: co_carrier) -> Self::Output {
        self.co_carrier()
    }
}
impl CoCarrier for AntiDualNum {
    type Output = Motor;
    fn co_carrier(self) -> Self::Output {
        use crate::elements::*;
        return Motor::from_groups(
            // e415, e425, e435, e12345
            Simd32x4::from([0.0, 0.0, 0.0, self.right_anti_dual()[e1234]]),
            // e235, e315, e125, e5
            Simd32x4::from([0.0, 0.0, 0.0, self.right_anti_dual()[scalar]]),
        );
    }
}
impl std::ops::Div<co_carrier> for AntiMysteryCircleRotor {
    type Output = Horizon;
    fn div(self, _rhs: co_carrier) -> Self::Output {
        self.co_carrier()
    }
}
impl CoCarrier for AntiMysteryCircleRotor {
    type Output = Horizon;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        3        0
    fn co_carrier(self) -> Self::Output {
        use crate::elements::*;
        return Horizon::from_groups(/* e3215 */ self.right_anti_dual()[e321]);
    }
}
impl std::ops::Div<co_carrier> for AntiMysteryDipoleInversion {
    type Output = LineAtInfinity;
    fn div(self, _rhs: co_carrier) -> Self::Output {
        self.co_carrier()
    }
}
impl CoCarrier for AntiMysteryDipoleInversion {
    type Output = LineAtInfinity;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        3        0
    fn co_carrier(self) -> Self::Output {
        use crate::elements::*;
        return LineAtInfinity::from_groups(
            // e235, e315, e125
            Simd32x3::from([self.right_anti_dual()[e23], self.right_anti_dual()[e31], self.right_anti_dual()[e12]]),
        );
    }
}
impl std::ops::Div<co_carrier> for AntiPlaneOnOrigin {
    type Output = AntiPlaneOnOrigin;
    fn div(self, _rhs: co_carrier) -> Self::Output {
        self.co_carrier()
    }
}
impl std::ops::DivAssign<co_carrier> for AntiPlaneOnOrigin {
    fn div_assign(&mut self, _rhs: co_carrier) {
        *self = self.co_carrier()
    }
}
impl CoCarrier for AntiPlaneOnOrigin {
    type Output = AntiPlaneOnOrigin;
    fn co_carrier(self) -> Self::Output {
        return self.right_anti_dual();
    }
}
impl std::ops::Div<co_carrier> for AntiScalar {
    type Output = Scalar;
    fn div(self, _rhs: co_carrier) -> Self::Output {
        self.co_carrier()
    }
}
impl CoCarrier for AntiScalar {
    type Output = Scalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn co_carrier(self) -> Self::Output {
        return self.right_anti_dual();
    }
}
impl std::ops::Div<co_carrier> for AntiSphereOnOrigin {
    type Output = AntiScalar;
    fn div(self, _rhs: co_carrier) -> Self::Output {
        self.co_carrier()
    }
}
impl CoCarrier for AntiSphereOnOrigin {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn co_carrier(self) -> Self::Output {
        use crate::elements::*;
        return AntiScalar::from_groups(/* e12345 */ self.right_anti_dual()[e1234]);
    }
}
impl std::ops::Div<co_carrier> for AntiVersorEvenOnOrigin {
    type Output = FlectorOnOrigin;
    fn div(self, _rhs: co_carrier) -> Self::Output {
        self.co_carrier()
    }
}
impl CoCarrier for AntiVersorEvenOnOrigin {
    type Output = FlectorOnOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0       24        0
    fn co_carrier(self) -> Self::Output {
        use crate::elements::*;
        return FlectorOnOrigin::from_groups(/* e45, e4235, e4315, e4125 */ Simd32x4::from([
            self.right_anti_dual()[e4],
            self.right_anti_dual()[e423],
            self.right_anti_dual()[e431],
            self.right_anti_dual()[e412],
        ]));
    }
}
impl std::ops::Div<co_carrier> for Circle {
    type Output = Line;
    fn div(self, _rhs: co_carrier) -> Self::Output {
        self.co_carrier()
    }
}
impl CoCarrier for Circle {
    type Output = Line;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        4        0
    fn co_carrier(self) -> Self::Output {
        use crate::elements::*;
        return Line::from_groups(
            // e415, e425, e435
            self.right_anti_dual().group0(),
            // e235, e315, e125
            Simd32x3::from([self.right_anti_dual()[e23], self.right_anti_dual()[e31], self.right_anti_dual()[e12]]),
        );
    }
}
impl std::ops::Div<co_carrier> for CircleAligningOrigin {
    type Output = CircleAligningOrigin;
    fn div(self, _rhs: co_carrier) -> Self::Output {
        self.co_carrier()
    }
}
impl std::ops::DivAssign<co_carrier> for CircleAligningOrigin {
    fn div_assign(&mut self, _rhs: co_carrier) {
        *self = self.co_carrier()
    }
}
impl CoCarrier for CircleAligningOrigin {
    type Output = CircleAligningOrigin;
    fn co_carrier(self) -> Self::Output {
        return self.right_anti_dual();
    }
}
impl std::ops::Div<co_carrier> for CircleAtInfinity {
    type Output = LineAtInfinity;
    fn div(self, _rhs: co_carrier) -> Self::Output {
        self.co_carrier()
    }
}
impl CoCarrier for CircleAtInfinity {
    type Output = LineAtInfinity;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        3        0
    fn co_carrier(self) -> Self::Output {
        use crate::elements::*;
        return LineAtInfinity::from_groups(
            // e235, e315, e125
            Simd32x3::from([self.right_anti_dual()[e23], self.right_anti_dual()[e31], self.right_anti_dual()[e12]]),
        );
    }
}
impl std::ops::Div<co_carrier> for CircleAtOrigin {
    type Output = CircleAtOrigin;
    fn div(self, _rhs: co_carrier) -> Self::Output {
        self.co_carrier()
    }
}
impl std::ops::DivAssign<co_carrier> for CircleAtOrigin {
    fn div_assign(&mut self, _rhs: co_carrier) {
        *self = self.co_carrier()
    }
}
impl CoCarrier for CircleAtOrigin {
    type Output = CircleAtOrigin;
    fn co_carrier(self) -> Self::Output {
        return self.right_anti_dual();
    }
}
impl std::ops::Div<co_carrier> for CircleOnOrigin {
    type Output = CircleOnOrigin;
    fn div(self, _rhs: co_carrier) -> Self::Output {
        self.co_carrier()
    }
}
impl std::ops::DivAssign<co_carrier> for CircleOnOrigin {
    fn div_assign(&mut self, _rhs: co_carrier) {
        *self = self.co_carrier()
    }
}
impl CoCarrier for CircleOnOrigin {
    type Output = CircleOnOrigin;
    fn co_carrier(self) -> Self::Output {
        return self.right_anti_dual();
    }
}
impl std::ops::Div<co_carrier> for CircleOrthogonalOrigin {
    type Output = LineOnOrigin;
    fn div(self, _rhs: co_carrier) -> Self::Output {
        self.co_carrier()
    }
}
impl CoCarrier for CircleOrthogonalOrigin {
    type Output = LineOnOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        3        0
    fn co_carrier(self) -> Self::Output {
        use crate::elements::*;
        return LineOnOrigin::from_groups(
            // e415, e425, e435
            Simd32x3::from([self.right_anti_dual()[e41], self.right_anti_dual()[e42], self.right_anti_dual()[e43]]),
        );
    }
}
impl std::ops::Div<co_carrier> for CircleRotor {
    type Output = Motor;
    fn div(self, _rhs: co_carrier) -> Self::Output {
        self.co_carrier()
    }
}
impl CoCarrier for CircleRotor {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0       10        0
    fn co_carrier(self) -> Self::Output {
        use crate::elements::*;
        return Motor::from_groups(
            // e415, e425, e435, e12345
            crate::swizzle!(self.right_anti_dual().group0(), 0, 1, 2).extend_to_4(0.0),
            // e235, e315, e125, e5
            Simd32x4::from([self.right_anti_dual()[e23], self.right_anti_dual()[e31], self.right_anti_dual()[e12], self.right_anti_dual()[scalar]]),
        );
    }
}
impl std::ops::Div<co_carrier> for CircleRotorAligningOrigin {
    type Output = Motor;
    fn div(self, _rhs: co_carrier) -> Self::Output {
        self.co_carrier()
    }
}
impl CoCarrier for CircleRotorAligningOrigin {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        3        0
    fn co_carrier(self) -> Self::Output {
        use crate::elements::*;
        return Motor::from_groups(
            // e415, e425, e435, e12345
            crate::swizzle!(self.right_anti_dual().group0(), 0, 1, 2).extend_to_4(0.0),
            // e235, e315, e125, e5
            crate::swizzle!(self.right_anti_dual().group1(), 0, 1, 2).extend_to_4(self.right_anti_dual()[scalar]),
        );
    }
}
impl std::ops::Div<co_carrier> for CircleRotorAligningOriginAtInfinity {
    type Output = MotorAtInfinity;
    fn div(self, _rhs: co_carrier) -> Self::Output {
        self.co_carrier()
    }
}
impl CoCarrier for CircleRotorAligningOriginAtInfinity {
    type Output = MotorAtInfinity;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        2        0
    fn co_carrier(self) -> Self::Output {
        use crate::elements::*;
        return MotorAtInfinity::from_groups(
            // e235, e315, e125, e5
            crate::swizzle!(self.right_anti_dual().group0(), 0, 1, 2).extend_to_4(self.right_anti_dual()[scalar]),
        );
    }
}
impl std::ops::Div<co_carrier> for CircleRotorAtInfinity {
    type Output = MotorAtInfinity;
    fn div(self, _rhs: co_carrier) -> Self::Output {
        self.co_carrier()
    }
}
impl CoCarrier for CircleRotorAtInfinity {
    type Output = MotorAtInfinity;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        8        0
    fn co_carrier(self) -> Self::Output {
        use crate::elements::*;
        return MotorAtInfinity::from_groups(/* e235, e315, e125, e5 */ Simd32x4::from([
            self.right_anti_dual()[e23],
            self.right_anti_dual()[e31],
            self.right_anti_dual()[e12],
            self.right_anti_dual()[scalar],
        ]));
    }
}
impl std::ops::Div<co_carrier> for CircleRotorOnOrigin {
    type Output = Motor;
    fn div(self, _rhs: co_carrier) -> Self::Output {
        self.co_carrier()
    }
}
impl CoCarrier for CircleRotorOnOrigin {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        5        0
    fn co_carrier(self) -> Self::Output {
        use crate::elements::*;
        return Motor::from_groups(
            // e415, e425, e435, e12345
            Simd32x4::from([self.right_anti_dual()[e41], self.right_anti_dual()[e42], self.right_anti_dual()[e43], 0.0]),
            // e235, e315, e125, e5
            crate::swizzle!(self.right_anti_dual().group1(), 0, 1, 2).extend_to_4(self.right_anti_dual()[scalar]),
        );
    }
}
impl std::ops::Div<co_carrier> for Dipole {
    type Output = Plane;
    fn div(self, _rhs: co_carrier) -> Self::Output {
        self.co_carrier()
    }
}
impl CoCarrier for Dipole {
    type Output = Plane;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        6        0
    //    simd3        0        4        0
    // Totals...
    // yes simd        0       10        0
    //  no simd        0       18        0
    fn co_carrier(self) -> Self::Output {
        use crate::elements::*;
        return Plane::from_groups(
            // e4235, e4315, e4125, e3215
            crate::swizzle!(self.right_anti_dual().group0(), 0, 1, 2).extend_to_4(self.right_anti_dual()[e321]),
        );
    }
}
impl std::ops::Div<co_carrier> for DipoleAligningOrigin {
    type Output = CircleOrthogonalOrigin;
    fn div(self, _rhs: co_carrier) -> Self::Output {
        self.co_carrier()
    }
}
impl CoCarrier for DipoleAligningOrigin {
    type Output = CircleOrthogonalOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        3        0
    //    simd3        0        1        0
    // Totals...
    // yes simd        0        4        0
    //  no simd        0        6        0
    fn co_carrier(self) -> Self::Output {
        return self.right_anti_dual();
    }
}
impl std::ops::Div<co_carrier> for DipoleAtInfinity {
    type Output = Horizon;
    fn div(self, _rhs: co_carrier) -> Self::Output {
        self.co_carrier()
    }
}
impl CoCarrier for DipoleAtInfinity {
    type Output = Horizon;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        3        0
    //    simd3        0        1        0
    // Totals...
    // yes simd        0        4        0
    //  no simd        0        6        0
    fn co_carrier(self) -> Self::Output {
        use crate::elements::*;
        return Horizon::from_groups(/* e3215 */ self.right_anti_dual()[e321]);
    }
}
impl std::ops::Div<co_carrier> for DipoleAtOrigin {
    type Output = CircleAtOrigin;
    fn div(self, _rhs: co_carrier) -> Self::Output {
        self.co_carrier()
    }
}
impl CoCarrier for DipoleAtOrigin {
    type Output = CircleAtOrigin;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        2        0
    // no simd        0        6        0
    fn co_carrier(self) -> Self::Output {
        return self.right_anti_dual();
    }
}
impl std::ops::Div<co_carrier> for DipoleInversion {
    type Output = Flector;
    fn div(self, _rhs: co_carrier) -> Self::Output {
        self.co_carrier()
    }
}
impl CoCarrier for DipoleInversion {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0       54        0
    //    simd3        0        6        0
    // Totals...
    // yes simd        0       60        0
    //  no simd        0       72        0
    fn co_carrier(self) -> Self::Output {
        use crate::elements::*;
        return Flector::from_groups(
            // e15, e25, e35, e45
            Simd32x4::from([self.right_anti_dual()[e1], self.right_anti_dual()[e2], self.right_anti_dual()[e3], self.right_anti_dual()[e4]]),
            // e4235, e4315, e4125, e3215
            crate::swizzle!(self.right_anti_dual().group0(), 0, 1, 2).extend_to_4(self.right_anti_dual()[e321]),
        );
    }
}
impl std::ops::Div<co_carrier> for DipoleInversionAligningOrigin {
    type Output = Flector;
    fn div(self, _rhs: co_carrier) -> Self::Output {
        self.co_carrier()
    }
}
impl CoCarrier for DipoleInversionAligningOrigin {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0       18        0
    fn co_carrier(self) -> Self::Output {
        return Flector::from_groups(
            // e15, e25, e35, e45
            self.right_anti_dual().group2(),
            // e4235, e4315, e4125, e3215
            self.right_anti_dual().group0(),
        );
    }
}
impl std::ops::Div<co_carrier> for DipoleInversionAtInfinity {
    type Output = FlectorAtInfinity;
    fn div(self, _rhs: co_carrier) -> Self::Output {
        self.co_carrier()
    }
}
impl CoCarrier for DipoleInversionAtInfinity {
    type Output = FlectorAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0       24        0
    //    simd3        0        4        0
    // Totals...
    // yes simd        0       28        0
    //  no simd        0       36        0
    fn co_carrier(self) -> Self::Output {
        use crate::elements::*;
        return FlectorAtInfinity::from_groups(/* e15, e25, e35, e3215 */ Simd32x4::from([
            self.right_anti_dual()[e1],
            self.right_anti_dual()[e2],
            self.right_anti_dual()[e3],
            self.right_anti_dual()[e321],
        ]));
    }
}
impl std::ops::Div<co_carrier> for DipoleInversionAtOrigin {
    type Output = FlectorOnOrigin;
    fn div(self, _rhs: co_carrier) -> Self::Output {
        self.co_carrier()
    }
}
impl CoCarrier for DipoleInversionAtOrigin {
    type Output = FlectorOnOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0       24        0
    fn co_carrier(self) -> Self::Output {
        use crate::elements::*;
        return FlectorOnOrigin::from_groups(/* e45, e4235, e4315, e4125 */ Simd32x4::from([
            self.right_anti_dual()[e4],
            self.right_anti_dual()[e423],
            self.right_anti_dual()[e431],
            self.right_anti_dual()[e412],
        ]));
    }
}
impl std::ops::Div<co_carrier> for DipoleInversionOnOrigin {
    type Output = Flector;
    fn div(self, _rhs: co_carrier) -> Self::Output {
        self.co_carrier()
    }
}
impl CoCarrier for DipoleInversionOnOrigin {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0       30        0
    fn co_carrier(self) -> Self::Output {
        use crate::elements::*;
        return Flector::from_groups(
            // e15, e25, e35, e45
            Simd32x4::from([self.right_anti_dual()[e1], self.right_anti_dual()[e2], self.right_anti_dual()[e3], self.right_anti_dual()[e4]]),
            // e4235, e4315, e4125, e3215
            self.right_anti_dual().group0(),
        );
    }
}
impl std::ops::Div<co_carrier> for DipoleInversionOrthogonalOrigin {
    type Output = FlectorOnOrigin;
    fn div(self, _rhs: co_carrier) -> Self::Output {
        self.co_carrier()
    }
}
impl CoCarrier for DipoleInversionOrthogonalOrigin {
    type Output = FlectorOnOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0       24        0
    //    simd3        0        4        0
    // Totals...
    // yes simd        0       28        0
    //  no simd        0       36        0
    fn co_carrier(self) -> Self::Output {
        use crate::elements::*;
        return FlectorOnOrigin::from_groups(/* e45, e4235, e4315, e4125 */ Simd32x4::from([
            self.right_anti_dual()[e4],
            self.right_anti_dual()[e423],
            self.right_anti_dual()[e431],
            self.right_anti_dual()[e412],
        ]));
    }
}
impl std::ops::Div<co_carrier> for DipoleOnOrigin {
    type Output = AntiDipoleOnOrigin;
    fn div(self, _rhs: co_carrier) -> Self::Output {
        self.co_carrier()
    }
}
impl CoCarrier for DipoleOnOrigin {
    type Output = AntiDipoleOnOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        3        0
    fn co_carrier(self) -> Self::Output {
        return self.right_anti_dual();
    }
}
impl std::ops::Div<co_carrier> for DipoleOrthogonalOrigin {
    type Output = CircleAligningOrigin;
    fn div(self, _rhs: co_carrier) -> Self::Output {
        self.co_carrier()
    }
}
impl CoCarrier for DipoleOrthogonalOrigin {
    type Output = CircleAligningOrigin;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        3        0
    // no simd        0        9        0
    fn co_carrier(self) -> Self::Output {
        return self.right_anti_dual();
    }
}
impl std::ops::Div<co_carrier> for DualNum {
    type Output = Motor;
    fn div(self, _rhs: co_carrier) -> Self::Output {
        self.co_carrier()
    }
}
impl CoCarrier for DualNum {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd2        0        2        0
    // no simd        0        4        0
    fn co_carrier(self) -> Self::Output {
        use crate::elements::*;
        return Motor::from_groups(
            // e415, e425, e435, e12345
            Simd32x4::from([0.0, 0.0, 0.0, self.right_anti_dual()[e1234]]),
            // e235, e315, e125, e5
            Simd32x4::from([0.0, 0.0, 0.0, self.right_anti_dual()[scalar]]),
        );
    }
}
impl std::ops::Div<co_carrier> for FlatPoint {
    type Output = Horizon;
    fn div(self, _rhs: co_carrier) -> Self::Output {
        self.co_carrier()
    }
}
impl CoCarrier for FlatPoint {
    type Output = Horizon;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        3        0
    fn co_carrier(self) -> Self::Output {
        use crate::elements::*;
        return Horizon::from_groups(/* e3215 */ self.right_anti_dual()[e321]);
    }
}
impl std::ops::Div<co_carrier> for Flector {
    type Output = FlectorAtInfinity;
    fn div(self, _rhs: co_carrier) -> Self::Output {
        self.co_carrier()
    }
}
impl CoCarrier for Flector {
    type Output = FlectorAtInfinity;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0       24        0
    fn co_carrier(self) -> Self::Output {
        use crate::elements::*;
        return FlectorAtInfinity::from_groups(/* e15, e25, e35, e3215 */ Simd32x4::from([
            self.right_anti_dual()[e1],
            self.right_anti_dual()[e2],
            self.right_anti_dual()[e3],
            self.right_anti_dual()[e321],
        ]));
    }
}
impl std::ops::Div<co_carrier> for FlectorOnOrigin {
    type Output = FlectorAtInfinity;
    fn div(self, _rhs: co_carrier) -> Self::Output {
        self.co_carrier()
    }
}
impl CoCarrier for FlectorOnOrigin {
    type Output = FlectorAtInfinity;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0       12        0
    fn co_carrier(self) -> Self::Output {
        use crate::elements::*;
        return FlectorAtInfinity::from_groups(/* e15, e25, e35, e3215 */ Simd32x4::from([
            self.right_anti_dual()[e1],
            self.right_anti_dual()[e2],
            self.right_anti_dual()[e3],
            self.right_anti_dual()[e321],
        ]));
    }
}
impl std::ops::Div<co_carrier> for Motor {
    type Output = AntiMotor;
    fn div(self, _rhs: co_carrier) -> Self::Output {
        self.co_carrier()
    }
}
impl CoCarrier for Motor {
    type Output = AntiMotor;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        2        0
    fn co_carrier(self) -> Self::Output {
        return self.right_anti_dual();
    }
}
impl std::ops::Div<co_carrier> for MotorOnOrigin {
    type Output = AntiMotorOnOrigin;
    fn div(self, _rhs: co_carrier) -> Self::Output {
        self.co_carrier()
    }
}
impl CoCarrier for MotorOnOrigin {
    type Output = AntiMotorOnOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn co_carrier(self) -> Self::Output {
        return self.right_anti_dual();
    }
}
impl std::ops::Div<co_carrier> for MultiVector {
    type Output = MultiVector;
    fn div(self, _rhs: co_carrier) -> Self::Output {
        self.co_carrier()
    }
}
impl std::ops::DivAssign<co_carrier> for MultiVector {
    fn div_assign(&mut self, _rhs: co_carrier) {
        *self = self.co_carrier()
    }
}
impl CoCarrier for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0      140        0
    //    simd3        0       28        0
    // Totals...
    // yes simd        0      168        0
    //  no simd        0      224        0
    fn co_carrier(self) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, self.right_anti_dual()[e1234]]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            self.right_anti_dual()[scalar],
            // e41, e42, e43, e45
            Simd32x4::from([0.0, 0.0, 0.0, self.right_anti_dual()[e4]]),
            // e15, e25, e35
            Simd32x3::from([self.right_anti_dual()[e1], self.right_anti_dual()[e2], self.right_anti_dual()[e3]]),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from([self.right_anti_dual()[e41], self.right_anti_dual()[e42], self.right_anti_dual()[e43], 0.0]),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            self.right_anti_dual().group5(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([0.0, self.right_anti_dual()[e423], self.right_anti_dual()[e431], self.right_anti_dual()[e412]]),
            // e3215
            self.right_anti_dual()[e321],
        );
    }
}
impl std::ops::Div<co_carrier> for MysteryCircle {
    type Output = LineAtInfinity;
    fn div(self, _rhs: co_carrier) -> Self::Output {
        self.co_carrier()
    }
}
impl CoCarrier for MysteryCircle {
    type Output = LineAtInfinity;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        3        0
    fn co_carrier(self) -> Self::Output {
        use crate::elements::*;
        return LineAtInfinity::from_groups(
            // e235, e315, e125
            Simd32x3::from([self.right_anti_dual()[e23], self.right_anti_dual()[e31], self.right_anti_dual()[e12]]),
        );
    }
}
impl std::ops::Div<co_carrier> for MysteryCircleRotor {
    type Output = MotorAtInfinity;
    fn div(self, _rhs: co_carrier) -> Self::Output {
        self.co_carrier()
    }
}
impl CoCarrier for MysteryCircleRotor {
    type Output = MotorAtInfinity;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        8        0
    fn co_carrier(self) -> Self::Output {
        use crate::elements::*;
        return MotorAtInfinity::from_groups(/* e235, e315, e125, e5 */ Simd32x4::from([
            self.right_anti_dual()[e23],
            self.right_anti_dual()[e31],
            self.right_anti_dual()[e12],
            self.right_anti_dual()[scalar],
        ]));
    }
}
impl std::ops::Div<co_carrier> for MysteryDipole {
    type Output = Horizon;
    fn div(self, _rhs: co_carrier) -> Self::Output {
        self.co_carrier()
    }
}
impl CoCarrier for MysteryDipole {
    type Output = Horizon;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        3        0
    fn co_carrier(self) -> Self::Output {
        use crate::elements::*;
        return Horizon::from_groups(/* e3215 */ self.right_anti_dual()[e321]);
    }
}
impl std::ops::Div<co_carrier> for MysteryDipoleInversion {
    type Output = FlectorAtInfinity;
    fn div(self, _rhs: co_carrier) -> Self::Output {
        self.co_carrier()
    }
}
impl CoCarrier for MysteryDipoleInversion {
    type Output = FlectorAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        6        0
    //    simd3        0        2        0
    // Totals...
    // yes simd        0        8        0
    //  no simd        0       12        0
    fn co_carrier(self) -> Self::Output {
        use crate::elements::*;
        return FlectorAtInfinity::from_groups(
            // e15, e25, e35, e3215
            crate::swizzle!(self.right_anti_dual().group1(), 0, 1, 2).extend_to_4(self.right_anti_dual()[e321]),
        );
    }
}
impl std::ops::Div<co_carrier> for MysteryVersorEven {
    type Output = MotorAtInfinity;
    fn div(self, _rhs: co_carrier) -> Self::Output {
        self.co_carrier()
    }
}
impl CoCarrier for MysteryVersorEven {
    type Output = MotorAtInfinity;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        8        0
    fn co_carrier(self) -> Self::Output {
        use crate::elements::*;
        return MotorAtInfinity::from_groups(/* e235, e315, e125, e5 */ Simd32x4::from([
            self.right_anti_dual()[e23],
            self.right_anti_dual()[e31],
            self.right_anti_dual()[e12],
            self.right_anti_dual()[scalar],
        ]));
    }
}
impl std::ops::Div<co_carrier> for MysteryVersorOdd {
    type Output = FlectorAtInfinity;
    fn div(self, _rhs: co_carrier) -> Self::Output {
        self.co_carrier()
    }
}
impl CoCarrier for MysteryVersorOdd {
    type Output = FlectorAtInfinity;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0       24        0
    fn co_carrier(self) -> Self::Output {
        use crate::elements::*;
        return FlectorAtInfinity::from_groups(/* e15, e25, e35, e3215 */ Simd32x4::from([
            self.right_anti_dual()[e1],
            self.right_anti_dual()[e2],
            self.right_anti_dual()[e3],
            self.right_anti_dual()[e321],
        ]));
    }
}
impl std::ops::Div<co_carrier> for NullCircleAtOrigin {
    type Output = NullCircleAtOrigin;
    fn div(self, _rhs: co_carrier) -> Self::Output {
        self.co_carrier()
    }
}
impl std::ops::DivAssign<co_carrier> for NullCircleAtOrigin {
    fn div_assign(&mut self, _rhs: co_carrier) {
        *self = self.co_carrier()
    }
}
impl CoCarrier for NullCircleAtOrigin {
    type Output = NullCircleAtOrigin;
    fn co_carrier(self) -> Self::Output {
        return self.right_anti_dual();
    }
}
impl std::ops::Div<co_carrier> for NullDipoleAtOrigin {
    type Output = NullCircleAtOrigin;
    fn div(self, _rhs: co_carrier) -> Self::Output {
        self.co_carrier()
    }
}
impl CoCarrier for NullDipoleAtOrigin {
    type Output = NullCircleAtOrigin;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        1        0
    // no simd        0        3        0
    fn co_carrier(self) -> Self::Output {
        return self.right_anti_dual();
    }
}
impl std::ops::Div<co_carrier> for NullDipoleInversionAtOrigin {
    type Output = FlectorOnOrigin;
    fn div(self, _rhs: co_carrier) -> Self::Output {
        self.co_carrier()
    }
}
impl CoCarrier for NullDipoleInversionAtOrigin {
    type Output = FlectorOnOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0       12        0
    fn co_carrier(self) -> Self::Output {
        use crate::elements::*;
        return FlectorOnOrigin::from_groups(/* e45, e4235, e4315, e4125 */ Simd32x4::from([
            self.right_anti_dual()[e4],
            self.right_anti_dual()[e423],
            self.right_anti_dual()[e431],
            self.right_anti_dual()[e412],
        ]));
    }
}
impl std::ops::Div<co_carrier> for NullSphereAtOrigin {
    type Output = NullSphereAtOrigin;
    fn div(self, _rhs: co_carrier) -> Self::Output {
        self.co_carrier()
    }
}
impl std::ops::DivAssign<co_carrier> for NullSphereAtOrigin {
    fn div_assign(&mut self, _rhs: co_carrier) {
        *self = self.co_carrier()
    }
}
impl CoCarrier for NullSphereAtOrigin {
    type Output = NullSphereAtOrigin;
    fn co_carrier(self) -> Self::Output {
        return self.right_anti_dual();
    }
}
impl std::ops::Div<co_carrier> for NullVersorEvenAtOrigin {
    type Output = NullDipoleInversionAtOrigin;
    fn div(self, _rhs: co_carrier) -> Self::Output {
        self.co_carrier()
    }
}
impl CoCarrier for NullVersorEvenAtOrigin {
    type Output = NullDipoleInversionAtOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn co_carrier(self) -> Self::Output {
        return self.right_anti_dual();
    }
}
impl std::ops::Div<co_carrier> for Origin {
    type Output = NullSphereAtOrigin;
    fn div(self, _rhs: co_carrier) -> Self::Output {
        self.co_carrier()
    }
}
impl CoCarrier for Origin {
    type Output = NullSphereAtOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn co_carrier(self) -> Self::Output {
        return self.right_anti_dual();
    }
}
impl std::ops::Div<co_carrier> for Plane {
    type Output = FlatPointAtInfinity;
    fn div(self, _rhs: co_carrier) -> Self::Output {
        self.co_carrier()
    }
}
impl CoCarrier for Plane {
    type Output = FlatPointAtInfinity;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        9        0
    fn co_carrier(self) -> Self::Output {
        use crate::elements::*;
        return FlatPointAtInfinity::from_groups(
            // e15, e25, e35
            Simd32x3::from([self.right_anti_dual()[e1], self.right_anti_dual()[e2], self.right_anti_dual()[e3]]),
        );
    }
}
impl std::ops::Div<co_carrier> for PlaneOnOrigin {
    type Output = AntiPlaneOnOrigin;
    fn div(self, _rhs: co_carrier) -> Self::Output {
        self.co_carrier()
    }
}
impl CoCarrier for PlaneOnOrigin {
    type Output = AntiPlaneOnOrigin;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        1        0
    // no simd        0        3        0
    fn co_carrier(self) -> Self::Output {
        return self.right_anti_dual();
    }
}
impl std::ops::Div<co_carrier> for RoundPoint {
    type Output = AntiScalar;
    fn div(self, _rhs: co_carrier) -> Self::Output {
        self.co_carrier()
    }
}
impl CoCarrier for RoundPoint {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        2        0
    fn co_carrier(self) -> Self::Output {
        use crate::elements::*;
        return AntiScalar::from_groups(/* e12345 */ self.right_anti_dual()[e1234]);
    }
}
impl std::ops::Div<co_carrier> for RoundPointAtOrigin {
    type Output = AntiScalar;
    fn div(self, _rhs: co_carrier) -> Self::Output {
        self.co_carrier()
    }
}
impl CoCarrier for RoundPointAtOrigin {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd2        0        1        0
    // no simd        0        2        0
    fn co_carrier(self) -> Self::Output {
        use crate::elements::*;
        return AntiScalar::from_groups(/* e12345 */ self.right_anti_dual()[e1234]);
    }
}
impl std::ops::Div<co_carrier> for Scalar {
    type Output = Scalar;
    fn div(self, _rhs: co_carrier) -> Self::Output {
        self.co_carrier()
    }
}
impl std::ops::DivAssign<co_carrier> for Scalar {
    fn div_assign(&mut self, _rhs: co_carrier) {
        *self = self.co_carrier()
    }
}
impl CoCarrier for Scalar {
    type Output = Scalar;
    fn co_carrier(self) -> Self::Output {
        return self.right_anti_dual();
    }
}
impl std::ops::Div<co_carrier> for Sphere {
    type Output = RoundPoint;
    fn div(self, _rhs: co_carrier) -> Self::Output {
        self.co_carrier()
    }
}
impl CoCarrier for Sphere {
    type Output = RoundPoint;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        3        0
    fn co_carrier(self) -> Self::Output {
        return self.right_anti_dual();
    }
}
impl std::ops::Div<co_carrier> for SphereAtOrigin {
    type Output = FlatOrigin;
    fn div(self, _rhs: co_carrier) -> Self::Output {
        self.co_carrier()
    }
}
impl CoCarrier for SphereAtOrigin {
    type Output = FlatOrigin;
    fn co_carrier(self) -> Self::Output {
        use crate::elements::*;
        return FlatOrigin::from_groups(/* e45 */ self.right_anti_dual()[e4]);
    }
}
impl std::ops::Div<co_carrier> for SphereOnOrigin {
    type Output = AntiSphereOnOrigin;
    fn div(self, _rhs: co_carrier) -> Self::Output {
        self.co_carrier()
    }
}
impl CoCarrier for SphereOnOrigin {
    type Output = AntiSphereOnOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        3        0
    fn co_carrier(self) -> Self::Output {
        return self.right_anti_dual();
    }
}
impl std::ops::Div<co_carrier> for VersorEven {
    type Output = Motor;
    fn div(self, _rhs: co_carrier) -> Self::Output {
        self.co_carrier()
    }
}
impl CoCarrier for VersorEven {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0       32        0
    fn co_carrier(self) -> Self::Output {
        use crate::elements::*;
        return Motor::from_groups(
            // e415, e425, e435, e12345
            Simd32x4::from([self.right_anti_dual()[e41], self.right_anti_dual()[e42], self.right_anti_dual()[e43], self.right_anti_dual()[e1234]]),
            // e235, e315, e125, e5
            Simd32x4::from([self.right_anti_dual()[e23], self.right_anti_dual()[e31], self.right_anti_dual()[e12], self.right_anti_dual()[scalar]]),
        );
    }
}
impl std::ops::Div<co_carrier> for VersorEvenAligningOrigin {
    type Output = Motor;
    fn div(self, _rhs: co_carrier) -> Self::Output {
        self.co_carrier()
    }
}
impl CoCarrier for VersorEvenAligningOrigin {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0       24        0
    fn co_carrier(self) -> Self::Output {
        use crate::elements::*;
        return Motor::from_groups(
            // e415, e425, e435, e12345
            Simd32x4::from([self.right_anti_dual()[e41], self.right_anti_dual()[e42], self.right_anti_dual()[e43], self.right_anti_dual()[e1234]]),
            // e235, e315, e125, e5
            Simd32x4::from([self.right_anti_dual()[e23], self.right_anti_dual()[e31], self.right_anti_dual()[e12], self.right_anti_dual()[scalar]]),
        );
    }
}
impl std::ops::Div<co_carrier> for VersorEvenAtInfinity {
    type Output = MotorAtInfinity;
    fn div(self, _rhs: co_carrier) -> Self::Output {
        self.co_carrier()
    }
}
impl CoCarrier for VersorEvenAtInfinity {
    type Output = MotorAtInfinity;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0       12        0
    fn co_carrier(self) -> Self::Output {
        use crate::elements::*;
        return MotorAtInfinity::from_groups(/* e235, e315, e125, e5 */ Simd32x4::from([
            self.right_anti_dual()[e23],
            self.right_anti_dual()[e31],
            self.right_anti_dual()[e12],
            self.right_anti_dual()[scalar],
        ]));
    }
}
impl std::ops::Div<co_carrier> for VersorEvenAtOrigin {
    type Output = MotorOnOrigin;
    fn div(self, _rhs: co_carrier) -> Self::Output {
        self.co_carrier()
    }
}
impl CoCarrier for VersorEvenAtOrigin {
    type Output = MotorOnOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        8        0
    fn co_carrier(self) -> Self::Output {
        use crate::elements::*;
        return MotorOnOrigin::from_groups(/* e415, e425, e435, e12345 */ Simd32x4::from([
            self.right_anti_dual()[e41],
            self.right_anti_dual()[e42],
            self.right_anti_dual()[e43],
            self.right_anti_dual()[e1234],
        ]));
    }
}
impl std::ops::Div<co_carrier> for VersorEvenOnOrigin {
    type Output = Motor;
    fn div(self, _rhs: co_carrier) -> Self::Output {
        self.co_carrier()
    }
}
impl CoCarrier for VersorEvenOnOrigin {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0       16        0
    fn co_carrier(self) -> Self::Output {
        use crate::elements::*;
        return Motor::from_groups(
            // e415, e425, e435, e12345
            Simd32x4::from([self.right_anti_dual()[e41], self.right_anti_dual()[e42], self.right_anti_dual()[e43], self.right_anti_dual()[e1234]]),
            // e235, e315, e125, e5
            Simd32x4::from([self.right_anti_dual()[e23], self.right_anti_dual()[e31], self.right_anti_dual()[e12], self.right_anti_dual()[scalar]]),
        );
    }
}
impl std::ops::Div<co_carrier> for VersorEvenOrthogonalOrigin {
    type Output = MotorOnOrigin;
    fn div(self, _rhs: co_carrier) -> Self::Output {
        self.co_carrier()
    }
}
impl CoCarrier for VersorEvenOrthogonalOrigin {
    type Output = MotorOnOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0       12        0
    fn co_carrier(self) -> Self::Output {
        use crate::elements::*;
        return MotorOnOrigin::from_groups(/* e415, e425, e435, e12345 */ Simd32x4::from([
            self.right_anti_dual()[e41],
            self.right_anti_dual()[e42],
            self.right_anti_dual()[e43],
            self.right_anti_dual()[e1234],
        ]));
    }
}
impl std::ops::Div<co_carrier> for VersorOdd {
    type Output = Flector;
    fn div(self, _rhs: co_carrier) -> Self::Output {
        self.co_carrier()
    }
}
impl CoCarrier for VersorOdd {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0       60        0
    fn co_carrier(self) -> Self::Output {
        use crate::elements::*;
        return Flector::from_groups(
            // e15, e25, e35, e45
            self.right_anti_dual().group3(),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([self.right_anti_dual()[e423], self.right_anti_dual()[e431], self.right_anti_dual()[e412], self.right_anti_dual()[e321]]),
        );
    }
}
impl std::ops::Div<co_carrier> for VersorOddAtInfinity {
    type Output = FlectorAtInfinity;
    fn div(self, _rhs: co_carrier) -> Self::Output {
        self.co_carrier()
    }
}
impl CoCarrier for VersorOddAtInfinity {
    type Output = FlectorAtInfinity;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0       36        0
    fn co_carrier(self) -> Self::Output {
        use crate::elements::*;
        return FlectorAtInfinity::from_groups(/* e15, e25, e35, e3215 */ Simd32x4::from([
            self.right_anti_dual()[e1],
            self.right_anti_dual()[e2],
            self.right_anti_dual()[e3],
            self.right_anti_dual()[e321],
        ]));
    }
}
impl std::ops::Div<co_carrier> for VersorOddOrthogonalOrigin {
    type Output = FlectorOnOrigin;
    fn div(self, _rhs: co_carrier) -> Self::Output {
        self.co_carrier()
    }
}
impl CoCarrier for VersorOddOrthogonalOrigin {
    type Output = FlectorOnOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0       36        0
    fn co_carrier(self) -> Self::Output {
        use crate::elements::*;
        return FlectorOnOrigin::from_groups(/* e45, e4235, e4315, e4125 */ Simd32x4::from([
            self.right_anti_dual()[e4],
            self.right_anti_dual()[e423],
            self.right_anti_dual()[e431],
            self.right_anti_dual()[e412],
        ]));
    }
}
