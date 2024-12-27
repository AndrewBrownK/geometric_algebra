extension AntiScalar {
    // Fancy infix trick
    func operator/(_rhs: AntiConstraintValidPrefixOrPostfix) -> AntiScalar {
        return this.anti_constraint_valid();
    }
    func operator/=(const AntiConstraintValidPrefixOrPostfix& _rhs) -> AntiScalar& {
        this = this.anti_constraint_valid();
        return *this;
    }
}
extension AntiScalar: AntiConstraintValid {
    associatedtype Output = AntiScalar;
    func anti_constraint_valid() -> Output {
        return this;
    }
}
extension Horizon {
    // Fancy infix trick
    func operator/(_rhs: AntiConstraintValidPrefixOrPostfix) -> Horizon {
        return this.anti_constraint_valid();
    }
    func operator/=(const AntiConstraintValidPrefixOrPostfix& _rhs) -> Horizon& {
        this = this.anti_constraint_valid();
        return *this;
    }
}
extension Horizon: AntiConstraintValid {
    associatedtype Output = Horizon;
    func anti_constraint_valid() -> Output {
        return this;
    }
}
extension Origin {
    // Fancy infix trick
    func operator/(_rhs: AntiConstraintValidPrefixOrPostfix) -> Origin {
        return this.anti_constraint_valid();
    }
    func operator/=(const AntiConstraintValidPrefixOrPostfix& _rhs) -> Origin& {
        this = this.anti_constraint_valid();
        return *this;
    }
}
extension Origin: AntiConstraintValid {
    associatedtype Output = Origin;
    func anti_constraint_valid() -> Output {
        return this;
    }
}
extension Plane {
    // Fancy infix trick
    func operator/(_rhs: AntiConstraintValidPrefixOrPostfix) -> Plane {
        return this.anti_constraint_valid();
    }
    func operator/=(const AntiConstraintValidPrefixOrPostfix& _rhs) -> Plane& {
        this = this.anti_constraint_valid();
        return *this;
    }
}
extension Plane: AntiConstraintValid {
    associatedtype Output = Plane;
    func anti_constraint_valid() -> Output {
        return this;
    }
}
extension Point {
    // Fancy infix trick
    func operator/(_rhs: AntiConstraintValidPrefixOrPostfix) -> Point {
        return this.anti_constraint_valid();
    }
    func operator/=(const AntiConstraintValidPrefixOrPostfix& _rhs) -> Point& {
        this = this.anti_constraint_valid();
        return *this;
    }
}
extension Point: AntiConstraintValid {
    associatedtype Output = Point;
    func anti_constraint_valid() -> Output {
        return this;
    }
}
extension Scalar {
    // Fancy infix trick
    func operator/(_rhs: AntiConstraintValidPrefixOrPostfix) -> Scalar {
        return this.anti_constraint_valid();
    }
    func operator/=(const AntiConstraintValidPrefixOrPostfix& _rhs) -> Scalar& {
        this = this.anti_constraint_valid();
        return *this;
    }
}
extension Scalar: AntiConstraintValid {
    associatedtype Output = Scalar;
    func anti_constraint_valid() -> Output {
        return this;
    }
}
