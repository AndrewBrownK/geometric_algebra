extension AntiFlatPoint {
    // Fancy infix trick
    func operator/(_rhs: AntiConstraintValidPrefixOrPostfix) -> AntiFlatPoint {
        return this.anti_constraint_valid();
    }
    func operator/=(const AntiConstraintValidPrefixOrPostfix& _rhs) -> AntiFlatPoint& {
        this = this.anti_constraint_valid();
        return *this;
    }
}
extension AntiFlatPoint: AntiConstraintValid {
    associatedtype Output = AntiFlatPoint;
    func anti_constraint_valid() -> Output {
        return this;
    }
}
extension AntiPlane {
    // Fancy infix trick
    func operator/(_rhs: AntiConstraintValidPrefixOrPostfix) -> AntiPlane {
        return this.anti_constraint_valid();
    }
    func operator/=(const AntiConstraintValidPrefixOrPostfix& _rhs) -> AntiPlane& {
        this = this.anti_constraint_valid();
        return *this;
    }
}
extension AntiPlane: AntiConstraintValid {
    associatedtype Output = AntiPlane;
    func anti_constraint_valid() -> Output {
        return this;
    }
}
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
extension FlatPoint {
    // Fancy infix trick
    func operator/(_rhs: AntiConstraintValidPrefixOrPostfix) -> FlatPoint {
        return this.anti_constraint_valid();
    }
    func operator/=(const AntiConstraintValidPrefixOrPostfix& _rhs) -> FlatPoint& {
        this = this.anti_constraint_valid();
        return *this;
    }
}
extension FlatPoint: AntiConstraintValid {
    associatedtype Output = FlatPoint;
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
extension RoundPoint {
    // Fancy infix trick
    func operator/(_rhs: AntiConstraintValidPrefixOrPostfix) -> RoundPoint {
        return this.anti_constraint_valid();
    }
    func operator/=(const AntiConstraintValidPrefixOrPostfix& _rhs) -> RoundPoint& {
        this = this.anti_constraint_valid();
        return *this;
    }
}
extension RoundPoint: AntiConstraintValid {
    associatedtype Output = RoundPoint;
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
extension Sphere {
    // Fancy infix trick
    func operator/(_rhs: AntiConstraintValidPrefixOrPostfix) -> Sphere {
        return this.anti_constraint_valid();
    }
    func operator/=(const AntiConstraintValidPrefixOrPostfix& _rhs) -> Sphere& {
        this = this.anti_constraint_valid();
        return *this;
    }
}
extension Sphere: AntiConstraintValid {
    associatedtype Output = Sphere;
    func anti_constraint_valid() -> Output {
        return this;
    }
}
