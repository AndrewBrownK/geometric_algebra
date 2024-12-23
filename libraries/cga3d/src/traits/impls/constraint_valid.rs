// Note on Operative Statistics:
// Operative Statistics are not a precise predictor of performance or performance comparisons.
// This is due to varying hardware capabilities and compiler optimizations.
// As always, where performance is a concern, there is no substitute for
// real measurements on real work-loads on real hardware.
// Disclaimer aside, enjoy the fun information =)
//
// Total Implementations: 8
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
impl std::ops::Div<constraint_valid> for AntiFlatPoint {
    type Output = AntiFlatPoint;
    fn div(self, _rhs: constraint_valid) -> Self::Output {
        self.constraint_valid()
    }
}
impl std::ops::DivAssign<constraint_valid> for AntiFlatPoint {
    fn div_assign(&mut self, _rhs: constraint_valid) {
        *self = self.constraint_valid()
    }
}
impl ConstraintValid for AntiFlatPoint {
    fn constraint_valid(self) -> Self {
        return self;
    }
}
impl std::ops::Div<constraint_valid> for AntiPlane {
    type Output = AntiPlane;
    fn div(self, _rhs: constraint_valid) -> Self::Output {
        self.constraint_valid()
    }
}
impl std::ops::DivAssign<constraint_valid> for AntiPlane {
    fn div_assign(&mut self, _rhs: constraint_valid) {
        *self = self.constraint_valid()
    }
}
impl ConstraintValid for AntiPlane {
    fn constraint_valid(self) -> Self {
        return self;
    }
}
impl std::ops::Div<constraint_valid> for AntiScalar {
    type Output = AntiScalar;
    fn div(self, _rhs: constraint_valid) -> Self::Output {
        self.constraint_valid()
    }
}
impl std::ops::DivAssign<constraint_valid> for AntiScalar {
    fn div_assign(&mut self, _rhs: constraint_valid) {
        *self = self.constraint_valid()
    }
}
impl ConstraintValid for AntiScalar {
    fn constraint_valid(self) -> Self {
        return self;
    }
}
impl std::ops::Div<constraint_valid> for FlatPoint {
    type Output = FlatPoint;
    fn div(self, _rhs: constraint_valid) -> Self::Output {
        self.constraint_valid()
    }
}
impl std::ops::DivAssign<constraint_valid> for FlatPoint {
    fn div_assign(&mut self, _rhs: constraint_valid) {
        *self = self.constraint_valid()
    }
}
impl ConstraintValid for FlatPoint {
    fn constraint_valid(self) -> Self {
        return self;
    }
}
impl std::ops::Div<constraint_valid> for Plane {
    type Output = Plane;
    fn div(self, _rhs: constraint_valid) -> Self::Output {
        self.constraint_valid()
    }
}
impl std::ops::DivAssign<constraint_valid> for Plane {
    fn div_assign(&mut self, _rhs: constraint_valid) {
        *self = self.constraint_valid()
    }
}
impl ConstraintValid for Plane {
    fn constraint_valid(self) -> Self {
        return self;
    }
}
impl std::ops::Div<constraint_valid> for RoundPoint {
    type Output = RoundPoint;
    fn div(self, _rhs: constraint_valid) -> Self::Output {
        self.constraint_valid()
    }
}
impl std::ops::DivAssign<constraint_valid> for RoundPoint {
    fn div_assign(&mut self, _rhs: constraint_valid) {
        *self = self.constraint_valid()
    }
}
impl ConstraintValid for RoundPoint {
    fn constraint_valid(self) -> Self {
        return self;
    }
}
impl std::ops::Div<constraint_valid> for Scalar {
    type Output = Scalar;
    fn div(self, _rhs: constraint_valid) -> Self::Output {
        self.constraint_valid()
    }
}
impl std::ops::DivAssign<constraint_valid> for Scalar {
    fn div_assign(&mut self, _rhs: constraint_valid) {
        *self = self.constraint_valid()
    }
}
impl ConstraintValid for Scalar {
    fn constraint_valid(self) -> Self {
        return self;
    }
}
impl std::ops::Div<constraint_valid> for Sphere {
    type Output = Sphere;
    fn div(self, _rhs: constraint_valid) -> Self::Output {
        self.constraint_valid()
    }
}
impl std::ops::DivAssign<constraint_valid> for Sphere {
    fn div_assign(&mut self, _rhs: constraint_valid) {
        *self = self.constraint_valid()
    }
}
impl ConstraintValid for Sphere {
    fn constraint_valid(self) -> Self {
        return self;
    }
}
