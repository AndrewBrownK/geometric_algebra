// Note on Operative Statistics:
// Operative Statistics are not a precise predictor of performance or performance comparisons.
// This is due to varying hardware capabilities and compiler optimizations.
// As always, where performance is a concern, there is no substitute for
// real measurements on real work-loads on real hardware.
// Disclaimer aside, enjoy the fun information =)
//
// Total Implementations: 77
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
impl std::ops::Div<carrier> for AntiCircleOnOrigin {
    type Output = Line;
    fn div(self, _rhs: carrier) -> Self::Output {
        self.carrier()
    }
}
impl Carrier for AntiCircleOnOrigin {
    type Output = Line;
    fn carrier(self) -> Self::Output {
        return Line::from_groups(/* e415, e425, e435 */ self.group0(), /* e235, e315, e125 */ self.group1());
    }
}
impl std::ops::Div<carrier> for AntiCircleRotor {
    type Output = Motor;
    fn div(self, _rhs: carrier) -> Self::Output {
        self.carrier()
    }
}
impl Carrier for AntiCircleRotor {
    type Output = Motor;
    fn carrier(self) -> Self::Output {
        use crate::elements::*;
        return Motor::from_groups(
            // e415, e425, e435, e12345
            self.group0().extend_to_4(0.0),
            // e235, e315, e125, e5
            Simd32x4::from([self[e23], self[e31], self[e12], self[scalar]]),
        );
    }
}
impl std::ops::Div<carrier> for AntiCircleRotorAligningOrigin {
    type Output = Motor;
    fn div(self, _rhs: carrier) -> Self::Output {
        self.carrier()
    }
}
impl Carrier for AntiCircleRotorAligningOrigin {
    type Output = Motor;
    fn carrier(self) -> Self::Output {
        use crate::elements::*;
        return Motor::from_groups(
            // e415, e425, e435, e12345
            self.group0().extend_to_4(0.0),
            // e235, e315, e125, e5
            self.group1().extend_to_4(self[scalar]),
        );
    }
}
impl std::ops::Div<carrier> for AntiCircleRotorAligningOriginAtInfinity {
    type Output = MotorAtInfinity;
    fn div(self, _rhs: carrier) -> Self::Output {
        self.carrier()
    }
}
impl Carrier for AntiCircleRotorAligningOriginAtInfinity {
    type Output = MotorAtInfinity;
    fn carrier(self) -> Self::Output {
        use crate::elements::*;
        return MotorAtInfinity::from_groups(/* e235, e315, e125, e5 */ self.group0().extend_to_4(self[scalar]));
    }
}
impl std::ops::Div<carrier> for AntiCircleRotorAtInfinity {
    type Output = MotorAtInfinity;
    fn div(self, _rhs: carrier) -> Self::Output {
        self.carrier()
    }
}
impl Carrier for AntiCircleRotorAtInfinity {
    type Output = MotorAtInfinity;
    fn carrier(self) -> Self::Output {
        use crate::elements::*;
        return MotorAtInfinity::from_groups(/* e235, e315, e125, e5 */ Simd32x4::from([self[e23], self[e31], self[e12], self[scalar]]));
    }
}
impl std::ops::Div<carrier> for AntiCircleRotorOnOrigin {
    type Output = Motor;
    fn div(self, _rhs: carrier) -> Self::Output {
        self.carrier()
    }
}
impl Carrier for AntiCircleRotorOnOrigin {
    type Output = Motor;
    fn carrier(self) -> Self::Output {
        use crate::elements::*;
        return Motor::from_groups(
            // e415, e425, e435, e12345
            Simd32x4::from([self[e41], self[e42], self[e43], 0.0]),
            // e235, e315, e125, e5
            self.group1().extend_to_4(self[scalar]),
        );
    }
}
impl std::ops::Div<carrier> for AntiDipoleInversion {
    type Output = Flector;
    fn div(self, _rhs: carrier) -> Self::Output {
        self.carrier()
    }
}
impl Carrier for AntiDipoleInversion {
    type Output = Flector;
    fn carrier(self) -> Self::Output {
        use crate::elements::*;
        return Flector::from_groups(
            // e15, e25, e35, e45
            Simd32x4::from([self[e1], self[e2], self[e3], self[e4]]),
            // e4235, e4315, e4125, e3215
            self.group0().extend_to_4(self[e321]),
        );
    }
}
impl std::ops::Div<carrier> for AntiDipoleInversionAtInfinity {
    type Output = FlectorAtInfinity;
    fn div(self, _rhs: carrier) -> Self::Output {
        self.carrier()
    }
}
impl Carrier for AntiDipoleInversionAtInfinity {
    type Output = FlectorAtInfinity;
    fn carrier(self) -> Self::Output {
        use crate::elements::*;
        return FlectorAtInfinity::from_groups(/* e15, e25, e35, e3215 */ Simd32x4::from([self[e1], self[e2], self[e3], self[e321]]));
    }
}
impl std::ops::Div<carrier> for AntiDipoleInversionOnOrigin {
    type Output = Flector;
    fn div(self, _rhs: carrier) -> Self::Output {
        self.carrier()
    }
}
impl Carrier for AntiDipoleInversionOnOrigin {
    type Output = Flector;
    fn carrier(self) -> Self::Output {
        return Flector::from_groups(
            // e15, e25, e35, e45
            crate::swizzle!(self.group1(), 1, 2, 3, 0),
            // e4235, e4315, e4125, e3215
            self.group0(),
        );
    }
}
impl std::ops::Div<carrier> for AntiDipoleInversionOrthogonalOrigin {
    type Output = FlectorOnOrigin;
    fn div(self, _rhs: carrier) -> Self::Output {
        self.carrier()
    }
}
impl Carrier for AntiDipoleInversionOrthogonalOrigin {
    type Output = FlectorOnOrigin;
    fn carrier(self) -> Self::Output {
        use crate::elements::*;
        return FlectorOnOrigin::from_groups(/* e45, e4235, e4315, e4125 */ Simd32x4::from([self[e4], self[e423], self[e431], self[e412]]));
    }
}
impl std::ops::Div<carrier> for AntiDipoleOnOrigin {
    type Output = Plane;
    fn div(self, _rhs: carrier) -> Self::Output {
        self.carrier()
    }
}
impl Carrier for AntiDipoleOnOrigin {
    type Output = Plane;
    fn carrier(self) -> Self::Output {
        return Plane::from_groups(/* e4235, e4315, e4125, e3215 */ self.group0());
    }
}
impl std::ops::Div<carrier> for AntiDualNum {
    type Output = Motor;
    fn div(self, _rhs: carrier) -> Self::Output {
        self.carrier()
    }
}
impl Carrier for AntiDualNum {
    type Output = Motor;
    fn carrier(self) -> Self::Output {
        use crate::elements::*;
        return Motor::from_groups(
            // e415, e425, e435, e12345
            Simd32x3::from(0.0).extend_to_4(self[e1234]),
            // e235, e315, e125, e5
            Simd32x3::from(0.0).extend_to_4(self[scalar]),
        );
    }
}
impl std::ops::Div<carrier> for AntiFlatOrigin {
    type Output = Horizon;
    fn div(self, _rhs: carrier) -> Self::Output {
        self.carrier()
    }
}
impl Carrier for AntiFlatOrigin {
    type Output = Horizon;
    fn carrier(self) -> Self::Output {
        use crate::elements::*;
        return Horizon::from_groups(/* e3215 */ self[e321]);
    }
}
impl std::ops::Div<carrier> for AntiFlatPoint {
    type Output = Horizon;
    fn div(self, _rhs: carrier) -> Self::Output {
        self.carrier()
    }
}
impl Carrier for AntiFlatPoint {
    type Output = Horizon;
    fn carrier(self) -> Self::Output {
        use crate::elements::*;
        return Horizon::from_groups(/* e3215 */ self[e321]);
    }
}
impl std::ops::Div<carrier> for AntiFlector {
    type Output = FlectorAtInfinity;
    fn div(self, _rhs: carrier) -> Self::Output {
        self.carrier()
    }
}
impl Carrier for AntiFlector {
    type Output = FlectorAtInfinity;
    fn carrier(self) -> Self::Output {
        use crate::elements::*;
        return FlectorAtInfinity::from_groups(/* e15, e25, e35, e3215 */ Simd32x4::from([self[e1], self[e2], self[e3], self[e321]]));
    }
}
impl std::ops::Div<carrier> for AntiFlectorOnOrigin {
    type Output = FlectorAtInfinity;
    fn div(self, _rhs: carrier) -> Self::Output {
        self.carrier()
    }
}
impl Carrier for AntiFlectorOnOrigin {
    type Output = FlectorAtInfinity;
    fn carrier(self) -> Self::Output {
        return FlectorAtInfinity::from_groups(/* e15, e25, e35, e3215 */ crate::swizzle!(self.group0(), 1, 2, 3, 0));
    }
}
impl std::ops::Div<carrier> for AntiLine {
    type Output = LineAtInfinity;
    fn div(self, _rhs: carrier) -> Self::Output {
        self.carrier()
    }
}
impl Carrier for AntiLine {
    type Output = LineAtInfinity;
    fn carrier(self) -> Self::Output {
        return LineAtInfinity::from_groups(/* e235, e315, e125 */ self.group0());
    }
}
impl std::ops::Div<carrier> for AntiLineOnOrigin {
    type Output = LineAtInfinity;
    fn div(self, _rhs: carrier) -> Self::Output {
        self.carrier()
    }
}
impl Carrier for AntiLineOnOrigin {
    type Output = LineAtInfinity;
    fn carrier(self) -> Self::Output {
        return LineAtInfinity::from_groups(/* e235, e315, e125 */ self.group0());
    }
}
impl std::ops::Div<carrier> for AntiMotor {
    type Output = MotorAtInfinity;
    fn div(self, _rhs: carrier) -> Self::Output {
        self.carrier()
    }
}
impl Carrier for AntiMotor {
    type Output = MotorAtInfinity;
    fn carrier(self) -> Self::Output {
        return MotorAtInfinity::from_groups(/* e235, e315, e125, e5 */ self.group0());
    }
}
impl std::ops::Div<carrier> for AntiMotorOnOrigin {
    type Output = MotorAtInfinity;
    fn div(self, _rhs: carrier) -> Self::Output {
        self.carrier()
    }
}
impl Carrier for AntiMotorOnOrigin {
    type Output = MotorAtInfinity;
    fn carrier(self) -> Self::Output {
        return MotorAtInfinity::from_groups(/* e235, e315, e125, e5 */ self.group0());
    }
}
impl std::ops::Div<carrier> for AntiMysteryCircleRotor {
    type Output = MotorAtInfinity;
    fn div(self, _rhs: carrier) -> Self::Output {
        self.carrier()
    }
}
impl Carrier for AntiMysteryCircleRotor {
    type Output = MotorAtInfinity;
    fn carrier(self) -> Self::Output {
        use crate::elements::*;
        return MotorAtInfinity::from_groups(/* e235, e315, e125, e5 */ Simd32x4::from([self[e23], self[e31], self[e12], self[scalar]]));
    }
}
impl std::ops::Div<carrier> for AntiMysteryDipoleInversion {
    type Output = FlectorAtInfinity;
    fn div(self, _rhs: carrier) -> Self::Output {
        self.carrier()
    }
}
impl Carrier for AntiMysteryDipoleInversion {
    type Output = FlectorAtInfinity;
    fn carrier(self) -> Self::Output {
        use crate::elements::*;
        return FlectorAtInfinity::from_groups(/* e15, e25, e35, e3215 */ self.group1().extend_to_4(self[e321]));
    }
}
impl std::ops::Div<carrier> for AntiPlane {
    type Output = FlatPointAtInfinity;
    fn div(self, _rhs: carrier) -> Self::Output {
        self.carrier()
    }
}
impl Carrier for AntiPlane {
    type Output = FlatPointAtInfinity;
    fn carrier(self) -> Self::Output {
        return FlatPointAtInfinity::from_groups(/* e15, e25, e35 */ self.group0().truncate_to_3());
    }
}
impl std::ops::Div<carrier> for AntiPlaneOnOrigin {
    type Output = FlatPointAtInfinity;
    fn div(self, _rhs: carrier) -> Self::Output {
        self.carrier()
    }
}
impl Carrier for AntiPlaneOnOrigin {
    type Output = FlatPointAtInfinity;
    fn carrier(self) -> Self::Output {
        return FlatPointAtInfinity::from_groups(/* e15, e25, e35 */ self.group0());
    }
}
impl std::ops::Div<carrier> for AntiSphereOnOrigin {
    type Output = FlatPoint;
    fn div(self, _rhs: carrier) -> Self::Output {
        self.carrier()
    }
}
impl Carrier for AntiSphereOnOrigin {
    type Output = FlatPoint;
    fn carrier(self) -> Self::Output {
        return FlatPoint::from_groups(/* e15, e25, e35, e45 */ self.group0());
    }
}
impl std::ops::Div<carrier> for AntiVersorEvenOnOrigin {
    type Output = Motor;
    fn div(self, _rhs: carrier) -> Self::Output {
        self.carrier()
    }
}
impl Carrier for AntiVersorEvenOnOrigin {
    type Output = Motor;
    fn carrier(self) -> Self::Output {
        use crate::elements::*;
        return Motor::from_groups(
            // e415, e425, e435, e12345
            Simd32x4::from([self[e41], self[e42], self[e43], self[e1234]]),
            // e235, e315, e125, e5
            Simd32x4::from([self[e23], self[e31], self[e12], self[scalar]]),
        );
    }
}
impl std::ops::Div<carrier> for Circle {
    type Output = Plane;
    fn div(self, _rhs: carrier) -> Self::Output {
        self.carrier()
    }
}
impl Carrier for Circle {
    type Output = Plane;
    fn carrier(self) -> Self::Output {
        use crate::elements::*;
        return Plane::from_groups(/* e4235, e4315, e4125, e3215 */ self.group0().extend_to_4(self[e321]));
    }
}
impl std::ops::Div<carrier> for CircleAligningOrigin {
    type Output = PlaneOnOrigin;
    fn div(self, _rhs: carrier) -> Self::Output {
        self.carrier()
    }
}
impl Carrier for CircleAligningOrigin {
    type Output = PlaneOnOrigin;
    fn carrier(self) -> Self::Output {
        return PlaneOnOrigin::from_groups(/* e4235, e4315, e4125 */ self.group0());
    }
}
impl std::ops::Div<carrier> for CircleAtInfinity {
    type Output = Horizon;
    fn div(self, _rhs: carrier) -> Self::Output {
        self.carrier()
    }
}
impl Carrier for CircleAtInfinity {
    type Output = Horizon;
    fn carrier(self) -> Self::Output {
        use crate::elements::*;
        return Horizon::from_groups(/* e3215 */ self[e321]);
    }
}
impl std::ops::Div<carrier> for CircleAtOrigin {
    type Output = PlaneOnOrigin;
    fn div(self, _rhs: carrier) -> Self::Output {
        self.carrier()
    }
}
impl Carrier for CircleAtOrigin {
    type Output = PlaneOnOrigin;
    fn carrier(self) -> Self::Output {
        return PlaneOnOrigin::from_groups(/* e4235, e4315, e4125 */ self.group0());
    }
}
impl std::ops::Div<carrier> for CircleOnOrigin {
    type Output = PlaneOnOrigin;
    fn div(self, _rhs: carrier) -> Self::Output {
        self.carrier()
    }
}
impl Carrier for CircleOnOrigin {
    type Output = PlaneOnOrigin;
    fn carrier(self) -> Self::Output {
        return PlaneOnOrigin::from_groups(/* e4235, e4315, e4125 */ self.group0());
    }
}
impl std::ops::Div<carrier> for CircleOrthogonalOrigin {
    type Output = Plane;
    fn div(self, _rhs: carrier) -> Self::Output {
        self.carrier()
    }
}
impl Carrier for CircleOrthogonalOrigin {
    type Output = Plane;
    fn carrier(self) -> Self::Output {
        return Plane::from_groups(/* e4235, e4315, e4125, e3215 */ self.group0());
    }
}
impl std::ops::Div<carrier> for CircleRotor {
    type Output = Plane;
    fn div(self, _rhs: carrier) -> Self::Output {
        self.carrier()
    }
}
impl Carrier for CircleRotor {
    type Output = Plane;
    fn carrier(self) -> Self::Output {
        use crate::elements::*;
        return Plane::from_groups(/* e4235, e4315, e4125, e3215 */ self.group0().extend_to_4(self[e321]));
    }
}
impl std::ops::Div<carrier> for CircleRotorAligningOrigin {
    type Output = PlaneOnOrigin;
    fn div(self, _rhs: carrier) -> Self::Output {
        self.carrier()
    }
}
impl Carrier for CircleRotorAligningOrigin {
    type Output = PlaneOnOrigin;
    fn carrier(self) -> Self::Output {
        return PlaneOnOrigin::from_groups(/* e4235, e4315, e4125 */ self.group0());
    }
}
impl std::ops::Div<carrier> for CircleRotorAtInfinity {
    type Output = Horizon;
    fn div(self, _rhs: carrier) -> Self::Output {
        self.carrier()
    }
}
impl Carrier for CircleRotorAtInfinity {
    type Output = Horizon;
    fn carrier(self) -> Self::Output {
        use crate::elements::*;
        return Horizon::from_groups(/* e3215 */ self[e321]);
    }
}
impl std::ops::Div<carrier> for CircleRotorOnOrigin {
    type Output = PlaneOnOrigin;
    fn div(self, _rhs: carrier) -> Self::Output {
        self.carrier()
    }
}
impl Carrier for CircleRotorOnOrigin {
    type Output = PlaneOnOrigin;
    fn carrier(self) -> Self::Output {
        return PlaneOnOrigin::from_groups(/* e4235, e4315, e4125 */ self.group0().truncate_to_3());
    }
}
impl std::ops::Div<carrier> for Dipole {
    type Output = Line;
    fn div(self, _rhs: carrier) -> Self::Output {
        self.carrier()
    }
}
impl Carrier for Dipole {
    type Output = Line;
    fn carrier(self) -> Self::Output {
        return Line::from_groups(/* e415, e425, e435 */ self.group0(), /* e235, e315, e125 */ self.group1().truncate_to_3());
    }
}
impl std::ops::Div<carrier> for DipoleAligningOrigin {
    type Output = LineOnOrigin;
    fn div(self, _rhs: carrier) -> Self::Output {
        self.carrier()
    }
}
impl Carrier for DipoleAligningOrigin {
    type Output = LineOnOrigin;
    fn carrier(self) -> Self::Output {
        return LineOnOrigin::from_groups(/* e415, e425, e435 */ self.group0().truncate_to_3());
    }
}
impl std::ops::Div<carrier> for DipoleAtInfinity {
    type Output = LineAtInfinity;
    fn div(self, _rhs: carrier) -> Self::Output {
        self.carrier()
    }
}
impl Carrier for DipoleAtInfinity {
    type Output = LineAtInfinity;
    fn carrier(self) -> Self::Output {
        return LineAtInfinity::from_groups(/* e235, e315, e125 */ self.group0().truncate_to_3());
    }
}
impl std::ops::Div<carrier> for DipoleAtOrigin {
    type Output = LineOnOrigin;
    fn div(self, _rhs: carrier) -> Self::Output {
        self.carrier()
    }
}
impl Carrier for DipoleAtOrigin {
    type Output = LineOnOrigin;
    fn carrier(self) -> Self::Output {
        return LineOnOrigin::from_groups(/* e415, e425, e435 */ self.group0());
    }
}
impl std::ops::Div<carrier> for DipoleInversion {
    type Output = CircleRotorAligningOriginAtInfinity;
    fn div(self, _rhs: carrier) -> Self::Output {
        self.carrier()
    }
}
impl Carrier for DipoleInversion {
    type Output = CircleRotorAligningOriginAtInfinity;
    fn carrier(self) -> Self::Output {
        use crate::elements::*;
        return CircleRotorAligningOriginAtInfinity::from_groups(
            // e415, e425, e435
            self.group0(),
            // e235, e315, e125, e12345
            Simd32x4::from([self[e23], self[e31], self[e12], self[e1234]]),
        );
    }
}
impl std::ops::Div<carrier> for DipoleInversionAligningOrigin {
    type Output = MotorOnOrigin;
    fn div(self, _rhs: carrier) -> Self::Output {
        self.carrier()
    }
}
impl Carrier for DipoleInversionAligningOrigin {
    type Output = MotorOnOrigin;
    fn carrier(self) -> Self::Output {
        use crate::elements::*;
        return MotorOnOrigin::from_groups(/* e415, e425, e435, e12345 */ Simd32x4::from([self[e41], self[e42], self[e43], self[e1234]]));
    }
}
impl std::ops::Div<carrier> for DipoleInversionAtInfinity {
    type Output = LineAtInfinity;
    fn div(self, _rhs: carrier) -> Self::Output {
        self.carrier()
    }
}
impl Carrier for DipoleInversionAtInfinity {
    type Output = LineAtInfinity;
    fn carrier(self) -> Self::Output {
        return LineAtInfinity::from_groups(/* e235, e315, e125 */ self.group0().truncate_to_3());
    }
}
impl std::ops::Div<carrier> for DipoleInversionAtOrigin {
    type Output = MotorOnOrigin;
    fn div(self, _rhs: carrier) -> Self::Output {
        self.carrier()
    }
}
impl Carrier for DipoleInversionAtOrigin {
    type Output = MotorOnOrigin;
    fn carrier(self) -> Self::Output {
        use crate::elements::*;
        return MotorOnOrigin::from_groups(/* e415, e425, e435, e12345 */ Simd32x4::from([self[e41], self[e42], self[e43], self[e1234]]));
    }
}
impl std::ops::Div<carrier> for DipoleInversionOnOrigin {
    type Output = MotorOnOrigin;
    fn div(self, _rhs: carrier) -> Self::Output {
        self.carrier()
    }
}
impl Carrier for DipoleInversionOnOrigin {
    type Output = MotorOnOrigin;
    fn carrier(self) -> Self::Output {
        use crate::elements::*;
        return MotorOnOrigin::from_groups(/* e415, e425, e435, e12345 */ Simd32x4::from([self[e41], self[e42], self[e43], self[e1234]]));
    }
}
impl std::ops::Div<carrier> for DipoleInversionOrthogonalOrigin {
    type Output = CircleRotorAligningOriginAtInfinity;
    fn div(self, _rhs: carrier) -> Self::Output {
        self.carrier()
    }
}
impl Carrier for DipoleInversionOrthogonalOrigin {
    type Output = CircleRotorAligningOriginAtInfinity;
    fn carrier(self) -> Self::Output {
        use crate::elements::*;
        return CircleRotorAligningOriginAtInfinity::from_groups(
            // e415, e425, e435
            self.group0().truncate_to_3(),
            // e235, e315, e125, e12345
            self.group1().extend_to_4(self[e1234]),
        );
    }
}
impl std::ops::Div<carrier> for DipoleOnOrigin {
    type Output = LineOnOrigin;
    fn div(self, _rhs: carrier) -> Self::Output {
        self.carrier()
    }
}
impl Carrier for DipoleOnOrigin {
    type Output = LineOnOrigin;
    fn carrier(self) -> Self::Output {
        return LineOnOrigin::from_groups(/* e415, e425, e435 */ self.group0().truncate_to_3());
    }
}
impl std::ops::Div<carrier> for DipoleOrthogonalOrigin {
    type Output = Line;
    fn div(self, _rhs: carrier) -> Self::Output {
        self.carrier()
    }
}
impl Carrier for DipoleOrthogonalOrigin {
    type Output = Line;
    fn carrier(self) -> Self::Output {
        return Line::from_groups(/* e415, e425, e435 */ self.group0(), /* e235, e315, e125 */ self.group1());
    }
}
impl std::ops::Div<carrier> for DualNum {
    type Output = FlatOrigin;
    fn div(self, _rhs: carrier) -> Self::Output {
        self.carrier()
    }
}
impl Carrier for DualNum {
    type Output = FlatOrigin;
    fn carrier(self) -> Self::Output {
        use crate::elements::*;
        return FlatOrigin::from_groups(/* e45 */ self[e4]);
    }
}
impl std::ops::Div<carrier> for MultiVector {
    type Output = MultiVector;
    fn div(self, _rhs: carrier) -> Self::Output {
        self.carrier()
    }
}
impl std::ops::DivAssign<carrier> for MultiVector {
    fn div_assign(&mut self, _rhs: carrier) {
        *self = self.carrier()
    }
}
impl Carrier for MultiVector {
    type Output = MultiVector;
    fn carrier(self) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, self[e1234]]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            self[scalar],
            // e41, e42, e43, e45
            Simd32x3::from(0.0).extend_to_4(self[e4]),
            // e15, e25, e35
            self.group1().truncate_to_3(),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from([self[e41], self[e42], self[e43], 0.0]),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            self.group5(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([0.0, self[e423], self[e431], self[e412]]),
            // e3215
            self[e321],
        );
    }
}
impl std::ops::Div<carrier> for MysteryCircle {
    type Output = Horizon;
    fn div(self, _rhs: carrier) -> Self::Output {
        self.carrier()
    }
}
impl Carrier for MysteryCircle {
    type Output = Horizon;
    fn carrier(self) -> Self::Output {
        use crate::elements::*;
        return Horizon::from_groups(/* e3215 */ self[e321]);
    }
}
impl std::ops::Div<carrier> for MysteryCircleRotor {
    type Output = Horizon;
    fn div(self, _rhs: carrier) -> Self::Output {
        self.carrier()
    }
}
impl Carrier for MysteryCircleRotor {
    type Output = Horizon;
    fn carrier(self) -> Self::Output {
        use crate::elements::*;
        return Horizon::from_groups(/* e3215 */ self[e321]);
    }
}
impl std::ops::Div<carrier> for MysteryDipole {
    type Output = LineAtInfinity;
    fn div(self, _rhs: carrier) -> Self::Output {
        self.carrier()
    }
}
impl Carrier for MysteryDipole {
    type Output = LineAtInfinity;
    fn carrier(self) -> Self::Output {
        return LineAtInfinity::from_groups(/* e235, e315, e125 */ self.group0().truncate_to_3());
    }
}
impl std::ops::Div<carrier> for MysteryDipoleInversion {
    type Output = LineAtInfinity;
    fn div(self, _rhs: carrier) -> Self::Output {
        self.carrier()
    }
}
impl Carrier for MysteryDipoleInversion {
    type Output = LineAtInfinity;
    fn carrier(self) -> Self::Output {
        return LineAtInfinity::from_groups(/* e235, e315, e125 */ self.group0().truncate_to_3());
    }
}
impl std::ops::Div<carrier> for MysteryVersorEven {
    type Output = FlectorAtInfinity;
    fn div(self, _rhs: carrier) -> Self::Output {
        self.carrier()
    }
}
impl Carrier for MysteryVersorEven {
    type Output = FlectorAtInfinity;
    fn carrier(self) -> Self::Output {
        use crate::elements::*;
        return FlectorAtInfinity::from_groups(/* e15, e25, e35, e3215 */ Simd32x4::from([self[e1], self[e2], self[e3], self[e321]]));
    }
}
impl std::ops::Div<carrier> for MysteryVersorOdd {
    type Output = MotorAtInfinity;
    fn div(self, _rhs: carrier) -> Self::Output {
        self.carrier()
    }
}
impl Carrier for MysteryVersorOdd {
    type Output = MotorAtInfinity;
    fn carrier(self) -> Self::Output {
        use crate::elements::*;
        return MotorAtInfinity::from_groups(/* e235, e315, e125, e5 */ Simd32x4::from([self[e23], self[e31], self[e12], self[scalar]]));
    }
}
impl std::ops::Div<carrier> for NullCircleAtOrigin {
    type Output = PlaneOnOrigin;
    fn div(self, _rhs: carrier) -> Self::Output {
        self.carrier()
    }
}
impl Carrier for NullCircleAtOrigin {
    type Output = PlaneOnOrigin;
    fn carrier(self) -> Self::Output {
        return PlaneOnOrigin::from_groups(/* e4235, e4315, e4125 */ self.group0());
    }
}
impl std::ops::Div<carrier> for NullDipoleAtOrigin {
    type Output = LineOnOrigin;
    fn div(self, _rhs: carrier) -> Self::Output {
        self.carrier()
    }
}
impl Carrier for NullDipoleAtOrigin {
    type Output = LineOnOrigin;
    fn carrier(self) -> Self::Output {
        return LineOnOrigin::from_groups(/* e415, e425, e435 */ self.group0());
    }
}
impl std::ops::Div<carrier> for NullDipoleInversionAtOrigin {
    type Output = MotorOnOrigin;
    fn div(self, _rhs: carrier) -> Self::Output {
        self.carrier()
    }
}
impl Carrier for NullDipoleInversionAtOrigin {
    type Output = MotorOnOrigin;
    fn carrier(self) -> Self::Output {
        return MotorOnOrigin::from_groups(/* e415, e425, e435, e12345 */ self.group0());
    }
}
impl std::ops::Div<carrier> for NullSphereAtOrigin {
    type Output = AntiScalar;
    fn div(self, _rhs: carrier) -> Self::Output {
        self.carrier()
    }
}
impl Carrier for NullSphereAtOrigin {
    type Output = AntiScalar;
    fn carrier(self) -> Self::Output {
        use crate::elements::*;
        return AntiScalar::from_groups(/* e12345 */ self[e1234]);
    }
}
impl std::ops::Div<carrier> for NullVersorEvenAtOrigin {
    type Output = FlectorOnOrigin;
    fn div(self, _rhs: carrier) -> Self::Output {
        self.carrier()
    }
}
impl Carrier for NullVersorEvenAtOrigin {
    type Output = FlectorOnOrigin;
    fn carrier(self) -> Self::Output {
        return FlectorOnOrigin::from_groups(/* e45, e4235, e4315, e4125 */ crate::swizzle!(self.group0(), 3, 0, 1, 2));
    }
}
impl std::ops::Div<carrier> for Origin {
    type Output = FlatOrigin;
    fn div(self, _rhs: carrier) -> Self::Output {
        self.carrier()
    }
}
impl Carrier for Origin {
    type Output = FlatOrigin;
    fn carrier(self) -> Self::Output {
        use crate::elements::*;
        return FlatOrigin::from_groups(/* e45 */ self[e4]);
    }
}
impl std::ops::Div<carrier> for RoundPoint {
    type Output = FlatPoint;
    fn div(self, _rhs: carrier) -> Self::Output {
        self.carrier()
    }
}
impl Carrier for RoundPoint {
    type Output = FlatPoint;
    fn carrier(self) -> Self::Output {
        return FlatPoint::from_groups(/* e15, e25, e35, e45 */ self.group0());
    }
}
impl std::ops::Div<carrier> for RoundPointAtOrigin {
    type Output = FlatOrigin;
    fn div(self, _rhs: carrier) -> Self::Output {
        self.carrier()
    }
}
impl Carrier for RoundPointAtOrigin {
    type Output = FlatOrigin;
    fn carrier(self) -> Self::Output {
        use crate::elements::*;
        return FlatOrigin::from_groups(/* e45 */ self[e4]);
    }
}
impl std::ops::Div<carrier> for Scalar {
    type Output = Infinity;
    fn div(self, _rhs: carrier) -> Self::Output {
        self.carrier()
    }
}
impl Carrier for Scalar {
    type Output = Infinity;
    fn carrier(self) -> Self::Output {
        use crate::elements::*;
        return Infinity::from_groups(/* e5 */ self[scalar]);
    }
}
impl std::ops::Div<carrier> for Sphere {
    type Output = AntiScalar;
    fn div(self, _rhs: carrier) -> Self::Output {
        self.carrier()
    }
}
impl Carrier for Sphere {
    type Output = AntiScalar;
    fn carrier(self) -> Self::Output {
        use crate::elements::*;
        return AntiScalar::from_groups(/* e12345 */ self[e1234]);
    }
}
impl std::ops::Div<carrier> for SphereAtOrigin {
    type Output = AntiScalar;
    fn div(self, _rhs: carrier) -> Self::Output {
        self.carrier()
    }
}
impl Carrier for SphereAtOrigin {
    type Output = AntiScalar;
    fn carrier(self) -> Self::Output {
        use crate::elements::*;
        return AntiScalar::from_groups(/* e12345 */ self[e1234]);
    }
}
impl std::ops::Div<carrier> for SphereOnOrigin {
    type Output = AntiScalar;
    fn div(self, _rhs: carrier) -> Self::Output {
        self.carrier()
    }
}
impl Carrier for SphereOnOrigin {
    type Output = AntiScalar;
    fn carrier(self) -> Self::Output {
        use crate::elements::*;
        return AntiScalar::from_groups(/* e12345 */ self[e1234]);
    }
}
impl std::ops::Div<carrier> for VersorEven {
    type Output = Flector;
    fn div(self, _rhs: carrier) -> Self::Output {
        self.carrier()
    }
}
impl Carrier for VersorEven {
    type Output = Flector;
    fn carrier(self) -> Self::Output {
        use crate::elements::*;
        return Flector::from_groups(
            // e15, e25, e35, e45
            self.group3(),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([self[e423], self[e431], self[e412], self[e321]]),
        );
    }
}
impl std::ops::Div<carrier> for VersorEvenAligningOrigin {
    type Output = FlectorOnOrigin;
    fn div(self, _rhs: carrier) -> Self::Output {
        self.carrier()
    }
}
impl Carrier for VersorEvenAligningOrigin {
    type Output = FlectorOnOrigin;
    fn carrier(self) -> Self::Output {
        use crate::elements::*;
        return FlectorOnOrigin::from_groups(/* e45, e4235, e4315, e4125 */ Simd32x4::from([self[e4], self[e423], self[e431], self[e412]]));
    }
}
impl std::ops::Div<carrier> for VersorEvenAtInfinity {
    type Output = FlectorAtInfinity;
    fn div(self, _rhs: carrier) -> Self::Output {
        self.carrier()
    }
}
impl Carrier for VersorEvenAtInfinity {
    type Output = FlectorAtInfinity;
    fn carrier(self) -> Self::Output {
        use crate::elements::*;
        return FlectorAtInfinity::from_groups(/* e15, e25, e35, e3215 */ Simd32x4::from([self[e1], self[e2], self[e3], self[e321]]));
    }
}
impl std::ops::Div<carrier> for VersorEvenAtOrigin {
    type Output = FlectorOnOrigin;
    fn div(self, _rhs: carrier) -> Self::Output {
        self.carrier()
    }
}
impl Carrier for VersorEvenAtOrigin {
    type Output = FlectorOnOrigin;
    fn carrier(self) -> Self::Output {
        return FlectorOnOrigin::from_groups(/* e45, e4235, e4315, e4125 */ crate::swizzle!(self.group0(), 3, 0, 1, 2));
    }
}
impl std::ops::Div<carrier> for VersorEvenOnOrigin {
    type Output = FlectorOnOrigin;
    fn div(self, _rhs: carrier) -> Self::Output {
        self.carrier()
    }
}
impl Carrier for VersorEvenOnOrigin {
    type Output = FlectorOnOrigin;
    fn carrier(self) -> Self::Output {
        use crate::elements::*;
        return FlectorOnOrigin::from_groups(/* e45, e4235, e4315, e4125 */ Simd32x4::from([self[e4], self[e423], self[e431], self[e412]]));
    }
}
impl std::ops::Div<carrier> for VersorEvenOrthogonalOrigin {
    type Output = Flector;
    fn div(self, _rhs: carrier) -> Self::Output {
        self.carrier()
    }
}
impl Carrier for VersorEvenOrthogonalOrigin {
    type Output = Flector;
    fn carrier(self) -> Self::Output {
        return Flector::from_groups(/* e15, e25, e35, e45 */ self.group2(), /* e4235, e4315, e4125, e3215 */ self.group0());
    }
}
impl std::ops::Div<carrier> for VersorOdd {
    type Output = Motor;
    fn div(self, _rhs: carrier) -> Self::Output {
        self.carrier()
    }
}
impl Carrier for VersorOdd {
    type Output = Motor;
    fn carrier(self) -> Self::Output {
        use crate::elements::*;
        return Motor::from_groups(
            // e415, e425, e435, e12345
            Simd32x4::from([self[e41], self[e42], self[e43], self[e1234]]),
            // e235, e315, e125, e5
            Simd32x4::from([self[e23], self[e31], self[e12], self[scalar]]),
        );
    }
}
impl std::ops::Div<carrier> for VersorOddAtInfinity {
    type Output = MotorAtInfinity;
    fn div(self, _rhs: carrier) -> Self::Output {
        self.carrier()
    }
}
impl Carrier for VersorOddAtInfinity {
    type Output = MotorAtInfinity;
    fn carrier(self) -> Self::Output {
        use crate::elements::*;
        return MotorAtInfinity::from_groups(/* e235, e315, e125, e5 */ Simd32x4::from([self[e23], self[e31], self[e12], self[scalar]]));
    }
}
impl std::ops::Div<carrier> for VersorOddOrthogonalOrigin {
    type Output = Motor;
    fn div(self, _rhs: carrier) -> Self::Output {
        self.carrier()
    }
}
impl Carrier for VersorOddOrthogonalOrigin {
    type Output = Motor;
    fn carrier(self) -> Self::Output {
        use crate::elements::*;
        return Motor::from_groups(
            // e415, e425, e435, e12345
            Simd32x4::from([self[e41], self[e42], self[e43], self[e1234]]),
            // e235, e315, e125, e5
            Simd32x4::from([self[e23], self[e31], self[e12], self[scalar]]),
        );
    }
}
