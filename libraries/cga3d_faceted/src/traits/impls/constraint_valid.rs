// Note on Operative Statistics:
// Operative Statistics are not a precise predictor of performance or performance comparisons.
// This is due to varying hardware capabilities and compiler optimizations.
// As always, where performance is a concern, there is no substitute for
// real measurements on real work-loads on real hardware.
// Disclaimer aside, enjoy the fun information =)
//
// Total Implementations: 12
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
impl std::ops::Div<constraint_valid> for FlatPointAtInfinity {
    type Output = FlatPointAtInfinity;
    fn div(self, _rhs: constraint_valid) -> Self::Output {
        self.constraint_valid()
    }
}
impl std::ops::DivAssign<constraint_valid> for FlatPointAtInfinity {
    fn div_assign(&mut self, _rhs: constraint_valid) {
        *self = self.constraint_valid()
    }
}
impl ConstraintValid for FlatPointAtInfinity {
    fn constraint_valid(self) -> Self {
        return self;
    }
}
impl std::ops::Div<constraint_valid> for FlectorAtInfinity {
    type Output = FlectorAtInfinity;
    fn div(self, _rhs: constraint_valid) -> Self::Output {
        self.constraint_valid()
    }
}
impl std::ops::DivAssign<constraint_valid> for FlectorAtInfinity {
    fn div_assign(&mut self, _rhs: constraint_valid) {
        *self = self.constraint_valid()
    }
}
impl ConstraintValid for FlectorAtInfinity {
    fn constraint_valid(self) -> Self {
        return self;
    }
}
impl std::ops::Div<constraint_valid> for Horizon {
    type Output = Horizon;
    fn div(self, _rhs: constraint_valid) -> Self::Output {
        self.constraint_valid()
    }
}
impl std::ops::DivAssign<constraint_valid> for Horizon {
    fn div_assign(&mut self, _rhs: constraint_valid) {
        *self = self.constraint_valid()
    }
}
impl ConstraintValid for Horizon {
    fn constraint_valid(self) -> Self {
        return self;
    }
}
impl std::ops::Div<constraint_valid> for Infinity {
    type Output = Infinity;
    fn div(self, _rhs: constraint_valid) -> Self::Output {
        self.constraint_valid()
    }
}
impl std::ops::DivAssign<constraint_valid> for Infinity {
    fn div_assign(&mut self, _rhs: constraint_valid) {
        *self = self.constraint_valid()
    }
}
impl ConstraintValid for Infinity {
    fn constraint_valid(self) -> Self {
        return self;
    }
}
impl std::ops::Div<constraint_valid> for LineAtInfinity {
    type Output = LineAtInfinity;
    fn div(self, _rhs: constraint_valid) -> Self::Output {
        self.constraint_valid()
    }
}
impl std::ops::DivAssign<constraint_valid> for LineAtInfinity {
    fn div_assign(&mut self, _rhs: constraint_valid) {
        *self = self.constraint_valid()
    }
}
impl ConstraintValid for LineAtInfinity {
    fn constraint_valid(self) -> Self {
        return self;
    }
}
impl std::ops::Div<constraint_valid> for MotorAtInfinity {
    type Output = MotorAtInfinity;
    fn div(self, _rhs: constraint_valid) -> Self::Output {
        self.constraint_valid()
    }
}
impl std::ops::DivAssign<constraint_valid> for MotorAtInfinity {
    fn div_assign(&mut self, _rhs: constraint_valid) {
        *self = self.constraint_valid()
    }
}
impl ConstraintValid for MotorAtInfinity {
    fn constraint_valid(self) -> Self {
        return self;
    }
}
impl std::ops::Div<constraint_valid> for NullCircleAtOrigin {
    type Output = NullCircleAtOrigin;
    fn div(self, _rhs: constraint_valid) -> Self::Output {
        self.constraint_valid()
    }
}
impl std::ops::DivAssign<constraint_valid> for NullCircleAtOrigin {
    fn div_assign(&mut self, _rhs: constraint_valid) {
        *self = self.constraint_valid()
    }
}
impl ConstraintValid for NullCircleAtOrigin {
    fn constraint_valid(self) -> Self {
        return self;
    }
}
impl std::ops::Div<constraint_valid> for NullDipoleAtOrigin {
    type Output = NullDipoleAtOrigin;
    fn div(self, _rhs: constraint_valid) -> Self::Output {
        self.constraint_valid()
    }
}
impl std::ops::DivAssign<constraint_valid> for NullDipoleAtOrigin {
    fn div_assign(&mut self, _rhs: constraint_valid) {
        *self = self.constraint_valid()
    }
}
impl ConstraintValid for NullDipoleAtOrigin {
    fn constraint_valid(self) -> Self {
        return self;
    }
}
impl std::ops::Div<constraint_valid> for NullDipoleInversionAtOrigin {
    type Output = NullDipoleInversionAtOrigin;
    fn div(self, _rhs: constraint_valid) -> Self::Output {
        self.constraint_valid()
    }
}
impl std::ops::DivAssign<constraint_valid> for NullDipoleInversionAtOrigin {
    fn div_assign(&mut self, _rhs: constraint_valid) {
        *self = self.constraint_valid()
    }
}
impl ConstraintValid for NullDipoleInversionAtOrigin {
    fn constraint_valid(self) -> Self {
        return self;
    }
}
impl std::ops::Div<constraint_valid> for NullSphereAtOrigin {
    type Output = NullSphereAtOrigin;
    fn div(self, _rhs: constraint_valid) -> Self::Output {
        self.constraint_valid()
    }
}
impl std::ops::DivAssign<constraint_valid> for NullSphereAtOrigin {
    fn div_assign(&mut self, _rhs: constraint_valid) {
        *self = self.constraint_valid()
    }
}
impl ConstraintValid for NullSphereAtOrigin {
    fn constraint_valid(self) -> Self {
        return self;
    }
}
impl std::ops::Div<constraint_valid> for NullVersorEvenAtOrigin {
    type Output = NullVersorEvenAtOrigin;
    fn div(self, _rhs: constraint_valid) -> Self::Output {
        self.constraint_valid()
    }
}
impl std::ops::DivAssign<constraint_valid> for NullVersorEvenAtOrigin {
    fn div_assign(&mut self, _rhs: constraint_valid) {
        *self = self.constraint_valid()
    }
}
impl ConstraintValid for NullVersorEvenAtOrigin {
    fn constraint_valid(self) -> Self {
        return self;
    }
}
impl std::ops::Div<constraint_valid> for Origin {
    type Output = Origin;
    fn div(self, _rhs: constraint_valid) -> Self::Output {
        self.constraint_valid()
    }
}
impl std::ops::DivAssign<constraint_valid> for Origin {
    fn div_assign(&mut self, _rhs: constraint_valid) {
        *self = self.constraint_valid()
    }
}
impl ConstraintValid for Origin {
    fn constraint_valid(self) -> Self {
        return self;
    }
}
