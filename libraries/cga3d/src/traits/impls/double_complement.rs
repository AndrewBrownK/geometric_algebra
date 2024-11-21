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
//   Median:         0       0       0
//  Average:         0       0       0
//  Maximum:         0       0       0
//
//  No SIMD:   add/sub     mul     div
//  Minimum:         0       0       0
//   Median:         0       0       0
//  Average:         0       0       0
//  Maximum:         0       0       0
impl std::ops::Div<double_complement> for AntiCircleRotor {
    type Output = AntiCircleRotor;
    fn div(self, _rhs: double_complement) -> Self::Output {
        self.double_complement()
    }
}
impl std::ops::DivAssign<double_complement> for AntiCircleRotor {
    fn div_assign(&mut self, _rhs: double_complement) {
        *self = self.double_complement()
    }
}
impl DoubleComplement for AntiCircleRotor {
    fn double_complement(self) -> Self {
        return self;
    }
}
impl std::ops::Div<double_complement> for AntiDipoleInversion {
    type Output = AntiDipoleInversion;
    fn div(self, _rhs: double_complement) -> Self::Output {
        self.double_complement()
    }
}
impl std::ops::DivAssign<double_complement> for AntiDipoleInversion {
    fn div_assign(&mut self, _rhs: double_complement) {
        *self = self.double_complement()
    }
}
impl DoubleComplement for AntiDipoleInversion {
    fn double_complement(self) -> Self {
        return self;
    }
}
impl std::ops::Div<double_complement> for AntiDualNum {
    type Output = AntiDualNum;
    fn div(self, _rhs: double_complement) -> Self::Output {
        self.double_complement()
    }
}
impl std::ops::DivAssign<double_complement> for AntiDualNum {
    fn div_assign(&mut self, _rhs: double_complement) {
        *self = self.double_complement()
    }
}
impl DoubleComplement for AntiDualNum {
    fn double_complement(self) -> Self {
        return self;
    }
}
impl std::ops::Div<double_complement> for AntiFlatPoint {
    type Output = AntiFlatPoint;
    fn div(self, _rhs: double_complement) -> Self::Output {
        self.double_complement()
    }
}
impl std::ops::DivAssign<double_complement> for AntiFlatPoint {
    fn div_assign(&mut self, _rhs: double_complement) {
        *self = self.double_complement()
    }
}
impl DoubleComplement for AntiFlatPoint {
    fn double_complement(self) -> Self {
        return self;
    }
}
impl std::ops::Div<double_complement> for AntiFlector {
    type Output = AntiFlector;
    fn div(self, _rhs: double_complement) -> Self::Output {
        self.double_complement()
    }
}
impl std::ops::DivAssign<double_complement> for AntiFlector {
    fn div_assign(&mut self, _rhs: double_complement) {
        *self = self.double_complement()
    }
}
impl DoubleComplement for AntiFlector {
    fn double_complement(self) -> Self {
        return self;
    }
}
impl std::ops::Div<double_complement> for AntiLine {
    type Output = AntiLine;
    fn div(self, _rhs: double_complement) -> Self::Output {
        self.double_complement()
    }
}
impl std::ops::DivAssign<double_complement> for AntiLine {
    fn div_assign(&mut self, _rhs: double_complement) {
        *self = self.double_complement()
    }
}
impl DoubleComplement for AntiLine {
    fn double_complement(self) -> Self {
        return self;
    }
}
impl std::ops::Div<double_complement> for AntiMotor {
    type Output = AntiMotor;
    fn div(self, _rhs: double_complement) -> Self::Output {
        self.double_complement()
    }
}
impl std::ops::DivAssign<double_complement> for AntiMotor {
    fn div_assign(&mut self, _rhs: double_complement) {
        *self = self.double_complement()
    }
}
impl DoubleComplement for AntiMotor {
    fn double_complement(self) -> Self {
        return self;
    }
}
impl std::ops::Div<double_complement> for AntiPlane {
    type Output = AntiPlane;
    fn div(self, _rhs: double_complement) -> Self::Output {
        self.double_complement()
    }
}
impl std::ops::DivAssign<double_complement> for AntiPlane {
    fn div_assign(&mut self, _rhs: double_complement) {
        *self = self.double_complement()
    }
}
impl DoubleComplement for AntiPlane {
    fn double_complement(self) -> Self {
        return self;
    }
}
impl std::ops::Div<double_complement> for AntiScalar {
    type Output = AntiScalar;
    fn div(self, _rhs: double_complement) -> Self::Output {
        self.double_complement()
    }
}
impl std::ops::DivAssign<double_complement> for AntiScalar {
    fn div_assign(&mut self, _rhs: double_complement) {
        *self = self.double_complement()
    }
}
impl DoubleComplement for AntiScalar {
    fn double_complement(self) -> Self {
        return self;
    }
}
impl std::ops::Div<double_complement> for Circle {
    type Output = Circle;
    fn div(self, _rhs: double_complement) -> Self::Output {
        self.double_complement()
    }
}
impl std::ops::DivAssign<double_complement> for Circle {
    fn div_assign(&mut self, _rhs: double_complement) {
        *self = self.double_complement()
    }
}
impl DoubleComplement for Circle {
    fn double_complement(self) -> Self {
        return self;
    }
}
impl std::ops::Div<double_complement> for CircleRotor {
    type Output = CircleRotor;
    fn div(self, _rhs: double_complement) -> Self::Output {
        self.double_complement()
    }
}
impl std::ops::DivAssign<double_complement> for CircleRotor {
    fn div_assign(&mut self, _rhs: double_complement) {
        *self = self.double_complement()
    }
}
impl DoubleComplement for CircleRotor {
    fn double_complement(self) -> Self {
        return self;
    }
}
impl std::ops::Div<double_complement> for Dipole {
    type Output = Dipole;
    fn div(self, _rhs: double_complement) -> Self::Output {
        self.double_complement()
    }
}
impl std::ops::DivAssign<double_complement> for Dipole {
    fn div_assign(&mut self, _rhs: double_complement) {
        *self = self.double_complement()
    }
}
impl DoubleComplement for Dipole {
    fn double_complement(self) -> Self {
        return self;
    }
}
impl std::ops::Div<double_complement> for DipoleInversion {
    type Output = DipoleInversion;
    fn div(self, _rhs: double_complement) -> Self::Output {
        self.double_complement()
    }
}
impl std::ops::DivAssign<double_complement> for DipoleInversion {
    fn div_assign(&mut self, _rhs: double_complement) {
        *self = self.double_complement()
    }
}
impl DoubleComplement for DipoleInversion {
    fn double_complement(self) -> Self {
        return self;
    }
}
impl std::ops::Div<double_complement> for DualNum {
    type Output = DualNum;
    fn div(self, _rhs: double_complement) -> Self::Output {
        self.double_complement()
    }
}
impl std::ops::DivAssign<double_complement> for DualNum {
    fn div_assign(&mut self, _rhs: double_complement) {
        *self = self.double_complement()
    }
}
impl DoubleComplement for DualNum {
    fn double_complement(self) -> Self {
        return self;
    }
}
impl std::ops::Div<double_complement> for FlatPoint {
    type Output = FlatPoint;
    fn div(self, _rhs: double_complement) -> Self::Output {
        self.double_complement()
    }
}
impl std::ops::DivAssign<double_complement> for FlatPoint {
    fn div_assign(&mut self, _rhs: double_complement) {
        *self = self.double_complement()
    }
}
impl DoubleComplement for FlatPoint {
    fn double_complement(self) -> Self {
        return self;
    }
}
impl std::ops::Div<double_complement> for Flector {
    type Output = Flector;
    fn div(self, _rhs: double_complement) -> Self::Output {
        self.double_complement()
    }
}
impl std::ops::DivAssign<double_complement> for Flector {
    fn div_assign(&mut self, _rhs: double_complement) {
        *self = self.double_complement()
    }
}
impl DoubleComplement for Flector {
    fn double_complement(self) -> Self {
        return self;
    }
}
impl std::ops::Div<double_complement> for Line {
    type Output = Line;
    fn div(self, _rhs: double_complement) -> Self::Output {
        self.double_complement()
    }
}
impl std::ops::DivAssign<double_complement> for Line {
    fn div_assign(&mut self, _rhs: double_complement) {
        *self = self.double_complement()
    }
}
impl DoubleComplement for Line {
    fn double_complement(self) -> Self {
        return self;
    }
}
impl std::ops::Div<double_complement> for Motor {
    type Output = Motor;
    fn div(self, _rhs: double_complement) -> Self::Output {
        self.double_complement()
    }
}
impl std::ops::DivAssign<double_complement> for Motor {
    fn div_assign(&mut self, _rhs: double_complement) {
        *self = self.double_complement()
    }
}
impl DoubleComplement for Motor {
    fn double_complement(self) -> Self {
        return self;
    }
}
impl std::ops::Div<double_complement> for MultiVector {
    type Output = MultiVector;
    fn div(self, _rhs: double_complement) -> Self::Output {
        self.double_complement()
    }
}
impl std::ops::DivAssign<double_complement> for MultiVector {
    fn div_assign(&mut self, _rhs: double_complement) {
        *self = self.double_complement()
    }
}
impl DoubleComplement for MultiVector {
    fn double_complement(self) -> Self {
        return self;
    }
}
impl std::ops::Div<double_complement> for Plane {
    type Output = Plane;
    fn div(self, _rhs: double_complement) -> Self::Output {
        self.double_complement()
    }
}
impl std::ops::DivAssign<double_complement> for Plane {
    fn div_assign(&mut self, _rhs: double_complement) {
        *self = self.double_complement()
    }
}
impl DoubleComplement for Plane {
    fn double_complement(self) -> Self {
        return self;
    }
}
impl std::ops::Div<double_complement> for RoundPoint {
    type Output = RoundPoint;
    fn div(self, _rhs: double_complement) -> Self::Output {
        self.double_complement()
    }
}
impl std::ops::DivAssign<double_complement> for RoundPoint {
    fn div_assign(&mut self, _rhs: double_complement) {
        *self = self.double_complement()
    }
}
impl DoubleComplement for RoundPoint {
    fn double_complement(self) -> Self {
        return self;
    }
}
impl std::ops::Div<double_complement> for Scalar {
    type Output = Scalar;
    fn div(self, _rhs: double_complement) -> Self::Output {
        self.double_complement()
    }
}
impl std::ops::DivAssign<double_complement> for Scalar {
    fn div_assign(&mut self, _rhs: double_complement) {
        *self = self.double_complement()
    }
}
impl DoubleComplement for Scalar {
    fn double_complement(self) -> Self {
        return self;
    }
}
impl std::ops::Div<double_complement> for Sphere {
    type Output = Sphere;
    fn div(self, _rhs: double_complement) -> Self::Output {
        self.double_complement()
    }
}
impl std::ops::DivAssign<double_complement> for Sphere {
    fn div_assign(&mut self, _rhs: double_complement) {
        *self = self.double_complement()
    }
}
impl DoubleComplement for Sphere {
    fn double_complement(self) -> Self {
        return self;
    }
}
impl std::ops::Div<double_complement> for VersorEven {
    type Output = VersorEven;
    fn div(self, _rhs: double_complement) -> Self::Output {
        self.double_complement()
    }
}
impl std::ops::DivAssign<double_complement> for VersorEven {
    fn div_assign(&mut self, _rhs: double_complement) {
        *self = self.double_complement()
    }
}
impl DoubleComplement for VersorEven {
    fn double_complement(self) -> Self {
        return self;
    }
}
impl std::ops::Div<double_complement> for VersorOdd {
    type Output = VersorOdd;
    fn div(self, _rhs: double_complement) -> Self::Output {
        self.double_complement()
    }
}
impl std::ops::DivAssign<double_complement> for VersorOdd {
    fn div_assign(&mut self, _rhs: double_complement) {
        *self = self.double_complement()
    }
}
impl DoubleComplement for VersorOdd {
    fn double_complement(self) -> Self {
        return self;
    }
}
