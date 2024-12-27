extension AntiScalar: AntiConstraintValid {
    associatedtype Output = AntiScalar;
    func anti_constraint_valid() -> Output {
        return this;
    }
    func operator/(_rhs: AntiConstraintValidInfix) -> AntiScalar {
        return this.anti_constraint_valid();
    }
    func operator/=(const AntiConstraintValidInfix& _rhs) -> AntiScalar& {
        this = this.anti_constraint_valid();
        return *this;
    }
}
extension Horizon: AntiConstraintValid {
    associatedtype Output = Horizon;
    func anti_constraint_valid() -> Output {
        return this;
    }
    func operator/(_rhs: AntiConstraintValidInfix) -> Horizon {
        return this.anti_constraint_valid();
    }
    func operator/=(const AntiConstraintValidInfix& _rhs) -> Horizon& {
        this = this.anti_constraint_valid();
        return *this;
    }
}
extension Origin: AntiConstraintValid {
    associatedtype Output = Origin;
    func anti_constraint_valid() -> Output {
        return this;
    }
    func operator/(_rhs: AntiConstraintValidInfix) -> Origin {
        return this.anti_constraint_valid();
    }
    func operator/=(const AntiConstraintValidInfix& _rhs) -> Origin& {
        this = this.anti_constraint_valid();
        return *this;
    }
}
extension Plane: AntiConstraintValid {
    associatedtype Output = Plane;
    func anti_constraint_valid() -> Output {
        return this;
    }
    func operator/(_rhs: AntiConstraintValidInfix) -> Plane {
        return this.anti_constraint_valid();
    }
    func operator/=(const AntiConstraintValidInfix& _rhs) -> Plane& {
        this = this.anti_constraint_valid();
        return *this;
    }
}
extension Point: AntiConstraintValid {
    associatedtype Output = Point;
    func anti_constraint_valid() -> Output {
        return this;
    }
    func operator/(_rhs: AntiConstraintValidInfix) -> Point {
        return this.anti_constraint_valid();
    }
    func operator/=(const AntiConstraintValidInfix& _rhs) -> Point& {
        this = this.anti_constraint_valid();
        return *this;
    }
}
extension Scalar: AntiConstraintValid {
    associatedtype Output = Scalar;
    func anti_constraint_valid() -> Output {
        return this;
    }
    func operator/(_rhs: AntiConstraintValidInfix) -> Scalar {
        return this.anti_constraint_valid();
    }
    func operator/=(const AntiConstraintValidInfix& _rhs) -> Scalar& {
        this = this.anti_constraint_valid();
        return *this;
    }
}
