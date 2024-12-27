extension AntiScalar {
    // Fancy infix trick
    func operator/(_rhs: ConstraintValidPrefixOrPostfix) -> AntiScalar {
        return this.constraint_valid();
    }
    func operator/=(const ConstraintValidPrefixOrPostfix& _rhs) -> AntiScalar& {
        this = this.constraint_valid();
        return *this;
    }
}
extension AntiScalar: ConstraintValid {
    associatedtype Output = AntiScalar;
    func constraint_valid() -> Output {
        return this;
    }
}
extension Horizon {
    // Fancy infix trick
    func operator/(_rhs: ConstraintValidPrefixOrPostfix) -> Horizon {
        return this.constraint_valid();
    }
    func operator/=(const ConstraintValidPrefixOrPostfix& _rhs) -> Horizon& {
        this = this.constraint_valid();
        return *this;
    }
}
extension Horizon: ConstraintValid {
    associatedtype Output = Horizon;
    func constraint_valid() -> Output {
        return this;
    }
}
extension Origin {
    // Fancy infix trick
    func operator/(_rhs: ConstraintValidPrefixOrPostfix) -> Origin {
        return this.constraint_valid();
    }
    func operator/=(const ConstraintValidPrefixOrPostfix& _rhs) -> Origin& {
        this = this.constraint_valid();
        return *this;
    }
}
extension Origin: ConstraintValid {
    associatedtype Output = Origin;
    func constraint_valid() -> Output {
        return this;
    }
}
extension Plane {
    // Fancy infix trick
    func operator/(_rhs: ConstraintValidPrefixOrPostfix) -> Plane {
        return this.constraint_valid();
    }
    func operator/=(const ConstraintValidPrefixOrPostfix& _rhs) -> Plane& {
        this = this.constraint_valid();
        return *this;
    }
}
extension Plane: ConstraintValid {
    associatedtype Output = Plane;
    func constraint_valid() -> Output {
        return this;
    }
}
extension Point {
    // Fancy infix trick
    func operator/(_rhs: ConstraintValidPrefixOrPostfix) -> Point {
        return this.constraint_valid();
    }
    func operator/=(const ConstraintValidPrefixOrPostfix& _rhs) -> Point& {
        this = this.constraint_valid();
        return *this;
    }
}
extension Point: ConstraintValid {
    associatedtype Output = Point;
    func constraint_valid() -> Output {
        return this;
    }
}
extension Scalar {
    // Fancy infix trick
    func operator/(_rhs: ConstraintValidPrefixOrPostfix) -> Scalar {
        return this.constraint_valid();
    }
    func operator/=(const ConstraintValidPrefixOrPostfix& _rhs) -> Scalar& {
        this = this.constraint_valid();
        return *this;
    }
}
extension Scalar: ConstraintValid {
    associatedtype Output = Scalar;
    func constraint_valid() -> Output {
        return this;
    }
}
