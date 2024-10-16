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
impl Carrier for AntiCircleOnOrigin {
    type Output = AntiCircleOnOrigin;
    fn carrier(self) -> Self::Output {
        return self;
    }
}
impl Carrier for AntiCircleRotor {
    type Output = Motor;
    fn carrier(self) -> Self::Output {
        return Motor::from_groups(
            // e415, e425, e435, e12345
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], 0.0]),
            // e235, e315, e125, e5
            Simd32x4::from([self.group1()[0], self.group1()[1], self.group1()[2], self.group2()[3]]),
        );
    }
}
impl Carrier for AntiCircleRotorAligningOrigin {
    type Output = Motor;
    fn carrier(self) -> Self::Output {
        return Motor::from_groups(
            // e415, e425, e435, e12345
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], 0.0]),
            // e235, e315, e125, e5
            Simd32x4::from([self.group1()[0], self.group1()[1], self.group1()[2], self.group2()[3]]),
        );
    }
}
impl Carrier for AntiCircleRotorAligningOriginAtInfinity {
    type Output = MotorAtInfinity;
    fn carrier(self) -> Self::Output {
        return MotorAtInfinity::from_groups(/* e235, e315, e125, e5 */ Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], self.group1()[3]]));
    }
}
impl Carrier for AntiCircleRotorAtInfinity {
    type Output = MotorAtInfinity;
    fn carrier(self) -> Self::Output {
        return MotorAtInfinity::from_groups(/* e235, e315, e125, e5 */ Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], self.group1()[3]]));
    }
}
impl Carrier for AntiCircleRotorOnOrigin {
    type Output = Motor;
    fn carrier(self) -> Self::Output {
        return Motor::from_groups(
            // e415, e425, e435, e12345
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], 0.0]),
            // e235, e315, e125, e5
            Simd32x4::from([self.group1()[0], self.group1()[1], self.group1()[2], self.group0()[3]]),
        );
    }
}
impl Carrier for AntiDipoleInversion {
    type Output = Flector;
    fn carrier(self) -> Self::Output {
        return Flector::from_groups(
            // e15, e25, e35, e45
            Simd32x4::from([self.group3()[0], self.group3()[1], self.group3()[2], self.group2()[3]]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], self.group1()[3]]),
        );
    }
}
impl Carrier for AntiDipoleInversionAtInfinity {
    type Output = FlectorAtInfinity;
    fn carrier(self) -> Self::Output {
        return FlectorAtInfinity::from_groups(/* e15, e25, e35, e3215 */ Simd32x4::from([self.group2()[0], self.group2()[1], self.group2()[2], self.group0()[3]]));
    }
}
impl Carrier for AntiDipoleInversionOnOrigin {
    type Output = Flector;
    fn carrier(self) -> Self::Output {
        return Flector::from_groups(
            // e15, e25, e35, e45
            Simd32x4::from([self.group1()[1], self.group1()[2], self.group1()[3], self.group1()[0]]),
            // e4235, e4315, e4125, e3215
            self.group0(),
        );
    }
}
impl Carrier for AntiDipoleInversionOrthogonalOrigin {
    type Output = FlectorOnOrigin;
    fn carrier(self) -> Self::Output {
        return FlectorOnOrigin::from_groups(
            // e45, e4235, e4315, e4125
            Simd32x4::from([self.group2()[3], self.group0()[0], self.group0()[1], self.group0()[2]]),
        );
    }
}
impl Carrier for AntiDipoleOnOrigin {
    type Output = AntiDipoleOnOrigin;
    fn carrier(self) -> Self::Output {
        return self;
    }
}
impl Carrier for AntiDualNum {
    type Output = Motor;
    fn carrier(self) -> Self::Output {
        return Motor::from_groups(
            // e415, e425, e435, e12345
            Simd32x4::from([0.0, 0.0, 0.0, self.group0()[0]]),
            // e235, e315, e125, e5
            Simd32x4::from([0.0, 0.0, 0.0, self.group0()[1]]),
        );
    }
}
impl Carrier for AntiFlatOrigin {
    type Output = AntiFlatOrigin;
    fn carrier(self) -> Self::Output {
        return self;
    }
}
impl Carrier for AntiFlatPoint {
    type Output = Horizon;
    fn carrier(self) -> Self::Output {
        return Horizon::from_groups(/* e3215 */ self.group0()[3]);
    }
}
impl Carrier for AntiFlector {
    type Output = FlectorAtInfinity;
    fn carrier(self) -> Self::Output {
        return FlectorAtInfinity::from_groups(/* e15, e25, e35, e3215 */ Simd32x4::from([self.group1()[0], self.group1()[1], self.group1()[2], self.group0()[3]]));
    }
}
impl Carrier for AntiFlectorOnOrigin {
    type Output = FlectorAtInfinity;
    fn carrier(self) -> Self::Output {
        return FlectorAtInfinity::from_groups(/* e15, e25, e35, e3215 */ Simd32x4::from([self.group0()[1], self.group0()[2], self.group0()[3], self.group0()[0]]));
    }
}
impl Carrier for AntiLine {
    type Output = AntiLine;
    fn carrier(self) -> Self::Output {
        return self;
    }
}
impl Carrier for AntiLineOnOrigin {
    type Output = AntiLineOnOrigin;
    fn carrier(self) -> Self::Output {
        return self;
    }
}
impl Carrier for AntiMotor {
    type Output = AntiMotor;
    fn carrier(self) -> Self::Output {
        return self;
    }
}
impl Carrier for AntiMotorOnOrigin {
    type Output = AntiMotorOnOrigin;
    fn carrier(self) -> Self::Output {
        return self;
    }
}
impl Carrier for AntiMysteryCircleRotor {
    type Output = MotorAtInfinity;
    fn carrier(self) -> Self::Output {
        use crate::elements::*;
        return MotorAtInfinity::from_groups(/* e235, e315, e125, e5 */ Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], self[e31]]));
    }
}
impl Carrier for AntiMysteryDipoleInversion {
    type Output = FlectorAtInfinity;
    fn carrier(self) -> Self::Output {
        return FlectorAtInfinity::from_groups(/* e15, e25, e35, e3215 */ Simd32x4::from([self.group1()[0], self.group1()[1], self.group1()[2], self.group0()[3]]));
    }
}
impl Carrier for AntiPlane {
    type Output = FlatPointAtInfinity;
    fn carrier(self) -> Self::Output {
        return FlatPointAtInfinity::from_groups(/* e15, e25, e35 */ Simd32x3::from([self.group0()[0], self.group0()[1], self.group0()[2]]));
    }
}
impl Carrier for AntiPlaneOnOrigin {
    type Output = AntiPlaneOnOrigin;
    fn carrier(self) -> Self::Output {
        return self;
    }
}
impl Carrier for AntiSphereOnOrigin {
    type Output = AntiSphereOnOrigin;
    fn carrier(self) -> Self::Output {
        return self;
    }
}
impl Carrier for AntiVersorEvenOnOrigin {
    type Output = Motor;
    fn carrier(self) -> Self::Output {
        return Motor::from_groups(
            // e415, e425, e435, e12345
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], self.group1()[3]]),
            // e235, e315, e125, e5
            Simd32x4::from([self.group1()[0], self.group1()[1], self.group1()[2], self.group0()[3]]),
        );
    }
}
impl Carrier for Circle {
    type Output = Plane;
    fn carrier(self) -> Self::Output {
        return Plane::from_groups(
            // e4235, e4315, e4125, e3215
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], self.group1()[3]]),
        );
    }
}
impl Carrier for CircleAligningOrigin {
    type Output = CircleAligningOrigin;
    fn carrier(self) -> Self::Output {
        return self;
    }
}
impl Carrier for CircleAtInfinity {
    type Output = Horizon;
    fn carrier(self) -> Self::Output {
        return Horizon::from_groups(/* e3215 */ self.group0()[3]);
    }
}
impl Carrier for CircleAtOrigin {
    type Output = CircleAtOrigin;
    fn carrier(self) -> Self::Output {
        return self;
    }
}
impl Carrier for CircleOnOrigin {
    type Output = CircleOnOrigin;
    fn carrier(self) -> Self::Output {
        return self;
    }
}
impl Carrier for CircleOrthogonalOrigin {
    type Output = CircleOrthogonalOrigin;
    fn carrier(self) -> Self::Output {
        return self;
    }
}
impl Carrier for CircleRotor {
    type Output = Plane;
    fn carrier(self) -> Self::Output {
        return Plane::from_groups(
            // e4235, e4315, e4125, e3215
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], self.group1()[3]]),
        );
    }
}
impl Carrier for CircleRotorAligningOrigin {
    type Output = CircleRotorAligningOrigin;
    fn carrier(self) -> Self::Output {
        return self;
    }
}
impl Carrier for CircleRotorAtInfinity {
    type Output = Horizon;
    fn carrier(self) -> Self::Output {
        return Horizon::from_groups(/* e3215 */ self.group0()[3]);
    }
}
impl Carrier for CircleRotorOnOrigin {
    type Output = PlaneOnOrigin;
    fn carrier(self) -> Self::Output {
        return PlaneOnOrigin::from_groups(/* e4235, e4315, e4125 */ Simd32x3::from([self.group0()[0], self.group0()[1], self.group0()[2]]));
    }
}
impl Carrier for Dipole {
    type Output = Line;
    fn carrier(self) -> Self::Output {
        return Line::from_groups(
            // e415, e425, e435
            self.group0(),
            // e235, e315, e125
            Simd32x3::from([self.group1()[0], self.group1()[1], self.group1()[2]]),
        );
    }
}
impl Carrier for DipoleAligningOrigin {
    type Output = LineOnOrigin;
    fn carrier(self) -> Self::Output {
        return LineOnOrigin::from_groups(/* e415, e425, e435 */ Simd32x3::from([self.group0()[0], self.group0()[1], self.group0()[2]]));
    }
}
impl Carrier for DipoleAtInfinity {
    type Output = LineAtInfinity;
    fn carrier(self) -> Self::Output {
        return LineAtInfinity::from_groups(/* e235, e315, e125 */ Simd32x3::from([self.group0()[0], self.group0()[1], self.group0()[2]]));
    }
}
impl Carrier for DipoleAtOrigin {
    type Output = DipoleAtOrigin;
    fn carrier(self) -> Self::Output {
        return self;
    }
}
impl Carrier for DipoleInversion {
    type Output = CircleRotorAligningOriginAtInfinity;
    fn carrier(self) -> Self::Output {
        return CircleRotorAligningOriginAtInfinity::from_groups(
            // e415, e425, e435
            self.group0(),
            // e235, e315, e125, e12345
            Simd32x4::from([self.group1()[0], self.group1()[1], self.group1()[2], self.group2()[3]]),
        );
    }
}
impl Carrier for DipoleInversionAligningOrigin {
    type Output = MotorOnOrigin;
    fn carrier(self) -> Self::Output {
        return MotorOnOrigin::from_groups(
            // e415, e425, e435, e12345
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], self.group1()[3]]),
        );
    }
}
impl Carrier for DipoleInversionAtInfinity {
    type Output = LineAtInfinity;
    fn carrier(self) -> Self::Output {
        return LineAtInfinity::from_groups(/* e235, e315, e125 */ Simd32x3::from([self.group0()[0], self.group0()[1], self.group0()[2]]));
    }
}
impl Carrier for DipoleInversionAtOrigin {
    type Output = MotorOnOrigin;
    fn carrier(self) -> Self::Output {
        return MotorOnOrigin::from_groups(
            // e415, e425, e435, e12345
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], self.group1()[3]]),
        );
    }
}
impl Carrier for DipoleInversionOnOrigin {
    type Output = MotorOnOrigin;
    fn carrier(self) -> Self::Output {
        return MotorOnOrigin::from_groups(
            // e415, e425, e435, e12345
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], self.group1()[0]]),
        );
    }
}
impl Carrier for DipoleInversionOrthogonalOrigin {
    type Output = CircleRotorAligningOriginAtInfinity;
    fn carrier(self) -> Self::Output {
        return CircleRotorAligningOriginAtInfinity::from_groups(
            // e415, e425, e435
            Simd32x3::from([self.group0()[0], self.group0()[1], self.group0()[2]]),
            // e235, e315, e125, e12345
            Simd32x4::from([self.group1()[0], self.group1()[1], self.group1()[2], self.group2()[3]]),
        );
    }
}
impl Carrier for DipoleOnOrigin {
    type Output = LineOnOrigin;
    fn carrier(self) -> Self::Output {
        return LineOnOrigin::from_groups(/* e415, e425, e435 */ Simd32x3::from([self.group0()[0], self.group0()[1], self.group0()[2]]));
    }
}
impl Carrier for DipoleOrthogonalOrigin {
    type Output = DipoleOrthogonalOrigin;
    fn carrier(self) -> Self::Output {
        return self;
    }
}
impl Carrier for DualNum {
    type Output = FlatOrigin;
    fn carrier(self) -> Self::Output {
        return FlatOrigin::from_groups(/* e45 */ self.group0()[0]);
    }
}
impl Carrier for MultiVector {
    type Output = MultiVector;
    fn carrier(self) -> Self::Output {
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, self.group9()[0]]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            self.group0()[0],
            // e41, e42, e43, e45
            Simd32x4::from([0.0, 0.0, 0.0, self.group1()[3]]),
            // e15, e25, e35
            Simd32x3::from([self.group1()[0], self.group1()[1], self.group1()[2]]),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from([self.group3()[0], self.group3()[1], self.group3()[2], 0.0]),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            self.group5(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([0.0, self.group7()[0], self.group7()[1], self.group7()[2]]),
            // e3215
            self.group6()[3],
        );
    }
}
impl Carrier for MysteryCircle {
    type Output = Horizon;
    fn carrier(self) -> Self::Output {
        return Horizon::from_groups(/* e3215 */ self.group0()[3]);
    }
}
impl Carrier for MysteryCircleRotor {
    type Output = Horizon;
    fn carrier(self) -> Self::Output {
        return Horizon::from_groups(/* e3215 */ self.group0()[3]);
    }
}
impl Carrier for MysteryDipole {
    type Output = LineAtInfinity;
    fn carrier(self) -> Self::Output {
        return LineAtInfinity::from_groups(/* e235, e315, e125 */ Simd32x3::from([self.group0()[0], self.group0()[1], self.group0()[2]]));
    }
}
impl Carrier for MysteryDipoleInversion {
    type Output = LineAtInfinity;
    fn carrier(self) -> Self::Output {
        return LineAtInfinity::from_groups(/* e235, e315, e125 */ Simd32x3::from([self.group0()[0], self.group0()[1], self.group0()[2]]));
    }
}
impl Carrier for MysteryVersorEven {
    type Output = FlectorAtInfinity;
    fn carrier(self) -> Self::Output {
        return FlectorAtInfinity::from_groups(/* e15, e25, e35, e3215 */ Simd32x4::from([self.group0()[1], self.group0()[2], self.group0()[3], self.group1()[3]]));
    }
}
impl Carrier for MysteryVersorOdd {
    type Output = MotorAtInfinity;
    fn carrier(self) -> Self::Output {
        return MotorAtInfinity::from_groups(/* e235, e315, e125, e5 */ Simd32x4::from([self.group1()[0], self.group1()[1], self.group1()[2], self.group0()[0]]));
    }
}
impl Carrier for NullCircleAtOrigin {
    type Output = NullCircleAtOrigin;
    fn carrier(self) -> Self::Output {
        return self;
    }
}
impl Carrier for NullDipoleAtOrigin {
    type Output = NullDipoleAtOrigin;
    fn carrier(self) -> Self::Output {
        return self;
    }
}
impl Carrier for NullDipoleInversionAtOrigin {
    type Output = NullDipoleInversionAtOrigin;
    fn carrier(self) -> Self::Output {
        return self;
    }
}
impl Carrier for NullSphereAtOrigin {
    type Output = NullSphereAtOrigin;
    fn carrier(self) -> Self::Output {
        return self;
    }
}
impl Carrier for NullVersorEvenAtOrigin {
    type Output = FlectorOnOrigin;
    fn carrier(self) -> Self::Output {
        return FlectorOnOrigin::from_groups(
            // e45, e4235, e4315, e4125
            Simd32x4::from([self.group0()[3], self.group0()[0], self.group0()[1], self.group0()[2]]),
        );
    }
}
impl Carrier for Origin {
    type Output = Origin;
    fn carrier(self) -> Self::Output {
        return self;
    }
}
impl Carrier for RoundPoint {
    type Output = RoundPoint;
    fn carrier(self) -> Self::Output {
        return self;
    }
}
impl Carrier for RoundPointAtOrigin {
    type Output = FlatOrigin;
    fn carrier(self) -> Self::Output {
        return FlatOrigin::from_groups(/* e45 */ self.group0()[0]);
    }
}
impl Carrier for Scalar {
    type Output = Scalar;
    fn carrier(self) -> Self::Output {
        return self;
    }
}
impl Carrier for Sphere {
    type Output = AntiScalar;
    fn carrier(self) -> Self::Output {
        use crate::elements::*;
        return AntiScalar::from_groups(/* e12345 */ self[e4315]);
    }
}
impl Carrier for SphereAtOrigin {
    type Output = AntiScalar;
    fn carrier(self) -> Self::Output {
        return AntiScalar::from_groups(/* e12345 */ self.group0()[1]);
    }
}
impl Carrier for SphereOnOrigin {
    type Output = AntiScalar;
    fn carrier(self) -> Self::Output {
        return AntiScalar::from_groups(/* e12345 */ self.group0()[3]);
    }
}
impl Carrier for VersorEven {
    type Output = Flector;
    fn carrier(self) -> Self::Output {
        return Flector::from_groups(
            // e15, e25, e35, e45
            self.group3(),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], self.group1()[3]]),
        );
    }
}
impl Carrier for VersorEvenAligningOrigin {
    type Output = FlectorOnOrigin;
    fn carrier(self) -> Self::Output {
        return FlectorOnOrigin::from_groups(
            // e45, e4235, e4315, e4125
            Simd32x4::from([self.group1()[3], self.group0()[0], self.group0()[1], self.group0()[2]]),
        );
    }
}
impl Carrier for VersorEvenAtInfinity {
    type Output = FlectorAtInfinity;
    fn carrier(self) -> Self::Output {
        return FlectorAtInfinity::from_groups(/* e15, e25, e35, e3215 */ Simd32x4::from([self.group0()[1], self.group0()[2], self.group0()[3], self.group1()[3]]));
    }
}
impl Carrier for VersorEvenAtOrigin {
    type Output = FlectorOnOrigin;
    fn carrier(self) -> Self::Output {
        return FlectorOnOrigin::from_groups(
            // e45, e4235, e4315, e4125
            Simd32x4::from([self.group0()[3], self.group0()[0], self.group0()[1], self.group0()[2]]),
        );
    }
}
impl Carrier for VersorEvenOnOrigin {
    type Output = FlectorOnOrigin;
    fn carrier(self) -> Self::Output {
        return FlectorOnOrigin::from_groups(
            // e45, e4235, e4315, e4125
            Simd32x4::from([self.group1()[3], self.group0()[0], self.group0()[1], self.group0()[2]]),
        );
    }
}
impl Carrier for VersorEvenOrthogonalOrigin {
    type Output = Flector;
    fn carrier(self) -> Self::Output {
        return Flector::from_groups(/* e15, e25, e35, e45 */ self.group2(), /* e4235, e4315, e4125, e3215 */ self.group0());
    }
}
impl Carrier for VersorOdd {
    type Output = Motor;
    fn carrier(self) -> Self::Output {
        return Motor::from_groups(
            // e415, e425, e435, e12345
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], self.group2()[3]]),
            // e235, e315, e125, e5
            Simd32x4::from([self.group1()[0], self.group1()[1], self.group1()[2], self.group0()[3]]),
        );
    }
}
impl Carrier for VersorOddAtInfinity {
    type Output = MotorAtInfinity;
    fn carrier(self) -> Self::Output {
        return MotorAtInfinity::from_groups(/* e235, e315, e125, e5 */ Simd32x4::from([self.group1()[0], self.group1()[1], self.group1()[2], self.group0()[0]]));
    }
}
impl Carrier for VersorOddOrthogonalOrigin {
    type Output = Motor;
    fn carrier(self) -> Self::Output {
        return Motor::from_groups(
            // e415, e425, e435, e12345
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], self.group2()[3]]),
            // e235, e315, e125, e5
            Simd32x4::from([self.group1()[0], self.group1()[1], self.group1()[2], self.group0()[3]]),
        );
    }
}
