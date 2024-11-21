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
//   Median:         0       1       0
//  Average:         0       1       0
//  Maximum:         0       6       0
//
//  No SIMD:   add/sub     mul     div
//  Minimum:         0       0       0
//   Median:         0       1       0
//  Average:         0       4       0
//  Maximum:         0      16       0
impl std::ops::Div<auto_morphism> for AntiCircleRotor {
    type Output = AntiCircleRotor;
    fn div(self, _rhs: auto_morphism) -> Self::Output {
        self.auto_morphism()
    }
}
impl std::ops::DivAssign<auto_morphism> for AntiCircleRotor {
    fn div_assign(&mut self, _rhs: auto_morphism) {
        *self = self.auto_morphism()
    }
}
impl AutoMorphism for AntiCircleRotor {
    fn auto_morphism(self) -> Self {
        return self;
    }
}
impl std::ops::Div<auto_morphism> for AntiDipoleInversion {
    type Output = AntiDipoleInversion;
    fn div(self, _rhs: auto_morphism) -> Self::Output {
        self.auto_morphism()
    }
}
impl std::ops::DivAssign<auto_morphism> for AntiDipoleInversion {
    fn div_assign(&mut self, _rhs: auto_morphism) {
        *self = self.auto_morphism()
    }
}
impl AutoMorphism for AntiDipoleInversion {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        0        1        0
    //    simd4        0        3        0
    // Totals...
    // yes simd        0        4        0
    //  no simd        0       15        0
    fn auto_morphism(self) -> Self {
        use crate::elements::*;
        return AntiDipoleInversion::from_groups(
            // e423, e431, e412
            Simd32x3::from([self[e423], self[e431], self[e412]]) * Simd32x3::from(-1.0),
            // e415, e425, e435, e321
            Simd32x4::from([self[e415], self[e425], self[e435], self[e321]]) * Simd32x4::from(-1.0),
            // e235, e315, e125, e4
            Simd32x4::from([self[e235], self[e315], self[e125], self[e4]]) * Simd32x4::from(-1.0),
            // e1, e2, e3, e5
            Simd32x4::from([self[e1], self[e2], self[e3], self[e5]]) * Simd32x4::from(-1.0),
        );
    }
}
impl std::ops::Div<auto_morphism> for AntiDualNum {
    type Output = AntiDualNum;
    fn div(self, _rhs: auto_morphism) -> Self::Output {
        self.auto_morphism()
    }
}
impl std::ops::DivAssign<auto_morphism> for AntiDualNum {
    fn div_assign(&mut self, _rhs: auto_morphism) {
        *self = self.auto_morphism()
    }
}
impl AutoMorphism for AntiDualNum {
    fn auto_morphism(self) -> Self {
        return self;
    }
}
impl std::ops::Div<auto_morphism> for AntiFlatPoint {
    type Output = AntiFlatPoint;
    fn div(self, _rhs: auto_morphism) -> Self::Output {
        self.auto_morphism()
    }
}
impl std::ops::DivAssign<auto_morphism> for AntiFlatPoint {
    fn div_assign(&mut self, _rhs: auto_morphism) {
        *self = self.auto_morphism()
    }
}
impl AutoMorphism for AntiFlatPoint {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn auto_morphism(self) -> Self {
        use crate::elements::*;
        return AntiFlatPoint::from_groups(
            // e235, e315, e125, e321
            Simd32x4::from([self[e235], self[e315], self[e125], self[e321]]) * Simd32x4::from(-1.0),
        );
    }
}
impl std::ops::Div<auto_morphism> for AntiFlector {
    type Output = AntiFlector;
    fn div(self, _rhs: auto_morphism) -> Self::Output {
        self.auto_morphism()
    }
}
impl std::ops::DivAssign<auto_morphism> for AntiFlector {
    fn div_assign(&mut self, _rhs: auto_morphism) {
        *self = self.auto_morphism()
    }
}
impl AutoMorphism for AntiFlector {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        2        0
    // no simd        0        8        0
    fn auto_morphism(self) -> Self {
        use crate::elements::*;
        return AntiFlector::from_groups(
            // e235, e315, e125, e321
            Simd32x4::from([self[e235], self[e315], self[e125], self[e321]]) * Simd32x4::from(-1.0),
            // e1, e2, e3, e5
            Simd32x4::from([self[e1], self[e2], self[e3], self[e5]]) * Simd32x4::from(-1.0),
        );
    }
}
impl std::ops::Div<auto_morphism> for AntiLine {
    type Output = AntiLine;
    fn div(self, _rhs: auto_morphism) -> Self::Output {
        self.auto_morphism()
    }
}
impl std::ops::DivAssign<auto_morphism> for AntiLine {
    fn div_assign(&mut self, _rhs: auto_morphism) {
        *self = self.auto_morphism()
    }
}
impl AutoMorphism for AntiLine {
    fn auto_morphism(self) -> Self {
        return self;
    }
}
impl std::ops::Div<auto_morphism> for AntiMotor {
    type Output = AntiMotor;
    fn div(self, _rhs: auto_morphism) -> Self::Output {
        self.auto_morphism()
    }
}
impl std::ops::DivAssign<auto_morphism> for AntiMotor {
    fn div_assign(&mut self, _rhs: auto_morphism) {
        *self = self.auto_morphism()
    }
}
impl AutoMorphism for AntiMotor {
    fn auto_morphism(self) -> Self {
        return self;
    }
}
impl std::ops::Div<auto_morphism> for AntiPlane {
    type Output = AntiPlane;
    fn div(self, _rhs: auto_morphism) -> Self::Output {
        self.auto_morphism()
    }
}
impl std::ops::DivAssign<auto_morphism> for AntiPlane {
    fn div_assign(&mut self, _rhs: auto_morphism) {
        *self = self.auto_morphism()
    }
}
impl AutoMorphism for AntiPlane {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn auto_morphism(self) -> Self {
        use crate::elements::*;
        return AntiPlane::from_groups(/* e1, e2, e3, e5 */ Simd32x4::from([self[e1], self[e2], self[e3], self[e5]]) * Simd32x4::from(-1.0));
    }
}
impl std::ops::Div<auto_morphism> for AntiScalar {
    type Output = AntiScalar;
    fn div(self, _rhs: auto_morphism) -> Self::Output {
        self.auto_morphism()
    }
}
impl std::ops::DivAssign<auto_morphism> for AntiScalar {
    fn div_assign(&mut self, _rhs: auto_morphism) {
        *self = self.auto_morphism()
    }
}
impl AutoMorphism for AntiScalar {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn auto_morphism(self) -> Self {
        use crate::elements::*;
        return AntiScalar::from_groups(/* e12345 */ self[e12345] * -1.0);
    }
}
impl std::ops::Div<auto_morphism> for Circle {
    type Output = Circle;
    fn div(self, _rhs: auto_morphism) -> Self::Output {
        self.auto_morphism()
    }
}
impl std::ops::DivAssign<auto_morphism> for Circle {
    fn div_assign(&mut self, _rhs: auto_morphism) {
        *self = self.auto_morphism()
    }
}
impl AutoMorphism for Circle {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        0        2        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        3        0
    //  no simd        0       10        0
    fn auto_morphism(self) -> Self {
        use crate::elements::*;
        return Circle::from_groups(
            // e423, e431, e412
            Simd32x3::from([self[e423], self[e431], self[e412]]) * Simd32x3::from(-1.0),
            // e415, e425, e435, e321
            Simd32x4::from([self[e415], self[e425], self[e435], self[e321]]) * Simd32x4::from(-1.0),
            // e235, e315, e125
            Simd32x3::from([self[e235], self[e315], self[e125]]) * Simd32x3::from(-1.0),
        );
    }
}
impl std::ops::Div<auto_morphism> for CircleRotor {
    type Output = CircleRotor;
    fn div(self, _rhs: auto_morphism) -> Self::Output {
        self.auto_morphism()
    }
}
impl std::ops::DivAssign<auto_morphism> for CircleRotor {
    fn div_assign(&mut self, _rhs: auto_morphism) {
        *self = self.auto_morphism()
    }
}
impl AutoMorphism for CircleRotor {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        0        1        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        0        3        0
    //  no simd        0       11        0
    fn auto_morphism(self) -> Self {
        use crate::elements::*;
        return CircleRotor::from_groups(
            // e423, e431, e412
            Simd32x3::from([self[e423], self[e431], self[e412]]) * Simd32x3::from(-1.0),
            // e415, e425, e435, e321
            Simd32x4::from([self[e415], self[e425], self[e435], self[e321]]) * Simd32x4::from(-1.0),
            // e235, e315, e125, e12345
            Simd32x4::from([self[e235], self[e315], self[e125], self[e12345]]) * Simd32x4::from(-1.0),
        );
    }
}
impl std::ops::Div<auto_morphism> for Dipole {
    type Output = Dipole;
    fn div(self, _rhs: auto_morphism) -> Self::Output {
        self.auto_morphism()
    }
}
impl std::ops::DivAssign<auto_morphism> for Dipole {
    fn div_assign(&mut self, _rhs: auto_morphism) {
        *self = self.auto_morphism()
    }
}
impl AutoMorphism for Dipole {
    fn auto_morphism(self) -> Self {
        return self;
    }
}
impl std::ops::Div<auto_morphism> for DipoleInversion {
    type Output = DipoleInversion;
    fn div(self, _rhs: auto_morphism) -> Self::Output {
        self.auto_morphism()
    }
}
impl std::ops::DivAssign<auto_morphism> for DipoleInversion {
    fn div_assign(&mut self, _rhs: auto_morphism) {
        *self = self.auto_morphism()
    }
}
impl AutoMorphism for DipoleInversion {
    fn auto_morphism(self) -> Self {
        return self;
    }
}
impl std::ops::Div<auto_morphism> for DualNum {
    type Output = DualNum;
    fn div(self, _rhs: auto_morphism) -> Self::Output {
        self.auto_morphism()
    }
}
impl std::ops::DivAssign<auto_morphism> for DualNum {
    fn div_assign(&mut self, _rhs: auto_morphism) {
        *self = self.auto_morphism()
    }
}
impl AutoMorphism for DualNum {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd2        0        1        0
    // no simd        0        2        0
    fn auto_morphism(self) -> Self {
        use crate::elements::*;
        return DualNum::from_groups(/* e4, e12345 */ Simd32x2::from([self[e4], self[e12345]]) * Simd32x2::from(-1.0));
    }
}
impl std::ops::Div<auto_morphism> for FlatPoint {
    type Output = FlatPoint;
    fn div(self, _rhs: auto_morphism) -> Self::Output {
        self.auto_morphism()
    }
}
impl std::ops::DivAssign<auto_morphism> for FlatPoint {
    fn div_assign(&mut self, _rhs: auto_morphism) {
        *self = self.auto_morphism()
    }
}
impl AutoMorphism for FlatPoint {
    fn auto_morphism(self) -> Self {
        return self;
    }
}
impl std::ops::Div<auto_morphism> for Flector {
    type Output = Flector;
    fn div(self, _rhs: auto_morphism) -> Self::Output {
        self.auto_morphism()
    }
}
impl std::ops::DivAssign<auto_morphism> for Flector {
    fn div_assign(&mut self, _rhs: auto_morphism) {
        *self = self.auto_morphism()
    }
}
impl AutoMorphism for Flector {
    fn auto_morphism(self) -> Self {
        return self;
    }
}
impl std::ops::Div<auto_morphism> for Line {
    type Output = Line;
    fn div(self, _rhs: auto_morphism) -> Self::Output {
        self.auto_morphism()
    }
}
impl std::ops::DivAssign<auto_morphism> for Line {
    fn div_assign(&mut self, _rhs: auto_morphism) {
        *self = self.auto_morphism()
    }
}
impl AutoMorphism for Line {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        2        0
    // no simd        0        6        0
    fn auto_morphism(self) -> Self {
        use crate::elements::*;
        return Line::from_groups(
            // e415, e425, e435
            Simd32x3::from([self[e415], self[e425], self[e435]]) * Simd32x3::from(-1.0),
            // e235, e315, e125
            Simd32x3::from([self[e235], self[e315], self[e125]]) * Simd32x3::from(-1.0),
        );
    }
}
impl std::ops::Div<auto_morphism> for Motor {
    type Output = Motor;
    fn div(self, _rhs: auto_morphism) -> Self::Output {
        self.auto_morphism()
    }
}
impl std::ops::DivAssign<auto_morphism> for Motor {
    fn div_assign(&mut self, _rhs: auto_morphism) {
        *self = self.auto_morphism()
    }
}
impl AutoMorphism for Motor {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        2        0
    // no simd        0        8        0
    fn auto_morphism(self) -> Self {
        use crate::elements::*;
        return Motor::from_groups(
            // e415, e425, e435, e12345
            Simd32x4::from([self[e415], self[e425], self[e435], self[e12345]]) * Simd32x4::from(-1.0),
            // e235, e315, e125, e5
            Simd32x4::from([self[e235], self[e315], self[e125], self[e5]]) * Simd32x4::from(-1.0),
        );
    }
}
impl std::ops::Div<auto_morphism> for MultiVector {
    type Output = MultiVector;
    fn div(self, _rhs: auto_morphism) -> Self::Output {
        self.auto_morphism()
    }
}
impl std::ops::DivAssign<auto_morphism> for MultiVector {
    fn div_assign(&mut self, _rhs: auto_morphism) {
        *self = self.auto_morphism()
    }
}
impl AutoMorphism for MultiVector {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        2        0
    //    simd3        0        2        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        0        6        0
    //  no simd        0       16        0
    fn auto_morphism(self) -> Self {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self[scalar], self[e12345] * -1.0]),
            // e1, e2, e3, e4
            Simd32x4::from([self[e1], self[e2], self[e3], self[e4]]) * Simd32x4::from(-1.0),
            // e5
            self[e5] * -1.0,
            // e15, e25, e35, e45
            self.group3(),
            // e41, e42, e43
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            Simd32x4::from([self[e415], self[e425], self[e435], self[e321]]) * Simd32x4::from(-1.0),
            // e423, e431, e412
            Simd32x3::from([self[e423], self[e431], self[e412]]) * Simd32x3::from(-1.0),
            // e235, e315, e125
            Simd32x3::from([self[e235], self[e315], self[e125]]) * Simd32x3::from(-1.0),
            // e4235, e4315, e4125, e3215
            self.group9(),
            // e1234
            self[e1234],
        );
    }
}
impl std::ops::Div<auto_morphism> for Plane {
    type Output = Plane;
    fn div(self, _rhs: auto_morphism) -> Self::Output {
        self.auto_morphism()
    }
}
impl std::ops::DivAssign<auto_morphism> for Plane {
    fn div_assign(&mut self, _rhs: auto_morphism) {
        *self = self.auto_morphism()
    }
}
impl AutoMorphism for Plane {
    fn auto_morphism(self) -> Self {
        return self;
    }
}
impl std::ops::Div<auto_morphism> for RoundPoint {
    type Output = RoundPoint;
    fn div(self, _rhs: auto_morphism) -> Self::Output {
        self.auto_morphism()
    }
}
impl std::ops::DivAssign<auto_morphism> for RoundPoint {
    fn div_assign(&mut self, _rhs: auto_morphism) {
        *self = self.auto_morphism()
    }
}
impl AutoMorphism for RoundPoint {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        2        0
    //  no simd        0        5        0
    fn auto_morphism(self) -> Self {
        use crate::elements::*;
        return RoundPoint::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from([self[e1], self[e2], self[e3], self[e4]]) * Simd32x4::from(-1.0),
            // e5
            self[e5] * -1.0,
        );
    }
}
impl std::ops::Div<auto_morphism> for Scalar {
    type Output = Scalar;
    fn div(self, _rhs: auto_morphism) -> Self::Output {
        self.auto_morphism()
    }
}
impl std::ops::DivAssign<auto_morphism> for Scalar {
    fn div_assign(&mut self, _rhs: auto_morphism) {
        *self = self.auto_morphism()
    }
}
impl AutoMorphism for Scalar {
    fn auto_morphism(self) -> Self {
        return self;
    }
}
impl std::ops::Div<auto_morphism> for Sphere {
    type Output = Sphere;
    fn div(self, _rhs: auto_morphism) -> Self::Output {
        self.auto_morphism()
    }
}
impl std::ops::DivAssign<auto_morphism> for Sphere {
    fn div_assign(&mut self, _rhs: auto_morphism) {
        *self = self.auto_morphism()
    }
}
impl AutoMorphism for Sphere {
    fn auto_morphism(self) -> Self {
        return self;
    }
}
impl std::ops::Div<auto_morphism> for VersorEven {
    type Output = VersorEven;
    fn div(self, _rhs: auto_morphism) -> Self::Output {
        self.auto_morphism()
    }
}
impl std::ops::DivAssign<auto_morphism> for VersorEven {
    fn div_assign(&mut self, _rhs: auto_morphism) {
        *self = self.auto_morphism()
    }
}
impl AutoMorphism for VersorEven {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        4        0
    // no simd        0       16        0
    fn auto_morphism(self) -> Self {
        use crate::elements::*;
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([self[e423], self[e431], self[e412], self[e12345]]) * Simd32x4::from(-1.0),
            // e415, e425, e435, e321
            Simd32x4::from([self[e415], self[e425], self[e435], self[e321]]) * Simd32x4::from(-1.0),
            // e235, e315, e125, e5
            Simd32x4::from([self[e235], self[e315], self[e125], self[e5]]) * Simd32x4::from(-1.0),
            // e1, e2, e3, e4
            Simd32x4::from([self[e1], self[e2], self[e3], self[e4]]) * Simd32x4::from(-1.0),
        );
    }
}
impl std::ops::Div<auto_morphism> for VersorOdd {
    type Output = VersorOdd;
    fn div(self, _rhs: auto_morphism) -> Self::Output {
        self.auto_morphism()
    }
}
impl std::ops::DivAssign<auto_morphism> for VersorOdd {
    fn div_assign(&mut self, _rhs: auto_morphism) {
        *self = self.auto_morphism()
    }
}
impl AutoMorphism for VersorOdd {
    fn auto_morphism(self) -> Self {
        return self;
    }
}
