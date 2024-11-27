use crate::traits::GeometricProduct;
use crate::traits::RightDual;
use crate::traits::Wedge;
// Note on Operative Statistics:
// Operative Statistics are not a precise predictor of performance or performance comparisons.
// This is due to varying hardware capabilities and compiler optimizations.
// As always, where performance is a concern, there is no substitute for
// real measurements on real work-loads on real hardware.
// Disclaimer aside, enjoy the fun information =)
//
// Total Implementations: 385
//
// Yes SIMD:   add/sub     mul     div
//  Minimum:         0       0       0
//   Median:         0       2       0
//  Average:         4       7       0
//  Maximum:        74      96       0
//
//  No SIMD:   add/sub     mul     div
//  Minimum:         0       0       0
//   Median:         0       6       0
//  Average:         5      10       0
//  Maximum:       104     128       0
impl std::ops::Add<AntiCircleOnOrigin> for MotorAtInfinity {
    type Output = MultiVector;
    fn add(self, other: AntiCircleOnOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            self[e5],
            // e41, e42, e43, e45
            crate::swizzle!(other.group0(), 0, 1, 2).extend_to_4(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            other.group1(),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            self.group0().truncate_to_3(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Add<AntiCircleRotor> for MotorAtInfinity {
    type Output = MultiVector;
    fn add(self, other: AntiCircleRotor) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([other[scalar], 0.0]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            self[e5],
            // e41, e42, e43, e45
            crate::swizzle!(other.group0(), 0, 1, 2).extend_to_4(other[e45]),
            // e15, e25, e35
            other.group2().truncate_to_3(),
            // e23, e31, e12
            other.group1().truncate_to_3(),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            self.group0().truncate_to_3(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Add<AntiCircleRotorAligningOrigin> for MotorAtInfinity {
    type Output = MultiVector;
    fn add(self, other: AntiCircleRotorAligningOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([other[scalar], 0.0]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            self[e5],
            // e41, e42, e43, e45
            crate::swizzle!(other.group0(), 0, 1, 2).extend_to_4(0.0),
            // e15, e25, e35
            other.group2().truncate_to_3(),
            // e23, e31, e12
            other.group1(),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            self.group0().truncate_to_3(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Add<AntiCircleRotorAligningOriginAtInfinity> for MotorAtInfinity {
    type Output = MultiVector;
    fn add(self, other: AntiCircleRotorAligningOriginAtInfinity) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([other[scalar], 0.0]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            self[e5],
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            other.group1().truncate_to_3(),
            // e23, e31, e12
            other.group0(),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            self.group0().truncate_to_3(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Add<AntiCircleRotorAtInfinity> for MotorAtInfinity {
    type Output = MultiVector;
    fn add(self, other: AntiCircleRotorAtInfinity) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([other[scalar], 0.0]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            self[e5],
            // e41, e42, e43, e45
            Simd32x3::from(0.0).extend_to_4(other[e45]),
            // e15, e25, e35
            other.group1().truncate_to_3(),
            // e23, e31, e12
            other.group0().truncate_to_3(),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            self.group0().truncate_to_3(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Add<AntiCircleRotorOnOrigin> for MotorAtInfinity {
    type Output = MultiVector;
    fn add(self, other: AntiCircleRotorOnOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([other[scalar], 0.0]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            self[e5],
            // e41, e42, e43, e45
            Simd32x4::from([other[e41], other[e42], other[e43], 0.0]),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            other.group1(),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            self.group0().truncate_to_3(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Add<AntiDipoleInversion> for MotorAtInfinity {
    type Output = AntiDipoleInversion;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        4        0        0
    fn add(self, other: AntiDipoleInversion) -> Self::Output {
        use crate::elements::*;
        return AntiDipoleInversion::from_groups(
            // e423, e431, e412
            other.group0(),
            // e415, e425, e435, e321
            other.group1(),
            // e235, e315, e125, e4
            Simd32x4::from([other[e235] + self[e235], other[e315] + self[e315], other[e125] + self[e125], other[e4]]),
            // e1, e2, e3, e5
            Simd32x4::from([other[e1], other[e2], other[e3], other[e5] + self[e5]]),
        );
    }
}
impl std::ops::Add<AntiDipoleInversionAtInfinity> for MotorAtInfinity {
    type Output = AntiDipoleInversionAtInfinity;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        4        0        0
    fn add(self, other: AntiDipoleInversionAtInfinity) -> Self::Output {
        use crate::elements::*;
        return AntiDipoleInversionAtInfinity::from_groups(
            // e415, e425, e435, e321
            other.group0(),
            // e235, e315, e125
            Simd32x3::from([other[e235] + self[e235], other[e315] + self[e315], other[e125] + self[e125]]),
            // e1, e2, e3, e5
            Simd32x4::from([other[e1], other[e2], other[e3], other[e5] + self[e5]]),
        );
    }
}
impl std::ops::Add<AntiDipoleInversionOnOrigin> for MotorAtInfinity {
    type Output = VersorEvenOrthogonalOrigin;
    fn add(self, other: AntiDipoleInversionOnOrigin) -> Self::Output {
        return VersorEvenOrthogonalOrigin::from_groups(
            // e423, e431, e412, e321
            other.group0(),
            // e235, e315, e125, e5
            self.group0(),
            // e1, e2, e3, e4
            crate::swizzle!(other.group1(), 1, 2, 3, 0),
        );
    }
}
impl std::ops::Add<AntiDipoleInversionOrthogonalOrigin> for MotorAtInfinity {
    type Output = AntiDipoleInversionOrthogonalOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        4        0        0
    fn add(self, other: AntiDipoleInversionOrthogonalOrigin) -> Self::Output {
        use crate::elements::*;
        return AntiDipoleInversionOrthogonalOrigin::from_groups(
            // e423, e431, e412, e5
            Simd32x4::from([other[e423], other[e431], other[e412], other[e5] + self[e5]]),
            // e415, e425, e435
            other.group1(),
            // e235, e315, e125, e4
            Simd32x4::from([other[e235] + self[e235], other[e315] + self[e315], other[e125] + self[e125], other[e4]]),
        );
    }
}
impl std::ops::Add<AntiDipoleOnOrigin> for MotorAtInfinity {
    type Output = VersorEvenOrthogonalOrigin;
    fn add(self, other: AntiDipoleOnOrigin) -> Self::Output {
        return VersorEvenOrthogonalOrigin::from_groups(
            // e423, e431, e412, e321
            other.group0(),
            // e235, e315, e125, e5
            self.group0(),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
        );
    }
}
impl std::ops::Add<AntiDualNum> for MotorAtInfinity {
    type Output = MultiVector;
    fn add(self, other: AntiDualNum) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([other[scalar], 0.0]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            self[e5],
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            self.group0().truncate_to_3(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([other[e1234], 0.0, 0.0, 0.0]),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Add<AntiFlatOrigin> for MotorAtInfinity {
    type Output = AntiFlector;
    fn add(self, other: AntiFlatOrigin) -> Self::Output {
        use crate::elements::*;
        return AntiFlector::from_groups(
            // e235, e315, e125, e321
            Simd32x4::from([self[e235], self[e315], self[e125], other[e321]]),
            // e1, e2, e3, e5
            Simd32x3::from(0.0).extend_to_4(self[e5]),
        );
    }
}
impl std::ops::Add<AntiFlatPoint> for MotorAtInfinity {
    type Output = AntiFlector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        0        0
    fn add(self, other: AntiFlatPoint) -> Self::Output {
        use crate::elements::*;
        return AntiFlector::from_groups(
            // e235, e315, e125, e321
            Simd32x4::from([other[e235] + self[e235], other[e315] + self[e315], other[e125] + self[e125], other[e321]]),
            // e1, e2, e3, e5
            Simd32x3::from(0.0).extend_to_4(self[e5]),
        );
    }
}
impl std::ops::Add<AntiFlector> for MotorAtInfinity {
    type Output = AntiFlector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        4        0        0
    fn add(self, other: AntiFlector) -> Self::Output {
        use crate::elements::*;
        return AntiFlector::from_groups(
            // e235, e315, e125, e321
            Simd32x4::from([other[e235] + self[e235], other[e315] + self[e315], other[e125] + self[e125], other[e321]]),
            // e1, e2, e3, e5
            Simd32x4::from([other[e1], other[e2], other[e3], other[e5] + self[e5]]),
        );
    }
}
impl std::ops::Add<AntiFlectorOnOrigin> for MotorAtInfinity {
    type Output = AntiFlector;
    fn add(self, other: AntiFlectorOnOrigin) -> Self::Output {
        use crate::elements::*;
        return AntiFlector::from_groups(
            // e235, e315, e125, e321
            Simd32x4::from([self[e235], self[e315], self[e125], other[e321]]),
            // e1, e2, e3, e5
            Simd32x4::from([other[e1], other[e2], other[e3], self[e5]]),
        );
    }
}
impl std::ops::Add<AntiLine> for MotorAtInfinity {
    type Output = MultiVector;
    fn add(self, other: AntiLine) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            self[e5],
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            other.group1(),
            // e23, e31, e12
            other.group0(),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            self.group0().truncate_to_3(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Add<AntiLineOnOrigin> for MotorAtInfinity {
    type Output = MultiVector;
    fn add(self, other: AntiLineOnOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            self[e5],
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            other.group0(),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            self.group0().truncate_to_3(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Add<AntiMotor> for MotorAtInfinity {
    type Output = MultiVector;
    fn add(self, other: AntiMotor) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([other[scalar], 0.0]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            self[e5],
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            other.group1().truncate_to_3(),
            // e23, e31, e12
            other.group0().truncate_to_3(),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            self.group0().truncate_to_3(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            other[e3215],
        );
    }
}
impl std::ops::Add<AntiMotorOnOrigin> for MotorAtInfinity {
    type Output = MultiVector;
    fn add(self, other: AntiMotorOnOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([other[scalar], 0.0]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            self[e5],
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            other.group0().truncate_to_3(),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            self.group0().truncate_to_3(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Add<AntiMysteryCircleRotor> for MotorAtInfinity {
    type Output = MultiVector;
    fn add(self, other: AntiMysteryCircleRotor) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([other[scalar], 0.0]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            self[e5],
            // e41, e42, e43, e45
            Simd32x3::from(0.0).extend_to_4(other[e45]),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            other.group0().truncate_to_3(),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            self.group0().truncate_to_3(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Add<AntiMysteryDipoleInversion> for MotorAtInfinity {
    type Output = AntiDipoleInversionAtInfinity;
    fn add(self, other: AntiMysteryDipoleInversion) -> Self::Output {
        use crate::elements::*;
        return AntiDipoleInversionAtInfinity::from_groups(
            // e415, e425, e435, e321
            other.group0(),
            // e235, e315, e125
            self.group0().truncate_to_3(),
            // e1, e2, e3, e5
            crate::swizzle!(other.group1(), 0, 1, 2).extend_to_4(self[e5]),
        );
    }
}
impl std::ops::Add<AntiPlane> for MotorAtInfinity {
    type Output = AntiFlector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        1        0        0
    fn add(self, other: AntiPlane) -> Self::Output {
        use crate::elements::*;
        return AntiFlector::from_groups(
            // e235, e315, e125, e321
            Simd32x4::from([self[e235], self[e315], self[e125], 0.0]),
            // e1, e2, e3, e5
            Simd32x4::from([other[e1], other[e2], other[e3], other[e5] + self[e5]]),
        );
    }
}
impl std::ops::Add<AntiPlaneOnOrigin> for MotorAtInfinity {
    type Output = AntiFlector;
    fn add(self, other: AntiPlaneOnOrigin) -> Self::Output {
        use crate::elements::*;
        return AntiFlector::from_groups(
            // e235, e315, e125, e321
            Simd32x4::from([self[e235], self[e315], self[e125], 0.0]),
            // e1, e2, e3, e5
            crate::swizzle!(other.group0(), 0, 1, 2).extend_to_4(self[e5]),
        );
    }
}
impl std::ops::Add<AntiScalar> for MotorAtInfinity {
    type Output = Motor;
    fn add(self, other: AntiScalar) -> Self::Output {
        use crate::elements::*;
        return Motor::from_groups(
            // e415, e425, e435, e12345
            Simd32x3::from(0.0).extend_to_4(other[e12345]),
            // e235, e315, e125, e5
            self.group0(),
        );
    }
}
impl std::ops::Add<AntiSphereOnOrigin> for MotorAtInfinity {
    type Output = VersorEvenOrthogonalOrigin;
    fn add(self, other: AntiSphereOnOrigin) -> Self::Output {
        return VersorEvenOrthogonalOrigin::from_groups(
            // e423, e431, e412, e321
            Simd32x4::from(0.0),
            // e235, e315, e125, e5
            self.group0(),
            // e1, e2, e3, e4
            other.group0(),
        );
    }
}
impl std::ops::Add<AntiVersorEvenOnOrigin> for MotorAtInfinity {
    type Output = MultiVector;
    fn add(self, other: AntiVersorEvenOnOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([other[scalar], 0.0]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            self[e5],
            // e41, e42, e43, e45
            Simd32x4::from([other[e41], other[e42], other[e43], 0.0]),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            other.group1().truncate_to_3(),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            self.group0().truncate_to_3(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([other[e1234], 0.0, 0.0, 0.0]),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Add<Circle> for MotorAtInfinity {
    type Output = AntiDipoleInversion;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        0        0
    fn add(self, other: Circle) -> Self::Output {
        use crate::elements::*;
        return AntiDipoleInversion::from_groups(
            // e423, e431, e412
            other.group0(),
            // e415, e425, e435, e321
            other.group1(),
            // e235, e315, e125, e4
            Simd32x4::from([other[e235] + self[e235], other[e315] + self[e315], other[e125] + self[e125], 0.0]),
            // e1, e2, e3, e5
            Simd32x3::from(0.0).extend_to_4(self[e5]),
        );
    }
}
impl std::ops::Add<CircleAligningOrigin> for MotorAtInfinity {
    type Output = AntiDipoleInversionOrthogonalOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        0        0
    fn add(self, other: CircleAligningOrigin) -> Self::Output {
        use crate::elements::*;
        return AntiDipoleInversionOrthogonalOrigin::from_groups(
            // e423, e431, e412, e5
            crate::swizzle!(other.group0(), 0, 1, 2).extend_to_4(self[e5]),
            // e415, e425, e435
            other.group1(),
            // e235, e315, e125, e4
            Simd32x4::from([other[e235] + self[e235], other[e315] + self[e315], other[e125] + self[e125], 0.0]),
        );
    }
}
impl std::ops::Add<CircleAtInfinity> for MotorAtInfinity {
    type Output = AntiDipoleInversionAtInfinity;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        0        0
    fn add(self, other: CircleAtInfinity) -> Self::Output {
        use crate::elements::*;
        return AntiDipoleInversionAtInfinity::from_groups(
            // e415, e425, e435, e321
            other.group0(),
            // e235, e315, e125
            Simd32x3::from([other[e235] + self[e235], other[e315] + self[e315], other[e125] + self[e125]]),
            // e1, e2, e3, e5
            Simd32x3::from(0.0).extend_to_4(self[e5]),
        );
    }
}
impl std::ops::Add<CircleAtOrigin> for MotorAtInfinity {
    type Output = VersorEvenAtOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        0        0
    fn add(self, other: CircleAtOrigin) -> Self::Output {
        use crate::elements::*;
        return VersorEvenAtOrigin::from_groups(
            // e423, e431, e412, e4
            crate::swizzle!(other.group0(), 0, 1, 2).extend_to_4(0.0),
            // e235, e315, e125, e5
            Simd32x4::from([other[e235] + self[e235], other[e315] + self[e315], other[e125] + self[e125], self[e5]]),
        );
    }
}
impl std::ops::Add<CircleOnOrigin> for MotorAtInfinity {
    type Output = AntiDipoleInversionOrthogonalOrigin;
    fn add(self, other: CircleOnOrigin) -> Self::Output {
        use crate::elements::*;
        return AntiDipoleInversionOrthogonalOrigin::from_groups(
            // e423, e431, e412, e5
            crate::swizzle!(other.group0(), 0, 1, 2).extend_to_4(self[e5]),
            // e415, e425, e435
            other.group1(),
            // e235, e315, e125, e4
            Simd32x4::from([self[e235], self[e315], self[e125], 0.0]),
        );
    }
}
impl std::ops::Add<CircleOrthogonalOrigin> for MotorAtInfinity {
    type Output = VersorEvenOrthogonalOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        0        0
    fn add(self, other: CircleOrthogonalOrigin) -> Self::Output {
        use crate::elements::*;
        return VersorEvenOrthogonalOrigin::from_groups(
            // e423, e431, e412, e321
            other.group0(),
            // e235, e315, e125, e5
            Simd32x4::from([other[e235] + self[e235], other[e315] + self[e315], other[e125] + self[e125], self[e5]]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
        );
    }
}
impl std::ops::Add<CircleRotor> for MotorAtInfinity {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        0        0
    fn add(self, other: CircleRotor) -> Self::Output {
        use crate::elements::*;
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            crate::swizzle!(other.group0(), 0, 1, 2).extend_to_4(other[e12345]),
            // e415, e425, e435, e321
            other.group1(),
            // e235, e315, e125, e5
            Simd32x4::from([other[e235] + self[e235], other[e315] + self[e315], other[e125] + self[e125], self[e5]]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
        );
    }
}
impl std::ops::Add<CircleRotorAligningOrigin> for MotorAtInfinity {
    type Output = VersorEvenAligningOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        0        0
    fn add(self, other: CircleRotorAligningOrigin) -> Self::Output {
        use crate::elements::*;
        return VersorEvenAligningOrigin::from_groups(
            // e423, e431, e412, e12345
            crate::swizzle!(other.group0(), 0, 1, 2).extend_to_4(other[e12345]),
            // e415, e425, e435, e4
            crate::swizzle!(other.group1(), 0, 1, 2).extend_to_4(0.0),
            // e235, e315, e125, e5
            Simd32x4::from([other[e235] + self[e235], other[e315] + self[e315], other[e125] + self[e125], self[e5]]),
        );
    }
}
impl std::ops::Add<CircleRotorAligningOriginAtInfinity> for MotorAtInfinity {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        0        0
    fn add(self, other: CircleRotorAligningOriginAtInfinity) -> Self::Output {
        use crate::elements::*;
        return Motor::from_groups(
            // e415, e425, e435, e12345
            crate::swizzle!(other.group0(), 0, 1, 2).extend_to_4(other[e12345]),
            // e235, e315, e125, e5
            Simd32x4::from([other[e235] + self[e235], other[e315] + self[e315], other[e125] + self[e125], self[e5]]),
        );
    }
}
impl std::ops::Add<CircleRotorAtInfinity> for MotorAtInfinity {
    type Output = VersorEvenAtInfinity;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        0        0
    fn add(self, other: CircleRotorAtInfinity) -> Self::Output {
        use crate::elements::*;
        return VersorEvenAtInfinity::from_groups(
            // e12345, e1, e2, e3
            Simd32x4::from([other[e12345], 0.0, 0.0, 0.0]),
            // e415, e425, e435, e321
            other.group0(),
            // e235, e315, e125, e5
            Simd32x4::from([other[e235] + self[e235], other[e315] + self[e315], other[e125] + self[e125], self[e5]]),
        );
    }
}
impl std::ops::Add<CircleRotorOnOrigin> for MotorAtInfinity {
    type Output = VersorEvenAligningOrigin;
    fn add(self, other: CircleRotorOnOrigin) -> Self::Output {
        return VersorEvenAligningOrigin::from_groups(
            // e423, e431, e412, e12345
            other.group0(),
            // e415, e425, e435, e4
            crate::swizzle!(other.group1(), 0, 1, 2).extend_to_4(0.0),
            // e235, e315, e125, e5
            self.group0(),
        );
    }
}
impl std::ops::Add<Dipole> for MotorAtInfinity {
    type Output = MultiVector;
    fn add(self, other: Dipole) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            self[e5],
            // e41, e42, e43, e45
            crate::swizzle!(other.group0(), 0, 1, 2).extend_to_4(other[e45]),
            // e15, e25, e35
            other.group2(),
            // e23, e31, e12
            other.group1().truncate_to_3(),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            self.group0().truncate_to_3(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Add<DipoleAligningOrigin> for MotorAtInfinity {
    type Output = MultiVector;
    fn add(self, other: DipoleAligningOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            self[e5],
            // e41, e42, e43, e45
            other.group0(),
            // e15, e25, e35
            other.group1(),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            self.group0().truncate_to_3(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Add<DipoleAtInfinity> for MotorAtInfinity {
    type Output = MultiVector;
    fn add(self, other: DipoleAtInfinity) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            self[e5],
            // e41, e42, e43, e45
            Simd32x3::from(0.0).extend_to_4(other[e45]),
            // e15, e25, e35
            other.group1(),
            // e23, e31, e12
            other.group0().truncate_to_3(),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            self.group0().truncate_to_3(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Add<DipoleAtOrigin> for MotorAtInfinity {
    type Output = MultiVector;
    fn add(self, other: DipoleAtOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            self[e5],
            // e41, e42, e43, e45
            crate::swizzle!(other.group0(), 0, 1, 2).extend_to_4(0.0),
            // e15, e25, e35
            other.group1(),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            self.group0().truncate_to_3(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Add<DipoleInversion> for MotorAtInfinity {
    type Output = MultiVector;
    fn add(self, other: DipoleInversion) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            self[e5],
            // e41, e42, e43, e45
            crate::swizzle!(other.group0(), 0, 1, 2).extend_to_4(other[e45]),
            // e15, e25, e35
            other.group2().truncate_to_3(),
            // e23, e31, e12
            other.group1().truncate_to_3(),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            self.group0().truncate_to_3(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([other[e1234], other[e4235], other[e4315], other[e4125]]),
            // e3215
            other[e3215],
        );
    }
}
impl std::ops::Add<DipoleInversionAligningOrigin> for MotorAtInfinity {
    type Output = MultiVector;
    fn add(self, other: DipoleInversionAligningOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            self[e5],
            // e41, e42, e43, e45
            other.group0(),
            // e15, e25, e35
            other.group1().truncate_to_3(),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            self.group0().truncate_to_3(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([other[e1234], other[e4235], other[e4315], other[e4125]]),
            // e3215
            other[e3215],
        );
    }
}
impl std::ops::Add<DipoleInversionAtInfinity> for MotorAtInfinity {
    type Output = MultiVector;
    fn add(self, other: DipoleInversionAtInfinity) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            self[e5],
            // e41, e42, e43, e45
            Simd32x3::from(0.0).extend_to_4(other[e45]),
            // e15, e25, e35
            other.group1(),
            // e23, e31, e12
            other.group0().truncate_to_3(),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            self.group0().truncate_to_3(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([0.0, other[e4235], other[e4315], other[e4125]]),
            // e3215
            other[e3215],
        );
    }
}
impl std::ops::Add<DipoleInversionAtOrigin> for MotorAtInfinity {
    type Output = MultiVector;
    fn add(self, other: DipoleInversionAtOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            self[e5],
            // e41, e42, e43, e45
            Simd32x4::from([other[e41], other[e42], other[e43], 0.0]),
            // e15, e25, e35
            other.group1().truncate_to_3(),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            self.group0().truncate_to_3(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([other[e1234], 0.0, 0.0, 0.0]),
            // e3215
            other[e3215],
        );
    }
}
impl std::ops::Add<DipoleInversionOnOrigin> for MotorAtInfinity {
    type Output = MultiVector;
    fn add(self, other: DipoleInversionOnOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            self[e5],
            // e41, e42, e43, e45
            other.group0(),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            self.group0().truncate_to_3(),
            // e1234, e4235, e4315, e4125
            other.group1(),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Add<DipoleInversionOrthogonalOrigin> for MotorAtInfinity {
    type Output = MultiVector;
    fn add(self, other: DipoleInversionOrthogonalOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            self[e5],
            // e41, e42, e43, e45
            Simd32x4::from([other[e41], other[e42], other[e43], 0.0]),
            // e15, e25, e35
            other.group2().truncate_to_3(),
            // e23, e31, e12
            other.group1(),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            self.group0().truncate_to_3(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([other[e1234], 0.0, 0.0, 0.0]),
            // e3215
            other[e3215],
        );
    }
}
impl std::ops::Add<DipoleOnOrigin> for MotorAtInfinity {
    type Output = MultiVector;
    fn add(self, other: DipoleOnOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            self[e5],
            // e41, e42, e43, e45
            other.group0(),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            self.group0().truncate_to_3(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Add<DipoleOrthogonalOrigin> for MotorAtInfinity {
    type Output = MultiVector;
    fn add(self, other: DipoleOrthogonalOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            self[e5],
            // e41, e42, e43, e45
            crate::swizzle!(other.group0(), 0, 1, 2).extend_to_4(0.0),
            // e15, e25, e35
            other.group2(),
            // e23, e31, e12
            other.group1(),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            self.group0().truncate_to_3(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Add<DualNum> for MotorAtInfinity {
    type Output = VersorEvenAligningOrigin;
    fn add(self, other: DualNum) -> Self::Output {
        use crate::elements::*;
        return VersorEvenAligningOrigin::from_groups(
            // e423, e431, e412, e12345
            Simd32x3::from(0.0).extend_to_4(other[e12345]),
            // e415, e425, e435, e4
            Simd32x3::from(0.0).extend_to_4(other[e4]),
            // e235, e315, e125, e5
            self.group0(),
        );
    }
}
impl std::ops::Add<FlatOrigin> for MotorAtInfinity {
    type Output = MultiVector;
    fn add(self, other: FlatOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            self[e5],
            // e41, e42, e43, e45
            Simd32x3::from(0.0).extend_to_4(other[e45]),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            self.group0().truncate_to_3(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Add<FlatPoint> for MotorAtInfinity {
    type Output = MultiVector;
    fn add(self, other: FlatPoint) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            self[e5],
            // e41, e42, e43, e45
            Simd32x3::from(0.0).extend_to_4(other[e45]),
            // e15, e25, e35
            other.group0().truncate_to_3(),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            self.group0().truncate_to_3(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Add<FlatPointAtInfinity> for MotorAtInfinity {
    type Output = MultiVector;
    fn add(self, other: FlatPointAtInfinity) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            self[e5],
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            other.group0(),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            self.group0().truncate_to_3(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Add<Flector> for MotorAtInfinity {
    type Output = MultiVector;
    fn add(self, other: Flector) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            self[e5],
            // e41, e42, e43, e45
            Simd32x3::from(0.0).extend_to_4(other[e45]),
            // e15, e25, e35
            other.group0().truncate_to_3(),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            self.group0().truncate_to_3(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([0.0, other[e4235], other[e4315], other[e4125]]),
            // e3215
            other[e3215],
        );
    }
}
impl std::ops::Add<FlectorAtInfinity> for MotorAtInfinity {
    type Output = MultiVector;
    fn add(self, other: FlectorAtInfinity) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            self[e5],
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            other.group0().truncate_to_3(),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            self.group0().truncate_to_3(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            other[e3215],
        );
    }
}
impl std::ops::Add<FlectorOnOrigin> for MotorAtInfinity {
    type Output = MultiVector;
    fn add(self, other: FlectorOnOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            self[e5],
            // e41, e42, e43, e45
            Simd32x3::from(0.0).extend_to_4(other[e45]),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            self.group0().truncate_to_3(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([0.0, other[e4235], other[e4315], other[e4125]]),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Add<Horizon> for MotorAtInfinity {
    type Output = MultiVector;
    fn add(self, other: Horizon) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            self[e5],
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            self.group0().truncate_to_3(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            other[e3215],
        );
    }
}
impl std::ops::Add<Infinity> for MotorAtInfinity {
    type Output = MotorAtInfinity;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        1        0        0
    fn add(self, other: Infinity) -> Self::Output {
        use crate::elements::*;
        return MotorAtInfinity::from_groups(/* e235, e315, e125, e5 */ Simd32x4::from([self[e235], self[e315], self[e125], other[e5] + self[e5]]));
    }
}
impl std::ops::AddAssign<Infinity> for MotorAtInfinity {
    fn add_assign(&mut self, other: Infinity) {
        use crate::elements::*;
        *self = MotorAtInfinity::from_groups(/* e235, e315, e125, e5 */ Simd32x4::from([self[e235], self[e315], self[e125], other[e5] + self[e5]]));
    }
}
impl std::ops::Add<Line> for MotorAtInfinity {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        0        0
    fn add(self, other: Line) -> Self::Output {
        use crate::elements::*;
        return Motor::from_groups(
            // e415, e425, e435, e12345
            crate::swizzle!(other.group0(), 0, 1, 2).extend_to_4(0.0),
            // e235, e315, e125, e5
            Simd32x4::from([other[e235] + self[e235], other[e315] + self[e315], other[e125] + self[e125], self[e5]]),
        );
    }
}
impl std::ops::Add<LineAtInfinity> for MotorAtInfinity {
    type Output = MotorAtInfinity;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        0        0
    fn add(self, other: LineAtInfinity) -> Self::Output {
        use crate::elements::*;
        return MotorAtInfinity::from_groups(
            // e235, e315, e125, e5
            Simd32x4::from([other[e235] + self[e235], other[e315] + self[e315], other[e125] + self[e125], self[e5]]),
        );
    }
}
impl std::ops::AddAssign<LineAtInfinity> for MotorAtInfinity {
    fn add_assign(&mut self, other: LineAtInfinity) {
        use crate::elements::*;
        *self = MotorAtInfinity::from_groups(
            // e235, e315, e125, e5
            Simd32x4::from([other[e235] + self[e235], other[e315] + self[e315], other[e125] + self[e125], self[e5]]),
        );
    }
}
impl std::ops::Add<LineOnOrigin> for MotorAtInfinity {
    type Output = Motor;
    fn add(self, other: LineOnOrigin) -> Self::Output {
        return Motor::from_groups(
            // e415, e425, e435, e12345
            crate::swizzle!(other.group0(), 0, 1, 2).extend_to_4(0.0),
            // e235, e315, e125, e5
            self.group0(),
        );
    }
}
impl std::ops::Add<Motor> for MotorAtInfinity {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        4        0        0
    fn add(self, other: Motor) -> Self::Output {
        use crate::elements::*;
        return Motor::from_groups(
            // e415, e425, e435, e12345
            other.group0(),
            // e235, e315, e125, e5
            Simd32x4::from([other[e235] + self[e235], other[e315] + self[e315], other[e125] + self[e125], other[e5] + self[e5]]),
        );
    }
}
impl std::ops::Add<MotorAtInfinity> for MotorAtInfinity {
    type Output = MotorAtInfinity;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        4        0        0
    fn add(self, other: MotorAtInfinity) -> Self::Output {
        use crate::elements::*;
        return MotorAtInfinity::from_groups(
            // e235, e315, e125, e5
            Simd32x4::from([other[e235] + self[e235], other[e315] + self[e315], other[e125] + self[e125], other[e5] + self[e5]]),
        );
    }
}
impl std::ops::AddAssign<MotorAtInfinity> for MotorAtInfinity {
    fn add_assign(&mut self, other: MotorAtInfinity) {
        use crate::elements::*;
        *self = MotorAtInfinity::from_groups(
            // e235, e315, e125, e5
            Simd32x4::from([other[e235] + self[e235], other[e315] + self[e315], other[e125] + self[e125], other[e5] + self[e5]]),
        );
    }
}
impl std::ops::Add<MotorOnOrigin> for MotorAtInfinity {
    type Output = Motor;
    fn add(self, other: MotorOnOrigin) -> Self::Output {
        return Motor::from_groups(/* e415, e425, e435, e12345 */ other.group0(), /* e235, e315, e125, e5 */ self.group0());
    }
}
impl std::ops::Add<MultiVector> for MotorAtInfinity {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        4        0        0
    fn add(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            other.group0(),
            // e1, e2, e3, e4
            other.group1(),
            // e5
            self[e5] + other[e5],
            // e41, e42, e43, e45
            other.group3(),
            // e15, e25, e35
            other.group4(),
            // e23, e31, e12
            other.group5(),
            // e415, e425, e435, e321
            other.group6(),
            // e423, e431, e412
            other.group7(),
            // e235, e315, e125
            Simd32x3::from([self[e235] + other[e235], self[e315] + other[e315], self[e125] + other[e125]]),
            // e1234, e4235, e4315, e4125
            other.group9(),
            // e3215
            other[e3215],
        );
    }
}
impl std::ops::Add<MysteryCircle> for MotorAtInfinity {
    type Output = AntiDipoleInversionAtInfinity;
    fn add(self, other: MysteryCircle) -> Self::Output {
        use crate::elements::*;
        return AntiDipoleInversionAtInfinity::from_groups(
            // e415, e425, e435, e321
            other.group0(),
            // e235, e315, e125
            self.group0().truncate_to_3(),
            // e1, e2, e3, e5
            Simd32x3::from(0.0).extend_to_4(self[e5]),
        );
    }
}
impl std::ops::Add<MysteryCircleRotor> for MotorAtInfinity {
    type Output = VersorEvenAtInfinity;
    fn add(self, other: MysteryCircleRotor) -> Self::Output {
        use crate::elements::*;
        return VersorEvenAtInfinity::from_groups(
            // e12345, e1, e2, e3
            Simd32x4::from([other[e12345], 0.0, 0.0, 0.0]),
            // e415, e425, e435, e321
            other.group0(),
            // e235, e315, e125, e5
            self.group0(),
        );
    }
}
impl std::ops::Add<MysteryDipole> for MotorAtInfinity {
    type Output = MultiVector;
    fn add(self, other: MysteryDipole) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            self[e5],
            // e41, e42, e43, e45
            Simd32x3::from(0.0).extend_to_4(other[e45]),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            other.group0().truncate_to_3(),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            self.group0().truncate_to_3(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Add<MysteryDipoleInversion> for MotorAtInfinity {
    type Output = MultiVector;
    fn add(self, other: MysteryDipoleInversion) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            self[e5],
            // e41, e42, e43, e45
            Simd32x3::from(0.0).extend_to_4(other[e45]),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            other.group0().truncate_to_3(),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            self.group0().truncate_to_3(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([0.0, other[e4235], other[e4315], other[e4125]]),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Add<MysteryVersorEven> for MotorAtInfinity {
    type Output = VersorEvenAtInfinity;
    fn add(self, other: MysteryVersorEven) -> Self::Output {
        return VersorEvenAtInfinity::from_groups(
            // e12345, e1, e2, e3
            other.group0(),
            // e415, e425, e435, e321
            other.group1(),
            // e235, e315, e125, e5
            self.group0(),
        );
    }
}
impl std::ops::Add<MysteryVersorOdd> for MotorAtInfinity {
    type Output = MultiVector;
    fn add(self, other: MysteryVersorOdd) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([other[scalar], 0.0]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            self[e5],
            // e41, e42, e43, e45
            Simd32x3::from(0.0).extend_to_4(other[e45]),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            other.group1().truncate_to_3(),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            self.group0().truncate_to_3(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([0.0, other[e4235], other[e4315], other[e4125]]),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Add<NullCircleAtOrigin> for MotorAtInfinity {
    type Output = VersorEvenAtOrigin;
    fn add(self, other: NullCircleAtOrigin) -> Self::Output {
        return VersorEvenAtOrigin::from_groups(
            // e423, e431, e412, e4
            crate::swizzle!(other.group0(), 0, 1, 2).extend_to_4(0.0),
            // e235, e315, e125, e5
            self.group0(),
        );
    }
}
impl std::ops::Add<NullDipoleAtOrigin> for MotorAtInfinity {
    type Output = MultiVector;
    fn add(self, other: NullDipoleAtOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            self[e5],
            // e41, e42, e43, e45
            crate::swizzle!(other.group0(), 0, 1, 2).extend_to_4(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            self.group0().truncate_to_3(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Add<NullDipoleInversionAtOrigin> for MotorAtInfinity {
    type Output = MultiVector;
    fn add(self, other: NullDipoleInversionAtOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            self[e5],
            // e41, e42, e43, e45
            Simd32x4::from([other[e41], other[e42], other[e43], 0.0]),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            self.group0().truncate_to_3(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([other[e1234], 0.0, 0.0, 0.0]),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Add<NullSphereAtOrigin> for MotorAtInfinity {
    type Output = MultiVector;
    fn add(self, other: NullSphereAtOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            self[e5],
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            self.group0().truncate_to_3(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([other[e1234], 0.0, 0.0, 0.0]),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Add<NullVersorEvenAtOrigin> for MotorAtInfinity {
    type Output = VersorEvenAtOrigin;
    fn add(self, other: NullVersorEvenAtOrigin) -> Self::Output {
        return VersorEvenAtOrigin::from_groups(/* e423, e431, e412, e4 */ other.group0(), /* e235, e315, e125, e5 */ self.group0());
    }
}
impl std::ops::Add<Origin> for MotorAtInfinity {
    type Output = VersorEvenAtOrigin;
    fn add(self, other: Origin) -> Self::Output {
        use crate::elements::*;
        return VersorEvenAtOrigin::from_groups(
            // e423, e431, e412, e4
            Simd32x3::from(0.0).extend_to_4(other[e4]),
            // e235, e315, e125, e5
            self.group0(),
        );
    }
}
impl std::ops::Add<Plane> for MotorAtInfinity {
    type Output = MultiVector;
    fn add(self, other: Plane) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            self[e5],
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            self.group0().truncate_to_3(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([0.0, other[e4235], other[e4315], other[e4125]]),
            // e3215
            other[e3215],
        );
    }
}
impl std::ops::Add<PlaneOnOrigin> for MotorAtInfinity {
    type Output = MultiVector;
    fn add(self, other: PlaneOnOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            self[e5],
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            self.group0().truncate_to_3(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([0.0, other[e4235], other[e4315], other[e4125]]),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Add<RoundPoint> for MotorAtInfinity {
    type Output = VersorEvenOrthogonalOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        1        0        0
    fn add(self, other: RoundPoint) -> Self::Output {
        use crate::elements::*;
        return VersorEvenOrthogonalOrigin::from_groups(
            // e423, e431, e412, e321
            Simd32x4::from(0.0),
            // e235, e315, e125, e5
            Simd32x4::from([self[e235], self[e315], self[e125], self[e5] + other[e5]]),
            // e1, e2, e3, e4
            other.group0(),
        );
    }
}
impl std::ops::Add<RoundPointAtOrigin> for MotorAtInfinity {
    type Output = VersorEvenAtOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        1        0        0
    fn add(self, other: RoundPointAtOrigin) -> Self::Output {
        use crate::elements::*;
        return VersorEvenAtOrigin::from_groups(
            // e423, e431, e412, e4
            Simd32x3::from(0.0).extend_to_4(other[e4]),
            // e235, e315, e125, e5
            Simd32x4::from([self[e235], self[e315], self[e125], self[e5] + other[e5]]),
        );
    }
}
impl std::ops::Add<Scalar> for MotorAtInfinity {
    type Output = MultiVector;
    fn add(self, other: Scalar) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([other[scalar], 0.0]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            self[e5],
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            self.group0().truncate_to_3(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Add<Sphere> for MotorAtInfinity {
    type Output = MultiVector;
    fn add(self, other: Sphere) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            self[e5],
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            self.group0().truncate_to_3(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([other[e1234], other[e4235], other[e4315], other[e4125]]),
            // e3215
            other[e3215],
        );
    }
}
impl std::ops::Add<SphereAtOrigin> for MotorAtInfinity {
    type Output = MultiVector;
    fn add(self, other: SphereAtOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            self[e5],
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            self.group0().truncate_to_3(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([other[e1234], 0.0, 0.0, 0.0]),
            // e3215
            other[e3215],
        );
    }
}
impl std::ops::Add<SphereOnOrigin> for MotorAtInfinity {
    type Output = MultiVector;
    fn add(self, other: SphereOnOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            self[e5],
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            self.group0().truncate_to_3(),
            // e1234, e4235, e4315, e4125
            crate::swizzle!(other.group0(), 3, 0, 1, 2),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Add<VersorEven> for MotorAtInfinity {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        4        0        0
    fn add(self, other: VersorEven) -> Self::Output {
        use crate::elements::*;
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            other.group0(),
            // e415, e425, e435, e321
            other.group1(),
            // e235, e315, e125, e5
            Simd32x4::from([self[e235] + other[e235], self[e315] + other[e315], self[e125] + other[e125], self[e5] + other[e5]]),
            // e1, e2, e3, e4
            other.group3(),
        );
    }
}
impl std::ops::Add<VersorEvenAligningOrigin> for MotorAtInfinity {
    type Output = VersorEvenAligningOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        4        0        0
    fn add(self, other: VersorEvenAligningOrigin) -> Self::Output {
        use crate::elements::*;
        return VersorEvenAligningOrigin::from_groups(
            // e423, e431, e412, e12345
            other.group0(),
            // e415, e425, e435, e4
            other.group1(),
            // e235, e315, e125, e5
            Simd32x4::from([self[e235] + other[e235], self[e315] + other[e315], self[e125] + other[e125], self[e5] + other[e5]]),
        );
    }
}
impl std::ops::Add<VersorEvenAtInfinity> for MotorAtInfinity {
    type Output = VersorEvenAtInfinity;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        4        0        0
    fn add(self, other: VersorEvenAtInfinity) -> Self::Output {
        use crate::elements::*;
        return VersorEvenAtInfinity::from_groups(
            // e12345, e1, e2, e3
            other.group0(),
            // e415, e425, e435, e321
            other.group1(),
            // e235, e315, e125, e5
            Simd32x4::from([self[e235] + other[e235], self[e315] + other[e315], self[e125] + other[e125], self[e5] + other[e5]]),
        );
    }
}
impl std::ops::Add<VersorEvenAtOrigin> for MotorAtInfinity {
    type Output = VersorEvenAtOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        4        0        0
    fn add(self, other: VersorEvenAtOrigin) -> Self::Output {
        use crate::elements::*;
        return VersorEvenAtOrigin::from_groups(
            // e423, e431, e412, e4
            other.group0(),
            // e235, e315, e125, e5
            Simd32x4::from([self[e235] + other[e235], self[e315] + other[e315], self[e125] + other[e125], self[e5] + other[e5]]),
        );
    }
}
impl std::ops::Add<VersorEvenOnOrigin> for MotorAtInfinity {
    type Output = VersorEvenAligningOrigin;
    fn add(self, other: VersorEvenOnOrigin) -> Self::Output {
        return VersorEvenAligningOrigin::from_groups(
            // e423, e431, e412, e12345
            other.group0(),
            // e415, e425, e435, e4
            other.group1(),
            // e235, e315, e125, e5
            self.group0(),
        );
    }
}
impl std::ops::Add<VersorEvenOrthogonalOrigin> for MotorAtInfinity {
    type Output = VersorEvenOrthogonalOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        4        0        0
    fn add(self, other: VersorEvenOrthogonalOrigin) -> Self::Output {
        use crate::elements::*;
        return VersorEvenOrthogonalOrigin::from_groups(
            // e423, e431, e412, e321
            other.group0(),
            // e235, e315, e125, e5
            Simd32x4::from([self[e235] + other[e235], self[e315] + other[e315], self[e125] + other[e125], self[e5] + other[e5]]),
            // e1, e2, e3, e4
            other.group2(),
        );
    }
}
impl std::ops::Add<VersorOdd> for MotorAtInfinity {
    type Output = MultiVector;
    fn add(self, other: VersorOdd) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([other[scalar], 0.0]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            self[e5],
            // e41, e42, e43, e45
            Simd32x4::from([other[e41], other[e42], other[e43], other[e45]]),
            // e15, e25, e35
            other.group2().truncate_to_3(),
            // e23, e31, e12
            other.group1().truncate_to_3(),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            self.group0().truncate_to_3(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([other[e1234], other[e4235], other[e4315], other[e4125]]),
            // e3215
            other[e3215],
        );
    }
}
impl std::ops::Add<VersorOddAtInfinity> for MotorAtInfinity {
    type Output = MultiVector;
    fn add(self, other: VersorOddAtInfinity) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([other[scalar], 0.0]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            self[e5],
            // e41, e42, e43, e45
            Simd32x3::from(0.0).extend_to_4(other[e45]),
            // e15, e25, e35
            crate::swizzle!(other.group0(), 1, 2, 3, _),
            // e23, e31, e12
            other.group1().truncate_to_3(),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            self.group0().truncate_to_3(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([0.0, other[e4235], other[e4315], other[e4125]]),
            // e3215
            other[e3215],
        );
    }
}
impl std::ops::Add<VersorOddOrthogonalOrigin> for MotorAtInfinity {
    type Output = MultiVector;
    fn add(self, other: VersorOddOrthogonalOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([other[scalar], 0.0]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            self[e5],
            // e41, e42, e43, e45
            Simd32x4::from([other[e41], other[e42], other[e43], 0.0]),
            // e15, e25, e35
            other.group2().truncate_to_3(),
            // e23, e31, e12
            other.group1().truncate_to_3(),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            self.group0().truncate_to_3(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([other[e1234], 0.0, 0.0, 0.0]),
            // e3215
            other[e3215],
        );
    }
}
impl std::ops::BitXor<AntiCircleOnOrigin> for MotorAtInfinity {
    type Output = CircleRotorAligningOriginAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        6        0
    //    simd3        0        1        0
    // Totals...
    // yes simd        2        7        0
    //  no simd        2        9        0
    fn bitxor(self, other: AntiCircleOnOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<AntiCircleRotor> for MotorAtInfinity {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        7        0
    //    simd3        1        2        0
    // Totals...
    // yes simd        3        9        0
    //  no simd        5       13        0
    fn bitxor(self, other: AntiCircleRotor) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<AntiCircleRotorAligningOrigin> for MotorAtInfinity {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        7        0
    //    simd3        1        2        0
    // Totals...
    // yes simd        3        9        0
    //  no simd        5       13        0
    fn bitxor(self, other: AntiCircleRotorAligningOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<AntiCircleRotorAligningOriginAtInfinity> for MotorAtInfinity {
    type Output = MotorAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd3        1        2        0
    // Totals...
    // yes simd        1        3        0
    //  no simd        3        7        0
    fn bitxor(self, other: AntiCircleRotorAligningOriginAtInfinity) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXorAssign<AntiCircleRotorAligningOriginAtInfinity> for MotorAtInfinity {
    fn bitxor_assign(&mut self, other: AntiCircleRotorAligningOriginAtInfinity) {
        *self = self.wedge(other);
    }
}
impl std::ops::BitXor<AntiCircleRotorAtInfinity> for MotorAtInfinity {
    type Output = MotorAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd3        1        2        0
    // Totals...
    // yes simd        1        3        0
    //  no simd        3        7        0
    fn bitxor(self, other: AntiCircleRotorAtInfinity) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXorAssign<AntiCircleRotorAtInfinity> for MotorAtInfinity {
    fn bitxor_assign(&mut self, other: AntiCircleRotorAtInfinity) {
        *self = self.wedge(other);
    }
}
impl std::ops::BitXor<AntiCircleRotorOnOrigin> for MotorAtInfinity {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        7        0
    //    simd3        1        2        0
    // Totals...
    // yes simd        3        9        0
    //  no simd        5       13        0
    fn bitxor(self, other: AntiCircleRotorOnOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<AntiDipoleInversion> for MotorAtInfinity {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        9        0
    //    simd4        1        3        0
    // Totals...
    // yes simd        3       12        0
    //  no simd        6       21        0
    fn bitxor(self, other: AntiDipoleInversion) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<AntiDipoleInversionAtInfinity> for MotorAtInfinity {
    type Output = FlectorAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        7        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        3        8        0
    //  no simd        3       11        0
    fn bitxor(self, other: AntiDipoleInversionAtInfinity) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<AntiDipoleInversionOnOrigin> for MotorAtInfinity {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        9        0
    //    simd4        1        3        0
    // Totals...
    // yes simd        3       12        0
    //  no simd        6       21        0
    fn bitxor(self, other: AntiDipoleInversionOnOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<AntiDipoleInversionOrthogonalOrigin> for MotorAtInfinity {
    type Output = FlectorOnOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        7        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        3        8        0
    //  no simd        3       11        0
    fn bitxor(self, other: AntiDipoleInversionOrthogonalOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<AntiDipoleOnOrigin> for MotorAtInfinity {
    type Output = Plane;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        2        0
    // no simd        0        8        0
    fn bitxor(self, other: AntiDipoleOnOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<AntiDualNum> for MotorAtInfinity {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        2        0
    //  no simd        0        5        0
    fn bitxor(self, other: AntiDualNum) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<AntiFlatOrigin> for MotorAtInfinity {
    type Output = Horizon;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        2        0
    fn bitxor(self, other: AntiFlatOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<AntiFlatPoint> for MotorAtInfinity {
    type Output = Horizon;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        2        0
    fn bitxor(self, other: AntiFlatPoint) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<AntiFlector> for MotorAtInfinity {
    type Output = FlectorAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        7        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        3        8        0
    //  no simd        3       11        0
    fn bitxor(self, other: AntiFlector) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<AntiFlectorOnOrigin> for MotorAtInfinity {
    type Output = FlectorAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        7        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        3        8        0
    //  no simd        3       11        0
    fn bitxor(self, other: AntiFlectorOnOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<AntiLine> for MotorAtInfinity {
    type Output = LineAtInfinity;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        1        0
    // no simd        0        3        0
    fn bitxor(self, other: AntiLine) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<AntiLineOnOrigin> for MotorAtInfinity {
    type Output = LineAtInfinity;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        1        0
    // no simd        0        3        0
    fn bitxor(self, other: AntiLineOnOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<AntiMotor> for MotorAtInfinity {
    type Output = MotorAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd3        1        2        0
    // Totals...
    // yes simd        1        3        0
    //  no simd        3        7        0
    fn bitxor(self, other: AntiMotor) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXorAssign<AntiMotor> for MotorAtInfinity {
    fn bitxor_assign(&mut self, other: AntiMotor) {
        *self = self.wedge(other);
    }
}
impl std::ops::BitXor<AntiMotorOnOrigin> for MotorAtInfinity {
    type Output = MotorAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd3        1        2        0
    // Totals...
    // yes simd        1        3        0
    //  no simd        3        7        0
    fn bitxor(self, other: AntiMotorOnOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXorAssign<AntiMotorOnOrigin> for MotorAtInfinity {
    fn bitxor_assign(&mut self, other: AntiMotorOnOrigin) {
        *self = self.wedge(other);
    }
}
impl std::ops::BitXor<AntiMysteryCircleRotor> for MotorAtInfinity {
    type Output = MotorAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd3        1        2        0
    // Totals...
    // yes simd        1        3        0
    //  no simd        3        7        0
    fn bitxor(self, other: AntiMysteryCircleRotor) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXorAssign<AntiMysteryCircleRotor> for MotorAtInfinity {
    fn bitxor_assign(&mut self, other: AntiMysteryCircleRotor) {
        *self = self.wedge(other);
    }
}
impl std::ops::BitXor<AntiMysteryDipoleInversion> for MotorAtInfinity {
    type Output = FlectorAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        7        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        3        8        0
    //  no simd        3       11        0
    fn bitxor(self, other: AntiMysteryDipoleInversion) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<AntiPlane> for MotorAtInfinity {
    type Output = FlectorAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        6        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        2        7        0
    //  no simd        2       10        0
    fn bitxor(self, other: AntiPlane) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<AntiPlaneOnOrigin> for MotorAtInfinity {
    type Output = FlectorAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        6        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        2        7        0
    //  no simd        2       10        0
    fn bitxor(self, other: AntiPlaneOnOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<AntiSphereOnOrigin> for MotorAtInfinity {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        6        0
    //    simd4        0        3        0
    // Totals...
    // yes simd        2        9        0
    //  no simd        2       18        0
    fn bitxor(self, other: AntiSphereOnOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<AntiVersorEvenOnOrigin> for MotorAtInfinity {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        8        0
    //    simd3        1        2        0
    // Totals...
    // yes simd        4       10        0
    //  no simd        6       14        0
    fn bitxor(self, other: AntiVersorEvenOnOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<Circle> for MotorAtInfinity {
    type Output = Plane;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        2        0
    // no simd        0        8        0
    fn bitxor(self, other: Circle) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<CircleAligningOrigin> for MotorAtInfinity {
    type Output = PlaneOnOrigin;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        2        0
    // no simd        0        6        0
    fn bitxor(self, other: CircleAligningOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<CircleAtInfinity> for MotorAtInfinity {
    type Output = Horizon;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        2        0
    fn bitxor(self, other: CircleAtInfinity) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<CircleAtOrigin> for MotorAtInfinity {
    type Output = PlaneOnOrigin;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        2        0
    // no simd        0        6        0
    fn bitxor(self, other: CircleAtOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<CircleOnOrigin> for MotorAtInfinity {
    type Output = PlaneOnOrigin;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        2        0
    // no simd        0        6        0
    fn bitxor(self, other: CircleOnOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<CircleOrthogonalOrigin> for MotorAtInfinity {
    type Output = Plane;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        2        0
    // no simd        0        8        0
    fn bitxor(self, other: CircleOrthogonalOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<CircleRotor> for MotorAtInfinity {
    type Output = Plane;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        2        0
    // no simd        0        8        0
    fn bitxor(self, other: CircleRotor) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<CircleRotorAligningOrigin> for MotorAtInfinity {
    type Output = PlaneOnOrigin;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        2        0
    // no simd        0        6        0
    fn bitxor(self, other: CircleRotorAligningOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<CircleRotorAtInfinity> for MotorAtInfinity {
    type Output = Horizon;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        2        0
    fn bitxor(self, other: CircleRotorAtInfinity) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<CircleRotorOnOrigin> for MotorAtInfinity {
    type Output = PlaneOnOrigin;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        2        0
    // no simd        0        6        0
    fn bitxor(self, other: CircleRotorOnOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<Dipole> for MotorAtInfinity {
    type Output = CircleRotorAligningOriginAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        6        0
    //    simd3        0        1        0
    // Totals...
    // yes simd        2        7        0
    //  no simd        2        9        0
    fn bitxor(self, other: Dipole) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<DipoleAligningOrigin> for MotorAtInfinity {
    type Output = MotorOnOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        6        0
    fn bitxor(self, other: DipoleAligningOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<DipoleAtInfinity> for MotorAtInfinity {
    type Output = LineAtInfinity;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        1        0
    // no simd        0        3        0
    fn bitxor(self, other: DipoleAtInfinity) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<DipoleAtOrigin> for MotorAtInfinity {
    type Output = MotorOnOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        6        0
    fn bitxor(self, other: DipoleAtOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<DipoleInversion> for MotorAtInfinity {
    type Output = CircleRotorAligningOriginAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        7        0
    //    simd3        0        1        0
    // Totals...
    // yes simd        3        8        0
    //  no simd        3       10        0
    fn bitxor(self, other: DipoleInversion) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<DipoleInversionAligningOrigin> for MotorAtInfinity {
    type Output = MotorOnOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        7        0
    fn bitxor(self, other: DipoleInversionAligningOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<DipoleInversionAtInfinity> for MotorAtInfinity {
    type Output = LineAtInfinity;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        1        0
    // no simd        0        3        0
    fn bitxor(self, other: DipoleInversionAtInfinity) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<DipoleInversionAtOrigin> for MotorAtInfinity {
    type Output = MotorOnOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        7        0
    fn bitxor(self, other: DipoleInversionAtOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<DipoleInversionOnOrigin> for MotorAtInfinity {
    type Output = MotorOnOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        7        0
    fn bitxor(self, other: DipoleInversionOnOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<DipoleInversionOrthogonalOrigin> for MotorAtInfinity {
    type Output = CircleRotorAligningOriginAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        7        0
    //    simd3        0        1        0
    // Totals...
    // yes simd        3        8        0
    //  no simd        3       10        0
    fn bitxor(self, other: DipoleInversionOrthogonalOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<DipoleOnOrigin> for MotorAtInfinity {
    type Output = MotorOnOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        6        0
    fn bitxor(self, other: DipoleOnOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<DipoleOrthogonalOrigin> for MotorAtInfinity {
    type Output = CircleRotorAligningOriginAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        6        0
    //    simd3        0        1        0
    // Totals...
    // yes simd        2        7        0
    //  no simd        2        9        0
    fn bitxor(self, other: DipoleOrthogonalOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<DualNum> for MotorAtInfinity {
    type Output = FlectorOnOrigin;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        2        0
    // no simd        0        8        0
    fn bitxor(self, other: DualNum) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<MultiVector> for MotorAtInfinity {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        9       20        0
    //    simd3        1        4        0
    //    simd4        0        1        0
    // Totals...
    // yes simd       10       25        0
    //  no simd       12       36        0
    fn bitxor(self, other: MultiVector) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<MysteryCircle> for MotorAtInfinity {
    type Output = Horizon;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        2        0
    fn bitxor(self, other: MysteryCircle) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<MysteryCircleRotor> for MotorAtInfinity {
    type Output = Horizon;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        2        0
    fn bitxor(self, other: MysteryCircleRotor) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<MysteryDipole> for MotorAtInfinity {
    type Output = LineAtInfinity;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        1        0
    // no simd        0        3        0
    fn bitxor(self, other: MysteryDipole) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<MysteryDipoleInversion> for MotorAtInfinity {
    type Output = LineAtInfinity;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        1        0
    // no simd        0        3        0
    fn bitxor(self, other: MysteryDipoleInversion) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<MysteryVersorEven> for MotorAtInfinity {
    type Output = FlectorAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        7        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        3        8        0
    //  no simd        3       11        0
    fn bitxor(self, other: MysteryVersorEven) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<MysteryVersorOdd> for MotorAtInfinity {
    type Output = MotorAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd3        1        2        0
    // Totals...
    // yes simd        1        3        0
    //  no simd        3        7        0
    fn bitxor(self, other: MysteryVersorOdd) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXorAssign<MysteryVersorOdd> for MotorAtInfinity {
    fn bitxor_assign(&mut self, other: MysteryVersorOdd) {
        *self = self.wedge(other);
    }
}
impl std::ops::BitXor<NullCircleAtOrigin> for MotorAtInfinity {
    type Output = PlaneOnOrigin;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        2        0
    // no simd        0        6        0
    fn bitxor(self, other: NullCircleAtOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<NullDipoleAtOrigin> for MotorAtInfinity {
    type Output = MotorOnOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        6        0
    fn bitxor(self, other: NullDipoleAtOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<NullDipoleInversionAtOrigin> for MotorAtInfinity {
    type Output = MotorOnOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        7        0
    fn bitxor(self, other: NullDipoleInversionAtOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<NullSphereAtOrigin> for MotorAtInfinity {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn bitxor(self, other: NullSphereAtOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<NullVersorEvenAtOrigin> for MotorAtInfinity {
    type Output = FlectorOnOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        7        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        3        8        0
    //  no simd        3       11        0
    fn bitxor(self, other: NullVersorEvenAtOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<Origin> for MotorAtInfinity {
    type Output = FlectorOnOrigin;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        2        0
    // no simd        0        8        0
    fn bitxor(self, other: Origin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<RoundPoint> for MotorAtInfinity {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        6        0
    //    simd4        0        3        0
    // Totals...
    // yes simd        2        9        0
    //  no simd        2       18        0
    fn bitxor(self, other: RoundPoint) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<RoundPointAtOrigin> for MotorAtInfinity {
    type Output = FlectorOnOrigin;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        2        0
    // no simd        0        8        0
    fn bitxor(self, other: RoundPointAtOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<Scalar> for MotorAtInfinity {
    type Output = MotorAtInfinity;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn bitxor(self, other: Scalar) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXorAssign<Scalar> for MotorAtInfinity {
    fn bitxor_assign(&mut self, other: Scalar) {
        *self = self.wedge(other);
    }
}
impl std::ops::BitXor<Sphere> for MotorAtInfinity {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn bitxor(self, other: Sphere) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<SphereAtOrigin> for MotorAtInfinity {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn bitxor(self, other: SphereAtOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<SphereOnOrigin> for MotorAtInfinity {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn bitxor(self, other: SphereOnOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<VersorEven> for MotorAtInfinity {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        9        0
    //    simd4        1        3        0
    // Totals...
    // yes simd        3       12        0
    //  no simd        6       21        0
    fn bitxor(self, other: VersorEven) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<VersorEvenAligningOrigin> for MotorAtInfinity {
    type Output = FlectorOnOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        7        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        3        8        0
    //  no simd        3       11        0
    fn bitxor(self, other: VersorEvenAligningOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<VersorEvenAtInfinity> for MotorAtInfinity {
    type Output = FlectorAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        7        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        3        8        0
    //  no simd        3       11        0
    fn bitxor(self, other: VersorEvenAtInfinity) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<VersorEvenAtOrigin> for MotorAtInfinity {
    type Output = FlectorOnOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        7        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        3        8        0
    //  no simd        3       11        0
    fn bitxor(self, other: VersorEvenAtOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<VersorEvenOnOrigin> for MotorAtInfinity {
    type Output = FlectorOnOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        7        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        3        8        0
    //  no simd        3       11        0
    fn bitxor(self, other: VersorEvenOnOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<VersorEvenOrthogonalOrigin> for MotorAtInfinity {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        9        0
    //    simd4        1        3        0
    // Totals...
    // yes simd        3       12        0
    //  no simd        6       21        0
    fn bitxor(self, other: VersorEvenOrthogonalOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<VersorOdd> for MotorAtInfinity {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        8        0
    //    simd3        1        2        0
    // Totals...
    // yes simd        4       10        0
    //  no simd        6       14        0
    fn bitxor(self, other: VersorOdd) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<VersorOddAtInfinity> for MotorAtInfinity {
    type Output = MotorAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd3        1        2        0
    // Totals...
    // yes simd        1        3        0
    //  no simd        3        7        0
    fn bitxor(self, other: VersorOddAtInfinity) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXorAssign<VersorOddAtInfinity> for MotorAtInfinity {
    fn bitxor_assign(&mut self, other: VersorOddAtInfinity) {
        *self = self.wedge(other);
    }
}
impl std::ops::BitXor<VersorOddOrthogonalOrigin> for MotorAtInfinity {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        8        0
    //    simd3        1        2        0
    // Totals...
    // yes simd        4       10        0
    //  no simd        6       14        0
    fn bitxor(self, other: VersorOddOrthogonalOrigin) -> Self::Output {
        return self.wedge(other);
    }
}

impl From<Infinity> for MotorAtInfinity {
    fn from(from_infinity: Infinity) -> Self {
        use crate::elements::*;
        return MotorAtInfinity::from_groups(/* e235, e315, e125, e5 */ Simd32x3::from(0.0).extend_to_4(from_infinity[e5]));
    }
}

impl From<LineAtInfinity> for MotorAtInfinity {
    fn from(from_line_at_infinity: LineAtInfinity) -> Self {
        use crate::elements::*;
        return MotorAtInfinity::from_groups(
            // e235, e315, e125, e5
            Simd32x4::from([from_line_at_infinity[e235], from_line_at_infinity[e315], from_line_at_infinity[e125], 0.0]),
        );
    }
}
impl std::ops::Mul<AntiCircleOnOrigin> for MotorAtInfinity {
    type Output = VersorEvenAtInfinity;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       24       36        0
    fn mul(self, other: AntiCircleOnOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiCircleRotor> for MotorAtInfinity {
    type Output = VersorEvenAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       24       36        0
    //    simd4        2        2        0
    // Totals...
    // yes simd       26       38        0
    //  no simd       32       44        0
    fn mul(self, other: AntiCircleRotor) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiCircleRotorAligningOrigin> for MotorAtInfinity {
    type Output = VersorEvenAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       24       36        0
    //    simd4        1        1        0
    // Totals...
    // yes simd       25       37        0
    //  no simd       28       40        0
    fn mul(self, other: AntiCircleRotorAligningOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiCircleRotorAligningOriginAtInfinity> for MotorAtInfinity {
    type Output = MotorAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        8       12        0
    //    simd4        1        1        0
    // Totals...
    // yes simd        9       13        0
    //  no simd       12       16        0
    fn mul(self, other: AntiCircleRotorAligningOriginAtInfinity) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::MulAssign<AntiCircleRotorAligningOriginAtInfinity> for MotorAtInfinity {
    fn mul_assign(&mut self, other: AntiCircleRotorAligningOriginAtInfinity) {
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiCircleRotorAtInfinity> for MotorAtInfinity {
    type Output = MotorAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        8       12        0
    //    simd4        2        2        0
    // Totals...
    // yes simd       10       14        0
    //  no simd       16       20        0
    fn mul(self, other: AntiCircleRotorAtInfinity) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::MulAssign<AntiCircleRotorAtInfinity> for MotorAtInfinity {
    fn mul_assign(&mut self, other: AntiCircleRotorAtInfinity) {
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiCircleRotorOnOrigin> for MotorAtInfinity {
    type Output = VersorEvenAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       24       36        0
    //    simd4        1        1        0
    // Totals...
    // yes simd       25       37        0
    //  no simd       28       40        0
    fn mul(self, other: AntiCircleRotorOnOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiDipoleInversion> for MotorAtInfinity {
    type Output = VersorOddAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       36       48        0
    //    simd4        3        3        0
    // Totals...
    // yes simd       39       51        0
    //  no simd       48       60        0
    fn mul(self, other: AntiDipoleInversion) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiDipoleInversionAtInfinity> for MotorAtInfinity {
    type Output = FlectorAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       20       24        0
    //    simd4        1        1        0
    // Totals...
    // yes simd       21       25        0
    //  no simd       24       28        0
    fn mul(self, other: AntiDipoleInversionAtInfinity) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiDipoleInversionOnOrigin> for MotorAtInfinity {
    type Output = VersorOddAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       24       36        0
    //    simd4        3        3        0
    // Totals...
    // yes simd       27       39        0
    //  no simd       36       48        0
    fn mul(self, other: AntiDipoleInversionOnOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiDipoleInversionOrthogonalOrigin> for MotorAtInfinity {
    type Output = VersorOddAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       24       36        0
    //    simd4        2        2        0
    // Totals...
    // yes simd       26       38        0
    //  no simd       32       44        0
    fn mul(self, other: AntiDipoleInversionOrthogonalOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiDipoleOnOrigin> for MotorAtInfinity {
    type Output = VersorOddAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       13       26        0
    //    simd3        1        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd       14       28        0
    //  no simd       16       33        0
    fn mul(self, other: AntiDipoleOnOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiDualNum> for MotorAtInfinity {
    type Output = VersorEvenAtInfinity;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        5        0
    // no simd        0       20        0
    fn mul(self, other: AntiDualNum) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiFlatOrigin> for MotorAtInfinity {
    type Output = FlectorAtInfinity;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        2        0
    // no simd        0        8        0
    fn mul(self, other: AntiFlatOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiFlatPoint> for MotorAtInfinity {
    type Output = FlectorAtInfinity;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        2        0
    // no simd        0        8        0
    fn mul(self, other: AntiFlatPoint) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiFlector> for MotorAtInfinity {
    type Output = FlectorAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        8       12        0
    //    simd4        1        1        0
    // Totals...
    // yes simd        9       13        0
    //  no simd       12       16        0
    fn mul(self, other: AntiFlector) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiFlectorOnOrigin> for MotorAtInfinity {
    type Output = FlectorAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        8       12        0
    //    simd4        1        1        0
    // Totals...
    // yes simd        9       13        0
    //  no simd       12       16        0
    fn mul(self, other: AntiFlectorOnOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiLine> for MotorAtInfinity {
    type Output = MotorAtInfinity;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        8       12        0
    fn mul(self, other: AntiLine) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::MulAssign<AntiLine> for MotorAtInfinity {
    fn mul_assign(&mut self, other: AntiLine) {
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiLineOnOrigin> for MotorAtInfinity {
    type Output = MotorAtInfinity;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        8       12        0
    fn mul(self, other: AntiLineOnOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::MulAssign<AntiLineOnOrigin> for MotorAtInfinity {
    fn mul_assign(&mut self, other: AntiLineOnOrigin) {
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiMotor> for MotorAtInfinity {
    type Output = MotorAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        8       12        0
    //    simd4        1        1        0
    // Totals...
    // yes simd        9       13        0
    //  no simd       12       16        0
    fn mul(self, other: AntiMotor) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::MulAssign<AntiMotor> for MotorAtInfinity {
    fn mul_assign(&mut self, other: AntiMotor) {
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiMotorOnOrigin> for MotorAtInfinity {
    type Output = MotorAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        8       12        0
    //    simd4        1        1        0
    // Totals...
    // yes simd        9       13        0
    //  no simd       12       16        0
    fn mul(self, other: AntiMotorOnOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::MulAssign<AntiMotorOnOrigin> for MotorAtInfinity {
    fn mul_assign(&mut self, other: AntiMotorOnOrigin) {
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiMysteryCircleRotor> for MotorAtInfinity {
    type Output = MotorAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        8       12        0
    //    simd4        2        2        0
    // Totals...
    // yes simd       10       14        0
    //  no simd       16       20        0
    fn mul(self, other: AntiMysteryCircleRotor) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::MulAssign<AntiMysteryCircleRotor> for MotorAtInfinity {
    fn mul_assign(&mut self, other: AntiMysteryCircleRotor) {
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiMysteryDipoleInversion> for MotorAtInfinity {
    type Output = FlectorAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       20       24        0
    //    simd4        1        1        0
    // Totals...
    // yes simd       21       25        0
    //  no simd       24       28        0
    fn mul(self, other: AntiMysteryDipoleInversion) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiPlane> for MotorAtInfinity {
    type Output = FlectorAtInfinity;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        8       12        0
    fn mul(self, other: AntiPlane) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiPlaneOnOrigin> for MotorAtInfinity {
    type Output = FlectorAtInfinity;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        8       12        0
    fn mul(self, other: AntiPlaneOnOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiScalar> for MotorAtInfinity {
    type Output = FlectorAtInfinity;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        2        0
    // no simd        0        8        0
    fn mul(self, other: AntiScalar) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiSphereOnOrigin> for MotorAtInfinity {
    type Output = VersorOddAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        8       16        0
    //    simd4        0        4        0
    // Totals...
    // yes simd        8       20        0
    //  no simd        8       32        0
    fn mul(self, other: AntiSphereOnOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiVersorEvenOnOrigin> for MotorAtInfinity {
    type Output = VersorEvenAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       32       44        0
    //    simd4        1        1        0
    // Totals...
    // yes simd       33       45        0
    //  no simd       36       48        0
    fn mul(self, other: AntiVersorEvenOnOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<Circle> for MotorAtInfinity {
    type Output = VersorOddAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       24       36        0
    //    simd4        1        1        0
    // Totals...
    // yes simd       25       37        0
    //  no simd       28       40        0
    fn mul(self, other: Circle) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<CircleAligningOrigin> for MotorAtInfinity {
    type Output = VersorOddAtInfinity;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       24       36        0
    fn mul(self, other: CircleAligningOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<CircleAtInfinity> for MotorAtInfinity {
    type Output = FlectorAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        8       12        0
    //    simd4        1        1        0
    // Totals...
    // yes simd        9       13        0
    //  no simd       12       16        0
    fn mul(self, other: CircleAtInfinity) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<CircleAtOrigin> for MotorAtInfinity {
    type Output = MysteryVersorOdd;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       16       24        0
    fn mul(self, other: CircleAtOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<CircleOnOrigin> for MotorAtInfinity {
    type Output = VersorOddAtInfinity;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       24       36        0
    fn mul(self, other: CircleOnOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<CircleOrthogonalOrigin> for MotorAtInfinity {
    type Output = VersorOddAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       13       26        0
    //    simd3        1        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd       14       28        0
    //  no simd       16       33        0
    fn mul(self, other: CircleOrthogonalOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<CircleRotor> for MotorAtInfinity {
    type Output = VersorOddAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       28       40        0
    //    simd4        1        1        0
    // Totals...
    // yes simd       29       41        0
    //  no simd       32       44        0
    fn mul(self, other: CircleRotor) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<CircleRotorAligningOrigin> for MotorAtInfinity {
    type Output = VersorOddAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       24       36        0
    //    simd4        1        1        0
    // Totals...
    // yes simd       25       37        0
    //  no simd       28       40        0
    fn mul(self, other: CircleRotorAligningOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<CircleRotorAligningOriginAtInfinity> for MotorAtInfinity {
    type Output = FlectorAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        8       12        0
    //    simd4        1        1        0
    // Totals...
    // yes simd        9       13        0
    //  no simd       12       16        0
    fn mul(self, other: CircleRotorAligningOriginAtInfinity) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<CircleRotorAtInfinity> for MotorAtInfinity {
    type Output = FlectorAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        8       12        0
    //    simd4        2        2        0
    // Totals...
    // yes simd       10       14        0
    //  no simd       16       20        0
    fn mul(self, other: CircleRotorAtInfinity) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<CircleRotorOnOrigin> for MotorAtInfinity {
    type Output = VersorOddAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       24       36        0
    //    simd4        1        1        0
    // Totals...
    // yes simd       25       37        0
    //  no simd       28       40        0
    fn mul(self, other: CircleRotorOnOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<Dipole> for MotorAtInfinity {
    type Output = VersorEvenAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       24       36        0
    //    simd4        1        1        0
    // Totals...
    // yes simd       25       37        0
    //  no simd       28       40        0
    fn mul(self, other: Dipole) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<DipoleAligningOrigin> for MotorAtInfinity {
    type Output = VersorEvenAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       16       24        0
    //    simd4        0        2        0
    // Totals...
    // yes simd       16       26        0
    //  no simd       16       32        0
    fn mul(self, other: DipoleAligningOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<DipoleAtInfinity> for MotorAtInfinity {
    type Output = MotorAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        8       12        0
    //    simd4        1        1        0
    // Totals...
    // yes simd        9       13        0
    //  no simd       12       16        0
    fn mul(self, other: DipoleAtInfinity) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::MulAssign<DipoleAtInfinity> for MotorAtInfinity {
    fn mul_assign(&mut self, other: DipoleAtInfinity) {
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<DipoleAtOrigin> for MotorAtInfinity {
    type Output = MysteryVersorEven;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       16       24        0
    fn mul(self, other: DipoleAtOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<DipoleInversion> for MotorAtInfinity {
    type Output = VersorEvenAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       32       44        0
    //    simd4        4        4        0
    // Totals...
    // yes simd       36       48        0
    //  no simd       48       60        0
    fn mul(self, other: DipoleInversion) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<DipoleInversionAligningOrigin> for MotorAtInfinity {
    type Output = VersorEvenAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       32       44        0
    //    simd4        1        1        0
    // Totals...
    // yes simd       33       45        0
    //  no simd       36       48        0
    fn mul(self, other: DipoleInversionAligningOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<DipoleInversionAtInfinity> for MotorAtInfinity {
    type Output = MotorAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        8       12        0
    //    simd4        4        4        0
    // Totals...
    // yes simd       12       16        0
    //  no simd       24       28        0
    fn mul(self, other: DipoleInversionAtInfinity) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::MulAssign<DipoleInversionAtInfinity> for MotorAtInfinity {
    fn mul_assign(&mut self, other: DipoleInversionAtInfinity) {
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<DipoleInversionAtOrigin> for MotorAtInfinity {
    type Output = MysteryVersorEven;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       24       32        0
    fn mul(self, other: DipoleInversionAtOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<DipoleInversionOnOrigin> for MotorAtInfinity {
    type Output = VersorEvenAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       32       44        0
    //    simd4        1        1        0
    // Totals...
    // yes simd       33       45        0
    //  no simd       36       48        0
    fn mul(self, other: DipoleInversionOnOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<DipoleInversionOrthogonalOrigin> for MotorAtInfinity {
    type Output = VersorEvenAtInfinity;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       32       44        0
    fn mul(self, other: DipoleInversionOrthogonalOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<DipoleOnOrigin> for MotorAtInfinity {
    type Output = VersorEvenAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       16       24        0
    //    simd4        0        2        0
    // Totals...
    // yes simd       16       26        0
    //  no simd       16       32        0
    fn mul(self, other: DipoleOnOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<DipoleOrthogonalOrigin> for MotorAtInfinity {
    type Output = VersorEvenAtInfinity;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       24       36        0
    fn mul(self, other: DipoleOrthogonalOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<DualNum> for MotorAtInfinity {
    type Output = VersorOddAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        8        0
    //    simd4        0        4        0
    // Totals...
    // yes simd        0       12        0
    //  no simd        0       24        0
    fn mul(self, other: DualNum) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<FlatOrigin> for MotorAtInfinity {
    type Output = MotorAtInfinity;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        2        0
    // no simd        0        8        0
    fn mul(self, other: FlatOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::MulAssign<FlatOrigin> for MotorAtInfinity {
    fn mul_assign(&mut self, other: FlatOrigin) {
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<FlatPoint> for MotorAtInfinity {
    type Output = MotorAtInfinity;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        2        0
    // no simd        0        8        0
    fn mul(self, other: FlatPoint) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::MulAssign<FlatPoint> for MotorAtInfinity {
    fn mul_assign(&mut self, other: FlatPoint) {
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<Flector> for MotorAtInfinity {
    type Output = MotorAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        8       12        0
    //    simd4        1        1        0
    // Totals...
    // yes simd        9       13        0
    //  no simd       12       16        0
    fn mul(self, other: Flector) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::MulAssign<Flector> for MotorAtInfinity {
    fn mul_assign(&mut self, other: Flector) {
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<FlectorOnOrigin> for MotorAtInfinity {
    type Output = MotorAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        8       12        0
    //    simd4        1        1        0
    // Totals...
    // yes simd        9       13        0
    //  no simd       12       16        0
    fn mul(self, other: FlectorOnOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::MulAssign<FlectorOnOrigin> for MotorAtInfinity {
    fn mul_assign(&mut self, other: FlectorOnOrigin) {
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<Line> for MotorAtInfinity {
    type Output = FlectorAtInfinity;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        8       12        0
    fn mul(self, other: Line) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<LineOnOrigin> for MotorAtInfinity {
    type Output = FlectorAtInfinity;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        8       12        0
    fn mul(self, other: LineOnOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<Motor> for MotorAtInfinity {
    type Output = FlectorAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        8       12        0
    //    simd4        1        1        0
    // Totals...
    // yes simd        9       13        0
    //  no simd       12       16        0
    fn mul(self, other: Motor) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<MotorOnOrigin> for MotorAtInfinity {
    type Output = FlectorAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        8       12        0
    //    simd4        1        1        0
    // Totals...
    // yes simd        9       13        0
    //  no simd       12       16        0
    fn mul(self, other: MotorOnOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<MultiVector> for MotorAtInfinity {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       59       80        0
    //    simd3       15       16        0
    // Totals...
    // yes simd       74       96        0
    //  no simd      104      128        0
    fn mul(self, other: MultiVector) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<MysteryCircle> for MotorAtInfinity {
    type Output = FlectorAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        8       12        0
    //    simd4        1        1        0
    // Totals...
    // yes simd        9       13        0
    //  no simd       12       16        0
    fn mul(self, other: MysteryCircle) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<MysteryCircleRotor> for MotorAtInfinity {
    type Output = FlectorAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        8       12        0
    //    simd4        2        2        0
    // Totals...
    // yes simd       10       14        0
    //  no simd       16       20        0
    fn mul(self, other: MysteryCircleRotor) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<MysteryDipole> for MotorAtInfinity {
    type Output = MotorAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        8       12        0
    //    simd4        1        1        0
    // Totals...
    // yes simd        9       13        0
    //  no simd       12       16        0
    fn mul(self, other: MysteryDipole) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::MulAssign<MysteryDipole> for MotorAtInfinity {
    fn mul_assign(&mut self, other: MysteryDipole) {
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<MysteryDipoleInversion> for MotorAtInfinity {
    type Output = MotorAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        8       12        0
    //    simd4        4        4        0
    // Totals...
    // yes simd       12       16        0
    //  no simd       24       28        0
    fn mul(self, other: MysteryDipoleInversion) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::MulAssign<MysteryDipoleInversion> for MotorAtInfinity {
    fn mul_assign(&mut self, other: MysteryDipoleInversion) {
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<MysteryVersorEven> for MotorAtInfinity {
    type Output = FlectorAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       20       24        0
    //    simd4        2        2        0
    // Totals...
    // yes simd       22       26        0
    //  no simd       28       32        0
    fn mul(self, other: MysteryVersorEven) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<MysteryVersorOdd> for MotorAtInfinity {
    type Output = MotorAtInfinity;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        7        8        0
    // no simd       28       32        0
    fn mul(self, other: MysteryVersorOdd) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::MulAssign<MysteryVersorOdd> for MotorAtInfinity {
    fn mul_assign(&mut self, other: MysteryVersorOdd) {
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<NullCircleAtOrigin> for MotorAtInfinity {
    type Output = MysteryVersorOdd;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       16       24        0
    fn mul(self, other: NullCircleAtOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<NullDipoleAtOrigin> for MotorAtInfinity {
    type Output = MysteryVersorEven;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       16       24        0
    fn mul(self, other: NullDipoleAtOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<NullDipoleInversionAtOrigin> for MotorAtInfinity {
    type Output = MysteryVersorEven;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       24       32        0
    fn mul(self, other: NullDipoleInversionAtOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<NullSphereAtOrigin> for MotorAtInfinity {
    type Output = MysteryVersorEven;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        4        0
    // no simd        0       16        0
    fn mul(self, other: NullSphereAtOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<NullVersorEvenAtOrigin> for MotorAtInfinity {
    type Output = MysteryVersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       16       24        0
    //    simd4        2        2        0
    // Totals...
    // yes simd       18       26        0
    //  no simd       24       32        0
    fn mul(self, other: NullVersorEvenAtOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<Origin> for MotorAtInfinity {
    type Output = MysteryVersorOdd;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        4        0
    // no simd        0       16        0
    fn mul(self, other: Origin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<Plane> for MotorAtInfinity {
    type Output = MotorAtInfinity;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        8       12        0
    fn mul(self, other: Plane) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::MulAssign<Plane> for MotorAtInfinity {
    fn mul_assign(&mut self, other: Plane) {
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<PlaneOnOrigin> for MotorAtInfinity {
    type Output = MotorAtInfinity;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        8       12        0
    fn mul(self, other: PlaneOnOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::MulAssign<PlaneOnOrigin> for MotorAtInfinity {
    fn mul_assign(&mut self, other: PlaneOnOrigin) {
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<RoundPoint> for MotorAtInfinity {
    type Output = VersorOddAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        8       16        0
    //    simd4        0        4        0
    // Totals...
    // yes simd        8       20        0
    //  no simd        8       32        0
    fn mul(self, other: RoundPoint) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<RoundPointAtOrigin> for MotorAtInfinity {
    type Output = MysteryVersorOdd;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        4        0
    // no simd        0       16        0
    fn mul(self, other: RoundPointAtOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<Scalar> for MotorAtInfinity {
    type Output = MotorAtInfinity;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn mul(self, other: Scalar) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::MulAssign<Scalar> for MotorAtInfinity {
    fn mul_assign(&mut self, other: Scalar) {
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<Sphere> for MotorAtInfinity {
    type Output = VersorEvenAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        8       12        0
    //    simd4        0        4        0
    // Totals...
    // yes simd        8       16        0
    //  no simd        8       28        0
    fn mul(self, other: Sphere) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<SphereAtOrigin> for MotorAtInfinity {
    type Output = MysteryVersorEven;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        4        0
    // no simd        0       16        0
    fn mul(self, other: SphereAtOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<SphereOnOrigin> for MotorAtInfinity {
    type Output = VersorEvenAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        8       12        0
    //    simd4        0        4        0
    // Totals...
    // yes simd        8       16        0
    //  no simd        8       28        0
    fn mul(self, other: SphereOnOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<VersorEven> for MotorAtInfinity {
    type Output = VersorOddAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       40       52        0
    //    simd4        3        3        0
    // Totals...
    // yes simd       43       55        0
    //  no simd       52       64        0
    fn mul(self, other: VersorEven) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<VersorEvenAligningOrigin> for MotorAtInfinity {
    type Output = VersorOddAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       24       36        0
    //    simd4        3        3        0
    // Totals...
    // yes simd       27       39        0
    //  no simd       36       48        0
    fn mul(self, other: VersorEvenAligningOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<VersorEvenAtInfinity> for MotorAtInfinity {
    type Output = FlectorAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       20       24        0
    //    simd4        2        2        0
    // Totals...
    // yes simd       22       26        0
    //  no simd       28       32        0
    fn mul(self, other: VersorEvenAtInfinity) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<VersorEvenAtOrigin> for MotorAtInfinity {
    type Output = MysteryVersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       16       24        0
    //    simd4        2        2        0
    // Totals...
    // yes simd       18       26        0
    //  no simd       24       32        0
    fn mul(self, other: VersorEvenAtOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<VersorEvenOnOrigin> for MotorAtInfinity {
    type Output = VersorOddAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       24       36        0
    //    simd4        3        3        0
    // Totals...
    // yes simd       27       39        0
    //  no simd       36       48        0
    fn mul(self, other: VersorEvenOnOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<VersorEvenOrthogonalOrigin> for MotorAtInfinity {
    type Output = VersorOddAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       24       36        0
    //    simd4        3        3        0
    // Totals...
    // yes simd       27       39        0
    //  no simd       36       48        0
    fn mul(self, other: VersorEvenOrthogonalOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<VersorOdd> for MotorAtInfinity {
    type Output = VersorEvenAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       24       32        0
    //    simd4        7        8        0
    // Totals...
    // yes simd       31       40        0
    //  no simd       52       64        0
    fn mul(self, other: VersorOdd) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<VersorOddAtInfinity> for MotorAtInfinity {
    type Output = MotorAtInfinity;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        7        8        0
    // no simd       28       32        0
    fn mul(self, other: VersorOddAtInfinity) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::MulAssign<VersorOddAtInfinity> for MotorAtInfinity {
    fn mul_assign(&mut self, other: VersorOddAtInfinity) {
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<VersorOddOrthogonalOrigin> for MotorAtInfinity {
    type Output = VersorEvenAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       32       44        0
    //    simd4        1        1        0
    // Totals...
    // yes simd       33       45        0
    //  no simd       36       48        0
    fn mul(self, other: VersorOddOrthogonalOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Neg for MotorAtInfinity {
    type Output = MotorAtInfinity;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn neg(self) -> Self::Output {
        return MotorAtInfinity::from_groups(/* e235, e315, e125, e5 */ self.group0() * Simd32x4::from(-1.0));
    }
}
impl std::ops::Not for MotorAtInfinity {
    type Output = FlectorAtInfinity;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn not(self) -> Self::Output {
        return self.right_dual();
    }
}
impl std::ops::Sub<AntiCircleOnOrigin> for MotorAtInfinity {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        2        0
    //  no simd        0        7        0
    fn sub(self, other: AntiCircleOnOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            self[e5],
            // e41, e42, e43, e45
            Simd32x4::from([other[e41], other[e42], other[e43], 1.0]) * Simd32x4::from([-1.0, -1.0, -1.0, 0.0]),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            other.group1() * Simd32x3::from(-1.0),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            self.group0().truncate_to_3(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Sub<AntiCircleRotor> for MotorAtInfinity {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd2        0        1        0
    //    simd3        0        2        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        4        0
    //  no simd        0       12        0
    fn sub(self, other: AntiCircleRotor) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([other[scalar], 1.0]) * Simd32x2::from([-1.0, 0.0]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            self[e5],
            // e41, e42, e43, e45
            Simd32x4::from([other[e41], other[e42], other[e43], other[e45]]) * Simd32x4::from(-1.0),
            // e15, e25, e35
            other.group2().truncate_to_3() * Simd32x3::from(-1.0),
            // e23, e31, e12
            other.group1().truncate_to_3() * Simd32x3::from(-1.0),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            self.group0().truncate_to_3(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Sub<AntiCircleRotorAligningOrigin> for MotorAtInfinity {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd2        0        1        0
    //    simd3        0        2        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        4        0
    //  no simd        0       12        0
    fn sub(self, other: AntiCircleRotorAligningOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([other[scalar], 1.0]) * Simd32x2::from([-1.0, 0.0]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            self[e5],
            // e41, e42, e43, e45
            Simd32x4::from([other[e41], other[e42], other[e43], 1.0]) * Simd32x4::from([-1.0, -1.0, -1.0, 0.0]),
            // e15, e25, e35
            other.group2().truncate_to_3() * Simd32x3::from(-1.0),
            // e23, e31, e12
            other.group1() * Simd32x3::from(-1.0),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            self.group0().truncate_to_3(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Sub<AntiCircleRotorAligningOriginAtInfinity> for MotorAtInfinity {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd2        0        1        0
    //    simd3        0        2        0
    // Totals...
    // yes simd        0        3        0
    //  no simd        0        8        0
    fn sub(self, other: AntiCircleRotorAligningOriginAtInfinity) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([other[scalar], 1.0]) * Simd32x2::from([-1.0, 0.0]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            self[e5],
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            other.group1().truncate_to_3() * Simd32x3::from(-1.0),
            // e23, e31, e12
            other.group0() * Simd32x3::from(-1.0),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            self.group0().truncate_to_3(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Sub<AntiCircleRotorAtInfinity> for MotorAtInfinity {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd2        0        1        0
    //    simd3        0        2        0
    // Totals...
    // yes simd        0        4        0
    //  no simd        0        9        0
    fn sub(self, other: AntiCircleRotorAtInfinity) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([other[scalar], 1.0]) * Simd32x2::from([-1.0, 0.0]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            self[e5],
            // e41, e42, e43, e45
            Simd32x3::from(0.0).extend_to_4(other[e45] * -1.0),
            // e15, e25, e35
            other.group1().truncate_to_3() * Simd32x3::from(-1.0),
            // e23, e31, e12
            other.group0().truncate_to_3() * Simd32x3::from(-1.0),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            self.group0().truncate_to_3(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Sub<AntiCircleRotorOnOrigin> for MotorAtInfinity {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd2        0        1        0
    //    simd3        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        3        0
    //  no simd        0        9        0
    fn sub(self, other: AntiCircleRotorOnOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([other[scalar], 1.0]) * Simd32x2::from([-1.0, 0.0]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            self[e5],
            // e41, e42, e43, e45
            Simd32x4::from([other[e41], other[e42], other[e43], 1.0]) * Simd32x4::from([-1.0, -1.0, -1.0, 0.0]),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            other.group1() * Simd32x3::from(-1.0),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            self.group0().truncate_to_3(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Sub<AntiDipoleInversion> for MotorAtInfinity {
    type Output = AntiDipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        0        0
    //    simd3        0        1        0
    //    simd4        0        3        0
    // Totals...
    // yes simd        4        4        0
    //  no simd        4       15        0
    fn sub(self, other: AntiDipoleInversion) -> Self::Output {
        use crate::elements::*;
        return AntiDipoleInversion::from_groups(
            // e423, e431, e412
            other.group0() * Simd32x3::from(-1.0),
            // e415, e425, e435, e321
            other.group1() * Simd32x4::from(-1.0),
            // e235, e315, e125, e4
            Simd32x4::from([self[e235] - other[e235], self[e315] - other[e315], self[e125] - other[e125], other[e4]]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e1, e2, e3, e5
            Simd32x4::from([other[e1], other[e2], other[e3], self[e5] - other[e5]]) * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
        );
    }
}
impl std::ops::Sub<AntiDipoleInversionAtInfinity> for MotorAtInfinity {
    type Output = AntiDipoleInversionAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        0        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        4        2        0
    //  no simd        4        8        0
    fn sub(self, other: AntiDipoleInversionAtInfinity) -> Self::Output {
        use crate::elements::*;
        return AntiDipoleInversionAtInfinity::from_groups(
            // e415, e425, e435, e321
            other.group0() * Simd32x4::from(-1.0),
            // e235, e315, e125
            Simd32x3::from([self[e235] - other[e235], self[e315] - other[e315], self[e125] - other[e125]]),
            // e1, e2, e3, e5
            Simd32x4::from([other[e1], other[e2], other[e3], self[e5] - other[e5]]) * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
        );
    }
}
impl std::ops::Sub<AntiDipoleInversionOnOrigin> for MotorAtInfinity {
    type Output = VersorEvenOrthogonalOrigin;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        2        0
    // no simd        0        8        0
    fn sub(self, other: AntiDipoleInversionOnOrigin) -> Self::Output {
        return VersorEvenOrthogonalOrigin::from_groups(
            // e423, e431, e412, e321
            other.group0() * Simd32x4::from(-1.0),
            // e235, e315, e125, e5
            self.group0(),
            // e1, e2, e3, e4
            crate::swizzle!(other.group1(), 1, 2, 3, 0) * Simd32x4::from(-1.0),
        );
    }
}
impl std::ops::Sub<AntiDipoleInversionOrthogonalOrigin> for MotorAtInfinity {
    type Output = AntiDipoleInversionOrthogonalOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        0        0
    //    simd3        0        1        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        4        3        0
    //  no simd        4       11        0
    fn sub(self, other: AntiDipoleInversionOrthogonalOrigin) -> Self::Output {
        use crate::elements::*;
        return AntiDipoleInversionOrthogonalOrigin::from_groups(
            // e423, e431, e412, e5
            Simd32x4::from([other[e423], other[e431], other[e412], self[e5] - other[e5]]) * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e415, e425, e435
            other.group1() * Simd32x3::from(-1.0),
            // e235, e315, e125, e4
            Simd32x4::from([self[e235] - other[e235], self[e315] - other[e315], self[e125] - other[e125], other[e4]]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
        );
    }
}
impl std::ops::Sub<AntiDipoleOnOrigin> for MotorAtInfinity {
    type Output = VersorEvenOrthogonalOrigin;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn sub(self, other: AntiDipoleOnOrigin) -> Self::Output {
        return VersorEvenOrthogonalOrigin::from_groups(
            // e423, e431, e412, e321
            other.group0() * Simd32x4::from(-1.0),
            // e235, e315, e125, e5
            self.group0(),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
        );
    }
}
impl std::ops::Sub<AntiDualNum> for MotorAtInfinity {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd2        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        2        0
    //  no simd        0        6        0
    fn sub(self, other: AntiDualNum) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([other[scalar], 1.0]) * Simd32x2::from([-1.0, 0.0]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            self[e5],
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            self.group0().truncate_to_3(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([other[e1234], 1.0, 1.0, 1.0]) * Simd32x4::from([-1.0, 0.0, 0.0, 0.0]),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Sub<AntiFlatOrigin> for MotorAtInfinity {
    type Output = AntiFlector;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn sub(self, other: AntiFlatOrigin) -> Self::Output {
        use crate::elements::*;
        return AntiFlector::from_groups(
            // e235, e315, e125, e321
            Simd32x4::from([self[e235], self[e315], self[e125], other[e321]]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e1, e2, e3, e5
            Simd32x3::from(0.0).extend_to_4(self[e5]),
        );
    }
}
impl std::ops::Sub<AntiFlatPoint> for MotorAtInfinity {
    type Output = AntiFlector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        0        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        3        1        0
    //  no simd        3        4        0
    fn sub(self, other: AntiFlatPoint) -> Self::Output {
        use crate::elements::*;
        return AntiFlector::from_groups(
            // e235, e315, e125, e321
            Simd32x4::from([self[e235] - other[e235], self[e315] - other[e315], self[e125] - other[e125], other[e321]]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e1, e2, e3, e5
            Simd32x3::from(0.0).extend_to_4(self[e5]),
        );
    }
}
impl std::ops::Sub<AntiFlector> for MotorAtInfinity {
    type Output = AntiFlector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        0        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        4        2        0
    //  no simd        4        8        0
    fn sub(self, other: AntiFlector) -> Self::Output {
        use crate::elements::*;
        return AntiFlector::from_groups(
            // e235, e315, e125, e321
            Simd32x4::from([self[e235] - other[e235], self[e315] - other[e315], self[e125] - other[e125], other[e321]]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e1, e2, e3, e5
            Simd32x4::from([other[e1], other[e2], other[e3], self[e5] - other[e5]]) * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
        );
    }
}
impl std::ops::Sub<AntiFlectorOnOrigin> for MotorAtInfinity {
    type Output = AntiFlector;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        2        0
    // no simd        0        8        0
    fn sub(self, other: AntiFlectorOnOrigin) -> Self::Output {
        use crate::elements::*;
        return AntiFlector::from_groups(
            // e235, e315, e125, e321
            Simd32x4::from([self[e235], self[e315], self[e125], other[e321]]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e1, e2, e3, e5
            Simd32x4::from([other[e1], other[e2], other[e3], self[e5]]) * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
        );
    }
}
impl std::ops::Sub<AntiLine> for MotorAtInfinity {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        2        0
    // no simd        0        6        0
    fn sub(self, other: AntiLine) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            self[e5],
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            other.group1() * Simd32x3::from(-1.0),
            // e23, e31, e12
            other.group0() * Simd32x3::from(-1.0),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            self.group0().truncate_to_3(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Sub<AntiLineOnOrigin> for MotorAtInfinity {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        1        0
    // no simd        0        3        0
    fn sub(self, other: AntiLineOnOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            self[e5],
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            other.group0() * Simd32x3::from(-1.0),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            self.group0().truncate_to_3(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Sub<AntiMotor> for MotorAtInfinity {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd2        0        1        0
    //    simd3        0        2        0
    // Totals...
    // yes simd        0        4        0
    //  no simd        0        9        0
    fn sub(self, other: AntiMotor) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([other[scalar], 1.0]) * Simd32x2::from([-1.0, 0.0]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            self[e5],
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            other.group1().truncate_to_3() * Simd32x3::from(-1.0),
            // e23, e31, e12
            other.group0().truncate_to_3() * Simd32x3::from(-1.0),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            self.group0().truncate_to_3(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            other[e3215] * -1.0,
        );
    }
}
impl std::ops::Sub<AntiMotorOnOrigin> for MotorAtInfinity {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd2        0        1        0
    //    simd3        0        1        0
    // Totals...
    // yes simd        0        2        0
    //  no simd        0        5        0
    fn sub(self, other: AntiMotorOnOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([other[scalar], 1.0]) * Simd32x2::from([-1.0, 0.0]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            self[e5],
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            other.group0().truncate_to_3() * Simd32x3::from(-1.0),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            self.group0().truncate_to_3(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Sub<AntiMysteryCircleRotor> for MotorAtInfinity {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd2        0        1        0
    //    simd3        0        1        0
    // Totals...
    // yes simd        0        3        0
    //  no simd        0        6        0
    fn sub(self, other: AntiMysteryCircleRotor) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([other[scalar], 1.0]) * Simd32x2::from([-1.0, 0.0]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            self[e5],
            // e41, e42, e43, e45
            Simd32x3::from(0.0).extend_to_4(other[e45] * -1.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            other.group0().truncate_to_3() * Simd32x3::from(-1.0),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            self.group0().truncate_to_3(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Sub<AntiMysteryDipoleInversion> for MotorAtInfinity {
    type Output = AntiDipoleInversionAtInfinity;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        2        0
    // no simd        0        8        0
    fn sub(self, other: AntiMysteryDipoleInversion) -> Self::Output {
        use crate::elements::*;
        return AntiDipoleInversionAtInfinity::from_groups(
            // e415, e425, e435, e321
            other.group0() * Simd32x4::from(-1.0),
            // e235, e315, e125
            self.group0().truncate_to_3(),
            // e1, e2, e3, e5
            Simd32x4::from([other[e1], other[e2], other[e3], self[e5]]) * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
        );
    }
}
impl std::ops::Sub<AntiPlane> for MotorAtInfinity {
    type Output = AntiFlector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        0        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        1        1        0
    //  no simd        1        4        0
    fn sub(self, other: AntiPlane) -> Self::Output {
        use crate::elements::*;
        return AntiFlector::from_groups(
            // e235, e315, e125, e321
            Simd32x4::from([self[e235], self[e315], self[e125], 0.0]),
            // e1, e2, e3, e5
            Simd32x4::from([other[e1], other[e2], other[e3], self[e5] - other[e5]]) * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
        );
    }
}
impl std::ops::Sub<AntiPlaneOnOrigin> for MotorAtInfinity {
    type Output = AntiFlector;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn sub(self, other: AntiPlaneOnOrigin) -> Self::Output {
        use crate::elements::*;
        return AntiFlector::from_groups(
            // e235, e315, e125, e321
            Simd32x4::from([self[e235], self[e315], self[e125], 0.0]),
            // e1, e2, e3, e5
            Simd32x4::from([other[e1], other[e2], other[e3], self[e5]]) * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
        );
    }
}
impl std::ops::Sub<AntiScalar> for MotorAtInfinity {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn sub(self, other: AntiScalar) -> Self::Output {
        use crate::elements::*;
        return Motor::from_groups(
            // e415, e425, e435, e12345
            Simd32x3::from(0.0).extend_to_4(other[e12345] * -1.0),
            // e235, e315, e125, e5
            self.group0(),
        );
    }
}
impl std::ops::Sub<AntiSphereOnOrigin> for MotorAtInfinity {
    type Output = VersorEvenOrthogonalOrigin;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn sub(self, other: AntiSphereOnOrigin) -> Self::Output {
        return VersorEvenOrthogonalOrigin::from_groups(
            // e423, e431, e412, e321
            Simd32x4::from(0.0),
            // e235, e315, e125, e5
            self.group0(),
            // e1, e2, e3, e4
            other.group0() * Simd32x4::from(-1.0),
        );
    }
}
impl std::ops::Sub<AntiVersorEvenOnOrigin> for MotorAtInfinity {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd2        0        1        0
    //    simd3        0        1        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        0        4        0
    //  no simd        0       13        0
    fn sub(self, other: AntiVersorEvenOnOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([other[scalar], 1.0]) * Simd32x2::from([-1.0, 0.0]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            self[e5],
            // e41, e42, e43, e45
            Simd32x4::from([other[e41], other[e42], other[e43], 1.0]) * Simd32x4::from([-1.0, -1.0, -1.0, 0.0]),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            other.group1().truncate_to_3() * Simd32x3::from(-1.0),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            self.group0().truncate_to_3(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([other[e1234], 1.0, 1.0, 1.0]) * Simd32x4::from([-1.0, 0.0, 0.0, 0.0]),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Sub<Circle> for MotorAtInfinity {
    type Output = AntiDipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        0        0
    //    simd3        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        3        2        0
    //  no simd        3        7        0
    fn sub(self, other: Circle) -> Self::Output {
        use crate::elements::*;
        return AntiDipoleInversion::from_groups(
            // e423, e431, e412
            other.group0() * Simd32x3::from(-1.0),
            // e415, e425, e435, e321
            other.group1() * Simd32x4::from(-1.0),
            // e235, e315, e125, e4
            Simd32x4::from([self[e235] - other[e235], self[e315] - other[e315], self[e125] - other[e125], 0.0]),
            // e1, e2, e3, e5
            Simd32x3::from(0.0).extend_to_4(self[e5]),
        );
    }
}
impl std::ops::Sub<CircleAligningOrigin> for MotorAtInfinity {
    type Output = AntiDipoleInversionOrthogonalOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        0        0
    //    simd3        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        3        2        0
    //  no simd        3        7        0
    fn sub(self, other: CircleAligningOrigin) -> Self::Output {
        use crate::elements::*;
        return AntiDipoleInversionOrthogonalOrigin::from_groups(
            // e423, e431, e412, e5
            Simd32x4::from([other[e423], other[e431], other[e412], self[e5]]) * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e415, e425, e435
            other.group1() * Simd32x3::from(-1.0),
            // e235, e315, e125, e4
            Simd32x4::from([self[e235] - other[e235], self[e315] - other[e315], self[e125] - other[e125], 0.0]),
        );
    }
}
impl std::ops::Sub<CircleAtInfinity> for MotorAtInfinity {
    type Output = AntiDipoleInversionAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        0        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        3        1        0
    //  no simd        3        4        0
    fn sub(self, other: CircleAtInfinity) -> Self::Output {
        use crate::elements::*;
        return AntiDipoleInversionAtInfinity::from_groups(
            // e415, e425, e435, e321
            other.group0() * Simd32x4::from(-1.0),
            // e235, e315, e125
            Simd32x3::from([self[e235] - other[e235], self[e315] - other[e315], self[e125] - other[e125]]),
            // e1, e2, e3, e5
            Simd32x3::from(0.0).extend_to_4(self[e5]),
        );
    }
}
impl std::ops::Sub<CircleAtOrigin> for MotorAtInfinity {
    type Output = VersorEvenAtOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        0        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        3        1        0
    //  no simd        3        4        0
    fn sub(self, other: CircleAtOrigin) -> Self::Output {
        use crate::elements::*;
        return VersorEvenAtOrigin::from_groups(
            // e423, e431, e412, e4
            Simd32x4::from([other[e423], other[e431], other[e412], 1.0]) * Simd32x4::from([-1.0, -1.0, -1.0, 0.0]),
            // e235, e315, e125, e5
            Simd32x4::from([self[e235] - other[e235], self[e315] - other[e315], self[e125] - other[e125], self[e5]]),
        );
    }
}
impl std::ops::Sub<CircleOnOrigin> for MotorAtInfinity {
    type Output = AntiDipoleInversionOrthogonalOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        2        0
    //  no simd        0        7        0
    fn sub(self, other: CircleOnOrigin) -> Self::Output {
        use crate::elements::*;
        return AntiDipoleInversionOrthogonalOrigin::from_groups(
            // e423, e431, e412, e5
            Simd32x4::from([other[e423], other[e431], other[e412], self[e5]]) * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e415, e425, e435
            other.group1() * Simd32x3::from(-1.0),
            // e235, e315, e125, e4
            Simd32x4::from([self[e235], self[e315], self[e125], 0.0]),
        );
    }
}
impl std::ops::Sub<CircleOrthogonalOrigin> for MotorAtInfinity {
    type Output = VersorEvenOrthogonalOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        0        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        3        1        0
    //  no simd        3        4        0
    fn sub(self, other: CircleOrthogonalOrigin) -> Self::Output {
        use crate::elements::*;
        return VersorEvenOrthogonalOrigin::from_groups(
            // e423, e431, e412, e321
            other.group0() * Simd32x4::from(-1.0),
            // e235, e315, e125, e5
            Simd32x4::from([self[e235] - other[e235], self[e315] - other[e315], self[e125] - other[e125], self[e5]]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
        );
    }
}
impl std::ops::Sub<CircleRotor> for MotorAtInfinity {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        0        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        3        2        0
    //  no simd        3        8        0
    fn sub(self, other: CircleRotor) -> Self::Output {
        use crate::elements::*;
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([other[e423], other[e431], other[e412], other[e12345]]) * Simd32x4::from(-1.0),
            // e415, e425, e435, e321
            other.group1() * Simd32x4::from(-1.0),
            // e235, e315, e125, e5
            Simd32x4::from([self[e235] - other[e235], self[e315] - other[e315], self[e125] - other[e125], self[e5]]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
        );
    }
}
impl std::ops::Sub<CircleRotorAligningOrigin> for MotorAtInfinity {
    type Output = VersorEvenAligningOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        0        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        3        2        0
    //  no simd        3        8        0
    fn sub(self, other: CircleRotorAligningOrigin) -> Self::Output {
        use crate::elements::*;
        return VersorEvenAligningOrigin::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([other[e423], other[e431], other[e412], other[e12345]]) * Simd32x4::from(-1.0),
            // e415, e425, e435, e4
            Simd32x4::from([other[e415], other[e425], other[e435], 1.0]) * Simd32x4::from([-1.0, -1.0, -1.0, 0.0]),
            // e235, e315, e125, e5
            Simd32x4::from([self[e235] - other[e235], self[e315] - other[e315], self[e125] - other[e125], self[e5]]),
        );
    }
}
impl std::ops::Sub<CircleRotorAligningOriginAtInfinity> for MotorAtInfinity {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        0        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        3        1        0
    //  no simd        3        4        0
    fn sub(self, other: CircleRotorAligningOriginAtInfinity) -> Self::Output {
        use crate::elements::*;
        return Motor::from_groups(
            // e415, e425, e435, e12345
            Simd32x4::from([other[e415], other[e425], other[e435], other[e12345]]) * Simd32x4::from(-1.0),
            // e235, e315, e125, e5
            Simd32x4::from([self[e235] - other[e235], self[e315] - other[e315], self[e125] - other[e125], self[e5]]),
        );
    }
}
impl std::ops::Sub<CircleRotorAtInfinity> for MotorAtInfinity {
    type Output = VersorEvenAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        0        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        3        2        0
    //  no simd        3        8        0
    fn sub(self, other: CircleRotorAtInfinity) -> Self::Output {
        use crate::elements::*;
        return VersorEvenAtInfinity::from_groups(
            // e12345, e1, e2, e3
            Simd32x4::from([other[e12345], 1.0, 1.0, 1.0]) * Simd32x4::from([-1.0, 0.0, 0.0, 0.0]),
            // e415, e425, e435, e321
            other.group0() * Simd32x4::from(-1.0),
            // e235, e315, e125, e5
            Simd32x4::from([self[e235] - other[e235], self[e315] - other[e315], self[e125] - other[e125], self[e5]]),
        );
    }
}
impl std::ops::Sub<CircleRotorOnOrigin> for MotorAtInfinity {
    type Output = VersorEvenAligningOrigin;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        2        0
    // no simd        0        8        0
    fn sub(self, other: CircleRotorOnOrigin) -> Self::Output {
        use crate::elements::*;
        return VersorEvenAligningOrigin::from_groups(
            // e423, e431, e412, e12345
            other.group0() * Simd32x4::from(-1.0),
            // e415, e425, e435, e4
            Simd32x4::from([other[e415], other[e425], other[e435], 1.0]) * Simd32x4::from([-1.0, -1.0, -1.0, 0.0]),
            // e235, e315, e125, e5
            self.group0(),
        );
    }
}
impl std::ops::Sub<Dipole> for MotorAtInfinity {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        0        2        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        3        0
    //  no simd        0       10        0
    fn sub(self, other: Dipole) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            self[e5],
            // e41, e42, e43, e45
            Simd32x4::from([other[e41], other[e42], other[e43], other[e45]]) * Simd32x4::from(-1.0),
            // e15, e25, e35
            other.group2() * Simd32x3::from(-1.0),
            // e23, e31, e12
            other.group1().truncate_to_3() * Simd32x3::from(-1.0),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            self.group0().truncate_to_3(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Sub<DipoleAligningOrigin> for MotorAtInfinity {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        2        0
    //  no simd        0        7        0
    fn sub(self, other: DipoleAligningOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            self[e5],
            // e41, e42, e43, e45
            other.group0() * Simd32x4::from(-1.0),
            // e15, e25, e35
            other.group1() * Simd32x3::from(-1.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            self.group0().truncate_to_3(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Sub<DipoleAtInfinity> for MotorAtInfinity {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd3        0        2        0
    // Totals...
    // yes simd        0        3        0
    //  no simd        0        7        0
    fn sub(self, other: DipoleAtInfinity) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            self[e5],
            // e41, e42, e43, e45
            Simd32x3::from(0.0).extend_to_4(other[e45] * -1.0),
            // e15, e25, e35
            other.group1() * Simd32x3::from(-1.0),
            // e23, e31, e12
            other.group0().truncate_to_3() * Simd32x3::from(-1.0),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            self.group0().truncate_to_3(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Sub<DipoleAtOrigin> for MotorAtInfinity {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        2        0
    //  no simd        0        7        0
    fn sub(self, other: DipoleAtOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            self[e5],
            // e41, e42, e43, e45
            Simd32x4::from([other[e41], other[e42], other[e43], 1.0]) * Simd32x4::from([-1.0, -1.0, -1.0, 0.0]),
            // e15, e25, e35
            other.group1() * Simd32x3::from(-1.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            self.group0().truncate_to_3(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Sub<DipoleInversion> for MotorAtInfinity {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd3        0        2        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        0        5        0
    //  no simd        0       15        0
    fn sub(self, other: DipoleInversion) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            self[e5],
            // e41, e42, e43, e45
            Simd32x4::from([other[e41], other[e42], other[e43], other[e45]]) * Simd32x4::from(-1.0),
            // e15, e25, e35
            other.group2().truncate_to_3() * Simd32x3::from(-1.0),
            // e23, e31, e12
            other.group1().truncate_to_3() * Simd32x3::from(-1.0),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            self.group0().truncate_to_3(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([other[e1234], other[e4235], other[e4315], other[e4125]]) * Simd32x4::from(-1.0),
            // e3215
            other[e3215] * -1.0,
        );
    }
}
impl std::ops::Sub<DipoleInversionAligningOrigin> for MotorAtInfinity {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd3        0        1        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        0        4        0
    //  no simd        0       12        0
    fn sub(self, other: DipoleInversionAligningOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            self[e5],
            // e41, e42, e43, e45
            other.group0() * Simd32x4::from(-1.0),
            // e15, e25, e35
            other.group1().truncate_to_3() * Simd32x3::from(-1.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            self.group0().truncate_to_3(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([other[e1234], other[e4235], other[e4315], other[e4125]]) * Simd32x4::from(-1.0),
            // e3215
            other[e3215] * -1.0,
        );
    }
}
impl std::ops::Sub<DipoleInversionAtInfinity> for MotorAtInfinity {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        2        0
    //    simd3        0        2        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        5        0
    //  no simd        0       12        0
    fn sub(self, other: DipoleInversionAtInfinity) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            self[e5],
            // e41, e42, e43, e45
            Simd32x3::from(0.0).extend_to_4(other[e45] * -1.0),
            // e15, e25, e35
            other.group1() * Simd32x3::from(-1.0),
            // e23, e31, e12
            other.group0().truncate_to_3() * Simd32x3::from(-1.0),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            self.group0().truncate_to_3(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([1.0, other[e4235], other[e4315], other[e4125]]) * Simd32x4::from([0.0, -1.0, -1.0, -1.0]),
            // e3215
            other[e3215] * -1.0,
        );
    }
}
impl std::ops::Sub<DipoleInversionAtOrigin> for MotorAtInfinity {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd3        0        1        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        0        4        0
    //  no simd        0       12        0
    fn sub(self, other: DipoleInversionAtOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            self[e5],
            // e41, e42, e43, e45
            Simd32x4::from([other[e41], other[e42], other[e43], 1.0]) * Simd32x4::from([-1.0, -1.0, -1.0, 0.0]),
            // e15, e25, e35
            other.group1().truncate_to_3() * Simd32x3::from(-1.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            self.group0().truncate_to_3(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([other[e1234], 1.0, 1.0, 1.0]) * Simd32x4::from([-1.0, 0.0, 0.0, 0.0]),
            // e3215
            other[e3215] * -1.0,
        );
    }
}
impl std::ops::Sub<DipoleInversionOnOrigin> for MotorAtInfinity {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        2        0
    // no simd        0        8        0
    fn sub(self, other: DipoleInversionOnOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            self[e5],
            // e41, e42, e43, e45
            other.group0() * Simd32x4::from(-1.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            self.group0().truncate_to_3(),
            // e1234, e4235, e4315, e4125
            other.group1() * Simd32x4::from(-1.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Sub<DipoleInversionOrthogonalOrigin> for MotorAtInfinity {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd3        0        2        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        0        5        0
    //  no simd        0       15        0
    fn sub(self, other: DipoleInversionOrthogonalOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            self[e5],
            // e41, e42, e43, e45
            Simd32x4::from([other[e41], other[e42], other[e43], 1.0]) * Simd32x4::from([-1.0, -1.0, -1.0, 0.0]),
            // e15, e25, e35
            other.group2().truncate_to_3() * Simd32x3::from(-1.0),
            // e23, e31, e12
            other.group1() * Simd32x3::from(-1.0),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            self.group0().truncate_to_3(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([other[e1234], 1.0, 1.0, 1.0]) * Simd32x4::from([-1.0, 0.0, 0.0, 0.0]),
            // e3215
            other[e3215] * -1.0,
        );
    }
}
impl std::ops::Sub<DipoleOnOrigin> for MotorAtInfinity {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn sub(self, other: DipoleOnOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            self[e5],
            // e41, e42, e43, e45
            other.group0() * Simd32x4::from(-1.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            self.group0().truncate_to_3(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Sub<DipoleOrthogonalOrigin> for MotorAtInfinity {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        0        2        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        3        0
    //  no simd        0       10        0
    fn sub(self, other: DipoleOrthogonalOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            self[e5],
            // e41, e42, e43, e45
            Simd32x4::from([other[e41], other[e42], other[e43], 1.0]) * Simd32x4::from([-1.0, -1.0, -1.0, 0.0]),
            // e15, e25, e35
            other.group2() * Simd32x3::from(-1.0),
            // e23, e31, e12
            other.group1() * Simd32x3::from(-1.0),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            self.group0().truncate_to_3(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Sub<DualNum> for MotorAtInfinity {
    type Output = VersorEvenAligningOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        2        0
    fn sub(self, other: DualNum) -> Self::Output {
        use crate::elements::*;
        return VersorEvenAligningOrigin::from_groups(
            // e423, e431, e412, e12345
            Simd32x3::from(0.0).extend_to_4(other[e12345] * -1.0),
            // e415, e425, e435, e4
            Simd32x3::from(0.0).extend_to_4(other[e4] * -1.0),
            // e235, e315, e125, e5
            self.group0(),
        );
    }
}
impl std::ops::Sub<FlatOrigin> for MotorAtInfinity {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn sub(self, other: FlatOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            self[e5],
            // e41, e42, e43, e45
            Simd32x3::from(0.0).extend_to_4(other[e45] * -1.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            self.group0().truncate_to_3(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Sub<FlatPoint> for MotorAtInfinity {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd3        0        1        0
    // Totals...
    // yes simd        0        2        0
    //  no simd        0        4        0
    fn sub(self, other: FlatPoint) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            self[e5],
            // e41, e42, e43, e45
            Simd32x3::from(0.0).extend_to_4(other[e45] * -1.0),
            // e15, e25, e35
            other.group0().truncate_to_3() * Simd32x3::from(-1.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            self.group0().truncate_to_3(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Sub<FlatPointAtInfinity> for MotorAtInfinity {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        1        0
    // no simd        0        3        0
    fn sub(self, other: FlatPointAtInfinity) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            self[e5],
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            other.group0() * Simd32x3::from(-1.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            self.group0().truncate_to_3(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Sub<Flector> for MotorAtInfinity {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        2        0
    //    simd3        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        4        0
    //  no simd        0        9        0
    fn sub(self, other: Flector) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            self[e5],
            // e41, e42, e43, e45
            Simd32x3::from(0.0).extend_to_4(other[e45] * -1.0),
            // e15, e25, e35
            other.group0().truncate_to_3() * Simd32x3::from(-1.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            self.group0().truncate_to_3(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([1.0, other[e4235], other[e4315], other[e4125]]) * Simd32x4::from([0.0, -1.0, -1.0, -1.0]),
            // e3215
            other[e3215] * -1.0,
        );
    }
}
impl std::ops::Sub<FlectorAtInfinity> for MotorAtInfinity {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd3        0        1        0
    // Totals...
    // yes simd        0        2        0
    //  no simd        0        4        0
    fn sub(self, other: FlectorAtInfinity) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            self[e5],
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            other.group0().truncate_to_3() * Simd32x3::from(-1.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            self.group0().truncate_to_3(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            other[e3215] * -1.0,
        );
    }
}
impl std::ops::Sub<FlectorOnOrigin> for MotorAtInfinity {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        2        0
    //  no simd        0        5        0
    fn sub(self, other: FlectorOnOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            self[e5],
            // e41, e42, e43, e45
            Simd32x3::from(0.0).extend_to_4(other[e45] * -1.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            self.group0().truncate_to_3(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([1.0, other[e4235], other[e4315], other[e4125]]) * Simd32x4::from([0.0, -1.0, -1.0, -1.0]),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Sub<Horizon> for MotorAtInfinity {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn sub(self, other: Horizon) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            self[e5],
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            self.group0().truncate_to_3(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            other[e3215] * -1.0,
        );
    }
}
impl std::ops::Sub<Infinity> for MotorAtInfinity {
    type Output = MotorAtInfinity;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        1        0        0
    fn sub(self, other: Infinity) -> Self::Output {
        use crate::elements::*;
        return MotorAtInfinity::from_groups(/* e235, e315, e125, e5 */ Simd32x4::from([self[e235], self[e315], self[e125], self[e5] - other[e5]]));
    }
}
impl std::ops::SubAssign<Infinity> for MotorAtInfinity {
    fn sub_assign(&mut self, other: Infinity) {
        use crate::elements::*;
        *self = MotorAtInfinity::from_groups(/* e235, e315, e125, e5 */ Simd32x4::from([self[e235], self[e315], self[e125], self[e5] - other[e5]]));
    }
}
impl std::ops::Sub<Line> for MotorAtInfinity {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        0        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        3        1        0
    //  no simd        3        4        0
    fn sub(self, other: Line) -> Self::Output {
        use crate::elements::*;
        return Motor::from_groups(
            // e415, e425, e435, e12345
            Simd32x4::from([other[e415], other[e425], other[e435], 1.0]) * Simd32x4::from([-1.0, -1.0, -1.0, 0.0]),
            // e235, e315, e125, e5
            Simd32x4::from([self[e235] - other[e235], self[e315] - other[e315], self[e125] - other[e125], self[e5]]),
        );
    }
}
impl std::ops::Sub<LineAtInfinity> for MotorAtInfinity {
    type Output = MotorAtInfinity;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        0        0
    fn sub(self, other: LineAtInfinity) -> Self::Output {
        use crate::elements::*;
        return MotorAtInfinity::from_groups(
            // e235, e315, e125, e5
            Simd32x4::from([self[e235] - other[e235], self[e315] - other[e315], self[e125] - other[e125], self[e5]]),
        );
    }
}
impl std::ops::SubAssign<LineAtInfinity> for MotorAtInfinity {
    fn sub_assign(&mut self, other: LineAtInfinity) {
        use crate::elements::*;
        *self = MotorAtInfinity::from_groups(
            // e235, e315, e125, e5
            Simd32x4::from([self[e235] - other[e235], self[e315] - other[e315], self[e125] - other[e125], self[e5]]),
        );
    }
}
impl std::ops::Sub<LineOnOrigin> for MotorAtInfinity {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn sub(self, other: LineOnOrigin) -> Self::Output {
        use crate::elements::*;
        return Motor::from_groups(
            // e415, e425, e435, e12345
            Simd32x4::from([other[e415], other[e425], other[e435], 1.0]) * Simd32x4::from([-1.0, -1.0, -1.0, 0.0]),
            // e235, e315, e125, e5
            self.group0(),
        );
    }
}
impl std::ops::Sub<Motor> for MotorAtInfinity {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        0        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        4        1        0
    //  no simd        4        4        0
    fn sub(self, other: Motor) -> Self::Output {
        use crate::elements::*;
        return Motor::from_groups(
            // e415, e425, e435, e12345
            other.group0() * Simd32x4::from(-1.0),
            // e235, e315, e125, e5
            Simd32x4::from([self[e235] - other[e235], self[e315] - other[e315], self[e125] - other[e125], self[e5] - other[e5]]),
        );
    }
}
impl std::ops::Sub<MotorAtInfinity> for MotorAtInfinity {
    type Output = MotorAtInfinity;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        4        0        0
    fn sub(self, other: MotorAtInfinity) -> Self::Output {
        use crate::elements::*;
        return MotorAtInfinity::from_groups(
            // e235, e315, e125, e5
            Simd32x4::from([self[e235] - other[e235], self[e315] - other[e315], self[e125] - other[e125], self[e5] - other[e5]]),
        );
    }
}
impl std::ops::SubAssign<MotorAtInfinity> for MotorAtInfinity {
    fn sub_assign(&mut self, other: MotorAtInfinity) {
        use crate::elements::*;
        *self = MotorAtInfinity::from_groups(
            // e235, e315, e125, e5
            Simd32x4::from([self[e235] - other[e235], self[e315] - other[e315], self[e125] - other[e125], self[e5] - other[e5]]),
        );
    }
}
impl std::ops::Sub<MotorOnOrigin> for MotorAtInfinity {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn sub(self, other: MotorOnOrigin) -> Self::Output {
        return Motor::from_groups(/* e415, e425, e435, e12345 */ other.group0() * Simd32x4::from(-1.0), /* e235, e315, e125, e5 */ self.group0());
    }
}
impl std::ops::Sub<MultiVector> for MotorAtInfinity {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        1        0
    //    simd2        0        1        0
    //    simd3        0        3        0
    //    simd4        0        4        0
    // Totals...
    // yes simd        4        9        0
    //  no simd        4       28        0
    fn sub(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            other.group0() * Simd32x2::from(-1.0),
            // e1, e2, e3, e4
            other.group1() * Simd32x4::from(-1.0),
            // e5
            self[e5] - other[e5],
            // e41, e42, e43, e45
            other.group3() * Simd32x4::from(-1.0),
            // e15, e25, e35
            other.group4() * Simd32x3::from(-1.0),
            // e23, e31, e12
            other.group5() * Simd32x3::from(-1.0),
            // e415, e425, e435, e321
            other.group6() * Simd32x4::from(-1.0),
            // e423, e431, e412
            other.group7() * Simd32x3::from(-1.0),
            // e235, e315, e125
            Simd32x3::from([self[e235] - other[e235], self[e315] - other[e315], self[e125] - other[e125]]),
            // e1234, e4235, e4315, e4125
            other.group9() * Simd32x4::from(-1.0),
            // e3215
            other[e3215] * -1.0,
        );
    }
}
impl std::ops::Sub<MysteryCircle> for MotorAtInfinity {
    type Output = AntiDipoleInversionAtInfinity;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn sub(self, other: MysteryCircle) -> Self::Output {
        use crate::elements::*;
        return AntiDipoleInversionAtInfinity::from_groups(
            // e415, e425, e435, e321
            other.group0() * Simd32x4::from(-1.0),
            // e235, e315, e125
            self.group0().truncate_to_3(),
            // e1, e2, e3, e5
            Simd32x3::from(0.0).extend_to_4(self[e5]),
        );
    }
}
impl std::ops::Sub<MysteryCircleRotor> for MotorAtInfinity {
    type Output = VersorEvenAtInfinity;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        2        0
    // no simd        0        8        0
    fn sub(self, other: MysteryCircleRotor) -> Self::Output {
        use crate::elements::*;
        return VersorEvenAtInfinity::from_groups(
            // e12345, e1, e2, e3
            Simd32x4::from([other[e12345], 1.0, 1.0, 1.0]) * Simd32x4::from([-1.0, 0.0, 0.0, 0.0]),
            // e415, e425, e435, e321
            other.group0() * Simd32x4::from(-1.0),
            // e235, e315, e125, e5
            self.group0(),
        );
    }
}
impl std::ops::Sub<MysteryDipole> for MotorAtInfinity {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd3        0        1        0
    // Totals...
    // yes simd        0        2        0
    //  no simd        0        4        0
    fn sub(self, other: MysteryDipole) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            self[e5],
            // e41, e42, e43, e45
            Simd32x3::from(0.0).extend_to_4(other[e45] * -1.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            other.group0().truncate_to_3() * Simd32x3::from(-1.0),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            self.group0().truncate_to_3(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Sub<MysteryDipoleInversion> for MotorAtInfinity {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd3        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        3        0
    //  no simd        0        8        0
    fn sub(self, other: MysteryDipoleInversion) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            self[e5],
            // e41, e42, e43, e45
            Simd32x3::from(0.0).extend_to_4(other[e45] * -1.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            other.group0().truncate_to_3() * Simd32x3::from(-1.0),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            self.group0().truncate_to_3(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([1.0, other[e4235], other[e4315], other[e4125]]) * Simd32x4::from([0.0, -1.0, -1.0, -1.0]),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Sub<MysteryVersorEven> for MotorAtInfinity {
    type Output = VersorEvenAtInfinity;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        2        0
    // no simd        0        8        0
    fn sub(self, other: MysteryVersorEven) -> Self::Output {
        return VersorEvenAtInfinity::from_groups(
            // e12345, e1, e2, e3
            other.group0() * Simd32x4::from(-1.0),
            // e415, e425, e435, e321
            other.group1() * Simd32x4::from(-1.0),
            // e235, e315, e125, e5
            self.group0(),
        );
    }
}
impl std::ops::Sub<MysteryVersorOdd> for MotorAtInfinity {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd2        0        1        0
    //    simd3        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        4        0
    //  no simd        0       10        0
    fn sub(self, other: MysteryVersorOdd) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([other[scalar], 1.0]) * Simd32x2::from([-1.0, 0.0]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            self[e5],
            // e41, e42, e43, e45
            Simd32x3::from(0.0).extend_to_4(other[e45] * -1.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            other.group1().truncate_to_3() * Simd32x3::from(-1.0),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            self.group0().truncate_to_3(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([1.0, other[e4235], other[e4315], other[e4125]]) * Simd32x4::from([0.0, -1.0, -1.0, -1.0]),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Sub<NullCircleAtOrigin> for MotorAtInfinity {
    type Output = VersorEvenAtOrigin;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn sub(self, other: NullCircleAtOrigin) -> Self::Output {
        use crate::elements::*;
        return VersorEvenAtOrigin::from_groups(
            // e423, e431, e412, e4
            Simd32x4::from([other[e423], other[e431], other[e412], 1.0]) * Simd32x4::from([-1.0, -1.0, -1.0, 0.0]),
            // e235, e315, e125, e5
            self.group0(),
        );
    }
}
impl std::ops::Sub<NullDipoleAtOrigin> for MotorAtInfinity {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn sub(self, other: NullDipoleAtOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            self[e5],
            // e41, e42, e43, e45
            Simd32x4::from([other[e41], other[e42], other[e43], 1.0]) * Simd32x4::from([-1.0, -1.0, -1.0, 0.0]),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            self.group0().truncate_to_3(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Sub<NullDipoleInversionAtOrigin> for MotorAtInfinity {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        2        0
    // no simd        0        8        0
    fn sub(self, other: NullDipoleInversionAtOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            self[e5],
            // e41, e42, e43, e45
            Simd32x4::from([other[e41], other[e42], other[e43], 1.0]) * Simd32x4::from([-1.0, -1.0, -1.0, 0.0]),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            self.group0().truncate_to_3(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([other[e1234], 1.0, 1.0, 1.0]) * Simd32x4::from([-1.0, 0.0, 0.0, 0.0]),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Sub<NullSphereAtOrigin> for MotorAtInfinity {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn sub(self, other: NullSphereAtOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            self[e5],
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            self.group0().truncate_to_3(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([other[e1234], 1.0, 1.0, 1.0]) * Simd32x4::from([-1.0, 0.0, 0.0, 0.0]),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Sub<NullVersorEvenAtOrigin> for MotorAtInfinity {
    type Output = VersorEvenAtOrigin;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn sub(self, other: NullVersorEvenAtOrigin) -> Self::Output {
        return VersorEvenAtOrigin::from_groups(/* e423, e431, e412, e4 */ other.group0() * Simd32x4::from(-1.0), /* e235, e315, e125, e5 */ self.group0());
    }
}
impl std::ops::Sub<Origin> for MotorAtInfinity {
    type Output = VersorEvenAtOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn sub(self, other: Origin) -> Self::Output {
        use crate::elements::*;
        return VersorEvenAtOrigin::from_groups(
            // e423, e431, e412, e4
            Simd32x3::from(0.0).extend_to_4(other[e4] * -1.0),
            // e235, e315, e125, e5
            self.group0(),
        );
    }
}
impl std::ops::Sub<Plane> for MotorAtInfinity {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        2        0
    //  no simd        0        5        0
    fn sub(self, other: Plane) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            self[e5],
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            self.group0().truncate_to_3(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([1.0, other[e4235], other[e4315], other[e4125]]) * Simd32x4::from([0.0, -1.0, -1.0, -1.0]),
            // e3215
            other[e3215] * -1.0,
        );
    }
}
impl std::ops::Sub<PlaneOnOrigin> for MotorAtInfinity {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn sub(self, other: PlaneOnOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            self[e5],
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            self.group0().truncate_to_3(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([1.0, other[e4235], other[e4315], other[e4125]]) * Simd32x4::from([0.0, -1.0, -1.0, -1.0]),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Sub<RoundPoint> for MotorAtInfinity {
    type Output = VersorEvenOrthogonalOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        0        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        1        1        0
    //  no simd        1        4        0
    fn sub(self, other: RoundPoint) -> Self::Output {
        use crate::elements::*;
        return VersorEvenOrthogonalOrigin::from_groups(
            // e423, e431, e412, e321
            Simd32x4::from(0.0),
            // e235, e315, e125, e5
            Simd32x4::from([self[e235], self[e315], self[e125], self[e5] - other[e5]]),
            // e1, e2, e3, e4
            other.group0() * Simd32x4::from(-1.0),
        );
    }
}
impl std::ops::Sub<RoundPointAtOrigin> for MotorAtInfinity {
    type Output = VersorEvenAtOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        1        1        0
    fn sub(self, other: RoundPointAtOrigin) -> Self::Output {
        use crate::elements::*;
        return VersorEvenAtOrigin::from_groups(
            // e423, e431, e412, e4
            Simd32x3::from(0.0).extend_to_4(other[e4] * -1.0),
            // e235, e315, e125, e5
            Simd32x4::from([self[e235], self[e315], self[e125], self[e5] - other[e5]]),
        );
    }
}
impl std::ops::Sub<Scalar> for MotorAtInfinity {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd2        0        1        0
    // no simd        0        2        0
    fn sub(self, other: Scalar) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([other[scalar], 1.0]) * Simd32x2::from([-1.0, 0.0]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            self[e5],
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            self.group0().truncate_to_3(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Sub<Sphere> for MotorAtInfinity {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        2        0
    //  no simd        0        5        0
    fn sub(self, other: Sphere) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            self[e5],
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            self.group0().truncate_to_3(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([other[e1234], other[e4235], other[e4315], other[e4125]]) * Simd32x4::from(-1.0),
            // e3215
            other[e3215] * -1.0,
        );
    }
}
impl std::ops::Sub<SphereAtOrigin> for MotorAtInfinity {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        2        0
    //  no simd        0        5        0
    fn sub(self, other: SphereAtOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            self[e5],
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            self.group0().truncate_to_3(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([other[e1234], 1.0, 1.0, 1.0]) * Simd32x4::from([-1.0, 0.0, 0.0, 0.0]),
            // e3215
            other[e3215] * -1.0,
        );
    }
}
impl std::ops::Sub<SphereOnOrigin> for MotorAtInfinity {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn sub(self, other: SphereOnOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            self[e5],
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            self.group0().truncate_to_3(),
            // e1234, e4235, e4315, e4125
            crate::swizzle!(other.group0(), 3, 0, 1, 2) * Simd32x4::from(-1.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Sub<VersorEven> for MotorAtInfinity {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        0        0
    //    simd4        0        3        0
    // Totals...
    // yes simd        4        3        0
    //  no simd        4       12        0
    fn sub(self, other: VersorEven) -> Self::Output {
        use crate::elements::*;
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            other.group0() * Simd32x4::from(-1.0),
            // e415, e425, e435, e321
            other.group1() * Simd32x4::from(-1.0),
            // e235, e315, e125, e5
            Simd32x4::from([self[e235] - other[e235], self[e315] - other[e315], self[e125] - other[e125], self[e5] - other[e5]]),
            // e1, e2, e3, e4
            other.group3() * Simd32x4::from(-1.0),
        );
    }
}
impl std::ops::Sub<VersorEvenAligningOrigin> for MotorAtInfinity {
    type Output = VersorEvenAligningOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        0        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        4        2        0
    //  no simd        4        8        0
    fn sub(self, other: VersorEvenAligningOrigin) -> Self::Output {
        use crate::elements::*;
        return VersorEvenAligningOrigin::from_groups(
            // e423, e431, e412, e12345
            other.group0() * Simd32x4::from(-1.0),
            // e415, e425, e435, e4
            other.group1() * Simd32x4::from(-1.0),
            // e235, e315, e125, e5
            Simd32x4::from([self[e235] - other[e235], self[e315] - other[e315], self[e125] - other[e125], self[e5] - other[e5]]),
        );
    }
}
impl std::ops::Sub<VersorEvenAtInfinity> for MotorAtInfinity {
    type Output = VersorEvenAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        0        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        4        2        0
    //  no simd        4        8        0
    fn sub(self, other: VersorEvenAtInfinity) -> Self::Output {
        use crate::elements::*;
        return VersorEvenAtInfinity::from_groups(
            // e12345, e1, e2, e3
            other.group0() * Simd32x4::from(-1.0),
            // e415, e425, e435, e321
            other.group1() * Simd32x4::from(-1.0),
            // e235, e315, e125, e5
            Simd32x4::from([self[e235] - other[e235], self[e315] - other[e315], self[e125] - other[e125], self[e5] - other[e5]]),
        );
    }
}
impl std::ops::Sub<VersorEvenAtOrigin> for MotorAtInfinity {
    type Output = VersorEvenAtOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        0        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        4        1        0
    //  no simd        4        4        0
    fn sub(self, other: VersorEvenAtOrigin) -> Self::Output {
        use crate::elements::*;
        return VersorEvenAtOrigin::from_groups(
            // e423, e431, e412, e4
            other.group0() * Simd32x4::from(-1.0),
            // e235, e315, e125, e5
            Simd32x4::from([self[e235] - other[e235], self[e315] - other[e315], self[e125] - other[e125], self[e5] - other[e5]]),
        );
    }
}
impl std::ops::Sub<VersorEvenOnOrigin> for MotorAtInfinity {
    type Output = VersorEvenAligningOrigin;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        2        0
    // no simd        0        8        0
    fn sub(self, other: VersorEvenOnOrigin) -> Self::Output {
        return VersorEvenAligningOrigin::from_groups(
            // e423, e431, e412, e12345
            other.group0() * Simd32x4::from(-1.0),
            // e415, e425, e435, e4
            other.group1() * Simd32x4::from(-1.0),
            // e235, e315, e125, e5
            self.group0(),
        );
    }
}
impl std::ops::Sub<VersorEvenOrthogonalOrigin> for MotorAtInfinity {
    type Output = VersorEvenOrthogonalOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        0        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        4        2        0
    //  no simd        4        8        0
    fn sub(self, other: VersorEvenOrthogonalOrigin) -> Self::Output {
        use crate::elements::*;
        return VersorEvenOrthogonalOrigin::from_groups(
            // e423, e431, e412, e321
            other.group0() * Simd32x4::from(-1.0),
            // e235, e315, e125, e5
            Simd32x4::from([self[e235] - other[e235], self[e315] - other[e315], self[e125] - other[e125], self[e5] - other[e5]]),
            // e1, e2, e3, e4
            other.group2() * Simd32x4::from(-1.0),
        );
    }
}
impl std::ops::Sub<VersorOdd> for MotorAtInfinity {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd2        0        1        0
    //    simd3        0        2        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        0        6        0
    //  no simd        0       17        0
    fn sub(self, other: VersorOdd) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([other[scalar], 1.0]) * Simd32x2::from([-1.0, 0.0]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            self[e5],
            // e41, e42, e43, e45
            Simd32x4::from([other[e41], other[e42], other[e43], other[e45]]) * Simd32x4::from(-1.0),
            // e15, e25, e35
            other.group2().truncate_to_3() * Simd32x3::from(-1.0),
            // e23, e31, e12
            other.group1().truncate_to_3() * Simd32x3::from(-1.0),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            self.group0().truncate_to_3(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([other[e1234], other[e4235], other[e4315], other[e4125]]) * Simd32x4::from(-1.0),
            // e3215
            other[e3215] * -1.0,
        );
    }
}
impl std::ops::Sub<VersorOddAtInfinity> for MotorAtInfinity {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        2        0
    //    simd2        0        1        0
    //    simd3        0        2        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        6        0
    //  no simd        0       14        0
    fn sub(self, other: VersorOddAtInfinity) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([other[scalar], 1.0]) * Simd32x2::from([-1.0, 0.0]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            self[e5],
            // e41, e42, e43, e45
            Simd32x3::from(0.0).extend_to_4(other[e45] * -1.0),
            // e15, e25, e35
            Simd32x3::from([other[e15], other[e25], other[e35]]) * Simd32x3::from(-1.0),
            // e23, e31, e12
            other.group1().truncate_to_3() * Simd32x3::from(-1.0),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            self.group0().truncate_to_3(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([1.0, other[e4235], other[e4315], other[e4125]]) * Simd32x4::from([0.0, -1.0, -1.0, -1.0]),
            // e3215
            other[e3215] * -1.0,
        );
    }
}
impl std::ops::Sub<VersorOddOrthogonalOrigin> for MotorAtInfinity {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd2        0        1        0
    //    simd3        0        2        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        0        6        0
    //  no simd        0       17        0
    fn sub(self, other: VersorOddOrthogonalOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([other[scalar], 1.0]) * Simd32x2::from([-1.0, 0.0]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            self[e5],
            // e41, e42, e43, e45
            Simd32x4::from([other[e41], other[e42], other[e43], 1.0]) * Simd32x4::from([-1.0, -1.0, -1.0, 0.0]),
            // e15, e25, e35
            other.group2().truncate_to_3() * Simd32x3::from(-1.0),
            // e23, e31, e12
            other.group1().truncate_to_3() * Simd32x3::from(-1.0),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            self.group0().truncate_to_3(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([other[e1234], 1.0, 1.0, 1.0]) * Simd32x4::from([-1.0, 0.0, 0.0, 0.0]),
            // e3215
            other[e3215] * -1.0,
        );
    }
}

impl TryFrom<AntiDipoleInversion> for MotorAtInfinity {
    type Error = String;
    fn try_from(anti_dipole_inversion: AntiDipoleInversion) -> Result<Self, Self::Error> {
        use crate::elements::*;
        let mut error_string = String::new();
        let mut fail = false;
        let el = anti_dipole_inversion[0];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e423: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = anti_dipole_inversion[1];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e431: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = anti_dipole_inversion[2];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e412: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = anti_dipole_inversion[3];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e415: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = anti_dipole_inversion[4];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e425: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = anti_dipole_inversion[5];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e435: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = anti_dipole_inversion[6];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e321: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = anti_dipole_inversion[10];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e4: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = anti_dipole_inversion[11];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e1: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = anti_dipole_inversion[12];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e2: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = anti_dipole_inversion[13];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e3: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from AntiDipoleInversion do not fit into MotorAtInfinity { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(MotorAtInfinity::from_groups(/* e235, e315, e125, e5 */ Simd32x4::from([
            anti_dipole_inversion[e235],
            anti_dipole_inversion[e315],
            anti_dipole_inversion[e125],
            anti_dipole_inversion[e5],
        ])));
    }
}

impl TryFrom<AntiDipoleInversionAtInfinity> for MotorAtInfinity {
    type Error = String;
    fn try_from(anti_dipole_inversion_at_infinity: AntiDipoleInversionAtInfinity) -> Result<Self, Self::Error> {
        use crate::elements::*;
        let mut error_string = String::new();
        let mut fail = false;
        let el = anti_dipole_inversion_at_infinity[0];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e415: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = anti_dipole_inversion_at_infinity[1];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e425: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = anti_dipole_inversion_at_infinity[2];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e435: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = anti_dipole_inversion_at_infinity[3];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e321: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = anti_dipole_inversion_at_infinity[7];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e1: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = anti_dipole_inversion_at_infinity[8];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e2: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = anti_dipole_inversion_at_infinity[9];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e3: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from AntiDipoleInversionAtInfinity do not fit into MotorAtInfinity { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(MotorAtInfinity::from_groups(/* e235, e315, e125, e5 */ Simd32x4::from([
            anti_dipole_inversion_at_infinity[e235],
            anti_dipole_inversion_at_infinity[e315],
            anti_dipole_inversion_at_infinity[e125],
            anti_dipole_inversion_at_infinity[e5],
        ])));
    }
}

impl TryFrom<AntiDipoleInversionOrthogonalOrigin> for MotorAtInfinity {
    type Error = String;
    fn try_from(anti_dipole_inversion_orthogonal_origin: AntiDipoleInversionOrthogonalOrigin) -> Result<Self, Self::Error> {
        use crate::elements::*;
        let mut error_string = String::new();
        let mut fail = false;
        let el = anti_dipole_inversion_orthogonal_origin[0];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e423: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = anti_dipole_inversion_orthogonal_origin[1];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e431: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = anti_dipole_inversion_orthogonal_origin[2];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e412: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = anti_dipole_inversion_orthogonal_origin[4];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e415: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = anti_dipole_inversion_orthogonal_origin[5];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e425: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = anti_dipole_inversion_orthogonal_origin[6];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e435: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = anti_dipole_inversion_orthogonal_origin[10];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e4: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from AntiDipoleInversionOrthogonalOrigin do not fit into MotorAtInfinity { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(MotorAtInfinity::from_groups(/* e235, e315, e125, e5 */ Simd32x4::from([
            anti_dipole_inversion_orthogonal_origin[e235],
            anti_dipole_inversion_orthogonal_origin[e315],
            anti_dipole_inversion_orthogonal_origin[e125],
            anti_dipole_inversion_orthogonal_origin[e5],
        ])));
    }
}

impl TryFrom<AntiFlatPoint> for MotorAtInfinity {
    type Error = String;
    fn try_from(anti_flat_point: AntiFlatPoint) -> Result<Self, Self::Error> {
        use crate::elements::*;
        let mut error_string = String::new();
        let mut fail = false;
        let el = anti_flat_point[3];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e321: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from AntiFlatPoint do not fit into MotorAtInfinity { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(MotorAtInfinity::from_groups(
            // e235, e315, e125, e5
            Simd32x4::from([anti_flat_point[e235], anti_flat_point[e315], anti_flat_point[e125], 0.0]),
        ));
    }
}

impl TryFrom<AntiFlector> for MotorAtInfinity {
    type Error = String;
    fn try_from(anti_flector: AntiFlector) -> Result<Self, Self::Error> {
        use crate::elements::*;
        let mut error_string = String::new();
        let mut fail = false;
        let el = anti_flector[3];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e321: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = anti_flector[4];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e1: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = anti_flector[5];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e2: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = anti_flector[6];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e3: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from AntiFlector do not fit into MotorAtInfinity { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(MotorAtInfinity::from_groups(/* e235, e315, e125, e5 */ Simd32x4::from([
            anti_flector[e235], anti_flector[e315], anti_flector[e125], anti_flector[e5],
        ])));
    }
}

impl TryFrom<AntiPlane> for MotorAtInfinity {
    type Error = String;
    fn try_from(anti_plane: AntiPlane) -> Result<Self, Self::Error> {
        use crate::elements::*;
        let mut error_string = String::new();
        let mut fail = false;
        let el = anti_plane[0];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e1: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = anti_plane[1];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e2: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = anti_plane[2];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e3: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from AntiPlane do not fit into MotorAtInfinity { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(MotorAtInfinity::from_groups(/* e235, e315, e125, e5 */ Simd32x3::from(0.0).extend_to_4(anti_plane[e5])));
    }
}

impl TryFrom<Circle> for MotorAtInfinity {
    type Error = String;
    fn try_from(circle: Circle) -> Result<Self, Self::Error> {
        use crate::elements::*;
        let mut error_string = String::new();
        let mut fail = false;
        let el = circle[0];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e423: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = circle[1];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e431: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = circle[2];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e412: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = circle[3];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e415: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = circle[4];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e425: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = circle[5];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e435: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = circle[6];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e321: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from Circle do not fit into MotorAtInfinity { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(MotorAtInfinity::from_groups(
            // e235, e315, e125, e5
            Simd32x4::from([circle[e235], circle[e315], circle[e125], 0.0]),
        ));
    }
}

impl TryFrom<CircleAligningOrigin> for MotorAtInfinity {
    type Error = String;
    fn try_from(circle_aligning_origin: CircleAligningOrigin) -> Result<Self, Self::Error> {
        use crate::elements::*;
        let mut error_string = String::new();
        let mut fail = false;
        let el = circle_aligning_origin[0];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e423: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = circle_aligning_origin[1];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e431: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = circle_aligning_origin[2];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e412: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = circle_aligning_origin[3];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e415: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = circle_aligning_origin[4];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e425: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = circle_aligning_origin[5];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e435: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from CircleAligningOrigin do not fit into MotorAtInfinity { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(MotorAtInfinity::from_groups(/* e235, e315, e125, e5 */ Simd32x4::from([
            circle_aligning_origin[e235],
            circle_aligning_origin[e315],
            circle_aligning_origin[e125],
            0.0,
        ])));
    }
}

impl TryFrom<CircleAtInfinity> for MotorAtInfinity {
    type Error = String;
    fn try_from(circle_at_infinity: CircleAtInfinity) -> Result<Self, Self::Error> {
        use crate::elements::*;
        let mut error_string = String::new();
        let mut fail = false;
        let el = circle_at_infinity[0];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e415: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = circle_at_infinity[1];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e425: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = circle_at_infinity[2];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e435: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = circle_at_infinity[3];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e321: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from CircleAtInfinity do not fit into MotorAtInfinity { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(MotorAtInfinity::from_groups(/* e235, e315, e125, e5 */ Simd32x4::from([
            circle_at_infinity[e235],
            circle_at_infinity[e315],
            circle_at_infinity[e125],
            0.0,
        ])));
    }
}

impl TryFrom<CircleAtOrigin> for MotorAtInfinity {
    type Error = String;
    fn try_from(circle_at_origin: CircleAtOrigin) -> Result<Self, Self::Error> {
        use crate::elements::*;
        let mut error_string = String::new();
        let mut fail = false;
        let el = circle_at_origin[0];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e423: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = circle_at_origin[1];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e431: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = circle_at_origin[2];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e412: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from CircleAtOrigin do not fit into MotorAtInfinity { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(MotorAtInfinity::from_groups(/* e235, e315, e125, e5 */ Simd32x4::from([
            circle_at_origin[e235],
            circle_at_origin[e315],
            circle_at_origin[e125],
            0.0,
        ])));
    }
}

impl TryFrom<CircleOrthogonalOrigin> for MotorAtInfinity {
    type Error = String;
    fn try_from(circle_orthogonal_origin: CircleOrthogonalOrigin) -> Result<Self, Self::Error> {
        use crate::elements::*;
        let mut error_string = String::new();
        let mut fail = false;
        let el = circle_orthogonal_origin[0];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e423: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = circle_orthogonal_origin[1];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e431: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = circle_orthogonal_origin[2];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e412: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = circle_orthogonal_origin[3];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e321: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from CircleOrthogonalOrigin do not fit into MotorAtInfinity { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(MotorAtInfinity::from_groups(/* e235, e315, e125, e5 */ Simd32x4::from([
            circle_orthogonal_origin[e235],
            circle_orthogonal_origin[e315],
            circle_orthogonal_origin[e125],
            0.0,
        ])));
    }
}

impl TryFrom<CircleRotor> for MotorAtInfinity {
    type Error = String;
    fn try_from(circle_rotor: CircleRotor) -> Result<Self, Self::Error> {
        use crate::elements::*;
        let mut error_string = String::new();
        let mut fail = false;
        let el = circle_rotor[0];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e423: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = circle_rotor[1];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e431: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = circle_rotor[2];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e412: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = circle_rotor[3];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e415: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = circle_rotor[4];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e425: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = circle_rotor[5];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e435: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = circle_rotor[6];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e321: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = circle_rotor[10];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e12345: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from CircleRotor do not fit into MotorAtInfinity { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(MotorAtInfinity::from_groups(
            // e235, e315, e125, e5
            Simd32x4::from([circle_rotor[e235], circle_rotor[e315], circle_rotor[e125], 0.0]),
        ));
    }
}

impl TryFrom<CircleRotorAligningOrigin> for MotorAtInfinity {
    type Error = String;
    fn try_from(circle_rotor_aligning_origin: CircleRotorAligningOrigin) -> Result<Self, Self::Error> {
        use crate::elements::*;
        let mut error_string = String::new();
        let mut fail = false;
        let el = circle_rotor_aligning_origin[0];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e423: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = circle_rotor_aligning_origin[1];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e431: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = circle_rotor_aligning_origin[2];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e412: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = circle_rotor_aligning_origin[3];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e415: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = circle_rotor_aligning_origin[4];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e425: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = circle_rotor_aligning_origin[5];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e435: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = circle_rotor_aligning_origin[9];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e12345: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from CircleRotorAligningOrigin do not fit into MotorAtInfinity { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(MotorAtInfinity::from_groups(/* e235, e315, e125, e5 */ Simd32x4::from([
            circle_rotor_aligning_origin[e235],
            circle_rotor_aligning_origin[e315],
            circle_rotor_aligning_origin[e125],
            0.0,
        ])));
    }
}

impl TryFrom<CircleRotorAligningOriginAtInfinity> for MotorAtInfinity {
    type Error = String;
    fn try_from(circle_rotor_aligning_origin_at_infinity: CircleRotorAligningOriginAtInfinity) -> Result<Self, Self::Error> {
        use crate::elements::*;
        let mut error_string = String::new();
        let mut fail = false;
        let el = circle_rotor_aligning_origin_at_infinity[0];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e415: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = circle_rotor_aligning_origin_at_infinity[1];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e425: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = circle_rotor_aligning_origin_at_infinity[2];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e435: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = circle_rotor_aligning_origin_at_infinity[6];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e12345: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from CircleRotorAligningOriginAtInfinity do not fit into MotorAtInfinity { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(MotorAtInfinity::from_groups(/* e235, e315, e125, e5 */ Simd32x4::from([
            circle_rotor_aligning_origin_at_infinity[e235],
            circle_rotor_aligning_origin_at_infinity[e315],
            circle_rotor_aligning_origin_at_infinity[e125],
            0.0,
        ])));
    }
}

impl TryFrom<CircleRotorAtInfinity> for MotorAtInfinity {
    type Error = String;
    fn try_from(circle_rotor_at_infinity: CircleRotorAtInfinity) -> Result<Self, Self::Error> {
        use crate::elements::*;
        let mut error_string = String::new();
        let mut fail = false;
        let el = circle_rotor_at_infinity[0];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e415: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = circle_rotor_at_infinity[1];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e425: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = circle_rotor_at_infinity[2];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e435: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = circle_rotor_at_infinity[3];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e321: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = circle_rotor_at_infinity[7];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e12345: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from CircleRotorAtInfinity do not fit into MotorAtInfinity { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(MotorAtInfinity::from_groups(/* e235, e315, e125, e5 */ Simd32x4::from([
            circle_rotor_at_infinity[e235],
            circle_rotor_at_infinity[e315],
            circle_rotor_at_infinity[e125],
            0.0,
        ])));
    }
}

impl TryFrom<Line> for MotorAtInfinity {
    type Error = String;
    fn try_from(line: Line) -> Result<Self, Self::Error> {
        use crate::elements::*;
        let mut error_string = String::new();
        let mut fail = false;
        let el = line[0];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e415: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = line[1];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e425: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = line[2];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e435: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from Line do not fit into MotorAtInfinity { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(MotorAtInfinity::from_groups(/* e235, e315, e125, e5 */ Simd32x4::from([line[e235], line[e315], line[e125], 0.0])));
    }
}

impl TryFrom<Motor> for MotorAtInfinity {
    type Error = String;
    fn try_from(motor: Motor) -> Result<Self, Self::Error> {
        let mut error_string = String::new();
        let mut fail = false;
        let el = motor[0];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e415: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = motor[1];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e425: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = motor[2];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e435: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = motor[3];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e12345: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from Motor do not fit into MotorAtInfinity { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(MotorAtInfinity::from_groups(/* e235, e315, e125, e5 */ motor.group1()));
    }
}

impl TryFrom<MultiVector> for MotorAtInfinity {
    type Error = String;
    fn try_from(multi_vector: MultiVector) -> Result<Self, Self::Error> {
        use crate::elements::*;
        let mut error_string = String::new();
        let mut fail = false;
        let el = multi_vector[0];
        if el != 0.0 {
            fail = true;
            error_string.push_str("scalar: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = multi_vector[1];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e12345: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = multi_vector[2];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e1: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = multi_vector[3];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e2: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = multi_vector[4];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e3: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = multi_vector[5];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e4: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = multi_vector[7];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e41: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = multi_vector[8];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e42: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = multi_vector[9];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e43: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = multi_vector[10];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e45: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = multi_vector[11];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e15: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = multi_vector[12];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e25: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = multi_vector[13];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e35: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = multi_vector[14];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e23: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = multi_vector[15];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e31: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = multi_vector[16];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e12: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = multi_vector[17];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e415: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = multi_vector[18];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e425: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = multi_vector[19];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e435: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = multi_vector[20];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e321: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = multi_vector[21];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e423: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = multi_vector[22];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e431: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = multi_vector[23];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e412: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = multi_vector[27];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e1234: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = multi_vector[28];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e4235: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = multi_vector[29];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e4315: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = multi_vector[30];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e4125: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = multi_vector[31];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e3215: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from MultiVector do not fit into MotorAtInfinity { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(MotorAtInfinity::from_groups(/* e235, e315, e125, e5 */ Simd32x4::from([
            multi_vector[e235], multi_vector[e315], multi_vector[e125], multi_vector[e5],
        ])));
    }
}

impl TryFrom<RoundPoint> for MotorAtInfinity {
    type Error = String;
    fn try_from(round_point: RoundPoint) -> Result<Self, Self::Error> {
        use crate::elements::*;
        let mut error_string = String::new();
        let mut fail = false;
        let el = round_point[0];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e1: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = round_point[1];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e2: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = round_point[2];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e3: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = round_point[3];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e4: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from RoundPoint do not fit into MotorAtInfinity { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(MotorAtInfinity::from_groups(/* e235, e315, e125, e5 */ Simd32x3::from(0.0).extend_to_4(round_point[e5])));
    }
}

impl TryFrom<RoundPointAtOrigin> for MotorAtInfinity {
    type Error = String;
    fn try_from(round_point_at_origin: RoundPointAtOrigin) -> Result<Self, Self::Error> {
        use crate::elements::*;
        let mut error_string = String::new();
        let mut fail = false;
        let el = round_point_at_origin[0];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e4: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from RoundPointAtOrigin do not fit into MotorAtInfinity { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(MotorAtInfinity::from_groups(/* e235, e315, e125, e5 */ Simd32x3::from(0.0).extend_to_4(round_point_at_origin[e5])));
    }
}

impl TryFrom<VersorEven> for MotorAtInfinity {
    type Error = String;
    fn try_from(versor_even: VersorEven) -> Result<Self, Self::Error> {
        let mut error_string = String::new();
        let mut fail = false;
        let el = versor_even[0];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e423: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = versor_even[1];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e431: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = versor_even[2];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e412: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = versor_even[3];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e12345: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = versor_even[4];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e415: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = versor_even[5];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e425: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = versor_even[6];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e435: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = versor_even[7];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e321: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = versor_even[12];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e1: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = versor_even[13];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e2: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = versor_even[14];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e3: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = versor_even[15];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e4: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from VersorEven do not fit into MotorAtInfinity { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(MotorAtInfinity::from_groups(/* e235, e315, e125, e5 */ versor_even.group2()));
    }
}

impl TryFrom<VersorEvenAligningOrigin> for MotorAtInfinity {
    type Error = String;
    fn try_from(versor_even_aligning_origin: VersorEvenAligningOrigin) -> Result<Self, Self::Error> {
        let mut error_string = String::new();
        let mut fail = false;
        let el = versor_even_aligning_origin[0];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e423: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = versor_even_aligning_origin[1];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e431: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = versor_even_aligning_origin[2];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e412: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = versor_even_aligning_origin[3];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e12345: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = versor_even_aligning_origin[4];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e415: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = versor_even_aligning_origin[5];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e425: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = versor_even_aligning_origin[6];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e435: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = versor_even_aligning_origin[7];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e4: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from VersorEvenAligningOrigin do not fit into MotorAtInfinity { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(MotorAtInfinity::from_groups(/* e235, e315, e125, e5 */ versor_even_aligning_origin.group2()));
    }
}

impl TryFrom<VersorEvenAtInfinity> for MotorAtInfinity {
    type Error = String;
    fn try_from(versor_even_at_infinity: VersorEvenAtInfinity) -> Result<Self, Self::Error> {
        let mut error_string = String::new();
        let mut fail = false;
        let el = versor_even_at_infinity[0];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e12345: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = versor_even_at_infinity[1];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e1: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = versor_even_at_infinity[2];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e2: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = versor_even_at_infinity[3];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e3: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = versor_even_at_infinity[4];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e415: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = versor_even_at_infinity[5];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e425: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = versor_even_at_infinity[6];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e435: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = versor_even_at_infinity[7];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e321: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from VersorEvenAtInfinity do not fit into MotorAtInfinity { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(MotorAtInfinity::from_groups(/* e235, e315, e125, e5 */ versor_even_at_infinity.group2()));
    }
}

impl TryFrom<VersorEvenAtOrigin> for MotorAtInfinity {
    type Error = String;
    fn try_from(versor_even_at_origin: VersorEvenAtOrigin) -> Result<Self, Self::Error> {
        let mut error_string = String::new();
        let mut fail = false;
        let el = versor_even_at_origin[0];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e423: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = versor_even_at_origin[1];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e431: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = versor_even_at_origin[2];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e412: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = versor_even_at_origin[3];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e4: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from VersorEvenAtOrigin do not fit into MotorAtInfinity { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(MotorAtInfinity::from_groups(/* e235, e315, e125, e5 */ versor_even_at_origin.group1()));
    }
}

impl TryFrom<VersorEvenOrthogonalOrigin> for MotorAtInfinity {
    type Error = String;
    fn try_from(versor_even_orthogonal_origin: VersorEvenOrthogonalOrigin) -> Result<Self, Self::Error> {
        let mut error_string = String::new();
        let mut fail = false;
        let el = versor_even_orthogonal_origin[0];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e423: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = versor_even_orthogonal_origin[1];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e431: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = versor_even_orthogonal_origin[2];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e412: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = versor_even_orthogonal_origin[3];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e321: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = versor_even_orthogonal_origin[8];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e1: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = versor_even_orthogonal_origin[9];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e2: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = versor_even_orthogonal_origin[10];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e3: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = versor_even_orthogonal_origin[11];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e4: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from VersorEvenOrthogonalOrigin do not fit into MotorAtInfinity { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(MotorAtInfinity::from_groups(/* e235, e315, e125, e5 */ versor_even_orthogonal_origin.group1()));
    }
}
