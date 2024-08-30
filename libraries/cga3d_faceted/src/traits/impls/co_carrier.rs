use crate::traits::RightAntiDual;
// Note on Operative Statistics:
// Operative Statistics are not a precise predictor of performance or performance comparisons.
// This is due to varying hardware capabilities and compiler optimizations.
// As always, where performance is a concern, there is no substitute for
// real measurements on real work-loads on real hardware.
// Disclaimer aside, enjoy the fun information =)
//
// Total Implementations: 88
//
// Yes SIMD:   add/sub     mul     div
//  Minimum:         0       0       0
//   Median:         0       3       0
//  Average:         0      11       0
//  Maximum:         0     168       0
//
//  No SIMD:   add/sub     mul     div
//  Minimum:         0       0       0
//   Median:         0       6       0
//  Average:         0      13       0
//  Maximum:         0     224       0
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
impl CoCarrier for AntiCircleRotor {
    type Output = Plane;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0       24        0
    //    simd3        0        4        0
    // Totals...
    // yes simd        0       28        0
    //  no simd        0       36        0
    fn co_carrier(self) -> Self::Output {
        return Plane::from_groups(/* e4235, e4315, e4125, e3215 */ Simd32x4::from([
            self.right_anti_dual().group0()[0],
            self.right_anti_dual().group0()[1],
            self.right_anti_dual().group0()[2],
            self.right_anti_dual().group1()[3],
        ]));
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
impl CoCarrier for AntiCircleRotorAtInfinity {
    type Output = Horizon;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        6        0
    fn co_carrier(self) -> Self::Output {
        return Horizon::from_groups(/* e3215 */ self.right_anti_dual().group0()[3]);
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
        return PlaneOnOrigin::from_groups(/* e4235, e4315, e4125 */ Simd32x3::from([
            self.right_anti_dual().group0()[0],
            self.right_anti_dual().group0()[1],
            self.right_anti_dual().group0()[2],
        ]));
    }
}
impl CoCarrier for AntiDipoleInversion {
    type Output = CircleRotorAligningOriginAtInfinity;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0       15        0
    fn co_carrier(self) -> Self::Output {
        return CircleRotorAligningOriginAtInfinity::from_groups(
            // e415, e425, e435
            self.right_anti_dual().group0(),
            // e235, e315, e125, e12345
            Simd32x4::from([
                self.right_anti_dual().group1()[0],
                self.right_anti_dual().group1()[1],
                self.right_anti_dual().group1()[2],
                self.right_anti_dual().group2()[3],
            ]),
        );
    }
}
impl CoCarrier for AntiDipoleInversionAtInfinity {
    type Output = LineAtInfinity;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        6        0
    fn co_carrier(self) -> Self::Output {
        return LineAtInfinity::from_groups(/* e235, e315, e125 */ Simd32x3::from([
            self.right_anti_dual().group0()[0],
            self.right_anti_dual().group0()[1],
            self.right_anti_dual().group0()[2],
        ]));
    }
}
impl CoCarrier for AntiDipoleInversionOnOrigin {
    type Output = MotorOnOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        8        0
    fn co_carrier(self) -> Self::Output {
        return MotorOnOrigin::from_groups(/* e415, e425, e435, e12345 */ Simd32x4::from([
            self.right_anti_dual().group0()[0],
            self.right_anti_dual().group0()[1],
            self.right_anti_dual().group0()[2],
            self.right_anti_dual().group1()[0],
        ]));
    }
}
impl CoCarrier for AntiDipoleInversionOrthogonalOrigin {
    type Output = CircleRotorAligningOriginAtInfinity;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0       14        0
    fn co_carrier(self) -> Self::Output {
        return CircleRotorAligningOriginAtInfinity::from_groups(
            // e415, e425, e435
            Simd32x3::from([self.right_anti_dual().group0()[0], self.right_anti_dual().group0()[1], self.right_anti_dual().group0()[2]]),
            // e235, e315, e125, e12345
            Simd32x4::from([
                self.right_anti_dual().group1()[0],
                self.right_anti_dual().group1()[1],
                self.right_anti_dual().group1()[2],
                self.right_anti_dual().group2()[3],
            ]),
        );
    }
}
impl CoCarrier for AntiDipoleOnOrigin {
    type Output = LineOnOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        3        0
    fn co_carrier(self) -> Self::Output {
        return LineOnOrigin::from_groups(/* e415, e425, e435 */ Simd32x3::from([
            self.right_anti_dual().group0()[0],
            self.right_anti_dual().group0()[1],
            self.right_anti_dual().group0()[2],
        ]));
    }
}
impl CoCarrier for AntiMysteryCircleRotor {
    type Output = Horizon;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        3        0
    fn co_carrier(self) -> Self::Output {
        use crate::elements::*;
        return Horizon::from_groups(/* e3215 */ self.right_anti_dual().group0()[3]);
    }
}
impl CoCarrier for AntiMysteryDipoleInversion {
    type Output = LineAtInfinity;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        3        0
    fn co_carrier(self) -> Self::Output {
        return LineAtInfinity::from_groups(/* e235, e315, e125 */ Simd32x3::from([
            self.right_anti_dual().group0()[0],
            self.right_anti_dual().group0()[1],
            self.right_anti_dual().group0()[2],
        ]));
    }
}
impl CoCarrier for AntiMysteryQuadNum {
    type Output = Infinity;
    fn co_carrier(self) -> Self::Output {
        return Infinity::from_groups(/* e5 */ self.right_anti_dual().group0()[1]);
    }
}
impl CoCarrier for AntiPlaneOnOrigin {
    type Output = AntiPlaneOnOrigin;
    fn co_carrier(self) -> Self::Output {
        return self.right_anti_dual();
    }
}
impl CoCarrier for AntiQuadNum {
    type Output = QuadNumAligningOriginAtInfinity;
    fn co_carrier(self) -> Self::Output {
        return QuadNumAligningOriginAtInfinity::from_groups(/* e5, e12345 */ Simd32x2::from([self.right_anti_dual().group0()[3], self.right_anti_dual().group0()[0]]));
    }
}
impl CoCarrier for AntiQuadNumAligningOrigin {
    type Output = QuadNumAligningOriginAtInfinity;
    fn co_carrier(self) -> Self::Output {
        return QuadNumAligningOriginAtInfinity::from_groups(/* e5, e12345 */ Simd32x2::from([self.right_anti_dual().group0()[2], self.right_anti_dual().group0()[0]]));
    }
}
impl CoCarrier for AntiQuadNumAligningOriginAtInfinity {
    type Output = Infinity;
    fn co_carrier(self) -> Self::Output {
        return Infinity::from_groups(/* e5 */ self.right_anti_dual().group0()[1]);
    }
}
impl CoCarrier for AntiQuadNumAtInfinity {
    type Output = Infinity;
    fn co_carrier(self) -> Self::Output {
        return Infinity::from_groups(/* e5 */ self.right_anti_dual().group0()[2]);
    }
}
impl CoCarrier for AntiQuadNumOnOrigin {
    type Output = QuadNumAligningOriginAtInfinity;
    fn co_carrier(self) -> Self::Output {
        return QuadNumAligningOriginAtInfinity::from_groups(/* e5, e12345 */ Simd32x2::from([self.right_anti_dual().group0()[1], self.right_anti_dual().group0()[0]]));
    }
}
impl CoCarrier for AntiQuadNumOrthogonalOrigin {
    type Output = AntiScalar;
    fn co_carrier(self) -> Self::Output {
        return AntiScalar::from_groups(/* e12345 */ self.right_anti_dual().group0()[0]);
    }
}
impl CoCarrier for AntiScalar {
    type Output = Scalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn co_carrier(self) -> Self::Output {
        use crate::elements::*;
        return self.right_anti_dual();
    }
}
impl CoCarrier for AntiSphereOnOrigin {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn co_carrier(self) -> Self::Output {
        return AntiScalar::from_groups(/* e12345 */ self.right_anti_dual().group0()[3]);
    }
}
impl CoCarrier for AntiVersorEvenOnOrigin {
    type Output = FlectorOnOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0       24        0
    fn co_carrier(self) -> Self::Output {
        return FlectorOnOrigin::from_groups(/* e45, e4235, e4315, e4125 */ Simd32x4::from([
            self.right_anti_dual().group1()[3],
            self.right_anti_dual().group0()[0],
            self.right_anti_dual().group0()[1],
            self.right_anti_dual().group0()[2],
        ]));
    }
}
impl CoCarrier for Circle {
    type Output = Line;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        4        0
    fn co_carrier(self) -> Self::Output {
        return Line::from_groups(
            // e415, e425, e435
            self.right_anti_dual().group0(),
            // e235, e315, e125
            Simd32x3::from([self.right_anti_dual().group1()[0], self.right_anti_dual().group1()[1], self.right_anti_dual().group1()[2]]),
        );
    }
}
impl CoCarrier for CircleAligningOrigin {
    type Output = CircleAligningOrigin;
    fn co_carrier(self) -> Self::Output {
        return self.right_anti_dual();
    }
}
impl CoCarrier for CircleAtInfinity {
    type Output = LineAtInfinity;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        3        0
    fn co_carrier(self) -> Self::Output {
        return LineAtInfinity::from_groups(/* e235, e315, e125 */ Simd32x3::from([
            self.right_anti_dual().group0()[0],
            self.right_anti_dual().group0()[1],
            self.right_anti_dual().group0()[2],
        ]));
    }
}
impl CoCarrier for CircleAtOrigin {
    type Output = CircleAtOrigin;
    fn co_carrier(self) -> Self::Output {
        return self.right_anti_dual();
    }
}
impl CoCarrier for CircleOnOrigin {
    type Output = CircleOnOrigin;
    fn co_carrier(self) -> Self::Output {
        return self.right_anti_dual();
    }
}
impl CoCarrier for CircleOrthogonalOrigin {
    type Output = LineOnOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        3        0
    fn co_carrier(self) -> Self::Output {
        return LineOnOrigin::from_groups(/* e415, e425, e435 */ Simd32x3::from([
            self.right_anti_dual().group0()[0],
            self.right_anti_dual().group0()[1],
            self.right_anti_dual().group0()[2],
        ]));
    }
}
impl CoCarrier for CircleRotor {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0       14        0
    fn co_carrier(self) -> Self::Output {
        return Motor::from_groups(
            // e415, e425, e435, e12345
            Simd32x4::from([self.right_anti_dual().group0()[0], self.right_anti_dual().group0()[1], self.right_anti_dual().group0()[2], 0.0]),
            // e235, e315, e125, e5
            Simd32x4::from([
                self.right_anti_dual().group1()[0],
                self.right_anti_dual().group1()[1],
                self.right_anti_dual().group1()[2],
                self.right_anti_dual().group2()[3],
            ]),
        );
    }
}
impl CoCarrier for CircleRotorAligningOrigin {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        7        0
    fn co_carrier(self) -> Self::Output {
        return Motor::from_groups(
            // e415, e425, e435, e12345
            Simd32x4::from([self.right_anti_dual().group0()[0], self.right_anti_dual().group0()[1], self.right_anti_dual().group0()[2], 0.0]),
            // e235, e315, e125, e5
            Simd32x4::from([
                self.right_anti_dual().group1()[0],
                self.right_anti_dual().group1()[1],
                self.right_anti_dual().group1()[2],
                self.right_anti_dual().group2()[3],
            ]),
        );
    }
}
impl CoCarrier for CircleRotorAligningOriginAtInfinity {
    type Output = MotorAtInfinity;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        4        0
    fn co_carrier(self) -> Self::Output {
        return MotorAtInfinity::from_groups(/* e235, e315, e125, e5 */ Simd32x4::from([
            self.right_anti_dual().group0()[0],
            self.right_anti_dual().group0()[1],
            self.right_anti_dual().group0()[2],
            self.right_anti_dual().group1()[3],
        ]));
    }
}
impl CoCarrier for CircleRotorAtInfinity {
    type Output = MotorAtInfinity;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        8        0
    fn co_carrier(self) -> Self::Output {
        return MotorAtInfinity::from_groups(/* e235, e315, e125, e5 */ Simd32x4::from([
            self.right_anti_dual().group0()[0],
            self.right_anti_dual().group0()[1],
            self.right_anti_dual().group0()[2],
            self.right_anti_dual().group1()[3],
        ]));
    }
}
impl CoCarrier for CircleRotorOnOrigin {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        7        0
    fn co_carrier(self) -> Self::Output {
        return Motor::from_groups(
            // e415, e425, e435, e12345
            Simd32x4::from([self.right_anti_dual().group0()[0], self.right_anti_dual().group0()[1], self.right_anti_dual().group0()[2], 0.0]),
            // e235, e315, e125, e5
            Simd32x4::from([
                self.right_anti_dual().group1()[0],
                self.right_anti_dual().group1()[1],
                self.right_anti_dual().group1()[2],
                self.right_anti_dual().group0()[3],
            ]),
        );
    }
}
impl CoCarrier for Dipole {
    type Output = Plane;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0       12        0
    //    simd3        0        8        0
    // Totals...
    // yes simd        0       20        0
    //  no simd        0       36        0
    fn co_carrier(self) -> Self::Output {
        return Plane::from_groups(/* e4235, e4315, e4125, e3215 */ Simd32x4::from([
            self.right_anti_dual().group0()[0],
            self.right_anti_dual().group0()[1],
            self.right_anti_dual().group0()[2],
            self.right_anti_dual().group1()[3],
        ]));
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
        return Horizon::from_groups(/* e3215 */ self.right_anti_dual().group0()[3]);
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
impl CoCarrier for DipoleInversion {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0       72        0
    //    simd3        0        8        0
    // Totals...
    // yes simd        0       80        0
    //  no simd        0       96        0
    fn co_carrier(self) -> Self::Output {
        return Flector::from_groups(
            // e15, e25, e35, e45
            Simd32x4::from([
                self.right_anti_dual().group3()[0],
                self.right_anti_dual().group3()[1],
                self.right_anti_dual().group3()[2],
                self.right_anti_dual().group2()[3],
            ]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([
                self.right_anti_dual().group0()[0],
                self.right_anti_dual().group0()[1],
                self.right_anti_dual().group0()[2],
                self.right_anti_dual().group1()[3],
            ]),
        );
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
        return FlectorAtInfinity::from_groups(/* e15, e25, e35, e3215 */ Simd32x4::from([
            self.right_anti_dual().group2()[0],
            self.right_anti_dual().group2()[1],
            self.right_anti_dual().group2()[2],
            self.right_anti_dual().group0()[3],
        ]));
    }
}
impl CoCarrier for DipoleInversionAtOrigin {
    type Output = FlectorOnOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0       24        0
    fn co_carrier(self) -> Self::Output {
        return FlectorOnOrigin::from_groups(/* e45, e4235, e4315, e4125 */ Simd32x4::from([
            self.right_anti_dual().group0()[3],
            self.right_anti_dual().group0()[0],
            self.right_anti_dual().group0()[1],
            self.right_anti_dual().group0()[2],
        ]));
    }
}
impl CoCarrier for DipoleInversionOnOrigin {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0       30        0
    fn co_carrier(self) -> Self::Output {
        return Flector::from_groups(
            // e15, e25, e35, e45
            Simd32x4::from([
                self.right_anti_dual().group1()[1],
                self.right_anti_dual().group1()[2],
                self.right_anti_dual().group1()[3],
                self.right_anti_dual().group1()[0],
            ]),
            // e4235, e4315, e4125, e3215
            self.right_anti_dual().group0(),
        );
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
        return FlectorOnOrigin::from_groups(/* e45, e4235, e4315, e4125 */ Simd32x4::from([
            self.right_anti_dual().group2()[3],
            self.right_anti_dual().group0()[0],
            self.right_anti_dual().group0()[1],
            self.right_anti_dual().group0()[2],
        ]));
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
impl CoCarrier for FlatPoint {
    type Output = Horizon;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        3        0
    fn co_carrier(self) -> Self::Output {
        return Horizon::from_groups(/* e3215 */ self.right_anti_dual().group0()[3]);
    }
}
impl CoCarrier for Flector {
    type Output = FlectorAtInfinity;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0       24        0
    fn co_carrier(self) -> Self::Output {
        return FlectorAtInfinity::from_groups(/* e15, e25, e35, e3215 */ Simd32x4::from([
            self.right_anti_dual().group1()[0],
            self.right_anti_dual().group1()[1],
            self.right_anti_dual().group1()[2],
            self.right_anti_dual().group0()[3],
        ]));
    }
}
impl CoCarrier for FlectorOnOrigin {
    type Output = FlectorAtInfinity;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0       12        0
    fn co_carrier(self) -> Self::Output {
        return FlectorAtInfinity::from_groups(/* e15, e25, e35, e3215 */ Simd32x4::from([
            self.right_anti_dual().group0()[1],
            self.right_anti_dual().group0()[2],
            self.right_anti_dual().group0()[3],
            self.right_anti_dual().group0()[0],
        ]));
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
impl CoCarrier for MotorOnOrigin {
    type Output = AntiMotorOnOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn co_carrier(self) -> Self::Output {
        return self.right_anti_dual();
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
            Simd32x2::from([0.0, self.right_anti_dual().group9()[0]]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            self.right_anti_dual().group0()[0],
            // e41, e42, e43, e45
            Simd32x4::from([0.0, 0.0, 0.0, self.right_anti_dual().group1()[3]]),
            // e15, e25, e35
            Simd32x3::from([self.right_anti_dual().group1()[0], self.right_anti_dual().group1()[1], self.right_anti_dual().group1()[2]]),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from([self.right_anti_dual().group3()[0], self.right_anti_dual().group3()[1], self.right_anti_dual().group3()[2], 0.0]),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            self.right_anti_dual().group5(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([0.0, self.right_anti_dual().group7()[0], self.right_anti_dual().group7()[1], self.right_anti_dual().group7()[2]]),
            // e3215
            self.right_anti_dual().group6()[3],
        );
    }
}
impl CoCarrier for MysteryCircle {
    type Output = LineAtInfinity;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        3        0
    fn co_carrier(self) -> Self::Output {
        return LineAtInfinity::from_groups(/* e235, e315, e125 */ Simd32x3::from([
            self.right_anti_dual().group0()[0],
            self.right_anti_dual().group0()[1],
            self.right_anti_dual().group0()[2],
        ]));
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
            self.right_anti_dual().group0()[0],
            self.right_anti_dual().group0()[1],
            self.right_anti_dual().group0()[2],
            self.right_anti_dual()[e31],
        ]));
    }
}
impl CoCarrier for MysteryDipole {
    type Output = Horizon;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        3        0
    fn co_carrier(self) -> Self::Output {
        return Horizon::from_groups(/* e3215 */ self.right_anti_dual().group0()[3]);
    }
}
impl CoCarrier for MysteryDipoleInversion {
    type Output = FlectorAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0       12        0
    //    simd3        0        4        0
    // Totals...
    // yes simd        0       16        0
    //  no simd        0       24        0
    fn co_carrier(self) -> Self::Output {
        return FlectorAtInfinity::from_groups(/* e15, e25, e35, e3215 */ Simd32x4::from([
            self.right_anti_dual().group1()[0],
            self.right_anti_dual().group1()[1],
            self.right_anti_dual().group1()[2],
            self.right_anti_dual().group0()[3],
        ]));
    }
}
impl CoCarrier for MysteryQuadNum {
    type Output = Infinity;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd2        0        1        0
    // no simd        0        2        0
    fn co_carrier(self) -> Self::Output {
        return Infinity::from_groups(/* e5 */ self.right_anti_dual().group0()[1]);
    }
}
impl CoCarrier for MysteryVersorEven {
    type Output = MotorAtInfinity;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        8        0
    fn co_carrier(self) -> Self::Output {
        return MotorAtInfinity::from_groups(/* e235, e315, e125, e5 */ Simd32x4::from([
            self.right_anti_dual().group1()[0],
            self.right_anti_dual().group1()[1],
            self.right_anti_dual().group1()[2],
            self.right_anti_dual().group0()[0],
        ]));
    }
}
impl CoCarrier for MysteryVersorOdd {
    type Output = FlectorAtInfinity;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0       24        0
    fn co_carrier(self) -> Self::Output {
        return FlectorAtInfinity::from_groups(/* e15, e25, e35, e3215 */ Simd32x4::from([
            self.right_anti_dual().group0()[1],
            self.right_anti_dual().group0()[2],
            self.right_anti_dual().group0()[3],
            self.right_anti_dual().group1()[3],
        ]));
    }
}
impl CoCarrier for NullCircleAtOrigin {
    type Output = NullCircleAtOrigin;
    fn co_carrier(self) -> Self::Output {
        return self.right_anti_dual();
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
impl CoCarrier for NullDipoleInversionAtOrigin {
    type Output = FlectorOnOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0       12        0
    fn co_carrier(self) -> Self::Output {
        return FlectorOnOrigin::from_groups(/* e45, e4235, e4315, e4125 */ Simd32x4::from([
            self.right_anti_dual().group0()[3],
            self.right_anti_dual().group0()[0],
            self.right_anti_dual().group0()[1],
            self.right_anti_dual().group0()[2],
        ]));
    }
}
impl CoCarrier for NullSphereAtOrigin {
    type Output = NullSphereAtOrigin;
    fn co_carrier(self) -> Self::Output {
        return self.right_anti_dual();
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
impl CoCarrier for Origin {
    type Output = NullSphereAtOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn co_carrier(self) -> Self::Output {
        use crate::elements::*;
        return self.right_anti_dual();
    }
}
impl CoCarrier for Plane {
    type Output = FlatPointAtInfinity;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        9        0
    fn co_carrier(self) -> Self::Output {
        return FlatPointAtInfinity::from_groups(/* e15, e25, e35 */ Simd32x3::from([
            self.right_anti_dual().group0()[0],
            self.right_anti_dual().group0()[1],
            self.right_anti_dual().group0()[2],
        ]));
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
impl CoCarrier for QuadNum {
    type Output = QuadNumAligningOriginAtInfinity;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        2        0
    // no simd        0        8        0
    fn co_carrier(self) -> Self::Output {
        return QuadNumAligningOriginAtInfinity::from_groups(/* e5, e12345 */ Simd32x2::from([self.right_anti_dual().group0()[3], self.right_anti_dual().group0()[0]]));
    }
}
impl CoCarrier for QuadNumAligningOrigin {
    type Output = QuadNumAligningOriginAtInfinity;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        2        0
    // no simd        0        6        0
    fn co_carrier(self) -> Self::Output {
        return QuadNumAligningOriginAtInfinity::from_groups(/* e5, e12345 */ Simd32x2::from([self.right_anti_dual().group0()[2], self.right_anti_dual().group0()[0]]));
    }
}
impl CoCarrier for QuadNumAligningOriginAtInfinity {
    type Output = Infinity;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd2        0        1        0
    // no simd        0        2        0
    fn co_carrier(self) -> Self::Output {
        return Infinity::from_groups(/* e5 */ self.right_anti_dual().group0()[1]);
    }
}
impl CoCarrier for QuadNumAtInfinity {
    type Output = Infinity;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        1        0
    // no simd        0        3        0
    fn co_carrier(self) -> Self::Output {
        return Infinity::from_groups(/* e5 */ self.right_anti_dual().group0()[2]);
    }
}
impl CoCarrier for QuadNumOnOrigin {
    type Output = QuadNumAligningOriginAtInfinity;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd2        0        2        0
    // no simd        0        4        0
    fn co_carrier(self) -> Self::Output {
        return QuadNumAligningOriginAtInfinity::from_groups(/* e5, e12345 */ Simd32x2::from([self.right_anti_dual().group0()[1], self.right_anti_dual().group0()[0]]));
    }
}
impl CoCarrier for QuadNumOrthogonalOrigin {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        1        0
    // no simd        0        3        0
    fn co_carrier(self) -> Self::Output {
        return AntiScalar::from_groups(/* e12345 */ self.right_anti_dual().group0()[0]);
    }
}
impl CoCarrier for RoundPoint {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        2        0
    fn co_carrier(self) -> Self::Output {
        use crate::elements::*;
        return AntiScalar::from_groups(/* e12345 */ self.right_anti_dual()[e4315]);
    }
}
impl CoCarrier for RoundPointAtOrigin {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd2        0        1        0
    // no simd        0        2        0
    fn co_carrier(self) -> Self::Output {
        return AntiScalar::from_groups(/* e12345 */ self.right_anti_dual().group0()[1]);
    }
}
impl CoCarrier for Scalar {
    type Output = Scalar;
    fn co_carrier(self) -> Self::Output {
        return self.right_anti_dual();
    }
}
impl CoCarrier for Sphere {
    type Output = RoundPoint;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        3        0
    fn co_carrier(self) -> Self::Output {
        use crate::elements::*;
        return self.right_anti_dual();
    }
}
impl CoCarrier for SphereAtOrigin {
    type Output = FlatOrigin;
    fn co_carrier(self) -> Self::Output {
        return FlatOrigin::from_groups(/* e45 */ self.right_anti_dual().group0()[0]);
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
impl CoCarrier for VersorEven {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0       32        0
    fn co_carrier(self) -> Self::Output {
        return Motor::from_groups(
            // e415, e425, e435, e12345
            Simd32x4::from([
                self.right_anti_dual().group0()[0],
                self.right_anti_dual().group0()[1],
                self.right_anti_dual().group0()[2],
                self.right_anti_dual().group2()[3],
            ]),
            // e235, e315, e125, e5
            Simd32x4::from([
                self.right_anti_dual().group1()[0],
                self.right_anti_dual().group1()[1],
                self.right_anti_dual().group1()[2],
                self.right_anti_dual().group0()[3],
            ]),
        );
    }
}
impl CoCarrier for VersorEvenAligningOrigin {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0       24        0
    fn co_carrier(self) -> Self::Output {
        return Motor::from_groups(
            // e415, e425, e435, e12345
            Simd32x4::from([
                self.right_anti_dual().group0()[0],
                self.right_anti_dual().group0()[1],
                self.right_anti_dual().group0()[2],
                self.right_anti_dual().group2()[3],
            ]),
            // e235, e315, e125, e5
            Simd32x4::from([
                self.right_anti_dual().group1()[0],
                self.right_anti_dual().group1()[1],
                self.right_anti_dual().group1()[2],
                self.right_anti_dual().group0()[3],
            ]),
        );
    }
}
impl CoCarrier for VersorEvenAtInfinity {
    type Output = MotorAtInfinity;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0       12        0
    fn co_carrier(self) -> Self::Output {
        return MotorAtInfinity::from_groups(/* e235, e315, e125, e5 */ Simd32x4::from([
            self.right_anti_dual().group1()[0],
            self.right_anti_dual().group1()[1],
            self.right_anti_dual().group1()[2],
            self.right_anti_dual().group0()[0],
        ]));
    }
}
impl CoCarrier for VersorEvenAtOrigin {
    type Output = MotorOnOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        8        0
    fn co_carrier(self) -> Self::Output {
        return MotorOnOrigin::from_groups(/* e415, e425, e435, e12345 */ Simd32x4::from([
            self.right_anti_dual().group0()[0],
            self.right_anti_dual().group0()[1],
            self.right_anti_dual().group0()[2],
            self.right_anti_dual().group1()[3],
        ]));
    }
}
impl CoCarrier for VersorEvenOnOrigin {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0       16        0
    fn co_carrier(self) -> Self::Output {
        return Motor::from_groups(
            // e415, e425, e435, e12345
            Simd32x4::from([
                self.right_anti_dual().group0()[0],
                self.right_anti_dual().group0()[1],
                self.right_anti_dual().group0()[2],
                self.right_anti_dual().group1()[3],
            ]),
            // e235, e315, e125, e5
            Simd32x4::from([
                self.right_anti_dual().group1()[0],
                self.right_anti_dual().group1()[1],
                self.right_anti_dual().group1()[2],
                self.right_anti_dual().group0()[3],
            ]),
        );
    }
}
impl CoCarrier for VersorEvenOrthogonalOrigin {
    type Output = MotorOnOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0       12        0
    fn co_carrier(self) -> Self::Output {
        return MotorOnOrigin::from_groups(/* e415, e425, e435, e12345 */ Simd32x4::from([
            self.right_anti_dual().group0()[0],
            self.right_anti_dual().group0()[1],
            self.right_anti_dual().group0()[2],
            self.right_anti_dual().group1()[3],
        ]));
    }
}
impl CoCarrier for VersorOdd {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0       60        0
    fn co_carrier(self) -> Self::Output {
        return Flector::from_groups(
            // e15, e25, e35, e45
            self.right_anti_dual().group3(),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([
                self.right_anti_dual().group0()[0],
                self.right_anti_dual().group0()[1],
                self.right_anti_dual().group0()[2],
                self.right_anti_dual().group1()[3],
            ]),
        );
    }
}
impl CoCarrier for VersorOddAtInfinity {
    type Output = FlectorAtInfinity;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0       36        0
    fn co_carrier(self) -> Self::Output {
        return FlectorAtInfinity::from_groups(/* e15, e25, e35, e3215 */ Simd32x4::from([
            self.right_anti_dual().group0()[1],
            self.right_anti_dual().group0()[2],
            self.right_anti_dual().group0()[3],
            self.right_anti_dual().group1()[3],
        ]));
    }
}
impl CoCarrier for VersorOddOrthogonalOrigin {
    type Output = FlectorOnOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0       36        0
    fn co_carrier(self) -> Self::Output {
        return FlectorOnOrigin::from_groups(/* e45, e4235, e4315, e4125 */ Simd32x4::from([
            self.right_anti_dual().group1()[3],
            self.right_anti_dual().group0()[0],
            self.right_anti_dual().group0()[1],
            self.right_anti_dual().group0()[2],
        ]));
    }
}
