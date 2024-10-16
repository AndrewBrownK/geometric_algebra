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
//  Minimum:         0       0       0
//   Median:         0       6       0
//  Average:         0       5       0
//  Maximum:         0      20       0
//
//  No SIMD:   add/sub     mul     div
//  Minimum:         0       0       0
//   Median:         0       6       0
//  Average:         0       5       0
//  Maximum:         0      20       0
impl Reverse for AntiCircleRotor {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0       10        0
    fn reverse(self) -> Self {
        return AntiCircleRotor::from_groups(
            // e41, e42, e43
            Simd32x3::from([(self.group0()[0] * -1.0), (self.group0()[1] * -1.0), (self.group0()[2] * -1.0)]),
            // e23, e31, e12, e45
            Simd32x4::from([(self.group1()[0] * -1.0), (self.group1()[1] * -1.0), (self.group1()[2] * -1.0), (self.group1()[3] * -1.0)]),
            // e15, e25, e35, scalar
            Simd32x4::from([(self.group2()[0] * -1.0), (self.group2()[1] * -1.0), (self.group2()[2] * -1.0), self.group2()[3]]),
        );
    }
}
impl Reverse for AntiDipoleInversion {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0       10        0
    fn reverse(self) -> Self {
        return AntiDipoleInversion::from_groups(
            // e423, e431, e412
            Simd32x3::from([(self.group0()[0] * -1.0), (self.group0()[1] * -1.0), (self.group0()[2] * -1.0)]),
            // e415, e425, e435, e321
            Simd32x4::from([(self.group1()[0] * -1.0), (self.group1()[1] * -1.0), (self.group1()[2] * -1.0), (self.group1()[3] * -1.0)]),
            // e235, e315, e125, e4
            Simd32x4::from([(self.group2()[0] * -1.0), (self.group2()[1] * -1.0), (self.group2()[2] * -1.0), self.group2()[3]]),
            // e1, e2, e3, e5
            self.group3(),
        );
    }
}
impl Reverse for AntiDualNum {
    fn reverse(self) -> Self {
        return self;
    }
}
impl Reverse for AntiFlatPoint {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        4        0
    fn reverse(self) -> Self {
        return AntiFlatPoint::from_groups(/* e235, e315, e125, e321 */ Simd32x4::from([
            (self.group0()[0] * -1.0),
            (self.group0()[1] * -1.0),
            (self.group0()[2] * -1.0),
            (self.group0()[3] * -1.0),
        ]));
    }
}
impl Reverse for AntiFlector {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        4        0
    fn reverse(self) -> Self {
        return AntiFlector::from_groups(
            // e235, e315, e125, e321
            Simd32x4::from([(self.group0()[0] * -1.0), (self.group0()[1] * -1.0), (self.group0()[2] * -1.0), (self.group0()[3] * -1.0)]),
            // e1, e2, e3, e5
            self.group1(),
        );
    }
}
impl Reverse for AntiLine {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        6        0
    fn reverse(self) -> Self {
        return AntiLine::from_groups(
            // e23, e31, e12
            Simd32x3::from([(self.group0()[0] * -1.0), (self.group0()[1] * -1.0), (self.group0()[2] * -1.0)]),
            // e15, e25, e35
            Simd32x3::from([(self.group1()[0] * -1.0), (self.group1()[1] * -1.0), (self.group1()[2] * -1.0)]),
        );
    }
}
impl Reverse for AntiMotor {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        6        0
    fn reverse(self) -> Self {
        return AntiMotor::from_groups(
            // e23, e31, e12, scalar
            Simd32x4::from([(self.group0()[0] * -1.0), (self.group0()[1] * -1.0), (self.group0()[2] * -1.0), self.group0()[3]]),
            // e15, e25, e35, e3215
            Simd32x4::from([(self.group1()[0] * -1.0), (self.group1()[1] * -1.0), (self.group1()[2] * -1.0), self.group1()[3]]),
        );
    }
}
impl Reverse for AntiPlane {
    fn reverse(self) -> Self {
        return self;
    }
}
impl Reverse for AntiScalar {
    fn reverse(self) -> Self {
        return self;
    }
}
impl Reverse for Circle {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0       10        0
    fn reverse(self) -> Self {
        return Circle::from_groups(
            // e423, e431, e412
            Simd32x3::from([(self.group0()[0] * -1.0), (self.group0()[1] * -1.0), (self.group0()[2] * -1.0)]),
            // e415, e425, e435, e321
            Simd32x4::from([(self.group1()[0] * -1.0), (self.group1()[1] * -1.0), (self.group1()[2] * -1.0), (self.group1()[3] * -1.0)]),
            // e235, e315, e125
            Simd32x3::from([(self.group2()[0] * -1.0), (self.group2()[1] * -1.0), (self.group2()[2] * -1.0)]),
        );
    }
}
impl Reverse for CircleRotor {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0       10        0
    fn reverse(self) -> Self {
        return CircleRotor::from_groups(
            // e423, e431, e412
            Simd32x3::from([(self.group0()[0] * -1.0), (self.group0()[1] * -1.0), (self.group0()[2] * -1.0)]),
            // e415, e425, e435, e321
            Simd32x4::from([(self.group1()[0] * -1.0), (self.group1()[1] * -1.0), (self.group1()[2] * -1.0), (self.group1()[3] * -1.0)]),
            // e235, e315, e125, e12345
            Simd32x4::from([(self.group2()[0] * -1.0), (self.group2()[1] * -1.0), (self.group2()[2] * -1.0), self.group2()[3]]),
        );
    }
}
impl Reverse for Dipole {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0       10        0
    fn reverse(self) -> Self {
        return Dipole::from_groups(
            // e41, e42, e43
            Simd32x3::from([(self.group0()[0] * -1.0), (self.group0()[1] * -1.0), (self.group0()[2] * -1.0)]),
            // e23, e31, e12, e45
            Simd32x4::from([(self.group1()[0] * -1.0), (self.group1()[1] * -1.0), (self.group1()[2] * -1.0), (self.group1()[3] * -1.0)]),
            // e15, e25, e35
            Simd32x3::from([(self.group2()[0] * -1.0), (self.group2()[1] * -1.0), (self.group2()[2] * -1.0)]),
        );
    }
}
impl Reverse for DipoleInversion {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0       10        0
    fn reverse(self) -> Self {
        return DipoleInversion::from_groups(
            // e41, e42, e43
            Simd32x3::from([(self.group0()[0] * -1.0), (self.group0()[1] * -1.0), (self.group0()[2] * -1.0)]),
            // e23, e31, e12, e45
            Simd32x4::from([(self.group1()[0] * -1.0), (self.group1()[1] * -1.0), (self.group1()[2] * -1.0), (self.group1()[3] * -1.0)]),
            // e15, e25, e35, e1234
            Simd32x4::from([(self.group2()[0] * -1.0), (self.group2()[1] * -1.0), (self.group2()[2] * -1.0), self.group2()[3]]),
            // e4235, e4315, e4125, e3215
            self.group3(),
        );
    }
}
impl Reverse for DualNum {
    fn reverse(self) -> Self {
        return self;
    }
}
impl Reverse for FlatPoint {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        4        0
    fn reverse(self) -> Self {
        return FlatPoint::from_groups(/* e15, e25, e35, e45 */ Simd32x4::from([
            (self.group0()[0] * -1.0),
            (self.group0()[1] * -1.0),
            (self.group0()[2] * -1.0),
            (self.group0()[3] * -1.0),
        ]));
    }
}
impl Reverse for Flector {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        4        0
    fn reverse(self) -> Self {
        return Flector::from_groups(
            // e15, e25, e35, e45
            Simd32x4::from([(self.group0()[0] * -1.0), (self.group0()[1] * -1.0), (self.group0()[2] * -1.0), (self.group0()[3] * -1.0)]),
            // e4235, e4315, e4125, e3215
            self.group1(),
        );
    }
}
impl Reverse for Line {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        6        0
    fn reverse(self) -> Self {
        return Line::from_groups(
            // e415, e425, e435
            Simd32x3::from([(self.group0()[0] * -1.0), (self.group0()[1] * -1.0), (self.group0()[2] * -1.0)]),
            // e235, e315, e125
            Simd32x3::from([(self.group1()[0] * -1.0), (self.group1()[1] * -1.0), (self.group1()[2] * -1.0)]),
        );
    }
}
impl Reverse for Motor {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        6        0
    fn reverse(self) -> Self {
        return Motor::from_groups(
            // e415, e425, e435, e12345
            Simd32x4::from([(self.group0()[0] * -1.0), (self.group0()[1] * -1.0), (self.group0()[2] * -1.0), self.group0()[3]]),
            // e235, e315, e125, e5
            Simd32x4::from([(self.group1()[0] * -1.0), (self.group1()[1] * -1.0), (self.group1()[2] * -1.0), self.group1()[3]]),
        );
    }
}
impl Reverse for MultiVector {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0       20        0
    fn reverse(self) -> Self {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e1],
            // e15, e25, e35, e45
            Simd32x4::from([(self.group3()[0] * -1.0), (self.group3()[1] * -1.0), (self.group3()[2] * -1.0), (self.group3()[3] * -1.0)]),
            // e41, e42, e43
            Simd32x3::from([(self.group4()[0] * -1.0), (self.group4()[1] * -1.0), (self.group4()[2] * -1.0)]),
            // e23, e31, e12
            Simd32x3::from([(self.group5()[0] * -1.0), (self.group5()[1] * -1.0), (self.group5()[2] * -1.0)]),
            // e415, e425, e435, e321
            Simd32x4::from([(self.group6()[0] * -1.0), (self.group6()[1] * -1.0), (self.group6()[2] * -1.0), (self.group6()[3] * -1.0)]),
            // e423, e431, e412
            Simd32x3::from([(self.group7()[0] * -1.0), (self.group7()[1] * -1.0), (self.group7()[2] * -1.0)]),
            // e235, e315, e125
            Simd32x3::from([(self.group8()[0] * -1.0), (self.group8()[1] * -1.0), (self.group8()[2] * -1.0)]),
            // e4235, e4315, e4125, e3215
            self.group9(),
            // e1234
            self[e45],
        );
    }
}
impl Reverse for Plane {
    fn reverse(self) -> Self {
        return self;
    }
}
impl Reverse for RoundPoint {
    fn reverse(self) -> Self {
        return self;
    }
}
impl Reverse for Scalar {
    fn reverse(self) -> Self {
        return self;
    }
}
impl Reverse for Sphere {
    fn reverse(self) -> Self {
        return self;
    }
}
impl Reverse for VersorEven {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0       10        0
    fn reverse(self) -> Self {
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([(self.group0()[0] * -1.0), (self.group0()[1] * -1.0), (self.group0()[2] * -1.0), self.group0()[3]]),
            // e415, e425, e435, e321
            Simd32x4::from([(self.group1()[0] * -1.0), (self.group1()[1] * -1.0), (self.group1()[2] * -1.0), (self.group1()[3] * -1.0)]),
            // e235, e315, e125, e5
            Simd32x4::from([(self.group2()[0] * -1.0), (self.group2()[1] * -1.0), (self.group2()[2] * -1.0), self.group2()[3]]),
            // e1, e2, e3, e4
            self.group3(),
        );
    }
}
impl Reverse for VersorOdd {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0       10        0
    fn reverse(self) -> Self {
        return VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([(self.group0()[0] * -1.0), (self.group0()[1] * -1.0), (self.group0()[2] * -1.0), self.group0()[3]]),
            // e23, e31, e12, e45
            Simd32x4::from([(self.group1()[0] * -1.0), (self.group1()[1] * -1.0), (self.group1()[2] * -1.0), (self.group1()[3] * -1.0)]),
            // e15, e25, e35, e1234
            Simd32x4::from([(self.group2()[0] * -1.0), (self.group2()[1] * -1.0), (self.group2()[2] * -1.0), self.group2()[3]]),
            // e4235, e4315, e4125, e3215
            self.group3(),
        );
    }
}
