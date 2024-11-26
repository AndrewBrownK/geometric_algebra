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
//   Median:         0       2       0
//  Average:         0       1       0
//  Maximum:         0       6       0
//
//  No SIMD:   add/sub     mul     div
//  Minimum:         0       0       0
//   Median:         0       7       0
//  Average:         0       6       0
//  Maximum:         0      20       0
impl std::ops::Div<right_complement> for AntiCircleRotor {
    type Output = CircleRotor;
    fn div(self, _rhs: right_complement) -> Self::Output {
        self.right_complement()
    }
}
impl RightComplement for AntiCircleRotor {
    type Output = CircleRotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        0        1        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        0        3        0
    //  no simd        0       11        0
    fn right_complement(self) -> Self::Output {
        use crate::elements::*;
        return CircleRotor::from_groups(
            // e423, e431, e412
            Simd32x3::from([self[e15], self[e25], self[e35]]) * Simd32x3::from(-1.0),
            // e415, e425, e435, e321
            self.group1() * Simd32x4::from(-1.0),
            // e235, e315, e125, e12345
            Simd32x4::from([self[e41], self[e42], self[e43], self[scalar]]) * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
        );
    }
}
impl std::ops::Div<right_complement> for AntiDipoleInversion {
    type Output = DipoleInversion;
    fn div(self, _rhs: right_complement) -> Self::Output {
        self.right_complement()
    }
}
impl RightComplement for AntiDipoleInversion {
    type Output = DipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        0        1        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        0        3        0
    //  no simd        0       11        0
    fn right_complement(self) -> Self::Output {
        use crate::elements::*;
        return DipoleInversion::from_groups(
            // e41, e42, e43
            Simd32x3::from([self[e235], self[e315], self[e125]]) * Simd32x3::from(-1.0),
            // e23, e31, e12, e45
            self.group1() * Simd32x4::from(-1.0),
            // e15, e25, e35, e1234
            Simd32x4::from([self[e423], self[e431], self[e412], self[e5]]) * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([self[e1], self[e2], self[e3], self[e4]]),
        );
    }
}
impl std::ops::Div<right_complement> for AntiDualNum {
    type Output = VersorEven;
    fn div(self, _rhs: right_complement) -> Self::Output {
        self.right_complement()
    }
}
impl RightComplement for AntiDualNum {
    type Output = VersorEven;
    fn right_complement(self) -> Self::Output {
        use crate::elements::*;
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([0.0, 0.0, 0.0, self[scalar]]),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e235, e315, e125, e5
            Simd32x4::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from([0.0, 0.0, 0.0, self[e3215]]),
        );
    }
}
impl std::ops::Div<right_complement> for AntiFlatPoint {
    type Output = Dipole;
    fn div(self, _rhs: right_complement) -> Self::Output {
        self.right_complement()
    }
}
impl RightComplement for AntiFlatPoint {
    type Output = Dipole;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        2        0
    //  no simd        0        7        0
    fn right_complement(self) -> Self::Output {
        use crate::elements::*;
        return Dipole::from_groups(
            // e41, e42, e43
            Simd32x3::from([self[e235], self[e315], self[e125]]) * Simd32x3::from(-1.0),
            // e23, e31, e12, e45
            Simd32x4::from([1.0, 1.0, 1.0, self[e321]]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]),
            // e15, e25, e35
            Simd32x3::from(0.0),
        );
    }
}
impl std::ops::Div<right_complement> for AntiFlector {
    type Output = DipoleInversion;
    fn div(self, _rhs: right_complement) -> Self::Output {
        self.right_complement()
    }
}
impl RightComplement for AntiFlector {
    type Output = DipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        2        0
    //  no simd        0        7        0
    fn right_complement(self) -> Self::Output {
        use crate::elements::*;
        return DipoleInversion::from_groups(
            // e41, e42, e43
            Simd32x3::from([self[e235], self[e315], self[e125]]) * Simd32x3::from(-1.0),
            // e23, e31, e12, e45
            Simd32x4::from([1.0, 1.0, 1.0, self[e321]]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]),
            // e15, e25, e35, e1234
            Simd32x4::from([0.0, 0.0, 0.0, self[e5]]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([self[e1], self[e2], self[e3], 0.0]),
        );
    }
}
impl std::ops::Div<right_complement> for AntiLine {
    type Output = Circle;
    fn div(self, _rhs: right_complement) -> Self::Output {
        self.right_complement()
    }
}
impl RightComplement for AntiLine {
    type Output = Circle;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        2        0
    //  no simd        0        7        0
    fn right_complement(self) -> Self::Output {
        use crate::elements::*;
        return Circle::from_groups(
            // e423, e431, e412
            self.group1() * Simd32x3::from(-1.0),
            // e415, e425, e435, e321
            Simd32x4::from([self[e23], self[e31], self[e12], 1.0]) * Simd32x4::from([-1.0, -1.0, -1.0, 0.0]),
            // e235, e315, e125
            Simd32x3::from(0.0),
        );
    }
}
impl std::ops::Div<right_complement> for AntiMotor {
    type Output = VersorEven;
    fn div(self, _rhs: right_complement) -> Self::Output {
        self.right_complement()
    }
}
impl RightComplement for AntiMotor {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        2        0
    // no simd        0        8        0
    fn right_complement(self) -> Self::Output {
        use crate::elements::*;
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([self[e15], self[e25], self[e35], self[scalar]]) * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e415, e425, e435, e321
            Simd32x4::from([self[e23], self[e31], self[e12], 1.0]) * Simd32x4::from([-1.0, -1.0, -1.0, 0.0]),
            // e235, e315, e125, e5
            Simd32x4::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from([0.0, 0.0, 0.0, self[e3215]]),
        );
    }
}
impl std::ops::Div<right_complement> for AntiPlane {
    type Output = Sphere;
    fn div(self, _rhs: right_complement) -> Self::Output {
        self.right_complement()
    }
}
impl RightComplement for AntiPlane {
    type Output = Sphere;
    fn right_complement(self) -> Self::Output {
        use crate::elements::*;
        return Sphere::from_groups(/* e4235, e4315, e4125, e3215 */ Simd32x4::from([self[e1], self[e2], self[e3], 0.0]), /* e1234 */ self[e5]);
    }
}
impl std::ops::Div<right_complement> for AntiScalar {
    type Output = Scalar;
    fn div(self, _rhs: right_complement) -> Self::Output {
        self.right_complement()
    }
}
impl RightComplement for AntiScalar {
    type Output = Scalar;
    fn right_complement(self) -> Self::Output {
        use crate::elements::*;
        return Scalar::from_groups(/* scalar */ self[e12345]);
    }
}
impl std::ops::Div<right_complement> for Circle {
    type Output = Dipole;
    fn div(self, _rhs: right_complement) -> Self::Output {
        self.right_complement()
    }
}
impl RightComplement for Circle {
    type Output = Dipole;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        0        2        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        3        0
    //  no simd        0       10        0
    fn right_complement(self) -> Self::Output {
        return Dipole::from_groups(
            // e41, e42, e43
            self.group2() * Simd32x3::from(-1.0),
            // e23, e31, e12, e45
            self.group1() * Simd32x4::from(-1.0),
            // e15, e25, e35
            self.group0() * Simd32x3::from(-1.0),
        );
    }
}
impl std::ops::Div<right_complement> for CircleRotor {
    type Output = AntiCircleRotor;
    fn div(self, _rhs: right_complement) -> Self::Output {
        self.right_complement()
    }
}
impl RightComplement for CircleRotor {
    type Output = AntiCircleRotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        0        1        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        0        3        0
    //  no simd        0       11        0
    fn right_complement(self) -> Self::Output {
        use crate::elements::*;
        return AntiCircleRotor::from_groups(
            // e41, e42, e43
            Simd32x3::from([self[e235], self[e315], self[e125]]) * Simd32x3::from(-1.0),
            // e23, e31, e12, e45
            self.group1() * Simd32x4::from(-1.0),
            // e15, e25, e35, scalar
            Simd32x4::from([self[e423], self[e431], self[e412], self[e12345]]) * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
        );
    }
}
impl std::ops::Div<right_complement> for Dipole {
    type Output = Circle;
    fn div(self, _rhs: right_complement) -> Self::Output {
        self.right_complement()
    }
}
impl RightComplement for Dipole {
    type Output = Circle;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        0        2        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        3        0
    //  no simd        0       10        0
    fn right_complement(self) -> Self::Output {
        return Circle::from_groups(
            // e423, e431, e412
            self.group2() * Simd32x3::from(-1.0),
            // e415, e425, e435, e321
            self.group1() * Simd32x4::from(-1.0),
            // e235, e315, e125
            self.group0() * Simd32x3::from(-1.0),
        );
    }
}
impl std::ops::Div<right_complement> for DipoleInversion {
    type Output = AntiDipoleInversion;
    fn div(self, _rhs: right_complement) -> Self::Output {
        self.right_complement()
    }
}
impl RightComplement for DipoleInversion {
    type Output = AntiDipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        0        1        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        0        3        0
    //  no simd        0       11        0
    fn right_complement(self) -> Self::Output {
        use crate::elements::*;
        return AntiDipoleInversion::from_groups(
            // e423, e431, e412
            Simd32x3::from([self[e15], self[e25], self[e35]]) * Simd32x3::from(-1.0),
            // e415, e425, e435, e321
            self.group1() * Simd32x4::from(-1.0),
            // e235, e315, e125, e4
            Simd32x4::from([self[e41], self[e42], self[e43], self[e3215]]) * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e1, e2, e3, e5
            Simd32x4::from([self[e4235], self[e4315], self[e4125], self[e1234]]),
        );
    }
}
impl std::ops::Div<right_complement> for DualNum {
    type Output = VersorOdd;
    fn div(self, _rhs: right_complement) -> Self::Output {
        self.right_complement()
    }
}
impl RightComplement for DualNum {
    type Output = VersorOdd;
    fn right_complement(self) -> Self::Output {
        use crate::elements::*;
        return VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([0.0, 0.0, 0.0, self[e12345]]),
            // e23, e31, e12, e45
            Simd32x4::from(0.0),
            // e15, e25, e35, e1234
            Simd32x4::from([0.0, 0.0, 0.0, self[e5]]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(0.0),
        );
    }
}
impl std::ops::Div<right_complement> for FlatPoint {
    type Output = Circle;
    fn div(self, _rhs: right_complement) -> Self::Output {
        self.right_complement()
    }
}
impl RightComplement for FlatPoint {
    type Output = Circle;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        2        0
    //  no simd        0        7        0
    fn right_complement(self) -> Self::Output {
        use crate::elements::*;
        return Circle::from_groups(
            // e423, e431, e412
            Simd32x3::from([self[e15], self[e25], self[e35]]) * Simd32x3::from(-1.0),
            // e415, e425, e435, e321
            Simd32x4::from([1.0, 1.0, 1.0, self[e45]]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]),
            // e235, e315, e125
            Simd32x3::from(0.0),
        );
    }
}
impl std::ops::Div<right_complement> for Flector {
    type Output = AntiDipoleInversion;
    fn div(self, _rhs: right_complement) -> Self::Output {
        self.right_complement()
    }
}
impl RightComplement for Flector {
    type Output = AntiDipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        2        0
    //  no simd        0        7        0
    fn right_complement(self) -> Self::Output {
        use crate::elements::*;
        return AntiDipoleInversion::from_groups(
            // e423, e431, e412
            Simd32x3::from([self[e15], self[e25], self[e35]]) * Simd32x3::from(-1.0),
            // e415, e425, e435, e321
            Simd32x4::from([1.0, 1.0, 1.0, self[e45]]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]),
            // e235, e315, e125, e4
            Simd32x4::from([0.0, 0.0, 0.0, self[e3215]]),
            // e1, e2, e3, e5
            Simd32x4::from([self[e4235], self[e4315], self[e4125], 0.0]),
        );
    }
}
impl std::ops::Div<right_complement> for Line {
    type Output = Dipole;
    fn div(self, _rhs: right_complement) -> Self::Output {
        self.right_complement()
    }
}
impl RightComplement for Line {
    type Output = Dipole;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        2        0
    //  no simd        0        7        0
    fn right_complement(self) -> Self::Output {
        use crate::elements::*;
        return Dipole::from_groups(
            // e41, e42, e43
            self.group1() * Simd32x3::from(-1.0),
            // e23, e31, e12, e45
            Simd32x4::from([self[e415], self[e425], self[e435], 1.0]) * Simd32x4::from([-1.0, -1.0, -1.0, 0.0]),
            // e15, e25, e35
            Simd32x3::from(0.0),
        );
    }
}
impl std::ops::Div<right_complement> for Motor {
    type Output = VersorOdd;
    fn div(self, _rhs: right_complement) -> Self::Output {
        self.right_complement()
    }
}
impl RightComplement for Motor {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        2        0
    // no simd        0        8        0
    fn right_complement(self) -> Self::Output {
        use crate::elements::*;
        return VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([self[e235], self[e315], self[e125], self[e12345]]) * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e23, e31, e12, e45
            Simd32x4::from([self[e415], self[e425], self[e435], 1.0]) * Simd32x4::from([-1.0, -1.0, -1.0, 0.0]),
            // e15, e25, e35, e1234
            Simd32x4::from([0.0, 0.0, 0.0, self[e5]]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(0.0),
        );
    }
}
impl std::ops::Div<right_complement> for MultiVector {
    type Output = MultiVector;
    fn div(self, _rhs: right_complement) -> Self::Output {
        self.right_complement()
    }
}
impl std::ops::DivAssign<right_complement> for MultiVector {
    fn div_assign(&mut self, _rhs: right_complement) {
        *self = self.right_complement()
    }
}
impl RightComplement for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        0        4        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        0        6        0
    //  no simd        0       20        0
    fn right_complement(self) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            crate::swizzle!(self.group0(), 1, 0),
            // e1, e2, e3, e4
            self.group9(),
            // e5
            self[e1234],
            // e15, e25, e35, e45
            Simd32x4::from([self[e423], self[e431], self[e412], self[e321]]) * Simd32x4::from(-1.0),
            // e41, e42, e43
            self.group8() * Simd32x3::from(-1.0),
            // e23, e31, e12
            Simd32x3::from([self[e415], self[e425], self[e435]]) * Simd32x3::from(-1.0),
            // e415, e425, e435, e321
            Simd32x4::from([self[e23], self[e31], self[e12], self[e45]]) * Simd32x4::from(-1.0),
            // e423, e431, e412
            Simd32x3::from([self[e15], self[e25], self[e35]]) * Simd32x3::from(-1.0),
            // e235, e315, e125
            self.group4() * Simd32x3::from(-1.0),
            // e4235, e4315, e4125, e3215
            self.group1(),
            // e1234
            self[e5],
        );
    }
}
impl std::ops::Div<right_complement> for Plane {
    type Output = RoundPoint;
    fn div(self, _rhs: right_complement) -> Self::Output {
        self.right_complement()
    }
}
impl RightComplement for Plane {
    type Output = RoundPoint;
    fn right_complement(self) -> Self::Output {
        return RoundPoint::from_groups(/* e1, e2, e3, e4 */ self.group0(), /* e5 */ 0.0);
    }
}
impl std::ops::Div<right_complement> for RoundPoint {
    type Output = Sphere;
    fn div(self, _rhs: right_complement) -> Self::Output {
        self.right_complement()
    }
}
impl RightComplement for RoundPoint {
    type Output = Sphere;
    fn right_complement(self) -> Self::Output {
        use crate::elements::*;
        return Sphere::from_groups(/* e4235, e4315, e4125, e3215 */ self.group0(), /* e1234 */ self[e5]);
    }
}
impl std::ops::Div<right_complement> for Scalar {
    type Output = AntiScalar;
    fn div(self, _rhs: right_complement) -> Self::Output {
        self.right_complement()
    }
}
impl RightComplement for Scalar {
    type Output = AntiScalar;
    fn right_complement(self) -> Self::Output {
        use crate::elements::*;
        return AntiScalar::from_groups(/* e12345 */ self[scalar]);
    }
}
impl std::ops::Div<right_complement> for Sphere {
    type Output = RoundPoint;
    fn div(self, _rhs: right_complement) -> Self::Output {
        self.right_complement()
    }
}
impl RightComplement for Sphere {
    type Output = RoundPoint;
    fn right_complement(self) -> Self::Output {
        use crate::elements::*;
        return RoundPoint::from_groups(/* e1, e2, e3, e4 */ self.group0(), /* e5 */ self[e1234]);
    }
}
impl std::ops::Div<right_complement> for VersorEven {
    type Output = VersorOdd;
    fn div(self, _rhs: right_complement) -> Self::Output {
        self.right_complement()
    }
}
impl RightComplement for VersorEven {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        3        0
    // no simd        0       12        0
    fn right_complement(self) -> Self::Output {
        use crate::elements::*;
        return VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([self[e235], self[e315], self[e125], self[e12345]]) * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e23, e31, e12, e45
            self.group1() * Simd32x4::from(-1.0),
            // e15, e25, e35, e1234
            Simd32x4::from([self[e423], self[e431], self[e412], self[e5]]) * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e4235, e4315, e4125, e3215
            self.group3(),
        );
    }
}
impl std::ops::Div<right_complement> for VersorOdd {
    type Output = VersorEven;
    fn div(self, _rhs: right_complement) -> Self::Output {
        self.right_complement()
    }
}
impl RightComplement for VersorOdd {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        3        0
    // no simd        0       12        0
    fn right_complement(self) -> Self::Output {
        use crate::elements::*;
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([self[e15], self[e25], self[e35], self[scalar]]) * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e415, e425, e435, e321
            self.group1() * Simd32x4::from(-1.0),
            // e235, e315, e125, e5
            Simd32x4::from([self[e41], self[e42], self[e43], self[e1234]]) * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e1, e2, e3, e4
            self.group3(),
        );
    }
}
