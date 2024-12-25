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
impl std::ops::Div<constraint_valid> for Point {
    type Output = Point;
    fn div(self, _rhs: constraint_valid) -> Self::Output {
        self.constraint_valid()
    }
}
impl std::ops::DivAssign<constraint_valid> for Point {
    fn div_assign(&mut self, _rhs: constraint_valid) {
        *self = self.constraint_valid()
    }
}
impl ConstraintValid for Point {
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
