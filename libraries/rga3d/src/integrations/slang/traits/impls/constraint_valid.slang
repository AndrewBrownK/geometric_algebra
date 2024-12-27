extension AntiScalar: ConstraintValid {
    associatedtype Output = AntiScalar;
    func constraint_valid() -> Output {
        return this;
    }
    func operator/(_rhs: ConstraintValidInfix) -> AntiScalar {
        return this.constraint_valid();
    }
    func operator/=(const ConstraintValidInfix& _rhs) -> AntiScalar& {
        this = this.constraint_valid();
        return *this;
    }
}
extension Horizon: ConstraintValid {
    associatedtype Output = Horizon;
    func constraint_valid() -> Output {
        return this;
    }
    func operator/(_rhs: ConstraintValidInfix) -> Horizon {
        return this.constraint_valid();
    }
    func operator/=(const ConstraintValidInfix& _rhs) -> Horizon& {
        this = this.constraint_valid();
        return *this;
    }
}
extension Origin: ConstraintValid {
    associatedtype Output = Origin;
    func constraint_valid() -> Output {
        return this;
    }
    func operator/(_rhs: ConstraintValidInfix) -> Origin {
        return this.constraint_valid();
    }
    func operator/=(const ConstraintValidInfix& _rhs) -> Origin& {
        this = this.constraint_valid();
        return *this;
    }
}
extension Plane: ConstraintValid {
    associatedtype Output = Plane;
    func constraint_valid() -> Output {
        return this;
    }
    func operator/(_rhs: ConstraintValidInfix) -> Plane {
        return this.constraint_valid();
    }
    func operator/=(const ConstraintValidInfix& _rhs) -> Plane& {
        this = this.constraint_valid();
        return *this;
    }
}
extension Point: ConstraintValid {
    associatedtype Output = Point;
    func constraint_valid() -> Output {
        return this;
    }
    func operator/(_rhs: ConstraintValidInfix) -> Point {
        return this.constraint_valid();
    }
    func operator/=(const ConstraintValidInfix& _rhs) -> Point& {
        this = this.constraint_valid();
        return *this;
    }
}
extension Scalar: ConstraintValid {
    associatedtype Output = Scalar;
    func constraint_valid() -> Output {
        return this;
    }
    func operator/(_rhs: ConstraintValidInfix) -> Scalar {
        return this.constraint_valid();
    }
    func operator/=(const ConstraintValidInfix& _rhs) -> Scalar& {
        this = this.constraint_valid();
        return *this;
    }
}
