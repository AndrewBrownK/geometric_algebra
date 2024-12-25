impl std::ops::Div<anti_constraint_valid> for AntiScalar {
    type Output = AntiScalar;
    fn div(self, _rhs: anti_constraint_valid) -> Self::Output {
        self.anti_constraint_valid()
    }
}
impl std::ops::DivAssign<anti_constraint_valid> for AntiScalar {
    fn div_assign(&mut self, _rhs: anti_constraint_valid) {
        *self = self.anti_constraint_valid()
    }
}
impl AntiConstraintValid for AntiScalar {
    fn anti_constraint_valid(self) -> Self {
        return self;
    }
}
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
impl std::ops::Div<anti_constraint_valid> for Origin {
    type Output = Origin;
    fn div(self, _rhs: anti_constraint_valid) -> Self::Output {
        self.anti_constraint_valid()
    }
}
impl std::ops::DivAssign<anti_constraint_valid> for Origin {
    fn div_assign(&mut self, _rhs: anti_constraint_valid) {
        *self = self.anti_constraint_valid()
    }
}
impl AntiConstraintValid for Origin {
    fn anti_constraint_valid(self) -> Self {
        return self;
    }
}
impl std::ops::Div<anti_constraint_valid> for Plane {
    type Output = Plane;
    fn div(self, _rhs: anti_constraint_valid) -> Self::Output {
        self.anti_constraint_valid()
    }
}
impl std::ops::DivAssign<anti_constraint_valid> for Plane {
    fn div_assign(&mut self, _rhs: anti_constraint_valid) {
        *self = self.anti_constraint_valid()
    }
}
impl AntiConstraintValid for Plane {
    fn anti_constraint_valid(self) -> Self {
        return self;
    }
}
impl std::ops::Div<anti_constraint_valid> for Point {
    type Output = Point;
    fn div(self, _rhs: anti_constraint_valid) -> Self::Output {
        self.anti_constraint_valid()
    }
}
impl std::ops::DivAssign<anti_constraint_valid> for Point {
    fn div_assign(&mut self, _rhs: anti_constraint_valid) {
        *self = self.anti_constraint_valid()
    }
}
impl AntiConstraintValid for Point {
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
