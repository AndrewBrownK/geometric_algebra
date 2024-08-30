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
impl AntiConstraintValid for FlatPointAtInfinity {
    fn anti_constraint_valid(self) -> Self {
        return self;
    }
}
impl AntiConstraintValid for FlectorAtInfinity {
    fn anti_constraint_valid(self) -> Self {
        return self;
    }
}
impl AntiConstraintValid for Horizon {
    fn anti_constraint_valid(self) -> Self {
        return self;
    }
}
impl AntiConstraintValid for Infinity {
    fn anti_constraint_valid(self) -> Self {
        return self;
    }
}
impl AntiConstraintValid for LineAtInfinity {
    fn anti_constraint_valid(self) -> Self {
        return self;
    }
}
impl AntiConstraintValid for MotorAtInfinity {
    fn anti_constraint_valid(self) -> Self {
        return self;
    }
}
impl AntiConstraintValid for NullCircleAtOrigin {
    fn anti_constraint_valid(self) -> Self {
        return self;
    }
}
impl AntiConstraintValid for NullDipoleAtOrigin {
    fn anti_constraint_valid(self) -> Self {
        return self;
    }
}
impl AntiConstraintValid for NullDipoleInversionAtOrigin {
    fn anti_constraint_valid(self) -> Self {
        return self;
    }
}
impl AntiConstraintValid for NullSphereAtOrigin {
    fn anti_constraint_valid(self) -> Self {
        return self;
    }
}
impl AntiConstraintValid for NullVersorEvenAtOrigin {
    fn anti_constraint_valid(self) -> Self {
        return self;
    }
}
impl AntiConstraintValid for Origin {
    fn anti_constraint_valid(self) -> Self {
        return self;
    }
}
