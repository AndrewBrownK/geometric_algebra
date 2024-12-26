extension AntiScalar {
    func div(_rhs: constraint_valid) -> 
AntiScalar {
        this.constraint_valid()
    }
}
extension AntiScalar {
    func div=(const constraint_valid& _rhs) -> AntiScalar& {
        this = this.constraint_valid()
        return *this;
    }
}
extension AntiScalar: ConstraintValid {
    func constraint_valid(self) -> AntiScalar {
        return self;
    }
}
extension Horizon {
    func div(_rhs: constraint_valid) -> 
Horizon {
        this.constraint_valid()
    }
}
extension Horizon {
    func div=(const constraint_valid& _rhs) -> Horizon& {
        this = this.constraint_valid()
        return *this;
    }
}
extension Horizon: ConstraintValid {
    func constraint_valid(self) -> Horizon {
        return self;
    }
}
extension Origin {
    func div(_rhs: constraint_valid) -> 
Origin {
        this.constraint_valid()
    }
}
extension Origin {
    func div=(const constraint_valid& _rhs) -> Origin& {
        this = this.constraint_valid()
        return *this;
    }
}
extension Origin: ConstraintValid {
    func constraint_valid(self) -> Origin {
        return self;
    }
}
extension Plane {
    func div(_rhs: constraint_valid) -> 
Plane {
        this.constraint_valid()
    }
}
extension Plane {
    func div=(const constraint_valid& _rhs) -> Plane& {
        this = this.constraint_valid()
        return *this;
    }
}
extension Plane: ConstraintValid {
    func constraint_valid(self) -> Plane {
        return self;
    }
}
extension Point {
    func div(_rhs: constraint_valid) -> 
Point {
        this.constraint_valid()
    }
}
extension Point {
    func div=(const constraint_valid& _rhs) -> Point& {
        this = this.constraint_valid()
        return *this;
    }
}
extension Point: ConstraintValid {
    func constraint_valid(self) -> Point {
        return self;
    }
}
extension Scalar {
    func div(_rhs: constraint_valid) -> 
Scalar {
        this.constraint_valid()
    }
}
extension Scalar {
    func div=(const constraint_valid& _rhs) -> Scalar& {
        this = this.constraint_valid()
        return *this;
    }
}
extension Scalar: ConstraintValid {
    func constraint_valid(self) -> Scalar {
        return self;
    }
}
