// Note on Operative Statistics:
// Operative Statistics are not a precise predictor of performance or performance comparisons.
// This is due to varying hardware capabilities and compiler optimizations.
// As always, where performance is a concern, there is no substitute for
// real measurements on real work-loads on real hardware.
// Disclaimer aside, enjoy the fun information =)
//
// Total Implementations: 2
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
impl std::ops::Div<anti_constraint_valid> for Horizon {
    type Output = Horizon;
    fn div(self, _rhs: anti_constraint_valid) -> Self::Output {
        self.anti_constraint_valid()
    }
}
impl std::ops::DivAssign<anti_constraint_valid> for Horizon {
    fn div_assign(&mut self, _rhs: anti_constraint_valid) {
        *self = self.anti_constraint_valid()
    }
}
impl AntiConstraintValid for Horizon {
    fn anti_constraint_valid(self) -> Self {
        return self;
    }
}
impl std::ops::Div<anti_constraint_valid> for Scalar {
    type Output = Scalar;
    fn div(self, _rhs: anti_constraint_valid) -> Self::Output {
        self.anti_constraint_valid()
    }
}
impl std::ops::DivAssign<anti_constraint_valid> for Scalar {
    fn div_assign(&mut self, _rhs: anti_constraint_valid) {
        *self = self.anti_constraint_valid()
    }
}
impl AntiConstraintValid for Scalar {
    fn anti_constraint_valid(self) -> Self {
        return self;
    }
}
